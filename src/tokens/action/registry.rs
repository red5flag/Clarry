// src/tokens/action/registry.rs
//
// Global action handler registry with concurrent access.

use std::sync::Arc;
use dashmap::DashMap;
use crate::tokens::node::Str;

pub type ActionHandler = Arc<dyn Fn() + Send + Sync>;

pub struct ActionRegistry {
    handlers: DashMap<Str, ActionHandler>,
}

impl Default for ActionRegistry {
    fn default() -> Self { Self::new() }
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self { handlers: DashMap::new() }
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
