// src/tokens/island.rs
//
// Leptos island constructors — reactive components embedded in a token tree.
//
// Every function returns a `Block` whose `dynamic_content` field carries the
// reactive closure.  `render_dom` calls the closure reactively, so any
// `RwSignal` captured inside gets a proper Leptos subscription.
//
// The wrapper `<div>` emitted by `render_dom` uses `display:contents` so it
// is transparent to flex/grid layout — children participate in the parent
// container as if the wrapper did not exist.

use std::sync::Arc;
use leptos::prelude::*;
use crate::tokens::builders::{Block, Container};
use crate::tokens::node::TokenNode;

fn next_id() -> crate::tokens::node::Str {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static N: AtomicUsize = AtomicUsize::new(1_000_000);
    Arc::from(format!("isl{}", N.fetch_add(1, Ordering::Relaxed)).as_str())
}

// ── Core constructor ──────────────────────────────────────────────────────────

/// Embed a fully-reactive Leptos component as a node in the token tree.
///
/// Use `display:contents` so the `<div>` wrapper is layout-transparent.
/// Capture `RwSignal`s freely — they are `Copy + Send + Sync`.
///
/// ```rust
/// col().add(island(move || view! {
///     <button on:click=move |_| count.update(|n| *n += 1)>
///         {move || count.get()}
///     </button>
/// }.into_any()))
/// ```
pub fn island(f: impl Fn() -> AnyView + Send + Sync + 'static) -> Block {
    let mut n = TokenNode::new(next_id());
    n.style.extra = Arc::from("display:contents;");
    n.dynamic_content = Some(Arc::new(f));
    Container { stack: vec![n] }
}

/// Inject a `<style>` block once into the document head.
pub fn style_inject(css: &'static str) -> Block {
    island(move || view! { <style>{css}</style> }.into_any())
}

// ── Shared number formatter ───────────────────────────────────────────────────

pub fn fmt_n(n: i32) -> String {
    if n >= 1_000_000      { format!("{:.1}M", n as f32 / 1_000_000.0) }
    else if n >= 1_000     { format!("{:.1}K", n as f32 / 1_000.0)     }
    else                   { n.to_string() }
}