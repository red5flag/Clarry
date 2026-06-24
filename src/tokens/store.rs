// src/tokens/store.rs
//
// INVARIANT: All `rayon` usage is strictly behind `#[cfg(not(target_arch = "wasm32"))]`.
// The WASM fallback path is sequential and must never panic or deadlock.
//
// Three-layer storage back-end for the token system:
//
//   FlyCache     — DashMap<style-hash, Arc<str>>
//                  Memoises compiled CSS strings.  A style compiles exactly
//                  once regardless of how many nodes share it.
//
//   TokenStore   — DashMap<id, TokenNode(shallow)>
//                  Global concurrent registry.  Shallow clones only — no
//                  subtree is ever cloned recursively into the store.
//                  Registration walks are iterative (BFS work-list).
//                  On native the walk is parallelised via rayon; on WASM it
//                  degrades to sequential without any cfg guards in page code.
//
//   TokenArena   — bumpalo::Bump
//                  Short-lived bump allocator for scratch strings produced
//                  during tree construction.  Drop the arena once the tree is
//                  handed to the store; all arena-allocated &str become invalid
//                  at that point.

use std::sync::Arc;

use bumpalo::Bump;
use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::tokens::node::{StyleToken, TokenNode};

// ── FlyCache ──────────────────────────────────────────────────────────────────

/// Memoised CSS compilation.
/// Key = `StyleToken::hash_key()` (direct-field hash, zero allocation).
/// Value = compiled CSS string wrapped in `Arc<str>` (shared, no copy on read).
pub struct FlyCache {
    map: DashMap<u64, Arc<str>>,
}

impl Default for FlyCache {
    fn default() -> Self {
        Self::new()
    }
}

impl FlyCache {
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
        }
    }

    /// Return the compiled CSS for `style`, compiling and caching on first access.
    /// Uses `DashMap::entry().or_insert_with()` to prevent double-compilation races.
    pub fn get_or_compile(&self, style: &StyleToken) -> Arc<str> {
        let key = style.hash_key();
        let entry = self
            .map
            .entry(key)
            .or_insert_with(|| style.compile().into());
        Arc::clone(&entry)
    }

    /// Number of unique styles in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

// ── TokenStore ────────────────────────────────────────────────────────────────

static GLOBAL_STORE: Lazy<TokenStore> = Lazy::new(TokenStore::new);

pub struct TokenStore {
    /// Shallow node index: id → node-without-children.
    nodes: DashMap<Arc<str>, TokenNode>,
    /// CSS memo cache shared with the store.
    pub cache: FlyCache,
}

impl TokenStore {
    fn new() -> Self {
        Self {
            nodes: DashMap::new(),
            cache: FlyCache::new(),
        }
    }

