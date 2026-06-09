// src/tokens/storage/entry.rs
//
// StoreEntry with TTL and subscription system.

use std::sync::Arc;
use std::time::Duration;
use instant::Instant;
use dashmap::DashMap;
use once_cell::sync::Lazy;

// ── Entry ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct StoreEntry {
    pub value:      Arc<str>,
    pub expires_at: Option<Instant>,
}

impl StoreEntry {
    pub fn new(value: impl Into<Arc<str>>) -> Self {
        Self { value: value.into(), expires_at: None }
    }
    pub fn with_ttl(value: impl Into<Arc<str>>, ttl: Duration) -> Self {
        Self { value: value.into(), expires_at: Some(Instant::now() + ttl) }
    }
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|t| Instant::now() > t).unwrap_or(false)
    }
}

// ── Subscriber map ────────────────────────────────────────────────────────────

type Handler = Arc<dyn Fn(&str) + Send + Sync>;

static SUBSCRIBERS: Lazy<DashMap<String, Vec<Handler>>> = Lazy::new(DashMap::new);

pub(crate) fn notify(key: &str, value: &str) {
    if let Some(handlers) = SUBSCRIBERS.get(key) {
        for h in handlers.iter() { h(value); }
    }
}

/// Subscribe to changes on `key`. Returns a handle that unsubscribes on drop.
pub fn subscribe(key: impl Into<String>, handler: impl Fn(&str) + Send + Sync + 'static) {
    SUBSCRIBERS
        .entry(key.into())
        .or_default()
        .push(Arc::new(handler));
}
