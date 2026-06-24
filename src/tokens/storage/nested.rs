// src/tokens/storage/nested.rs
//
// Type-safe nested storage with dot-notation key resolution, LRU cache,
// and serde serialization. All reads hit the cache first, then fall back
// to the active backend (localStorage on WASM, MemoryStore on SSR).
//
// Keys use dot-notation for nested access: "demo.messages.0.text"
// reads the `text` field of the first item in the `messages` array
// stored under the root key `demo`.

use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use super::backends::LocalStore;
#[cfg(not(target_arch = "wasm32"))]
use super::backends::MemoryStore;
use super::entry::notify;

// ── Cache entry with TTL ─────────────────────────────────────────────────────

struct CacheEntry {
    value: Arc<str>,
    expires_at: Option<Instant>,
}

impl CacheEntry {
    fn new(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
            expires_at: None,
        }
    }
    fn _with_ttl(value: impl Into<Arc<str>>, ttl: Duration) -> Self {
        Self {
            value: value.into(),
            expires_at: Some(Instant::now() + ttl),
        }
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
        Self {
            cache: DashMap::new(),
        }
    }
    pub fn global() -> &'static Self {
        &NESTED_STORE
    }

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
        self.cache
            .insert(key.to_string(), CacheEntry::new(value.as_ref()));
    }

    fn cache_remove(&self, key: &str) {
        self.cache.remove(key);
    }

    fn backend_get(&self, key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::get(key)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    fn backend_set(&self, key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::set(key, value);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().set(key, value);
        }
        notify(key, value);
    }

    fn backend_delete(&self, key: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::delete(key);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
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
        if parts.is_empty() {
            return None;
        }

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
        if parts.is_empty() {
            return;
        }

        let root = parts[0];
        let rest = &parts[1..];

        let mut current: Value = self
            .get_raw(root)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(Value::Object(serde_json::Map::new()));

        let mut cursor = &mut current;
        for (i, part) in rest.iter().enumerate() {
            let is_last = i == rest.len() - 1;
            if is_last {
                // Parse the new value to JSON, or store as string if invalid JSON
                let new_val = serde_json::from_str(value)
                    .unwrap_or_else(|_| Value::String(value.to_string()));
                match cursor {
                    Value::Object(map) => {
                        map.insert((*part).to_string(), new_val);
                    }
                    Value::Array(arr) => {
                        let idx: usize = part.parse().unwrap_or(arr.len());
                        if idx < arr.len() {
                            arr[idx] = new_val;
                        } else {
                            arr.push(new_val);
                        }
                    }
                    _ => {}
                }
                break;
            }
            // Navigate or create intermediate
            cursor = match cursor {
                Value::Object(map) => map
                    .entry((*part).to_string())
                    .or_insert_with(|| Value::Object(serde_json::Map::new())),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_store() -> NestedStore {
        NestedStore::new()
    }

    #[test]
    fn set_raw_and_get_raw_flat_key() {
        let store = fresh_store();
        store.set_raw("flat", "value");
        assert_eq!(store.get_raw("flat").unwrap(), "value");
    }

    #[test]
    fn get_raw_missing_returns_none() {
        let store = fresh_store();
        assert!(store.get_raw("missing").is_none());
    }

    #[test]
    fn delete_removes_key() {
        let store = fresh_store();
        store.set_raw("del_key", "x");
        store.delete("del_key");
        assert!(store.get_raw("del_key").is_none());
    }

    #[test]
    fn nested_get_object_field() {
        let store = fresh_store();
        store.set_raw("user", r#"{"name":"Alice","age":30}"#);
        let name = store.get_raw("user.name").unwrap();
        assert_eq!(name, "\"Alice\"");
    }

    #[test]
    fn nested_get_array_index() {
        let store = fresh_store();
        store.set_raw("items", r#"["a","b","c"]"#);
        let item = store.get_raw("items.1").unwrap();
        assert_eq!(item, "\"b\"");
    }

    #[test]
    fn nested_set_creates_path() {
        let store = fresh_store();
        store.set_raw("config.theme", "dark");
        let root = store.get_raw("config").unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&root).unwrap();
        assert_eq!(parsed["theme"], "dark");
    }

    #[test]
    fn nested_set_updates_existing_field() {
        let store = fresh_store();
        store.set_raw("obj", r#"{"a":1,"b":2}"#);
        store.set_raw("obj.a", "99");
        let root = store.get_raw("obj").unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&root).unwrap();
        assert_eq!(parsed["a"], 99);
    }

    #[test]
    fn set_typed_and_get_typed() {
        let store = fresh_store();
        store.set_typed("num", &42i32);
        let val: i32 = store.get_typed("num").unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn set_typed_struct() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct User {
            name: String,
            age: u32,
        }

        let store = fresh_store();
        let user = User {
            name: "Bob".into(),
            age: 25,
        };
        store.set_typed("user_obj", &user);
        let loaded: User = store.get_typed("user_obj").unwrap();
        assert_eq!(loaded, user);
    }

    #[test]
    fn preload_json() {
        let store = fresh_store();
        store.preload_json("data", r#"{"items":[1,2,3]}"#);
        let val = store.get_raw("data.items.0").unwrap();
        assert_eq!(val, "1");
    }

    #[test]
    fn cache_hit_after_set() {
        let store = fresh_store();
        store.set_raw("cached", "hello");
        // Second read should come from cache
        assert_eq!(store.get_raw("cached").unwrap(), "hello");
    }

    #[test]
    fn nested_deep_path() {
        let store = fresh_store();
        store.set_raw("a.b.c", "deep");
        let root = store.get_raw("a").unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&root).unwrap();
        assert_eq!(parsed["b"]["c"], "deep");
    }
}
