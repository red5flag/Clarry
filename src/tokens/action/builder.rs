// src/tokens/action/builder.rs
//
// Shorthand constructors for TokenAction variants and default registrations.

#[cfg(target_arch = "wasm32")]
use crate::tokens::action::registry::ActionRegistry;
use crate::tokens::action::types::{DataTarget, LogLevel, TokenAction};
use crate::tokens::node::Str;

// ── Shorthand constructors ────────────────────────────────────────────────────

pub fn log(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log {
        level: LogLevel::Info,
        message: msg.into(),
    }
}

pub fn debug(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log {
        level: LogLevel::Debug,
        message: msg.into(),
    }
}

pub fn warn(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log {
        level: LogLevel::Warn,
        message: msg.into(),
    }
}

pub fn chain(actions: Vec<TokenAction>) -> TokenAction {
    TokenAction::Chain(actions)
}

pub fn show(id: impl Into<Str>) -> TokenAction {
    TokenAction::Show {
        show: id.into(),
        hide: vec![],
    }
}

pub fn show_hiding(show_id: impl Into<Str>, hide_ids: Vec<impl Into<Str>>) -> TokenAction {
    TokenAction::Show {
        show: show_id.into(),
        hide: hide_ids.into_iter().map(|s| s.into()).collect(),
    }
}

pub fn hide(id: impl Into<Str>) -> TokenAction {
    TokenAction::Hide(id.into())
}

pub fn hide_all_modals() -> TokenAction {
    TokenAction::HideAllModals
}

pub fn toggle_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleClass {
        target: target.into(),
        class: class.into(),
    }
}

pub fn add_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::SetStyle {
        target: target.into(),
        property: "class".into(),
        value: class.into(),
    }
}

pub fn remove_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleClass {
        target: target.into(),
        class: class.into(),
    }
}

pub fn set_style(
    target: impl Into<Str>,
    property: impl Into<Str>,
    value: impl Into<Str>,
) -> TokenAction {
    TokenAction::SetStyle {
        target: target.into(),
        property: property.into(),
        value: value.into(),
    }
}

pub fn set_attr(
    target: impl Into<Str>,
    attr: impl Into<Str>,
    value: impl Into<Str>,
) -> TokenAction {
    TokenAction::SetStyle {
        target: target.into(),
        property: attr.into(),
        value: value.into(),
    }
}

pub fn navigate(page: impl Into<Str>) -> TokenAction {
    TokenAction::Navigate(page.into())
}
pub fn nav(page: impl Into<Str>) -> TokenAction {
    navigate(page)
}

pub fn open_url(url: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("open_url:{}", url.into()).into())
}
pub fn url(u: impl Into<Str>) -> TokenAction {
    open_url(u)
}

pub fn open_url_new_tab(url: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("open_url_new:{}", url.into()).into())
}

pub fn trigger_upload(accept: impl Into<Str>) -> TokenAction {
    TokenAction::TriggerFileInput {
        accept: Some(accept.into()),
        multiple: false,
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
    TokenAction::StoreDelete { key: key.into() }
}

/// Append the current value of `input_key` (from input state) as a new item
/// into the JSON array stored at `key`. Creates the array if absent.
pub fn store_push(key: impl Into<Str>, input_key: impl Into<Str>) -> TokenAction {
    TokenAction::StorePush {
        key: key.into(),
        input_key: input_key.into(),
    }
}

/// Remove all items from the JSON array at `key` that match the current value
/// of `input_key` (from input state).
pub fn store_remove(key: impl Into<Str>, input_key: impl Into<Str>) -> TokenAction {
    TokenAction::StoreRemove {
        key: key.into(),
        input_key: input_key.into(),
    }
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
    TokenAction::StoreWriteToPath {
        path_input: path_input.into(),
        val_input: val_input.into(),
    }
}

// ── State actions ──────────────────────────────────────────────────────────────

pub fn toggle_state(key: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleState {
        key: key.into(),
        on_state: "true".into(),
        off_state: "false".into(),
    }
}
pub fn tog(key: impl Into<Str>) -> TokenAction {
    toggle_state(key)
}

pub fn cycle_state(key: impl Into<Str>, values: Vec<impl Into<Str>>) -> TokenAction {
    let key = key.into();
    let values_str = values
        .into_iter()
        .map(|v| v.into().to_string())
        .collect::<Vec<_>>()
        .join(",");
    TokenAction::Custom(format!("cycle:{}:{}", key, values_str).into())
}
pub fn cyc(key: impl Into<Str>, values: Vec<impl Into<Str>>) -> TokenAction {
    cycle_state(key, values)
}

/// Increment a counter by 1 (default)
pub fn increment(key: impl Into<Str>) -> TokenAction {
    TokenAction::Increment {
        key: key.into(),
        by: 1,
    }
}
pub fn inc(key: impl Into<Str>) -> TokenAction {
    increment(key)
}

/// Increment a counter by a specific amount
pub fn increment_by(key: impl Into<Str>, amount: i32) -> TokenAction {
    TokenAction::Increment {
        key: key.into(),
        by: amount,
    }
}

/// Decrement a counter by 1 (default)
pub fn decrement(key: impl Into<Str>) -> TokenAction {
    TokenAction::Decrement {
        key: key.into(),
        by: 1,
    }
}
pub fn dec(key: impl Into<Str>) -> TokenAction {
    decrement(key)
}

/// Decrement a counter by a specific amount
pub fn decrement_by(key: impl Into<Str>, amount: i32) -> TokenAction {
    TokenAction::Decrement {
        key: key.into(),
        by: amount,
    }
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
    TokenAction::Preload {
        key: key.into(),
        endpoint: endpoint.into(),
    }
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
    TokenAction::SetThemeVar {
        name: name.into(),
        value: value.into(),
    }
}

/// Toggle a drawer's open/closed state by flipping its visibility signal.
pub fn toggle_drawer(id: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("toggle_drawer:{}", id.into()).into())
}

