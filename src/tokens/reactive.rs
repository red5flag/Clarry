// src/tokens/reactive.rs
//
// Reactive context for the token render tree.
// Replaces imperative DOM manipulation with Leptos signal-driven state.
// TokenCtx is Copy (all fields are RwSignal which is Copy), so it can
// be freely moved into closures without cloning.

use leptos::prelude::*;
use std::collections::HashMap;

/// Reactive context threaded through every node in a token render tree.
///
/// Provided once at `render_dom` and accessible via `use_context::<TokenCtx>()`.
#[derive(Clone, Copy)]
pub struct TokenCtx {
    /// Element visibility by id. Missing key = render at CSS default.
    pub visibility: RwSignal<HashMap<String, bool>>,
    /// Integer counters by id (likes, followers, scroll depth, …).
    pub counters: RwSignal<HashMap<String, i32>>,
    /// Extra CSS class lists by id (appended after base class).
    pub classes: RwSignal<HashMap<String, Vec<String>>>,
    /// String values by id (form inputs, store_get results, text content).
    pub strings: RwSignal<HashMap<String, String>>,
    /// List revision counter — bump this when ListStore changes to trigger re-render.
    pub list_rev: RwSignal<u64>,
    /// CSS custom property overrides for theme system.
    pub theme: RwSignal<HashMap<String, String>>,
    /// Active toasts: (id, message)
    pub toasts: RwSignal<Vec<(String, String)>>,
    /// Chat messages by room key: Vec<message>
    pub messages: RwSignal<HashMap<String, Vec<String>>>,
}

impl Default for TokenCtx {
    fn default() -> Self { Self::new() }
}

impl TokenCtx {
    pub fn new() -> Self {
        leptos::logging::log!("[TOKEN_CTX] Creating new TokenCtx");
        Self {
            visibility: RwSignal::new(HashMap::new()),
            counters:   RwSignal::new(HashMap::new()),
            classes:    RwSignal::new(HashMap::new()),
            strings:    RwSignal::new(HashMap::new()),
            list_rev:   RwSignal::new(0),
            theme:      RwSignal::new(HashMap::new()),
            toasts:     RwSignal::new(Vec::new()),
            messages:   RwSignal::new(HashMap::new()),
        }
    }

    // ── Tree seeding ───────────────────────────────────────────────────

    /// Walk a token tree and pre-populate visibility for any node that carries
    /// `display:none` in its compiled style.  Call before `provide_context`.
    pub fn seed_from_tree(&self, root: &crate::tokens::node::TokenNode) {
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();
        leptos::logging::log!("[TOKEN_CTX] seed_from_tree START - root_id: {}", root.id);

        let mut stack = vec![root];
        let mut seeded_count = 0;

        while let Some(node) = stack.pop() {
            if !node.id.is_empty() {
                let has_display_none = node.style.extra.contains("display:none");
                let has_hidden_class = node.class.as_ref().split_whitespace().any(|p| p == "hidden");
                if has_display_none || has_hidden_class {
                    self.visibility.update(|m| { m.insert(node.id.to_string(), false); });
                    seeded_count += 1;
                }
                // Seed tab visual state for panel nodes
                if node.id.ends_with("_panel") {
                    let stem = node.id.trim_end_matches("_panel");
                    let tab_id = format!("{}_tab", stem);
                    if has_hidden_class || has_display_none {
                        // Inactive tab
                        self.classes.update(|m| {
                            let entry = m.entry(tab_id).or_default();
                            entry.retain(|c| c != "border-t-2" && c != "border-white" && c != "font-semibold");
                            if !entry.contains(&"text-gray-400".to_string()) {
                                entry.push("text-gray-400".to_string());
                            }
                        });
                    } else {
                        // Active tab
                        self.classes.update(|m| {
                            let entry = m.entry(tab_id).or_default();
                            entry.retain(|c| c != "text-gray-400");
                            for cls in ["border-t-2", "border-white", "font-semibold"] {
                                if !entry.contains(&cls.to_string()) {
                                    entry.push(cls.to_string());
                                }
                            }
                        });
                    }
                }
            }
            for child in &node.children { stack.push(child); }
        }

        #[cfg(not(target_arch = "wasm32"))]
        leptos::logging::log!("[TOKEN_CTX] seed_from_tree COMPLETE - {} nodes seeded in {}ms", seeded_count, start_time.elapsed().as_millis());
        #[cfg(target_arch = "wasm32")]
        leptos::logging::log!("[TOKEN_CTX] seed_from_tree COMPLETE - {} nodes seeded", seeded_count);
    }

    // ── Visibility ─────────────────────────────────────────────────────

    pub fn show(&self, id: &str) {
        leptos::logging::log!("[TOKEN_CTX] show - id: {}", id);
        self.visibility.update(|m| { m.insert(id.to_string(), true); });
        self.classes.update(|m| {
            if let Some(entry) = m.get_mut(id) {
                entry.retain(|c| c != "hidden");
            }
        });
    }

    pub fn hide(&self, id: &str) {
        leptos::logging::log!("[TOKEN_CTX] hide - id: {}", id);
        self.visibility.update(|m| { m.insert(id.to_string(), false); });
    }

