// src/tokens/action/registry.rs
//
// Global action handler registry with concurrent access.

use crate::tokens::node::Str;
use dashmap::DashMap;
use std::sync::Arc;

pub type ActionHandler = Arc<dyn Fn() + Send + Sync>;

pub struct ActionRegistry {
    handlers: DashMap<Str, ActionHandler>,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            handlers: DashMap::new(),
        }
    }

    pub fn global() -> &'static Self {
        use once_cell::sync::OnceCell;
        static GLOBAL: OnceCell<ActionRegistry> = OnceCell::new();
        GLOBAL.get_or_init(Self::new)
    }

    pub fn register(&self, name: impl Into<Str>, handler: impl Fn() + Send + Sync + 'static) {
        self.handlers.insert(name.into(), Arc::new(handler));
    }

    pub fn register_once(&self, name: impl Into<Str>, handler: impl Fn() + Send + Sync + 'static) {
        let name = name.into();
        if !self.handlers.contains_key(&name) {
            self.handlers.insert(name, Arc::new(handler));
        }
    }

    pub fn execute(&self, name: &str) {
        if let Some(h) = self.handlers.get(name) {
            h();
        }
    }

    pub fn get(&self, name: &str) -> Option<ActionHandler> {
        self.handlers.get(name).map(|h| Arc::clone(&*h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn registry_new_is_empty() {
        let reg = ActionRegistry::new();
        assert!(reg.get("anything").is_none());
    }

    #[test]
    fn registry_default() {
        let reg = ActionRegistry::default();
        assert!(reg.get("anything").is_none());
    }

    #[test]
    fn register_and_execute() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();
        let reg = ActionRegistry::new();
        reg.register("bump", move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
        reg.execute("bump");
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn execute_missing_does_not_panic() {
        let reg = ActionRegistry::new();
        reg.execute("missing");
    }

    #[test]
    fn register_once_prevents_overwrite() {
        let first = Arc::new(AtomicUsize::new(0));
        let second = Arc::new(AtomicUsize::new(0));
        let f = first.clone();
        let s = second.clone();

        let reg = ActionRegistry::new();
        reg.register_once("action", move || {
            f.fetch_add(1, Ordering::SeqCst);
        });
        reg.register_once("action", move || {
            s.fetch_add(1, Ordering::SeqCst);
        });

        reg.execute("action");
        assert_eq!(first.load(Ordering::SeqCst), 1);
        assert_eq!(second.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn register_overwrites() {
        let first = Arc::new(AtomicUsize::new(0));
        let second = Arc::new(AtomicUsize::new(0));
        let f = first.clone();
        let s = second.clone();

        let reg = ActionRegistry::new();
        reg.register("action", move || {
            f.fetch_add(1, Ordering::SeqCst);
        });
        reg.register("action", move || {
            s.fetch_add(1, Ordering::SeqCst);
        });

        reg.execute("action");
        assert_eq!(first.load(Ordering::SeqCst), 0);
        assert_eq!(second.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn get_returns_handler() {
        let reg = ActionRegistry::new();
        reg.register("test", || {});
        assert!(reg.get("test").is_some());
        assert!(reg.get("other").is_none());
    }
}
