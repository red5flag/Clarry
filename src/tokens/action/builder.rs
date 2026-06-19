// src/tokens/action/builder.rs
//
// Shorthand constructors for TokenAction variants and default registrations.

use crate::tokens::node::Str;
use crate::tokens::action::types::{TokenAction, LogLevel, DataTarget};
#[cfg(target_arch = "wasm32")]
use crate::tokens::action::registry::ActionRegistry;

// ── Shorthand constructors ────────────────────────────────────────────────────

pub fn log(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log { level: LogLevel::Info, message: msg.into() }
}

pub fn debug(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log { level: LogLevel::Debug, message: msg.into() }
}

pub fn warn(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log { level: LogLevel::Warn, message: msg.into() }
}

pub fn chain(actions: Vec<TokenAction>) -> TokenAction {
    TokenAction::Chain(actions)
}

pub fn show(id: impl Into<Str>) -> TokenAction {
    TokenAction::Show { show: id.into(), hide: vec![] }
}

pub fn show_hiding(show_id: impl Into<Str>, hide_ids: Vec<impl Into<Str>>) -> TokenAction {
    TokenAction::Show { 
        show: show_id.into(), 
        hide: hide_ids.into_iter().map(|s| s.into()).collect() 
    }
}

pub fn hide(id: impl Into<Str>) -> TokenAction {
    TokenAction::Hide(id.into())
}

pub fn hide_all_modals() -> TokenAction {
    TokenAction::HideAllModals
}

pub fn toggle_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleClass { target: target.into(), class: class.into() }
}

pub fn add_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::SetStyle { 
        target: target.into(), 
        property: "class".into(), 
        value: class.into() 
    }
}

pub fn remove_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleClass { target: target.into(), class: class.into() }
}

pub fn set_style(target: impl Into<Str>, property: impl Into<Str>, value: impl Into<Str>) -> TokenAction {
    TokenAction::SetStyle { 
        target: target.into(), 
        property: property.into(), 
        value: value.into() 
    }
}

pub fn set_attr(target: impl Into<Str>, attr: impl Into<Str>, value: impl Into<Str>) -> TokenAction {
    TokenAction::SetStyle { 
        target: target.into(), 
        property: attr.into(), 
        value: value.into() 
    }
}

pub fn navigate(page: impl Into<Str>) -> TokenAction {
    TokenAction::Navigate(page.into())
}
pub fn nav(page: impl Into<Str>) -> TokenAction { navigate(page) }

pub fn open_url(url: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("open_url:{}", url.into()).into())
}
pub fn url(u: impl Into<Str>) -> TokenAction { open_url(u) }

pub fn open_url_new_tab(url: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("open_url_new:{}", url.into()).into())
}

pub fn trigger_upload(accept: impl Into<Str>) -> TokenAction {
    TokenAction::TriggerFileInput { 
        accept: Some(accept.into()), 
        multiple: false 
    }
}

pub fn copy_to_clipboard(text: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("copy:{}", text.into()).into())
}

// ── Shorthand action functions for token DSL ─────────────────────────────────

pub fn route(path: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("route:{}", path.into()).into())
}

pub fn form(name: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("form:{}", name.into()).into())
}

pub fn toggle(state: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("toggle:{}", state.into()).into())
}

pub fn drag(name: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("drag:{}", name.into()).into())
}

pub fn val(name: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("val:{}", name.into()).into())
}

pub fn search(query: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("search:{}", query.into()).into())
}

pub fn scroll(handler: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("scroll:{}", handler.into()).into())
}

pub fn key(key: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("key:{}", key.into()).into())
}

pub fn resize(handler: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("resize:{}", handler.into()).into())
}

pub fn intersect(handler: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("intersect:{}", handler.into()).into())
}

pub fn in_(name: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("in:{}", name.into()).into())
}

// ── Storage actions ────────────────────────────────────────────────────────────

pub fn store_set(key: impl Into<Str>, val: impl Into<Str>) -> TokenAction {
    TokenAction::StoreSet {
        key: key.into(),
        value: val.into(),
    }
}

/// Read the current value from `input_key` in `ctx.strings` and store it under `key`.
pub fn store_set_input(key: impl Into<Str>, input_key: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("store_set_input:{}:{}", key.into(), input_key.into()).into())
}

pub fn store_get(key: impl Into<Str>, target: impl Into<Str>) -> TokenAction {
    TokenAction::StoreGet { 
        key: key.into(), 
        target: DataTarget::Element(target.into()), 
    }
}

pub fn store_delete(key: impl Into<Str>) -> TokenAction {
    TokenAction::StoreDelete { 
        key: key.into(), 
    }
}

/// Append the current value of `input_key` (from input state) as a new item
/// into the JSON array stored at `key`. Creates the array if absent.
pub fn store_push(key: impl Into<Str>, input_key: impl Into<Str>) -> TokenAction {
    TokenAction::StorePush { key: key.into(), input_key: input_key.into() }
}

/// Remove all items from the JSON array at `key` that match the current value
/// of `input_key` (from input state).
pub fn store_remove(key: impl Into<Str>, input_key: impl Into<Str>) -> TokenAction {
    TokenAction::StoreRemove { key: key.into(), input_key: input_key.into() }
}

