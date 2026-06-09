// src/tokens/kv_cache.rs
//
// KV Cache quantization using TurboQuant for LLM inference optimization.
//
// This module provides quantized key-value cache storage for transformer models,
// enabling memory-efficient inference with minimal accuracy loss.

use std::sync::Arc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use turboquant::{QuantizedKVCache, KVCacheConfig, TurboQuantError};

/// Quantized KV cache storage layer.
/// 
/// This structure maintains quantized representations of key-value caches
/// for transformer attention mechanisms, reducing memory footprint while
/// preserving inference accuracy.
pub struct QuantizedCacheStore {
    /// Map from cache identifier to quantized KV cache
    caches: DashMap<String, Arc<QuantizedKVCache>>,
}

impl QuantizedCacheStore {
    /// Create a new quantized cache store
    pub fn new() -> Self {
        Self {
            caches: DashMap::new(),
        }
    }

    /// Get or create a quantized KV cache for the given identifier
    pub fn get_or_create(&self, id: &str, config: KVCacheConfig) -> Result<Arc<QuantizedKVCache>, TurboQuantError> {
        if let Some(cache) = self.caches.get(id) {
            return Ok(Arc::clone(&cache));
        }

        // Create a new quantized KV cache with the provided config
        let cache = Arc::new(QuantizedKVCache::new(config)?);
        self.caches.insert(id.to_string(), Arc::clone(&cache));
        Ok(cache)
    }

    /// Store a pre-quantized KV cache
    pub fn insert(&self, id: String, cache: Arc<QuantizedKVCache>) {
        self.caches.insert(id, cache);
    }

    /// Remove a cache from the store
    pub fn remove(&self, id: &str) {
        self.caches.remove(id);
    }

    /// Clear all caches
    pub fn clear(&self) {
        self.caches.clear();
    }

    /// Get the number of cached KV pairs
    pub fn len(&self) -> usize {
        self.caches.len()
    }
}

impl Default for QuantizedCacheStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Global quantized cache store instance
pub static GLOBAL_KV_CACHE: Lazy<QuantizedCacheStore> = Lazy::new(QuantizedCacheStore::new);

/// Get the global quantized cache store
pub fn global_kv_cache() -> &'static QuantizedCacheStore {
    &GLOBAL_KV_CACHE
}

#[cfg(test)]
mod tests {
    use super::*;
    use turboquant::KVCacheConfig;

    #[test]
    fn test_cache_store_creation() {
        let store = QuantizedCacheStore::new();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_cache_insert_retrieve() {
        let store = QuantizedCacheStore::new();
        let config = KVCacheConfig::default();
        let cache = Arc::new(QuantizedKVCache::new(config).unwrap());
        store.insert("test".to_string(), Arc::clone(&cache));
        
        let retrieved = store.caches.get("test");
        assert!(retrieved.is_some());
    }
}
