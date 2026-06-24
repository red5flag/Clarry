// src/tokens/storage/backends.rs
//
// Storage backends: MemoryStore, LocalStore, SessionStore, RedisStore.

use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use super::entry::{StoreEntry, notify};

// ── MemoryStore ───────────────────────────────────────────────────────────────

static MEMORY_STORE: Lazy<MemoryStore> = Lazy::new(MemoryStore::new);

pub struct MemoryStore {
    map: DashMap<String, StoreEntry>,
}

impl MemoryStore {
    fn new() -> Self { Self { map: DashMap::new() } }

    pub fn global() -> &'static Self { &MEMORY_STORE }

    pub fn set(&self, key: &str, value: impl Into<Arc<str>>) {
        let entry = StoreEntry::new(value);
        notify(key, &entry.value);
        self.map.insert(key.to_string(), entry);
    }

    pub fn set_ttl(&self, key: &str, value: impl Into<Arc<str>>, ttl: Duration) {
        let entry = StoreEntry::with_ttl(value, ttl);
        notify(key, &entry.value);
        self.map.insert(key.to_string(), entry);
    }

    pub fn get(&self, key: &str) -> Option<Arc<str>> {
        let entry = self.map.get(key)?;
        if entry.is_expired() {
            drop(entry);
            self.map.remove(key);
            return None;
        }
        Some(Arc::clone(&entry.value))
    }

    pub fn delete(&self, key: &str) { self.map.remove(key); }

    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.map.iter()
            .filter(|e| e.key().starts_with(prefix) && !e.value().is_expired())
            .map(|e| e.key().clone())
            .collect()
    }

    /// Bulk-load from an iterator. On native, insertion runs in parallel via rayon.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bulk_load(&self, pairs: impl IntoIterator<Item = (String, String)>) {
        use rayon::prelude::*;
        let pairs: Vec<_> = pairs.into_iter().collect();
        pairs.into_par_iter().for_each(|(k, v)| {
            let entry = StoreEntry::new(v.as_str());
            MEMORY_STORE.map.insert(k, entry);
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn bulk_load(&self, pairs: impl IntoIterator<Item = (String, String)>) {
        for (k, v) in pairs {
            self.map.insert(k, StoreEntry::new(v.as_str()));
        }
    }

    pub fn len(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn clear(&self) { self.map.clear(); }
}

// ── LocalStore (WASM localStorage) ───────────────────────────────────────────

pub struct LocalStore;
pub struct SessionStore;

impl LocalStore {
    pub fn set(key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")] {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.local_storage() {
                    if let Some(storage) = storage {
                        if let Err(e) = storage.set_item(key, value) {
                            leptos::logging::warn!("localStorage.setItem({}) failed: {:?}", key, e);
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))] {
            // SSR: fall through to memory
            MemoryStore::global().set(key, value);
        }
        notify(key, value);
    }

    pub fn get(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")] {
            web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item(key).ok())
                .flatten()
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    pub fn delete(key: &str) {
        #[cfg(target_arch = "wasm32")] {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.local_storage() {
                    if let Some(storage) = storage {
                        if let Err(e) = storage.delete(key) {
                            leptos::logging::warn!("localStorage.delete({}) failed: {:?}", key, e);
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().delete(key);
        }
    }
}

impl SessionStore {
    pub fn set(key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")] {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.session_storage() {
                    if let Some(storage) = storage {
                        if let Err(e) = storage.set_item(key, value) {
                            leptos::logging::warn!("sessionStorage.setItem({}) failed: {:?}", key, e);
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().set(key, value);
        }
        notify(key, value);
    }

    pub fn get(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")] {
            web_sys::window()
                .and_then(|w| w.session_storage().ok())
                .flatten()
                .and_then(|s| s.get_item(key).ok())
                .flatten()
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    pub fn delete(key: &str) {
        #[cfg(target_arch = "wasm32")] {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.session_storage() {
                    if let Some(storage) = storage {
                        if let Err(e) = storage.delete(key) {
                            leptos::logging::warn!("sessionStorage.delete({}) failed: {:?}", key, e);
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))] {
            MemoryStore::global().delete(key);
        }
    }
}
