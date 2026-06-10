// src/tokens/storage/primitive.rs
//
// Simple read/write storage primitive with dot-notation path support.
// Replaces the heavier NestedStore for common use-cases.
//
// Examples:
//   Store::read("alice.message.0")      -> read first message
//   Store::write("user.message.#", "hi") -> append to messages array
//   Store::write("user.name", "Bob")     -> set a scalar

use serde_json::Value;
use super::backends::{LocalStore, MemoryStore};

pub struct Store;

impl Store {
    /// Read a value at a dot-notation path.
    /// `alice.message.1`  → load key "alice", traverse {"message": […]}, index 1
    pub fn read(path: &str) -> Option<String> {
        let (root, rest) = split_root(path);
        let raw = Self::backend_get(root)?;
        if rest.is_empty() {
            return Some(raw);
        }
        let mut val: Value = serde_json::from_str(&raw).ok()?;
        for seg in rest.split('.') {
            val = navigate(&val, seg)?;
        }
        Some(val_to_string(&val))
    }

    /// Write a value at a dot-notation path.
    /// `user.message.#` appends to the array under key "user" → message.
    /// `user.name` sets a scalar.
    pub fn write(path: &str, value: &str) {
        let (root, rest) = split_root(path);
        if rest.is_empty() {
            Self::backend_set(root, value);
            return;
        }
        let mut root_val: Value = Self::backend_get(root)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let segments: Vec<&str> = rest.split('.').collect();
        write_path(&mut root_val, &segments, value);
        if let Ok(json) = serde_json::to_string(&root_val) {
            Self::backend_set(root, &json);
        }
    }

    /// Same as write but for JSON-serializable values.
    pub fn write_json<T: serde::Serialize>(path: &str, value: &T) {
        if let Ok(json) = serde_json::to_string(value) {
            Self::write(path, &json);
        }
    }

    /// Delete the entire root key.
    pub fn delete_root(key: &str) {
        #[cfg(target_arch = "wasm32")] {
            LocalStore::delete(key);
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().delete(key);
        }
    }

    // ── Backend helpers ──────────────────────────────────────────────────────────

    fn backend_get(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")] {
            LocalStore::get(key)
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    fn backend_set(key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")] {
            LocalStore::set(key, value);
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().set(key, value);
        }
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn split_root(path: &str) -> (&str, &str) {
    match path.find('.') {
        Some(pos) => (&path[..pos], &path[pos + 1..]),
        None => (path, ""),
    }
}

fn navigate(val: &Value, seg: &str) -> Option<Value> {
    if let Ok(idx) = seg.parse::<usize>() {
        val.as_array()?.get(idx).cloned()
    } else {
        val.get(seg).cloned()
    }
}

fn val_to_string(val: &Value) -> String {
    match val {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Recursively walk `segments` and write `value` at the leaf.
/// `#` as a leaf segment means "append to array".
fn write_path(val: &mut Value, segments: &[&str], value: &str) {
    if segments.is_empty() {
        return;
    }
    let seg = segments[0];
    if segments.len() == 1 {
        // Leaf
        if seg == "#" {
            if let Some(arr) = val.as_array_mut() {
                arr.push(json_str(value));
            }
            return;
        }
        if let Ok(idx) = seg.parse::<usize>() {
            if let Some(arr) = val.as_array_mut() {
                if arr.len() <= idx {
                    arr.resize_with(idx + 1, || Value::Null);
                }
                arr[idx] = json_str(value);
            }
            return;
        }
        if let Some(obj) = val.as_object_mut() {
            obj.insert(seg.to_string(), json_str(value));
        }
        return;
    }
    // Intermediate
    if let Ok(idx) = seg.parse::<usize>() {
        if let Some(arr) = val.as_array_mut() {
            if arr.len() <= idx {
                arr.resize_with(idx + 1, || Value::Object(serde_json::Map::new()));
            }
            write_path(&mut arr[idx], &segments[1..], value);
        }
        return;
    }
    if let Some(obj) = val.as_object_mut() {
        if !obj.contains_key(seg) {
            // Peek next segment to decide array vs object
            let next = segments[1];
            if next.parse::<usize>().is_ok() || next == "#" {
                obj.insert(seg.to_string(), Value::Array(vec![]));
            } else {
                obj.insert(seg.to_string(), Value::Object(serde_json::Map::new()));
            }
        }
        let child = obj.get_mut(seg).unwrap();
        write_path(child, &segments[1..], value);
    }
}

fn json_str(s: &str) -> Value {
    // If it looks like JSON, parse it; otherwise store as string
    serde_json::from_str(s).unwrap_or_else(|_| Value::String(s.to_string()))
}
