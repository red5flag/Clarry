#![recursion_limit = "256"]

pub mod tokens;
pub mod page_loader;
pub mod pages;
pub mod data;

#[cfg(feature = "ssr")]
pub mod api;

pub use tokens::*;
pub use tokens::builders::store;

/// Declarative UI macro — eliminates `.child()` boilerplate.
///
/// Syntax: `ui!(parent => { child_expr => { grandchild... }, child_expr, ... })`
///
/// Example:
/// ```rust,ignore
/// ui!(col() => {
///     card("Title") => {
///         text("Hello world")
///     },
///     btn("Click").act(increment("c"))
/// })
/// ```
#[macro_export]
macro_rules! ui {
    ($parent:expr => { $($body:tt)* }) => {{
        let mut __parent = $parent;
        $crate::ui!(@items __parent, $($body)*);
        __parent
    }};

    // Node with children, followed by more items
    (@items $parent:ident, $node:expr => { $($children:tt)* }, $($rest:tt)*) => {{
        let __child = $crate::ui!($node => { $($children)* });
        $parent = $crate::prelude::TokenBuilder::child($parent, __child);
        $crate::ui!(@items $parent, $($rest)*)
    }};

    // Leaf node, followed by more items
    (@items $parent:ident, $node:expr, $($rest:tt)*) => {{
        $parent = $crate::prelude::TokenBuilder::child($parent, $node);
        $crate::ui!(@items $parent, $($rest)*)
    }};

    // Node with children, last item
    (@items $parent:ident, $node:expr => { $($children:tt)* }) => {{
        let __child = $crate::ui!($node => { $($children)* });
        $parent = $crate::prelude::TokenBuilder::child($parent, __child);
    }};

    // Leaf node, last item
    (@items $parent:ident, $node:expr) => {{
        $parent = $crate::prelude::TokenBuilder::child($parent, $node);
    }};

    // Empty
    (@items $parent:ident,) => {};
}

use leptos::prelude::*;

