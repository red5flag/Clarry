// src/tokens/storage/primitive.rs
//
// Simple read/write storage primitive with dot-notation path support.
// Replaces the heavier NestedStore for common use-cases.
//
// Examples:
//   Store::read("alice.message.0")      -> read first message
//   Store::write("user.message.#", "hi") -> append to messages array
//   Store::write("user.name", "Bob")     -> set a scalar

#[cfg(target_arch = "wasm32")]
use super::backends::LocalStore;
#[cfg(not(target_arch = "wasm32"))]
use super::backends::MemoryStore;
use serde_json::Value;

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
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::delete(key);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().delete(key);
        }
    }

    // ── Backend helpers ──────────────────────────────────────────────────────────

    fn backend_get(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::get(key)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    fn backend_set(key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::set(key, value);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn clean(key: &str) {
        Store::delete_root(key);
    }

    #[test]
    fn write_and_read_flat_key() {
        clean("prim_flat");
        Store::write("prim_flat", "hello");
        assert_eq!(Store::read("prim_flat").unwrap(), "hello");
    }

    #[test]
    fn read_missing_returns_none() {
        assert!(Store::read("prim_nonexistent_xyz").is_none());
    }

    #[test]
    fn write_and_read_nested_object() {
        clean("prim_user");
        Store::write("prim_user", r#"{"name":"Alice"}"#);
        let name = Store::read("prim_user.name").unwrap();
        assert_eq!(name, "Alice");
    }

    #[test]
    fn write_nested_creates_structure() {
        clean("prim_cfg");
        Store::write("prim_cfg.theme", "dark");
        let raw = Store::read("prim_cfg").unwrap();
        let parsed: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(parsed["theme"], "dark");
    }

    #[test]
    fn write_nested_array_append() {
        clean("prim_arr");
        Store::write("prim_arr", r#"{"items":["a"]}"#);
        Store::write("prim_arr.items.#", "b");
        let raw = Store::read("prim_arr").unwrap();
        let parsed: Value = serde_json::from_str(&raw).unwrap();
        let items = parsed["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[1], "b");
    }

    #[test]
    fn write_nested_array_index() {
        clean("prim_idx");
        Store::write("prim_idx", r#"{"vals":["x","y","z"]}"#);
        Store::write("prim_idx.vals.1", "replaced");
        let val = Store::read("prim_idx.vals.1").unwrap();
        assert_eq!(val, "replaced");
    }

    #[test]
    fn write_json_typed() {
        clean("prim_json");
        Store::write_json("prim_json", &vec![1, 2, 3]);
        let raw = Store::read("prim_json").unwrap();
        let parsed: Vec<i32> = serde_json::from_str(&raw).unwrap();
        assert_eq!(parsed, vec![1, 2, 3]);
    }

    #[test]
    fn delete_root_removes_key() {
        Store::write("prim_del", "x");
        Store::delete_root("prim_del");
        assert!(Store::read("prim_del").is_none());
    }

    #[test]
    fn split_root_with_dot() {
        let (root, rest) = split_root("a.b.c");
        assert_eq!(root, "a");
        assert_eq!(rest, "b.c");
    }

    #[test]
    fn split_root_no_dot() {
        let (root, rest) = split_root("simple");
        assert_eq!(root, "simple");
        assert_eq!(rest, "");
    }

    #[test]
    fn navigate_object_field() {
        let val: Value = serde_json::from_str(r#"{"x":42}"#).unwrap();
        let result = navigate(&val, "x").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn navigate_array_index() {
        let val: Value = serde_json::from_str(r#"[10,20,30]"#).unwrap();
        let result = navigate(&val, "1").unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    fn val_to_string_for_string() {
        let val = Value::String("hello".into());
        assert_eq!(val_to_string(&val), "hello");
    }

    #[test]
    fn val_to_string_for_number() {
        let val = Value::Number(42.into());
        assert_eq!(val_to_string(&val), "42");
    }

    #[test]
    fn json_str_parses_json() {
        let val = json_str("42");
        assert_eq!(val, Value::Number(42.into()));
    }

    #[test]
    fn json_str_falls_back_to_string() {
        let val = json_str("not json");
        assert_eq!(val, Value::String("not json".into()));
    }
}
