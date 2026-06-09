// src/tokens/storage/manager.rs
//
// StoreManager routing and typed JSON helpers.

use super::backends::{MemoryStore, LocalStore, SessionStore};

// ── StoreManager — unified routing ───────────────────────────────────────────

/// Route a store operation to the correct backend.
/// Backend names: "memory" | "local" | "session" | "redis"
pub struct StoreManager;

impl StoreManager {
    pub fn set(backend: &str, key: &str, value: &str) {
        match backend {
            "local"   => LocalStore::set(key, value),
            "session" => SessionStore::set(key, value),
            "redis"   => {
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
            "local"   => LocalStore::get(key),
            "session" => SessionStore::get(key),
            _ => MemoryStore::global().get(key).map(|s| s.to_string()),
        }
    }

    pub fn delete(backend: &str, key: &str) {
        match backend {
            "local"   => LocalStore::delete(key),
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