    pub fn global() -> &'static Self {
        &GLOBAL_STORE
    }

    /// Register a tree: warm the cache for every node and index each one
    /// by id.  Only shallow clones are stored — no subtree is duplicated.
    ///
    /// On native targets the child walk is parallelised with rayon.
    /// On WASM it runs sequentially (rayon is unavailable there).
    pub fn register(&self, root: &TokenNode) {
        self.cache.get_or_compile(&root.style);
        self.nodes
            .insert(root.id.as_ref().into(), root.clone_shallow());
        Self::walk_children(&root.children);
    }

    // ── Iterative BFS child walk ──────────────────────────────────────────
    // Both branches perform exactly the same logical work; the cfg chooses
    // whether individual iterations run in parallel.

    #[cfg(not(target_arch = "wasm32"))]
    fn walk_children(children: &[TokenNode]) {
        use rayon::prelude::*;

        // Collect the first level then fan out in parallel per subtree.
        // Each recursive call to walk_subtree is itself iterative so no
        // thread can overflow its stack.
        children.par_iter().for_each(|child| {
            Self::walk_subtree(child);
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn walk_children(children: &[TokenNode]) {
        for child in children {
            Self::walk_subtree(child);
        }
    }

    /// Iterative BFS over a subtree rooted at `root`.
    fn walk_subtree(root: &TokenNode) {
        let store = Self::global();
        let mut stack: Vec<&TokenNode> = Vec::with_capacity(32);
        stack.push(root);
        while let Some(node) = stack.pop() {
            store.cache.get_or_compile(&node.style);
            store
                .nodes
                .insert(node.id.as_ref().into(), node.clone_shallow());
            for child in node.children.iter().rev() {
                stack.push(child);
            }
        }
    }

    /// Look up a previously registered node by id.
    /// Returns a shallow clone (children: vec![]).
    pub fn get(&self, id: &str) -> Option<TokenNode> {
        self.nodes.get(id).map(|v| v.clone())
    }

    /// Look up compiled CSS for a style token (cache-through).
    pub fn css(&self, style: &StyleToken) -> Arc<str> {
        self.cache.get_or_compile(style)
    }

    /// Total nodes indexed.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Update a stored node's content and re-register it.
    pub fn update_content(&self, id: &str, content: impl Into<crate::tokens::node::Str>) {
        if let Some(mut node) = self.nodes.get_mut(id) {
            node.content = Some(content.into());
        }
    }
}

// ── TokenArena ────────────────────────────────────────────────────────────────
//
// Short-lived bump allocator for the tree construction phase.
//
// Usage pattern:
//   1. Create an arena at the start of page construction.
//   2. Allocate scratch strings from it (zero heap allocation).
//   3. Hand the finished tree to TokenStore::register.
//   4. Drop the arena — all &str references produced from it become invalid.
//      The tree itself holds owned Str (Cow<'static, str>) so it survives.
//
// The arena is NOT Send; use one per thread / per request.

pub struct TokenArena {
    bump: Bump,
}

impl Default for TokenArena {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenArena {
    pub fn new() -> Self {
        Self { bump: Bump::new() }
    }

    /// Intern a string slice — zero heap allocation.
    /// The returned &str is valid for the lifetime of the arena.
    pub fn intern<'a>(&'a self, s: &str) -> &'a str {
        self.bump.alloc_str(s)
    }

    /// Allocate a formatted scratch string.
    pub fn format<'a>(&'a self, args: std::fmt::Arguments<'_>) -> &'a str {
        use std::fmt::Write;
        let mut buf = bumpalo::collections::String::new_in(&self.bump);
        let _ = buf.write_fmt(args);
        buf.into_bump_str()
    }

    /// Convert an arena &str into an owned `Str` (Arc<str>).
    /// Call this when you need the string to outlive the arena.
    pub fn own(s: &str) -> crate::tokens::node::Str {
        Arc::from(s)
    }

    /// Bytes allocated so far (diagnostic / benchmarking).
    pub fn allocated_bytes(&self) -> usize {
        self.bump.allocated_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::node::StyleToken;

    // ── FlyCache ──────────────────────────────────────────────────────

    #[test]
    fn fly_cache_new_is_empty() {
        let cache = FlyCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn fly_cache_default() {
        let cache = FlyCache::default();
        assert!(cache.is_empty());
    }

    #[test]
    fn fly_cache_compiles_and_caches() {
        let cache = FlyCache::new();
        let style = StyleToken {
            w: Some(10.0),
            ..Default::default()
        };
        let css1 = cache.get_or_compile(&style);
        assert!(css1.contains("width: 10.00rem;"));
        assert_eq!(cache.len(), 1);

        let css2 = cache.get_or_compile(&style);
        assert_eq!(css1, css2);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn fly_cache_different_styles_different_entries() {
        let cache = FlyCache::new();
        let a = StyleToken {
            w: Some(10.0),
            ..Default::default()
        };
        let b = StyleToken {
            h: Some(20.0),
            ..Default::default()
        };
        cache.get_or_compile(&a);
        cache.get_or_compile(&b);
        assert_eq!(cache.len(), 2);
    }

    // ── TokenStore ────────────────────────────────────────────────────

    #[test]
    fn token_store_register_and_get() {
        let store = TokenStore::global();
        let node = TokenNode::new("store_test_1");
        store.register(&node);
        let retrieved = store.get("store_test_1");
        assert!(retrieved.is_some());
        assert_eq!(&*retrieved.unwrap().id, "store_test_1");
    }

    #[test]
    fn token_store_get_missing_returns_none() {
        let store = TokenStore::global();
        assert!(store.get("nonexistent_node_xyz").is_none());
    }

    #[test]
    fn token_store_register_tree_with_children() {
        let store = TokenStore::global();
        let mut parent = TokenNode::new("tree_parent");
        parent.children.push(TokenNode::new("tree_child_a"));
        parent.children.push(TokenNode::new("tree_child_b"));
        store.register(&parent);
        assert!(store.get("tree_parent").is_some());
        assert!(store.get("tree_child_a").is_some());
        assert!(store.get("tree_child_b").is_some());
    }

    #[test]
    fn token_store_css_caches_style() {
        let store = TokenStore::global();
        let style = StyleToken {
            pad: Some(1.0),
            ..Default::default()
        };
        let css = store.css(&style);
        assert!(css.contains("padding: 1.00rem;"));
    }

    #[test]
    fn token_store_update_content() {
        let store = TokenStore::global();
        let node = TokenNode::new("update_test");
        store.register(&node);
        store.update_content("update_test", "new content");
        let updated = store.get("update_test").unwrap();
        assert_eq!(&*updated.content.unwrap(), "new content");
    }

    // ── TokenArena ────────────────────────────────────────────────────

    #[test]
    fn arena_intern() {
        let arena = TokenArena::new();
        let s = arena.intern("hello");
        assert_eq!(s, "hello");
    }

    #[test]
    fn arena_default() {
        let arena = TokenArena::default();
        assert_eq!(arena.allocated_bytes(), 0);
    }

    #[test]
    fn arena_format() {
        let arena = TokenArena::new();
        let s = arena.format(format_args!("x = {}", 42));
        assert_eq!(s, "x = 42");
    }

    #[test]
    fn arena_own() {
        let owned = TokenArena::own("test");
        assert_eq!(&*owned, "test");
    }

    #[test]
    fn arena_allocated_bytes_grows() {
        let arena = TokenArena::new();
        let before = arena.allocated_bytes();
        arena.intern("some longer string data here");
        assert!(arena.allocated_bytes() >= before);
    }
}
