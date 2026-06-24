// src/tokens/storage/list.rs
//
// INVARIANT: All `rayon` usage is strictly behind `#[cfg(not(target_arch = "wasm32"))]`.
// The WASM fallback path is sequential and must never panic or deadlock.
//
// ListStore — typed list storage with caching and persistence.

#[cfg(target_arch = "wasm32")]
use super::backends::LocalStore;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::Value;

static LIST_STORE: Lazy<ListStore> = Lazy::new(ListStore::new);

pub struct ListStore {
    lists: DashMap<String, Vec<Value>>,
}

impl Default for ListStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ListStore {
    pub fn new() -> Self {
        Self {
            lists: DashMap::new(),
        }
    }
    pub fn global() -> &'static Self {
        &LIST_STORE
    }

    pub fn get_list(&self, key: &str) -> Vec<Value> {
        #[cfg(target_arch = "wasm32")]
        self.maybe_load_from_local_storage();
        self.lists.get(key).map(|e| e.clone()).unwrap_or_default()
    }

    /// Auto-load all persisted lists on first access (WASM only).
    #[cfg(target_arch = "wasm32")]
    fn maybe_load_from_local_storage(&self) {
        use std::sync::OnceLock;
        static LOADED: OnceLock<bool> = OnceLock::new();
        if LOADED.get().is_none() {
            let _ = LOADED.set(true);
            self.load_from_local_storage();
        }
    }

    pub fn has_list(&self, key: &str) -> bool {
        self.lists.contains_key(key)
    }

    pub fn set_list(&self, key: &str, items: Vec<Value>) {
        self.lists.insert(key.to_string(), items.clone());
        // Persist to localStorage on WASM
        #[cfg(target_arch = "wasm32")]
        {
            if let Ok(json) = serde_json::to_string(&items) {
                LocalStore::set(&format!("_list_{}", key), &json);
            }
        }
    }

    pub fn append(&self, key: &str, item: Value) {
        let mut list = self.get_list(key);
        list.push(item);
        self.set_list(key, list);
    }

    pub fn clear(&self, key: &str) {
        self.lists.remove(key);
        #[cfg(target_arch = "wasm32")]
        {
            LocalStore::delete(&format!("_list_{}", key));
        }
    }

    /// Bulk load from an iterator. On native, insertion runs in parallel via rayon.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bulk_load(&self, pairs: impl IntoIterator<Item = (String, Vec<Value>)>) {
        use rayon::prelude::*;
        let pairs: Vec<_> = pairs.into_iter().collect();
        pairs.into_par_iter().for_each(|(k, v)| {
            LIST_STORE.lists.insert(k, v);
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn bulk_load(&self, pairs: impl IntoIterator<Item = (String, Vec<Value>)>) {
        for (k, v) in pairs {
            self.lists.insert(k, v);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn len(&self) -> usize {
        self.lists.len()
    }

    /// Load all persisted lists from localStorage (WASM only).
    #[cfg(target_arch = "wasm32")]
    pub fn load_from_local_storage(&self) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let len = storage.length().unwrap_or(0);
                for i in 0..len {
                    if let Ok(Some(key)) = storage.key(i) {
                        if key.starts_with("_list_") {
                            let list_key = key.trim_start_matches("_list_");
                            if self.lists.contains_key(list_key) {
                                continue;
                            }
                            if let Ok(Some(raw)) = storage.get_item(&key) {
                                if let Ok(items) = serde_json::from_str::<Vec<Value>>(&raw) {
                                    self.lists.insert(list_key.to_string(), items);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fresh_list_store() -> ListStore {
        ListStore::new()
    }

    #[test]
    fn new_list_store_is_empty() {
        let store = fresh_list_store();
        assert!(!store.has_list("anything"));
        assert!(store.get_list("anything").is_empty());
    }

    #[test]
    fn default_trait() {
        let store = ListStore::default();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn set_and_get_list() {
        let store = fresh_list_store();
        store.set_list("colors", vec![json!("red"), json!("blue")]);
        let list = store.get_list("colors");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], json!("red"));
    }

    #[test]
    fn has_list_after_set() {
        let store = fresh_list_store();
        assert!(!store.has_list("items"));
        store.set_list("items", vec![json!(1)]);
        assert!(store.has_list("items"));
    }

    #[test]
    fn append_to_list() {
        let store = fresh_list_store();
        store.set_list("nums", vec![json!(1)]);
        store.append("nums", json!(2));
        let list = store.get_list("nums");
        assert_eq!(list.len(), 2);
        assert_eq!(list[1], json!(2));
    }

    #[test]
    fn append_to_nonexistent_list() {
        let store = fresh_list_store();
        store.append("new_list", json!("first"));
        let list = store.get_list("new_list");
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn clear_list() {
        let store = fresh_list_store();
        store.set_list("clearme", vec![json!(1), json!(2)]);
        store.clear("clearme");
        assert!(!store.has_list("clearme"));
        assert!(store.get_list("clearme").is_empty());
    }

    #[test]
    fn bulk_load() {
        let store = ListStore::global();
        let pairs = vec![
            ("bl_a".to_string(), vec![json!(1), json!(2)]),
            ("bl_b".to_string(), vec![json!(3)]),
        ];
        store.bulk_load(pairs);
        assert_eq!(store.get_list("bl_a").len(), 2);
        assert_eq!(store.get_list("bl_b").len(), 1);
    }
}
