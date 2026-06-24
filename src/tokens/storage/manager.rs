// src/tokens/storage/manager.rs
//
// StoreManager routing and typed JSON helpers.

use super::backends::{LocalStore, MemoryStore, SessionStore};

// ── StoreManager — unified routing ───────────────────────────────────────────

/// Route a store operation to the correct backend.
/// Backend names: "memory" | "local" | "session" | "redis"
pub struct StoreManager;

impl StoreManager {
    pub fn set(backend: &str, key: &str, value: &str) {
        match backend {
            "local" => LocalStore::set(key, value),
            "session" => SessionStore::set(key, value),
            "redis" => {
                #[cfg(not(target_arch = "wasm32"))]
                leptos::logging::log!("redis::set {key} (async — use StoreManager::set_async)");
                // sync Redis not available; fall through to memory
                MemoryStore::global().set(key, value);
            }
            _ => MemoryStore::global().set(key, value),
        }
    }

    pub fn get(backend: &str, key: &str) -> Option<String> {
        match backend {
            "local" => LocalStore::get(key),
            "session" => SessionStore::get(key),
            _ => MemoryStore::global().get(key).map(|s| s.to_string()),
        }
    }

    pub fn delete(backend: &str, key: &str) {
        match backend {
            "local" => LocalStore::delete(key),
            "session" => SessionStore::delete(key),
            _ => MemoryStore::global().delete(key),
        }
    }

    /// Async Redis set (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn set_async(backend: &str, key: &str, value: &str) {
        if backend == "redis" {
            // Actual Redis call would go here via the `redis` crate.
            // Stubbed: falls through to memory for now.
            leptos::logging::log!("redis::set_async {key}");
        }
        MemoryStore::global().set(key, value);
    }
}

// ── Typed store helpers ───────────────────────────────────────────────────────

/// Serialize a value to JSON and store it.
pub fn store_json<T: serde::Serialize>(backend: &str, key: &str, value: &T) {
    if let Ok(json) = serde_json::to_string(value) {
        StoreManager::set(backend, key, &json);
    }
}

/// Deserialize a value from a JSON store entry.
pub fn load_json<T: serde::de::DeserializeOwned>(backend: &str, key: &str) -> Option<T> {
    let raw = StoreManager::get(backend, key)?;
    serde_json::from_str(&raw).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_memory_set_get() {
        StoreManager::set("memory", "mgr_mem", "val");
        assert_eq!(StoreManager::get("memory", "mgr_mem").unwrap(), "val");
    }

    #[test]
    fn manager_local_set_get_ssr() {
        StoreManager::set("local", "mgr_loc", "local_val");
        assert_eq!(StoreManager::get("local", "mgr_loc").unwrap(), "local_val");
    }

    #[test]
    fn manager_session_set_get_ssr() {
        StoreManager::set("session", "mgr_ses", "ses_val");
        assert_eq!(StoreManager::get("session", "mgr_ses").unwrap(), "ses_val");
    }

    #[test]
    fn manager_default_backend_is_memory() {
        StoreManager::set("unknown_backend", "mgr_def", "x");
        assert_eq!(
            StoreManager::get("unknown_backend", "mgr_def").unwrap(),
            "x"
        );
    }

    #[test]
    fn manager_delete() {
        StoreManager::set("memory", "mgr_del", "x");
        StoreManager::delete("memory", "mgr_del");
        assert!(StoreManager::get("memory", "mgr_del").is_none());
    }

    #[test]
    fn store_json_and_load_json() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct Item {
            name: String,
            qty: u32,
        }

        let item = Item {
            name: "widget".into(),
            qty: 5,
        };
        store_json("memory", "mgr_json", &item);
        let loaded: Item = load_json("memory", "mgr_json").unwrap();
        assert_eq!(loaded, item);
    }

    #[test]
    fn load_json_missing_returns_none() {
        let result: Option<i32> = load_json("memory", "mgr_missing_json");
        assert!(result.is_none());
    }
}
