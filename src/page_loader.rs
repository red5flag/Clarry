// src/page_loader.rs
//
// Data-driven page registry. Pages register themselves via the global
// `register(name, handler)` function. No hardcoded page lists.

use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, OnceLock};
use leptos::prelude::*;

use crate::tokens::action::ActionRegistry;
use crate::tokens::render::render_dom;

/// Page registry for dynamic page loading
pub struct PageRegistry {
    pages: HashMap<String, Arc<dyn Fn() -> AnyView + Send + Sync>>,
}

impl Default for PageRegistry {
    fn default() -> Self { Self::new() }
}

impl PageRegistry {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    /// Register a page with the given name and handler
    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn() -> AnyView + Send + Sync + 'static
    {
        self.pages.insert(name.to_string(), Arc::new(handler));
    }

    /// Get a page by name
    pub fn get_page(&self, name: &str) -> Option<Arc<dyn Fn() -> AnyView + Send + Sync>> {
        self.pages.get(name).cloned()
    }

    /// List all registered page names
    pub fn list_pages(&self) -> Vec<String> {
        self.pages.keys().cloned().collect()
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

/// Thread-safe global page registry. Initialized lazily on first access.
static PAGE_REGISTRY: OnceLock<Mutex<PageRegistry>> = OnceLock::new();

fn get_registry() -> &'static Mutex<PageRegistry> {
    PAGE_REGISTRY.get_or_init(|| Mutex::new(PageRegistry::new()))
}

/// Convenience accessor — locks the global registry for read/write.
fn with_registry<T>(f: impl FnOnce(&mut PageRegistry) -> T) -> T {
    f(get_registry().lock().unwrap().deref_mut())
}

/// Register a page globally. Call from page modules at load time.
pub fn register<F>(name: &str, handler: F)
where
    F: Fn() -> AnyView + Send + Sync + 'static
{
    with_registry(|r| r.register(name, handler));
}

/// Helper: wrap a token tree page for registration
pub fn register_token_page<T, F>(name: &str, token_fn: F)
where
    T: crate::tokens::node::IntoToken,
    F: Fn() -> T + Send + Sync + 'static
{
    register(name, move || {
        let node = token_fn().into_node();
        view! { { render_dom(node) } }.into_any()
    });
}

/// Initialize the page loader system.
/// Pages should have already called `register()` by the time this runs.
pub fn init_page_loader() {
    leptos::logging::log!("🔧 Initializing page loader...");

    // Seed storage-driven data (idempotent — skips if already seeded)
    crate::data::app_data::seed_instagram_storage();

    // Register built-in demo pages
    register_token_page("demo", crate::pages::demo::page_token);
    register_token_page("instagram",                  crate::pages::instagram::page_token);
    register_token_page("instagram_home",             crate::pages::instagram::home::page_token);
    register_token_page("instagram_edit",             crate::pages::instagram::edit::page_token);
    register_token_page("instagram_create",           crate::pages::instagram::create::page_token);
    register_token_page("instagram_profile",          crate::pages::instagram::profile::page_token);
    register_token_page("instagram_post",             crate::pages::instagram::post::page_token);
    register_token_page("instagram_explore",          crate::pages::instagram::explore::page_token);
    register_token_page("instagram_notifications",    crate::pages::instagram::notifications::page_token);
    register_token_page("instagram_messages",         crate::pages::instagram::messages::page_token);
    register_token_page("instagram_messages_detail",  crate::pages::instagram::messages_detail::page_token);
    register_token_page("instagram_story",            crate::pages::instagram::story::page_token);
    register_token_page("instagram_reels",            crate::pages::instagram::reels::page_token);
    register_token_page("instagram_saved",            crate::pages::instagram::saved::page_token);
    register_token_page("instagram_tagged",           crate::pages::instagram::tagged::page_token);
    register_token_page("twitter",    crate::pages::twitter::page_token);
    register_token_page("chat",       crate::pages::chat::page_token);
    register_token_page("shop",       crate::pages::shop::page_token);
    register_token_page("dashboard",  crate::pages::dashboard::page_token);
    register_token_page("feed",       crate::pages::feed::page_token);

    // Register default generic actions
    register_default_actions();

    with_registry(|r| {
        leptos::logging::log!("🚀 Page loader initialized with {} pages", r.page_count());
        leptos::logging::log!("📄 Available pages: {:?}", r.list_pages());
    });
}

/// Get a page by name
pub fn get_page(name: &str) -> Option<Arc<dyn Fn() -> AnyView + Send + Sync>> {
    with_registry(|r| r.get_page(name))
}

/// Render a page by name
pub fn render_page(name: &str) -> Option<AnyView> {
    if let Some(page_fn) = get_page(name) {
        Some(page_fn())
    } else {
        leptos::logging::log!("Page not found: {}", name);
        None
    }
}

/// Render a page by route (alias for render_page)
pub fn render_route(route: &str) -> AnyView {
    leptos::logging::log!("🎯 Rendering route: {}", route);
    render_page(route).unwrap_or_else(|| {
        leptos::logging::log!("❌ Page not found: {}", route);
        view! {
            <div class="p-8 flex flex-col items-center justify-center min-h-screen gap-4">
                <h1 class="text-2xl font-bold text-red-600">Page Not Found</h1>
                <p class="text-gray-600">No page registered for route: <code>{route}</code></p>
                <p class="text-sm text-gray-500">Available: {move || {
                    with_registry(|r| r.list_pages().join(", "))
                }}</p>
            </div>
        }.into_any()
    })
}

// DOM Helpers (WASM-only)
#[cfg(target_arch = "wasm32")]
mod dom {
    use web_sys::wasm_bindgen::JsCast;

    pub fn show_element(show: &str, hide: &[&str]) {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            for id in hide {
                if let Some(el) = doc.get_element_by_id(id) {
                    let _ = el.unchecked_into::<web_sys::HtmlElement>()
                        .style().set_property("display", "none");
                }
            }
            if let Some(el) = doc.get_element_by_id(show) {
                let _ = el.unchecked_into::<web_sys::HtmlElement>()
                    .style().set_property("display", "block");
            }
        }
    }

    pub fn hide_all_modals() {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Ok(modals) = doc.query_selector_all("[id^='modal_']") {
                for i in 0..modals.length() {
                    if let Some(el) = modals.item(i)
                        .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let _ = el.style().set_property("display", "none");
                    }
                }
            }
        }
    }

    pub fn toggle_class(target: &str, class: &str) {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(target))
        {
            let _ = el.class_list().toggle(class);
        }
    }

    pub fn set_style(target: &str, property: &str, value: &str) {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(target))
        {
            let _ = el.unchecked_into::<web_sys::HtmlElement>()
                .style().set_property(property, value);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    #![allow(dead_code)]
    pub fn show_element(_show: &str, _hide: &[&str]) {}
    pub fn hide_all_modals() {}
    pub fn toggle_class(_target: &str, _class: &str) {}
    pub fn set_style(_target: &str, _property: &str, _value: &str) {}
}

// Default Action Handlers (Called Once)
pub fn register_default_actions() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let reg = ActionRegistry::global();

        // Generic modal management
        reg.register("close_modal", || {
            #[cfg(target_arch = "wasm32")] dom::hide_all_modals();
        });

        // Generic load-more
        reg.register("load_more", || leptos::logging::log!("Loading more content (default)"));
    });
}
