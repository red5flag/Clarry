// src/tokens/storage/entry.rs
//
// StoreEntry with TTL and subscription system.

use dashmap::DashMap;
use instant::Instant;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;

// ── Entry ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct StoreEntry {
    pub value: Arc<str>,
    pub expires_at: Option<Instant>,
}

impl StoreEntry {
    pub fn new(value: impl Into<Arc<str>>) -> Self {
        Self {
            value: value.into(),
            expires_at: None,
        }
    }
    pub fn with_ttl(value: impl Into<Arc<str>>, ttl: Duration) -> Self {
        Self {
            value: value.into(),
            expires_at: Some(Instant::now() + ttl),
        }
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
        for h in handlers.iter() {
            h(value);
        }
    }
}

/// Subscribe to changes on `key`. Returns a handle that unsubscribes on drop.
pub fn subscribe(key: impl Into<String>, handler: impl Fn(&str) + Send + Sync + 'static) {
    SUBSCRIBERS
        .entry(key.into())
        .or_default()
        .push(Arc::new(handler));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn entry_new() {
        let entry = StoreEntry::new("hello");
        assert_eq!(&*entry.value, "hello");
        assert!(entry.expires_at.is_none());
        assert!(!entry.is_expired());
    }

    #[test]
    fn entry_with_ttl_not_expired_immediately() {
        let entry = StoreEntry::with_ttl("val", Duration::from_secs(60));
        assert!(!entry.is_expired());
    }

    #[test]
    fn entry_with_zero_ttl_expires() {
        let entry = StoreEntry::with_ttl("val", Duration::from_millis(0));
        std::thread::sleep(Duration::from_millis(1));
        assert!(entry.is_expired());
    }

    #[test]
    fn no_ttl_never_expires() {
        let entry = StoreEntry::new("persistent");
        assert!(!entry.is_expired());
    }

    #[test]
    fn notify_calls_subscribers() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();
        subscribe("notify_test_key", move |_val| {
            c.fetch_add(1, Ordering::SeqCst);
        });
        notify("notify_test_key", "data");
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn notify_no_subscribers_does_not_panic() {
        notify("unsubscribed_key_xyz", "data");
    }

    #[test]
    fn subscribe_receives_value() {
        let received = Arc::new(std::sync::Mutex::new(String::new()));
        let r = received.clone();
        subscribe("val_check_key", move |val| {
            *r.lock().unwrap() = val.to_string();
        });
        notify("val_check_key", "hello");
        assert_eq!(&*received.lock().unwrap(), "hello");
    }
}