/// Hydrate entry point - called by the WASM bundle on client-side
/// This mounts the App component to the body and hydrates the SSR output
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::prelude::*;
    use leptos::mount::mount_to_body;

    // ── PROMINENT CSR START LOGGING ─────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&"═══════════════════════════════════════════════════════════".into());
        web_sys::console::log_1(&"🌐🌐🌐 [HYDRATE_ENTRY] hydrate() CALLED - WASM BUNDLE LOADED!".into());
        web_sys::console::log_1(&"═══════════════════════════════════════════════════════════".into());
    }

    // Initialize page loader before mounting
    crate::page_loader::init_page_loader();

    leptos::logging::log!("[HYDRATE_ENTRY] 🌐 CSR: hydrate() entry point executing");
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&"[HYDRATE_ENTRY] 🌐 CSR: hydrate() entry point executing".into());
    }

    // Initialize panic hook for better debugging
    console_error_panic_hook::set_once();

    // ── Environment Detection ───────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&"[HYDRATE_ENTRY] 🌐 Running on WASM target".into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        leptos::logging::warn!("[HYDRATE_ENTRY] ⚠️ NOT running on WASM - hydration may fail");
    }

    // ── DOM State Inspection Before Hydration ───────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let location = window.location();
            if let Ok(href) = location.href() {
                leptos::logging::log!("[HYDRATE_ENTRY] 🌐 Current URL: {}", href);
                web_sys::console::log_1(&format!("[HYDRATE_ENTRY] 🌐 Current URL: {}", href).into());
            }

            if let Some(document) = window.document() {
                // Check for SSR root element
                let has_ssr_root = document.get_element_by_id("ssr-root").is_some();
                let has_app_body = document.get_element_by_id("app-body").is_some();

                web_sys::console::log_1(&format!("[HYDRATE_ENTRY] 📋 DOM check - ssr-root: {}, app-body: {}",
                    has_ssr_root, has_app_body).into());

                if let Some(body) = document.body() {
                    let body_html = body.inner_html();
                    let body_children = body.child_nodes().length();

                    leptos::logging::log!("[HYDRATE_ENTRY] 📋 Body: {} chars, {} child nodes",
                        body_html.len(), body_children);
                    web_sys::console::log_1(&format!("[HYDRATE_ENTRY] 📋 Body: {} chars, {} child nodes",
                        body_html.len(), body_children).into());

                    // Critical hydration checks
                    if body_children == 0 {
                        web_sys::console::error_1(&"⛔ [HYDRATE_ENTRY] ERROR: Body is EMPTY - SSR returned no HTML!".into());
                        leptos::logging::error!("[HYDRATE_ENTRY] ⛔ Body empty - hydration will fail");
                    }

                    // Generic hydration check: verify the app container exists
                    let app_container = document.get_element_by_id("app_container");
                    let has_app = app_container.is_some();
                    web_sys::console::log_1(&format!(
                        "[HYDRATE_ENTRY] 🔍 app_container: {}",
                        if has_app { "✓ present" } else { "✗ MISSING" }
                    ).into());
                    if !has_app {
                        web_sys::console::warn_1(&"[HYDRATE_ENTRY] ⚠️ app_container not found - hydration may mismatch".into());
                    }
                }
            }
        }
    }

    // ── WASM LOAD METRICS ─────────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        let start = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        web_sys::console::log_1(&format!(
            "🚀 [HYDRATE_METRIC] hydrate() entry - timestamp: {}ms",
            start
        ).into());
    }

    // ── DOM INSPECTION METRICS ───────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let body_children = body.child_nodes().length();
                let body_html_len = body.inner_html().len();

                web_sys::console::log_1(&format!(
                    "📊 [HYDRATE_METRIC] Pre-mount DOM state: children={}, html_len={}",
                    body_children, body_html_len
                ).into());

                // Generic SSR element check: verify app_container exists
                let app_exists = document.get_element_by_id("app_container").is_some();
                web_sys::console::log_1(&format!(
                    "🔍 [HYDRATE_METRIC] app_container: {}",
                    if app_exists { "✓ present" } else { "✗ MISSING" }
                ).into());
            }
        }
    }

    // ── ID Counter Reset (CRITICAL for hydration matching) ──────────────
    // This MUST run before any token tree construction
    crate::tokens::builders::reset_id_counter();
    leptos::logging::log!("[HYDRATE_ENTRY] 🔢 ID counter reset for hydration matching");

    // ── MOUNT TIMING ─────────────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    let mount_start = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    let mount_result = std::panic::catch_unwind(|| {
        leptos::logging::log!("[HYDRATE_ENTRY] 📦 Calling mount_to_body(App)...");
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(&"[HYDRATE_ENTRY] 📦 Calling mount_to_body(App)...".into());
        }

        mount_to_body(App);

        leptos::logging::log!("[HYDRATE_ENTRY] ✅ mount_to_body completed");
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(&"[HYDRATE_ENTRY] ✅ mount_to_body completed".into());
        }
    });

    #[cfg(target_arch = "wasm32")]
    {
        let mount_end = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);
        let duration = mount_end - mount_start;

        match mount_result {
            Ok(_) => {
                web_sys::console::log_1(&format!(
                    "✅ [HYDRATE_METRIC] mount_to_body completed in {:.2}ms",
                    duration
                ).into());

                // Post-mount verification: generic app_container check
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        let app = document.get_element_by_id("app_container");
                        web_sys::console::log_1(&format!(
                            "🔘 [HYDRATE_METRIC] app_container post-hydration: {}",
                            if app.is_some() { "✓ attached" } else { "✗ detached" }
                        ).into());
                    }
                }
            }
            Err(e) => {
                web_sys::console::error_1(&format!(
                    "⛔ [HYDRATE_METRIC] mount_to_body PANIC after {:.2}ms: {:?}",
                    duration, e
                ).into());
            }
        }
    }

    // ── Hydration Mismatch Warning ──────────────────────────────────────
    leptos::logging::warn!("[HYDRATE_ENTRY] If you see 'hydration mismatch' warnings above, check:");
    leptos::logging::warn!("[HYDRATE_ENTRY]   1. SSR and CSR produce identical DOM structure");
    leptos::logging::warn!("[HYDRATE_ENTRY]   2. Element IDs match exactly (reset_id_counter called)");
    leptos::logging::warn!("[HYDRATE_ENTRY]   3. No conditional rendering differences between SSR/CSR");
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::warn_1(&"[HYDRATE_ENTRY] Check browser console for hydration mismatch details".into());
    }
}

/// The root App component - reads initial route from window.__INITIAL_ROUTE__
#[component]
#[allow(unused_variables)]
pub fn App() -> impl IntoView {
    let (current_route, set_current_route) = signal("demo".to_string());

    {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            let route = window
                .get("__INITIAL_ROUTE__")
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| {
                    window.location().pathname().unwrap_or_else(|_| "/".to_string())
                });
            let route_name = route.trim_start_matches('/').to_string();
            let route_name = if route_name.is_empty() { "demo".to_string() } else { route_name };
            set_current_route.set(route_name);
        }
    }

    view! {
        <div id="app_container">
            {move || {
                crate::tokens::builders::reset_id_counter();
                crate::page_loader::render_route(current_route.get().as_str())
            }}
        </div>
    }
}