    /// Hide every element whose id begins with `prefix` (e.g. "modal_").
    pub fn hide_prefixed(&self, prefix: &str) {
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();
        leptos::logging::log!("[TOKEN_CTX] hide_prefixed START - prefix: {}", prefix);

        let pfx = prefix.to_string();
        let mut hidden_count = 0;

        self.visibility.update(|m| {
            for (k, v) in m.iter_mut() {
                if k.starts_with(&pfx) {
                    *v = false;
                    hidden_count += 1;
                }
            }
        });

        #[cfg(not(target_arch = "wasm32"))]
        leptos::logging::log!("[TOKEN_CTX] hide_prefixed COMPLETE - {} nodes hidden in {}ms", hidden_count, start_time.elapsed().as_millis());
        #[cfg(target_arch = "wasm32")]
        leptos::logging::log!("[TOKEN_CTX] hide_prefixed COMPLETE - {} nodes hidden", hidden_count);
    }

    /// Returns a reactive closure that produces the correct CSS string.
    /// Any hard-coded `display:none;` in `base_style` is removed; the
    /// visibility signal takes over.
    pub fn style_fn(&self, id: String, base_style: String) -> impl Fn() -> String + Clone {
        let clean = base_style.replace("display:none;", "");
        let vis   = self.visibility;
        move || match vis.get().get(&id).copied() {
            Some(false) => format!("{}display:none;", clean),
            _           => clean.clone(),
        }
    }

    // ── Counters ───────────────────────────────────────────────────────

    pub fn increment(&self, id: &str) {
        leptos::logging::log!("[TOKEN_CTX] increment - id: {}", id);
        self.counters.update(|m| { *m.entry(id.to_string()).or_insert(0) += 1; });
    }

    pub fn decrement(&self, id: &str) {
        leptos::logging::log!("[TOKEN_CTX] decrement - id: {}", id);
        self.counters.update(|m| {
            let v = m.entry(id.to_string()).or_insert(0);
            *v = (*v - 1).max(0);
        });
    }

    pub fn set_counter(&self, id: &str, val: i32) {
        leptos::logging::log!("[TOKEN_CTX] set_counter - id: {}, value: {}", id, val);
        self.counters.update(|m| { m.insert(id.to_string(), val); });
    }

    /// Reactive accessor — subscribe by reading inside `move || counter_fn()`
    pub fn counter_fn(&self, id: String) -> impl Fn() -> i32 + Clone {
        leptos::logging::log!("[TOKEN_CTX] counter_fn - id: {}", id);
        let c = self.counters;
        move || c.get().get(&id).copied().unwrap_or(0)
    }

    // ── Classes ────────────────────────────────────────────────────────

    pub fn toggle_class(&self, id: &str, class: &str) {
        leptos::logging::log!("[TOKEN_CTX] toggle_class - id: {}, class: {}", id, class);
        self.classes.update(|m| {
            let entry = m.entry(id.to_string()).or_default();
            let cls   = class.to_string();
            if let Some(pos) = entry.iter().position(|c| c == &cls) {
                entry.remove(pos);
            } else {
                entry.push(cls);
            }
        });
    }

    /// Reactive accessor for extra classes appended to a node.
    pub fn extra_class_fn(&self, id: String) -> impl Fn() -> String + Clone {
        leptos::logging::log!("[TOKEN_CTX] extra_class_fn - id: {}", id);
        let c = self.classes;
        move || c.get().get(&id).map(|v| v.join(" ")).unwrap_or_default()
    }

    // ── Strings ────────────────────────────────────────────────────────

    pub fn set_string(&self, id: &str, val: impl Into<String>) {
        leptos::logging::log!("[TOKEN_CTX] set_string - id: {}", id);
        self.strings.update(|m| { m.insert(id.to_string(), val.into()); });
    }

    pub fn string_fn(&self, id: String) -> impl Fn() -> String + Clone {
        let s = self.strings;
        move || s.get().get(&id).cloned().unwrap_or_default()
    }

    // ── Lists ──────────────────────────────────────────────────────────

    pub fn bump_list_rev(&self) {
        self.list_rev.update(|r| *r += 1);
    }

    // ── Theme ────────────────────────────────────────────────────────────

    pub fn set_theme_var(&self, name: &str, value: &str) {
        leptos::logging::log!("[TOKEN_CTX] set_theme_var - {}: {}", name, value);
        self.theme.update(|m| { m.insert(name.to_string(), value.to_string()); });
    }

    // ── Toasts ─────────────────────────────────────────────────────────

    pub fn show_toast(&self, message: impl AsRef<str>) {
        let msg = message.as_ref().to_string();
        static TOAST_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = format!("toast_{}", TOAST_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        leptos::logging::log!("[TOKEN_CTX] show_toast - id: {}, message: {}", id, msg);
        self.toasts.update(|v| v.push((id, msg)));
    }

    pub fn dismiss_toast(&self, id: &str) {
        leptos::logging::log!("[TOKEN_CTX] dismiss_toast - id: {}", id);
        self.toasts.update(|v| v.retain(|(tid, _)| tid != id));
    }

    pub fn toast_fn(&self) -> impl Fn() -> Vec<(String, String)> + Clone {
        let t = self.toasts;
        move || t.get()
    }

    pub fn add_message(&self, room: &str, msg: impl AsRef<str>) {
        let m = msg.as_ref().to_string();
        leptos::logging::log!("[TOKEN_CTX] add_message - room: {}, msg: {}", room, m);
        self.messages.update(|map| {
            map.entry(room.to_string()).or_default().push(m);
        });
    }

    pub fn room_messages(&self, room: impl Into<String>) -> leptos::prelude::Memo<Vec<String>> {
        let room: String = room.into();
        let m = self.messages;
        leptos::prelude::Memo::new(move |_| m.get().get(&room).cloned().unwrap_or_default())
    }
}
