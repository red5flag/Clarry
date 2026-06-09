// src/tokens/action/web.rs
//
// Non-reactive action executor and WASM DOM helpers.

use crate::tokens::action::types::{TokenAction, LogLevel};
use crate::tokens::action::registry::ActionRegistry;

pub fn execute_token_action(action: &TokenAction) {
    match action {
        TokenAction::Log { level, message } => {
            match level {
                LogLevel::Debug => println!("DEBUG: {}", message),
                LogLevel::Info => println!("{}", message),
                LogLevel::Warn => eprintln!("WARN: {}", message),
                LogLevel::Error => eprintln!("ERROR: {}", message),
            }
        }
        TokenAction::Custom(name) => {
            ActionRegistry::global().execute(name);
        }
        TokenAction::Chain(actions) => {
            for a in actions {
                execute_token_action(a);
            }
        }
        _ => {}
    }
}

// ── DOM helpers (WASM-only; SSR stubs are no-ops) ─────────────────────────────

#[cfg(target_arch = "wasm32")]
pub mod dom {
    use web_sys::wasm_bindgen::JsCast;

    fn doc() -> Option<web_sys::Document> {
        web_sys::window()?.document()
    }
    fn el(id: &str) -> Option<web_sys::HtmlElement> {
        doc()?.get_element_by_id(id)
            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    }

    pub fn show_element(show: &str, hide: &[impl AsRef<str>]) {
        if let Some(d) = doc() {
            for id in hide {
                if let Some(e) = d.get_element_by_id(id.as_ref())
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                { let _ = e.style().set_property("display", "none"); }
            }
            if let Some(e) = el(show) { let _ = e.style().set_property("display", "flex"); }
        }
    }
    pub fn hide_element(id: &str) {
        if let Some(e) = el(id) { let _ = e.style().set_property("display", "none"); }
    }
    pub fn hide_all_modals() {
        if let Some(d) = doc() {
            if let Ok(nl) = d.query_selector_all("[id^='modal_']") {
                for i in 0..nl.length() {
                    if let Some(e) = nl.item(i)
                        .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                    { let _ = e.style().set_property("display", "none"); }
                }
            }
        }
    }
    pub fn toggle_class(id: &str, class: &str) {
        if let Some(e) = el(id) { let _ = e.class_list().toggle(class); }
    }
    pub fn set_style(id: &str, prop: &str, val: &str) {
        if let Some(e) = el(id) { let _ = e.style().set_property(prop, val); }
    }
    pub fn set_active_in_group(group: &str, active_id: &str, active_css: &str, inactive_css: &str) {
        if let Some(d) = doc() {
            if let Ok(nl) = d.query_selector_all(&format!("[data-tab-group='{group}']")) {
                for i in 0..nl.length() {
                    if let Some(e) = nl.item(i).and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let css = if e.id() == active_id { active_css } else { inactive_css };
                        let _ = e.set_attribute("style", css);
                    }
                }
            }
        }
    }
    pub fn trigger_file_input(accept: Option<&str>, multiple: bool) {
        if let Some(d) = doc() {
            if let Ok(el) = d.create_element("input") {
                if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                    let _ = input.set_type("file");
                    if let Some(a) = accept { let _ = input.set_accept(a); }
                    if multiple { let _ = input.set_multiple(true); }
                    let _ = input.click();
                }
            }
        }
    }
    pub fn local_storage_set(key: &str, value: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let _ = storage.set_item(key, value);
                }
            }
        }
    }
    pub fn local_storage_get(key: &str) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|s| s.get_item(key).ok())
            .flatten()
    }
    pub fn local_storage_delete(key: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let _ = storage.delete(key);
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod dom {
    /// No JS injection - pure Leptos handles all interactivity
    pub fn action_js() -> &'static str {
        ""
    }
    pub fn show_element(_: &str, _: &[impl AsRef<str>]) {}
    pub fn hide_element(_: &str) {}
    pub fn hide_all_modals() {}
    pub fn toggle_class(_: &str, _: &str) {}
    pub fn set_style(_: &str, _: &str, _: &str) {}
    pub fn set_active_in_group(_: &str, _: &str, _: &str, _: &str) {}
    pub fn trigger_file_input(_: Option<&str>, _: bool) {}
    pub fn local_storage_set(_: &str, _: &str) {}
    pub fn local_storage_get(_: &str) -> Option<String> { None }
    pub fn local_storage_delete(_: &str) {}
}