/// Cycle through a list of drawers: close the currently open one, open the next.
/// If the last drawer is open, close all. IDs are comma-separated.
pub fn cycle_drawer(drawer_ids: Vec<impl Into<Str>>) -> TokenAction {
    let ids: Vec<String> = drawer_ids
        .into_iter()
        .map(|s| s.into().to_string())
        .collect();
    TokenAction::Custom(format!("cycle_drawer:{}", ids.join(",")).into())
}

/// Send a chat message: reads `input_key` from TokenCtx strings, appends a
/// JSON message object to the array at `storage_key`, and clears the input.
pub fn chat_send(
    input_key: impl Into<Str>,
    storage_key: impl Into<Str>,
    sender: impl Into<Str>,
) -> TokenAction {
    TokenAction::Custom(
        format!(
            "chat_send:{}:{}:{}",
            input_key.into(),
            storage_key.into(),
            sender.into()
        )
        .into(),
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::action::types::{LogLevel, ScrollBehavior, TokenAction};

    #[test]
    fn log_creates_info_level() {
        if let TokenAction::Log { level, message } = log("test") {
            assert_eq!(level, LogLevel::Info);
            assert_eq!(&*message, "test");
        } else {
            panic!("expected Log");
        }
    }

    #[test]
    fn debug_creates_debug_level() {
        if let TokenAction::Log { level, .. } = debug("msg") {
            assert_eq!(level, LogLevel::Debug);
        } else {
            panic!("expected Log");
        }
    }

    #[test]
    fn warn_creates_warn_level() {
        if let TokenAction::Log { level, .. } = warn("msg") {
            assert_eq!(level, LogLevel::Warn);
        } else {
            panic!("expected Log");
        }
    }

    #[test]
    fn chain_wraps_actions() {
        let c = chain(vec![hide("a"), hide("b")]);
        if let TokenAction::Chain(actions) = c {
            assert_eq!(actions.len(), 2);
        } else {
            panic!("expected Chain");
        }
    }

    #[test]
    fn show_action() {
        if let TokenAction::Show { show: s, hide: h } = show("modal") {
            assert_eq!(&*s, "modal");
            assert!(h.is_empty());
        } else {
            panic!("expected Show");
        }
    }

    #[test]
    fn show_hiding_action() {
        let action = show_hiding("modal1", vec!["modal2", "modal3"]);
        if let TokenAction::Show { show: s, hide: h } = action {
            assert_eq!(&*s, "modal1");
            assert_eq!(h.len(), 2);
        } else {
            panic!("expected Show");
        }
    }

    #[test]
    fn hide_action() {
        if let TokenAction::Hide(id) = hide("modal") {
            assert_eq!(&*id, "modal");
        } else {
            panic!("expected Hide");
        }
    }

    #[test]
    fn hide_all_modals_action() {
        assert_eq!(hide_all_modals(), TokenAction::HideAllModals);
    }

    #[test]
    fn toggle_class_action() {
        if let TokenAction::ToggleClass { target, class } = toggle_class("el", "active") {
            assert_eq!(&*target, "el");
            assert_eq!(&*class, "active");
        } else {
            panic!("expected ToggleClass");
        }
    }

    #[test]
    fn set_style_action() {
        if let TokenAction::SetStyle {
            target,
            property,
            value,
        } = set_style("el", "color", "red")
        {
            assert_eq!(&*target, "el");
            assert_eq!(&*property, "color");
            assert_eq!(&*value, "red");
        } else {
            panic!("expected SetStyle");
        }
    }

    #[test]
    fn navigate_action() {
        if let TokenAction::Navigate(page) = navigate("home") {
            assert_eq!(&*page, "home");
        } else {
            panic!("expected Navigate");
        }
    }

    #[test]
    fn nav_is_navigate_alias() {
        let a = navigate("x");
        let b = nav("x");
        assert_eq!(a, b);
    }

    #[test]
    fn store_set_action() {
        if let TokenAction::StoreSet { key, value } = store_set("k", "v") {
            assert_eq!(&*key, "k");
            assert_eq!(&*value, "v");
        } else {
            panic!("expected StoreSet");
        }
    }

    #[test]
    fn store_delete_action() {
        if let TokenAction::StoreDelete { key } = store_delete("k") {
            assert_eq!(&*key, "k");
        } else {
            panic!("expected StoreDelete");
        }
    }

    #[test]
    fn store_push_action() {
        if let TokenAction::StorePush { key, input_key } = store_push("list", "input") {
            assert_eq!(&*key, "list");
            assert_eq!(&*input_key, "input");
        } else {
            panic!("expected StorePush");
        }
    }

    #[test]
    fn increment_action() {
        if let TokenAction::Increment { key, by } = increment("c") {
            assert_eq!(&*key, "c");
            assert_eq!(by, 1);
        } else {
            panic!("expected Increment");
        }
    }

    #[test]
    fn increment_by_action() {
        if let TokenAction::Increment { by, .. } = increment_by("c", 5) {
            assert_eq!(by, 5);
        } else {
            panic!("expected Increment");
        }
    }

    #[test]
    fn decrement_action() {
        if let TokenAction::Decrement { key, by } = decrement("c") {
            assert_eq!(&*key, "c");
            assert_eq!(by, 1);
        } else {
            panic!("expected Decrement");
        }
    }

    #[test]
    fn toggle_state_action() {
        if let TokenAction::ToggleState {
            key,
            on_state,
            off_state,
        } = toggle_state("dark")
        {
            assert_eq!(&*key, "dark");
            assert_eq!(&*on_state, "true");
            assert_eq!(&*off_state, "false");
        } else {
            panic!("expected ToggleState");
        }
    }

    #[test]
    fn tog_is_toggle_state_alias() {
        assert_eq!(tog("x"), toggle_state("x"));
    }

    #[test]
    fn inc_dec_aliases() {
        assert_eq!(inc("x"), increment("x"));
        assert_eq!(dec("x"), decrement("x"));
    }

    #[test]
    fn preload_action() {
        if let TokenAction::Preload { key, endpoint } = preload("data", "/api") {
            assert_eq!(&*key, "data");
            assert_eq!(&*endpoint, "/api");
        } else {
            panic!("expected Preload");
        }
    }

    #[test]
    fn store_set_ttl_action() {
        if let TokenAction::StoreSetTtl {
            key,
            value,
            ttl_seconds,
        } = store_set_ttl("k", "v", 60)
        {
            assert_eq!(&*key, "k");
            assert_eq!(&*value, "v");
            assert_eq!(ttl_seconds, 60);
        } else {
            panic!("expected StoreSetTtl");
        }
    }

    #[test]
    fn set_theme_var_action() {
        if let TokenAction::SetThemeVar { name, value } = set_theme_var("--bg", "#fff") {
            assert_eq!(&*name, "--bg");
            assert_eq!(&*value, "#fff");
        } else {
            panic!("expected SetThemeVar");
        }
    }

    #[test]
    fn trigger_upload_action() {
        if let TokenAction::TriggerFileInput { accept, multiple } = trigger_upload("image/*") {
            assert_eq!(&*accept.unwrap(), "image/*");
            assert!(!multiple);
        } else {
            panic!("expected TriggerFileInput");
        }
    }

    #[test]
    fn custom_action_builders() {
        assert!(matches!(route("home"), TokenAction::Custom(_)));
        assert!(matches!(form("login"), TokenAction::Custom(_)));
        assert!(matches!(toggle("dark"), TokenAction::Custom(_)));
        assert!(matches!(val("name"), TokenAction::Custom(_)));
        assert!(matches!(search("q"), TokenAction::Custom(_)));
        assert!(matches!(key("Enter"), TokenAction::Custom(_)));
    }
}
