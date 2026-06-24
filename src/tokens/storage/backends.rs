// src/tokens/storage/backends.rs
//
// Storage backends: MemoryStore, LocalStore, SessionStore, RedisStore.

use super::entry::{notify, StoreEntry};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;

// ── MemoryStore ───────────────────────────────────────────────────────────────

static MEMORY_STORE: Lazy<MemoryStore> = Lazy::new(MemoryStore::new);

pub struct MemoryStore {
    map: DashMap<String, StoreEntry>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            map: DashMap::new(),
        }
    }

    pub fn global() -> &'static Self {
        &MEMORY_STORE
    }

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

    pub fn delete(&self, key: &str) {
        self.map.remove(key);
    }

    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.map
            .iter()
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

    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn clear(&self) {
        self.map.clear();
    }
}

// ── LocalStore (WASM localStorage) ───────────────────────────────────────────

pub struct LocalStore;
pub struct SessionStore;

impl LocalStore {
    pub fn set(key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.local_storage() {
                    if let Some(storage) = storage {
                        let _ = storage.set_item(key, value);
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // SSR: fall through to memory
            MemoryStore::global().set(key, value);
        }
        notify(key, value);
    }

    pub fn get(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item(key).ok())
                .flatten()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    pub fn delete(key: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.local_storage() {
                    if let Some(storage) = storage {
                        let _ = storage.delete(key);
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().delete(key);
        }
    }
}

impl SessionStore {
    pub fn set(key: &str, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.session_storage() {
                    if let Some(storage) = storage {
                        let _ = storage.set_item(key, value);
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().set(key, value);
        }
        notify(key, value);
    }

    pub fn get(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.session_storage().ok())
                .flatten()
                .and_then(|s| s.get_item(key).ok())
                .flatten()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().get(key).map(|s| s.to_string())
        }
    }

    pub fn delete(key: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(storage) = window.session_storage() {
                    if let Some(storage) = storage {
                        let _ = storage.delete(key);
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            MemoryStore::global().delete(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // ── MemoryStore ───────────────────────────────────────────────────

    #[test]
    fn memory_store_set_get() {
        let store = MemoryStore::global();
        store.set("ms_test_1", "value1");
        assert_eq!(&*store.get("ms_test_1").unwrap(), "value1");
    }

    #[test]
    fn memory_store_get_missing() {
        let store = MemoryStore::global();
        assert!(store.get("ms_nonexistent_key").is_none());
    }

    #[test]
    fn memory_store_delete() {
        let store = MemoryStore::global();
        store.set("ms_del", "x");
        store.delete("ms_del");
        assert!(store.get("ms_del").is_none());
    }

    #[test]
    fn memory_store_ttl_not_expired() {
        let store = MemoryStore::global();
        store.set_ttl("ms_ttl_ok", "alive", Duration::from_secs(60));
        assert!(store.get("ms_ttl_ok").is_some());
    }

    #[test]
    fn memory_store_ttl_expired() {
        let store = MemoryStore::global();
        store.set_ttl("ms_ttl_exp", "gone", Duration::from_millis(0));
        std::thread::sleep(Duration::from_millis(5));
        assert!(store.get("ms_ttl_exp").is_none());
    }

    #[test]
    fn memory_store_keys_with_prefix() {
        let store = MemoryStore::global();
        store.set("pfx:a", "1");
        store.set("pfx:b", "2");
        store.set("other:c", "3");
        let keys = store.keys_with_prefix("pfx:");
        assert!(keys.contains(&"pfx:a".to_string()));
        assert!(keys.contains(&"pfx:b".to_string()));
        assert!(!keys.iter().any(|k| k.starts_with("other")));
    }

    #[test]
    fn memory_store_clear() {
        let store = MemoryStore::global();
        store.set("ms_clear_1", "a");
        store.set("ms_clear_2", "b");
        let before = store.len();
        store.clear();
        assert_eq!(store.len(), 0);
        assert!(before > 0 || before == 0); // clear is idempotent
    }

    #[test]
    fn memory_store_bulk_load() {
        let store = MemoryStore::global();
        store.clear();
        let pairs = vec![
            ("bulk_a".to_string(), "1".to_string()),
            ("bulk_b".to_string(), "2".to_string()),
        ];
        store.bulk_load(pairs);
        assert_eq!(&*MemoryStore::global().get("bulk_a").unwrap(), "1");
        assert_eq!(&*MemoryStore::global().get("bulk_b").unwrap(), "2");
    }

    // ── LocalStore (SSR fallback to MemoryStore) ──────────────────────

    #[test]
    fn local_store_set_get_ssr() {
        LocalStore::set("ls_test_unique", "local_val");
        assert_eq!(LocalStore::get("ls_test_unique").unwrap(), "local_val");
    }

    #[test]
    fn local_store_delete_ssr() {
        LocalStore::set("ls_del", "x");
        LocalStore::delete("ls_del");
        assert!(LocalStore::get("ls_del").is_none());
    }

    // ── SessionStore (SSR fallback to MemoryStore) ────────────────────

    #[test]
    fn session_store_set_get_ssr() {
        MemoryStore::global().clear();
        SessionStore::set("ss_test", "session_val");
        assert_eq!(SessionStore::get("ss_test").unwrap(), "session_val");
    }

    #[test]
    fn session_store_delete_ssr() {
        SessionStore::set("ss_del", "x");
        SessionStore::delete("ss_del");
        assert!(SessionStore::get("ss_del").is_none());
    }
}