/// Increment a numeric value at a dotted storage path by 1.
/// Reads the current value, parses it as i32, writes (current + 1) back.
pub fn store_inc(key: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("store_inc:{}", key.into()).into())
}

/// Toggle a boolean value at a dotted storage path.
pub fn store_tog(key: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("store_toggle:{}", key.into()).into())
}

/// Write to a dynamic storage path: both the path and value come from input fields
/// identified by `path_input` and `val_input` element IDs.
pub fn store_write_to_path(path_input: impl Into<Str>, val_input: impl Into<Str>) -> TokenAction {
    TokenAction::StoreWriteToPath { path_input: path_input.into(), val_input: val_input.into() }
}

// ── State actions ──────────────────────────────────────────────────────────────

pub fn toggle_state(key: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleState {
        key: key.into(),
        on_state: "true".into(),
        off_state: "false".into(),
    }
}
pub fn tog(key: impl Into<Str>) -> TokenAction { toggle_state(key) }

pub fn cycle_state(key: impl Into<Str>, values: Vec<impl Into<Str>>) -> TokenAction {
    let key = key.into();
    let values_str = values.into_iter().map(|v| v.into().to_string()).collect::<Vec<_>>().join(",");
    TokenAction::Custom(format!("cycle:{}:{}", key, values_str).into())
}
pub fn cyc(key: impl Into<Str>, values: Vec<impl Into<Str>>) -> TokenAction { cycle_state(key, values) }

/// Increment a counter by 1 (default)
pub fn increment(key: impl Into<Str>) -> TokenAction {
    TokenAction::Increment { key: key.into(), by: 1 }
}
pub fn inc(key: impl Into<Str>) -> TokenAction { increment(key) }

/// Increment a counter by a specific amount
pub fn increment_by(key: impl Into<Str>, amount: i32) -> TokenAction {
    TokenAction::Increment { key: key.into(), by: amount }
}

/// Decrement a counter by 1 (default)
pub fn decrement(key: impl Into<Str>) -> TokenAction {
    TokenAction::Decrement { key: key.into(), by: 1 }
}
pub fn dec(key: impl Into<Str>) -> TokenAction { decrement(key) }

/// Decrement a counter by a specific amount
pub fn decrement_by(key: impl Into<Str>, amount: i32) -> TokenAction {
    TokenAction::Decrement { key: key.into(), by: amount }
}

// ── Form actions ───────────────────────────────────────────────────────────────

pub fn submit_form(form_id: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("submit:{}", form_id.into()).into())
}

pub fn fetch_get(endpoint: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("fetch:get:{}", endpoint.into()).into())
}

/// Preload data from an endpoint and store it under a key
pub fn preload(key: impl Into<Str>, endpoint: impl Into<Str>) -> TokenAction {
    TokenAction::Preload { key: key.into(), endpoint: endpoint.into() }
}

/// Watch a storage key for changes and trigger reactive updates
pub fn store_watch(key: impl Into<Str>) -> TokenAction {
    TokenAction::Watch { key: key.into() }
}

/// Store a value with a TTL (time-to-live in seconds)
pub fn store_set_ttl(key: impl Into<Str>, val: impl Into<Str>, ttl_seconds: u64) -> TokenAction {
    TokenAction::StoreSetTtl {
        key: key.into(),
        value: val.into(),
        ttl_seconds,
    }
}

pub fn store_from_val(storage_key: impl Into<Str>, val_key: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("store_from_val:{}:{}", storage_key.into(), val_key.into()).into())
}

pub fn set_theme_var(name: impl Into<Str>, value: impl Into<Str>) -> TokenAction {
    TokenAction::SetThemeVar { name: name.into(), value: value.into() }
}

/// Toggle a drawer's open/closed state by flipping its visibility signal.
pub fn toggle_drawer(id: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("toggle_drawer:{}", id.into()).into())
}

/// Cycle through a list of drawers: close the currently open one, open the next.
/// If the last drawer is open, close all. IDs are comma-separated.
pub fn cycle_drawer(drawer_ids: Vec<impl Into<Str>>) -> TokenAction {
    let ids: Vec<String> = drawer_ids.into_iter().map(|s| s.into().to_string()).collect();
    TokenAction::Custom(format!("cycle_drawer:{}", ids.join(",")).into())
}

/// Send a chat message: reads `input_key` from TokenCtx strings, appends a
/// JSON message object to the array at `storage_key`, and clears the input.
pub fn chat_send(input_key: impl Into<Str>, storage_key: impl Into<Str>, sender: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("chat_send:{}:{}:{}", input_key.into(), storage_key.into(), sender.into()).into())
}

/// Apply debounce to an action via EventBinding (use .debounce_ms() on the binding instead)
pub fn debounce(_action: TokenAction, _ms: u32) -> TokenAction {
    TokenAction::Custom("debounce:stub".into())
}

/// Apply throttle to an action via EventBinding (use .throttle_ms() on the binding instead)
pub fn throttle(_action: TokenAction, _ms: u32) -> TokenAction {
    TokenAction::Custom("throttle:stub".into())
}

// ── Navigation defaults ───────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
pub fn register_navigation_defaults() {
    let r = ActionRegistry::global();
    r.register("navigate_home", || {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href("/");
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn register_navigation_defaults() {
    // No-op on SSR
}

// ── Execute token action ─────────────────────────────────────────────────────
