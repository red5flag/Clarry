// src/tokens/storage/list.rs
//
// INVARIANT: All `rayon` usage is strictly behind `#[cfg(not(target_arch = "wasm32"))]`.
// The WASM fallback path is sequential and must never panic or deadlock.
//
// ListStore — typed list storage with caching and persistence.

use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::Value;
#[cfg(target_arch = "wasm32")]
use super::backends::LocalStore;

static LIST_STORE: Lazy<ListStore> = Lazy::new(ListStore::new);

pub struct ListStore {
    lists: DashMap<String, Vec<Value>>,
}

impl Default for ListStore {
    fn default() -> Self { Self::new() }
}

impl ListStore {
    pub fn new() -> Self { Self { lists: DashMap::new() } }
    pub fn global() -> &'static Self { &LIST_STORE }

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
        #[cfg(target_arch = "wasm32")] {
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
        #[cfg(target_arch = "wasm32")] {
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