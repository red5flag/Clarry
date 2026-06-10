// src/tokens/storage/nested.rs
//
// Type-safe nested storage with dot-notation key resolution, LRU cache,
// and serde serialization. All reads hit the cache first, then fall back
// to the active backend (localStorage on WASM, MemoryStore on SSR).
//
// Keys use dot-notation for nested access: "demo.messages.0.text"
// reads the `text` field of the first item in the `messages` array
// stored under the root key `demo`.

use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::backends::{LocalStore, MemoryStore};
use super::entry::notify;

// ── Cache entry with TTL ─────────────────────────────────────────────────────

struct CacheEntry {
    value: Arc<str>,
    expires_at: Option<Instant>,
}

impl CacheEntry {
    fn new(value: impl Into<Arc<str>>) -> Self {
        Self { value: value.into(), expires_at: None }
    }
    fn with_ttl(value: impl Into<Arc<str>>, ttl: Duration) -> Self {
        Self { value: value.into(), expires_at: Some(Instant::now() + ttl) }
    }
    fn is_expired(&self) -> bool {
        self.expires_at.map(|t| Instant::now() > t).unwrap_or(false)
    }
}

// ── NestedStore ──────────────────────────────────────────────────────────────

static NESTED_STORE: Lazy<NestedStore> = Lazy::new(NestedStore::new);

pub struct NestedStore {
    cache: DashMap<String, CacheEntry>,
}

impl NestedStore {
    pub fn new() -> Self {
        Self { cache: DashMap::new() }
    }
    pub fn global() -> &'static Self { &NESTED_STORE }

    // ── Low-level raw operations ─────────────────────────────────────

    fn cache_get(&self, key: &str) -> Option<Arc<str>> {
        let entry = self.cache.get(key)?;
        if entry.is_expired() {
            drop(entry);
            self.cache.remove(key);
            return None;
        }
        Some(Arc::clone(&entry.value))
    }

    fn cache_set(&self, key: &str, value: impl AsRef<str>) {
        self.cache.insert(key.to_string(), CacheEntry::new(value.as_ref()));
    }

    fn cache_remove(&self, key: &str) {
        self.cache.remove(key);
    }

    fn backend_get(&self, key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")] {
            LocalStore::get(key)
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    fn backend_set(&self, key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")] {
            LocalStore::set(key, value);
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().set(key, value);
        }
        notify(key, value);
    }

    fn backend_delete(&self, key: &str) {
        #[cfg(target_arch = "wasm32")] {
            LocalStore::delete(key);
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().delete(key);
        }
    }

    // ── Typed get / set / delete ─────────────────────────────────────

    /// Store a serializable value. The key may be dot-notation.
    pub fn set_typed<T: Serialize>(&self, key: &str, value: &T) {
        if let Ok(json) = serde_json::to_string(value) {
            self.set_raw(key, &json);
        }
    }

    /// Deserialize a value. Dot-notation is supported.
    pub fn get_typed<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let raw = self.get_raw(key)?;
        serde_json::from_str(&raw).ok()
    }

    /// Store a raw string. Dot-notation triggers nested resolution.
    pub fn set_raw(&self, key: &str, value: &str) {
        if key.contains('.') {
            self.set_nested(key, value);
        } else {
            self.cache_set(key, value);
            self.backend_set(key, value);
        }
    }

    /// Retrieve a raw string. Dot-notation triggers nested traversal.
    pub fn get_raw(&self, key: &str) -> Option<String> {
        if key.contains('.') {
            self.get_nested(key)
        } else {
            // Check cache first
            if let Some(cached) = self.cache_get(key) {
                return Some(cached.to_string());
            }
            // Fall back to backend
            if let Some(val) = self.backend_get(key) {
                self.cache_set(key, &val);
                return Some(val);
            }
            None
        }
    }

    /// Delete a key (and its cached copy).
    pub fn delete(&self, key: &str) {
        self.cache_remove(key);
        self.backend_delete(key);
    }

    /// Pre-load a JSON object so that nested reads are fast.
    pub fn preload_json(&self, key: &str, json: &str) {
        self.cache_set(key, json);
        self.backend_set(key, json);
    }

    // ── Nested key resolution ──────────────────────────────────────────

    fn get_nested(&self, dotted: &str) -> Option<String> {
        let parts: Vec<&str> = dotted.split('.').collect();
        if parts.is_empty() { return None; }

        let root = parts[0];
        let rest = &parts[1..];

        let root_val = self.get_raw(root)?;
        let mut current: Value = serde_json::from_str(&root_val).ok()?;

        for part in rest {
            current = match current {
                Value::Object(map) => map.get(*part)?.clone(),
                Value::Array(arr) => {
                    let idx: usize = part.parse().ok()?;
                    arr.get(idx)?.clone()
                }
                _ => return None,
            };
        }

        Some(current.to_string())
    }

    fn set_nested(&self, dotted: &str, value: &str) {
        let parts: Vec<&str> = dotted.split('.').collect();
        if parts.is_empty() { return; }

        let root = parts[0];
        let rest = &parts[1..];

        let mut current: Value = self.get_raw(root)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(Value::Object(serde_json::Map::new()));

        let mut cursor = &mut current;
        for (i, part) in rest.iter().enumerate() {
            let is_last = i == rest.len() - 1;
            if is_last {
                // Parse the new value to JSON, or store as string if invalid JSON
                let new_val = serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()));
                match cursor {
                    Value::Object(map) => { map.insert((*part).to_string(), new_val); }
                    Value::Array(arr) => {
                        let idx: usize = part.parse().unwrap_or(arr.len());
                        if idx < arr.len() { arr[idx] = new_val; }
                        else { arr.push(new_val); }
                    }
                    _ => {}
                }
                break;
            }
            // Navigate or create intermediate
            cursor = match cursor {
                Value::Object(map) => {
                    map.entry((*part).to_string())
                        .or_insert_with(|| Value::Object(serde_json::Map::new()))
                }
                Value::Array(arr) => {
                    let idx: usize = part.parse().unwrap_or(arr.len());
                    if idx >= arr.len() {
                        arr.push(Value::Object(serde_json::Map::new()));
                    }
                    &mut arr[idx]
                }
                _ => {
                    // Replace scalar with object to continue path
                    *cursor = Value::Object(serde_json::Map::new());
                    if let Value::Object(map) = cursor {
                        map.entry((*part).to_string())
                            .or_insert_with(|| Value::Object(serde_json::Map::new()))
                    } else {
                        unreachable!()
                    }
                }
            };
        }

        if let Ok(json) = serde_json::to_string(&current) {
            self.cache_set(root, &json);
            self.backend_set(root, &json);
        }
    }
}

// ── Public typed helpers ─────────────────────────────────────────────────────

/// Store a serializable value under a (possibly dotted) key.
pub fn nested_store_json<T: Serialize>(key: &str, value: &T) {
    NestedStore::global().set_typed(key, value);
}

/// Load and deserialize a value from a (possibly dotted) key.
pub fn nested_load_json<T: DeserializeOwned>(key: &str) -> Option<T> {
    NestedStore::global().get_typed(key)
}

/// Store a raw string.
pub fn store_raw(key: &str, value: &str) {
    NestedStore::global().set_raw(key, value);
}

/// Load a raw string.
pub fn load_raw(key: &str) -> Option<String> {
    NestedStore::global().get_raw(key)
}

/// Delete a key.
pub fn delete_raw(key: &str) {
    NestedStore::global().delete(key);
}
