// src/tokens/render/executor.rs
//
// Reactive token action executor.
// Iterative; no recursion. Updates TokenCtx signals instead of poking the DOM.

use leptos::prelude::*;

use crate::tokens::action::{TokenAction, LogLevel, ActionRegistry};
use crate::tokens::reactive::TokenCtx;
use crate::tokens::debug::inspector_log;

#[cfg(target_arch = "wasm32")]
use super::dom;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use js_sys;

#[allow(unused_variables)]
pub(crate) fn execute_token_action_reactive(actions: &[TokenAction], ctx: TokenCtx) {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();
    leptos::logging::log!("[TOKEN_ACTION_EXECUTOR] START - actions_count: {}", actions.len());

    let mut work: Vec<&TokenAction> = actions.iter().rev().collect();
    let mut executed_count = 0;

    while let Some(action) = work.pop() {
        executed_count += 1;
        leptos::logging::log!("[TOKEN_ACTION] Processing action #{}: {:?}", executed_count, action);

        match action {
            TokenAction::Chain(inner) => execute_token_action_reactive(inner, ctx),
            TokenAction::Show { show, hide } => {
                inspector_log(format!("[HYDRATE_TRACE] 👁️ Action: Show '{}'", show));
                ctx.visibility.update(|m| { m.insert(show.to_string(), true); for h in hide { m.insert(h.to_string(), false); } });
                // If this is a panel switch, also update tab button visuals
                if show.ends_with("_panel") {
                    let active_tab = format!("{}_tab", show.trim_end_matches("_panel"));
                    let inactive_tabs: Vec<String> = hide.iter()
                        .filter(|h| h.ends_with("_panel"))
                        .map(|h| format!("{}_tab", h.trim_end_matches("_panel")))
                        .collect();
                    let inactive_refs: Vec<&str> = inactive_tabs.iter().map(|s| s.as_str()).collect();
                    #[cfg(target_arch = "wasm32")]
                    dom::update_tab_visuals(&active_tab, &inactive_refs);
                }
            }
            TokenAction::Hide(id) => {
                inspector_log(format!("[HYDRATE_TRACE] 🙈 Action: Hide '{}'", id));
                ctx.visibility.update(|m| { m.insert(id.to_string(), false); });
            }
            TokenAction::HideAllModals => {
                inspector_log("[HYDRATE_TRACE] 🚪 Action: HideAllModals".to_string());
                ctx.visibility.update(|m| { for (k, v) in m.iter_mut() { if k.starts_with("modal_") { *v = false; } } });
            }
            TokenAction::ToggleClass { target, class } => {
                leptos::logging::log!("[TOKEN_ACTION] ToggleClass - target: {}, class: {}", target, class);
                ctx.toggle_class(target, class);
            }
            TokenAction::Log { level, message } => match level {
                LogLevel::Debug => leptos::logging::log!("[TOKEN_LOG] DEBUG: {}", message),
                LogLevel::Info  => leptos::logging::log!("[TOKEN_LOG] INFO: {}", message),
                LogLevel::Warn  => leptos::logging::warn!("[TOKEN_LOG] WARN: {}", message),
                LogLevel::Error => leptos::logging::error!("[TOKEN_LOG] ERROR: {}", message),
            },
            TokenAction::TriggerFileInput { accept, multiple } => {
                leptos::logging::log!("[TOKEN_ACTION] TriggerFileInput - accept: {:?}, multiple: {}", accept, multiple);
                #[cfg(target_arch = "wasm32")]
                dom::trigger_file_input(accept.as_deref(), *multiple);
            }
            TokenAction::SetStyle { target, property, value } => {
                leptos::logging::log!("[TOKEN_ACTION] SetStyle - target: {}, property: {}, value: {}", target, property, value);
                #[cfg(target_arch = "wasm32")]
                dom::set_style(target, property, value);
            }
            TokenAction::Navigate(page) => {
                leptos::logging::log!("[TOKEN_ACTION] Navigate - page: {}", page);
                #[cfg(target_arch = "wasm32")]
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href(&format!("/{}", page));
                }
            }
            TokenAction::ScrollTo { target, behavior } => {
                leptos::logging::log!("[TOKEN_ACTION] ScrollTo - target: {}", target);
                #[cfg(target_arch = "wasm32")]
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(el) = doc.get_element_by_id(target) {
                        let _ = el.scroll_into_view();
                    }
                }
            }
            TokenAction::StoreSet { key, value } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreSet - key: {}, value: {}", key, value);
                #[cfg(target_arch = "wasm32")]
                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok()).flatten()
                {
                    let _ = storage.set_item(key, value);
                }
            }
            TokenAction::StoreSetTtl { key, value, ttl_seconds } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreSetTtl - key: {}, ttl: {}s", key, ttl_seconds);
                #[cfg(target_arch = "wasm32")]
                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok()).flatten()
                {
                    let _ = storage.set_item(key, value);
                    let ttl_key = format!("{}:_ttl", key);
                    let now = web_sys::window()
                        .and_then(|w| w.performance())
                        .map(|p| p.now() as u64)
                        .unwrap_or(0);
                    let expires = now + (*ttl_seconds as u64 * 1000);
                    let _ = storage.set_item(&ttl_key, &expires.to_string());
                }
            }
            TokenAction::Preload { key, endpoint } => {
                leptos::logging::log!("[TOKEN_ACTION] Preload - key: {}, endpoint: {}", key, endpoint);
                #[cfg(target_arch = "wasm32")]
                {
                    let endpoint = endpoint.to_string();
                    let key = key.to_string();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            let promise = window.fetch_with_str(&endpoint);
                            let resp = wasm_bindgen_futures::JsFuture::from(promise).await;
                            if let Ok(resp) = resp {
                                if let Ok(response) = resp.dyn_into::<web_sys::Response>() {
                                    if let Ok(text_promise) = response.text() {
                                        let text = wasm_bindgen_futures::JsFuture::from(text_promise).await;
                                        if let Ok(text) = text {
                                            if let Some(text_str) = text.as_string() {
                                                leptos::logging::log!("[TOKEN_ACTION] Preload success: {}", text_str);
                                                if let Some(storage) = web_sys::window()
                                                    .and_then(|w| w.local_storage().ok()).flatten()
                                                {
                                                    let _ = storage.set_item(&key, &text_str);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            }
            TokenAction::Watch { key } => {
                leptos::logging::log!("[TOKEN_ACTION] Watch - key: {}", key);
                #[cfg(target_arch = "wasm32")]
                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok()).flatten()
                {
                    if let Ok(Some(val)) = storage.get_item(key) {
                        leptos::logging::log!("[TOKEN_ACTION] Watch updated key: {} -> {}", key, &val);
                        ctx.set_string(key, val);
                        ctx.bump_list_rev();
                    }
                }
            }
            TokenAction::StoreGet { key, target } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreGet - key: {}", key);
                #[cfg(target_arch = "wasm32")]
                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok()).flatten()
                {
                    if let Ok(Some(val)) = storage.get_item(key) {
                        match target {
                            crate::tokens::action::DataTarget::Signal(id) => {
                                ctx.set_string(id, val);
                            }
                            crate::tokens::action::DataTarget::Element(id) => {
                                ctx.set_string(id, val);
                            }
                        }
                    }
                }
            }
            TokenAction::StoreDelete { key } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreDelete - key: {}", key);
                #[cfg(target_arch = "wasm32")]
                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok()).flatten()
                {
                    let _ = storage.remove_item(key);
                }
            }
            TokenAction::Increment { key, by } => {
                leptos::logging::log!("[TOKEN_ACTION] Increment - key: {}, by: {}", key, by);
                ctx.counters.update(|m| { *m.entry(key.to_string()).or_insert(0) += by; });
            }
            TokenAction::Decrement { key, by } => {
                leptos::logging::log!("[TOKEN_ACTION] Decrement - key: {}, by: {}", key, by);
                ctx.counters.update(|m| {
                    let v = m.entry(key.to_string()).or_insert(0);
                    *v = (*v - by).max(0);
                });
            }
            TokenAction::ToggleState { key, on_state: _, off_state: _ } => {
                leptos::logging::log!("[TOKEN_ACTION] ToggleState - key: {}", key);
                // Toggle the visibility signal for the key
                ctx.visibility.update(|m| {
                    let id = key.to_string();
                    if let Some(current) = m.get(&id).copied() {
                        m.insert(id, !current);
                    } else {
                        m.insert(id, true); // Default to showing if not set
                    }
                });
            }
            TokenAction::RequestFullscreen => {
                #[cfg(target_arch = "wasm32")]
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(el) = doc.document_element() {
                        if let Ok(elem) = el.dyn_into::<web_sys::HtmlElement>() {
                            let _ = elem.request_fullscreen();
                        }
                    }
                }
            }
            TokenAction::ExitFullscreen => {
                #[cfg(target_arch = "wasm32")]
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    let _ = doc.exit_fullscreen();
                }
            }
            TokenAction::RequestPointerLock => {
                #[cfg(target_arch = "wasm32")]
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(el) = doc.body() {
                        let _ = el.request_pointer_lock();
                    }
                }
            }
            TokenAction::Vibrate { pattern } => {
                #[cfg(target_arch = "wasm32")]
                if let Some(nav) = web_sys::window().map(|w| w.navigator()) {
                    let arr = js_sys::Array::new();
                    for ms in pattern { arr.push(&js_sys::Number::from(*ms)); }
                    let _ = js_sys::Reflect::get(&nav, &"vibrate".into())
                        .ok()
                        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
                        .and_then(|f| f.call1(&nav.into(), &arr).ok());
                }
            }
            TokenAction::Notify { title, body: _, icon: _ } => {
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = web_sys::Notification::new(title.as_ref());
                }
            }
            TokenAction::Share { title, text, url } => {
                #[cfg(target_arch = "wasm32")]
                if let Some(nav) = web_sys::window().map(|w| w.navigator()) {
                    let obj = js_sys::Object::new();
                    let _ = js_sys::Reflect::set(&obj, &"title".into(), &JsValue::from_str(title.as_ref()));
                    let _ = js_sys::Reflect::set(&obj, &"text".into(), &JsValue::from_str(text.as_ref()));
                    if let Some(u) = url {
                        let _ = js_sys::Reflect::set(&obj, &"url".into(), &JsValue::from_str(u.as_ref()));
                    }
                    let _ = js_sys::Reflect::get(&nav, &"share".into())
                        .ok()
                        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
                        .and_then(|f| f.call1(&nav.into(), &obj).ok());
                }
            }
            TokenAction::Custom(name) => {
                execute_custom_action(name, ctx);
            }
            TokenAction::OpenUrl { url, new_tab } => {
                if *new_tab {
                    leptos::logging::log!("[TOKEN_ACTION] OpenUrl - url: {}, new_tab: true", url);
                    #[cfg(target_arch = "wasm32")] {
                        if let Some(win) = web_sys::window() {
                            let _ = win.open_with_url_and_target(url, "_blank");
                        }
                    }
                } else {
                    leptos::logging::log!("[TOKEN_ACTION] OpenUrl - url: {}, new_tab: false", url);
                    #[cfg(target_arch = "wasm32")] {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_href(url);
                        }
                    }
                }
            }
            TokenAction::CopyToClipboard(text) => {
                leptos::logging::log!("[TOKEN_ACTION] CopyToClipboard - text: {}", text);
                #[cfg(target_arch = "wasm32")] {
                    if let Some(win) = web_sys::window() {
                        if let Ok(nav) = win.navigator().dyn_into::<web_sys::Navigator>() {
                            let clipboard = nav.clipboard();
                            let _ = clipboard.write_text(text);
                        }
                    }
                }
            }
            TokenAction::SetThemeVar { name, value } => {
                leptos::logging::log!("[TOKEN_ACTION] SetThemeVar - {}: {}", name, value);
                ctx.set_theme_var(name, value);
                #[cfg(target_arch = "wasm32")] {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        let _ = doc.document_element().map(|el| {
                            let _ = el.dyn_into::<web_sys::HtmlElement>().map(|h| {
                                h.style().set_property(&format!("--{}", name), value).ok()
                            });
                        });
                    }
                }
            }
            TokenAction::SetActive { group, active_id, active_css, inactive_css } => {
                leptos::logging::log!("[TOKEN_ACTION] SetActive - group: {}, active: {} (css: {} / {})", group, active_id, active_css, inactive_css);
                ctx.visibility.update(|m| { m.insert(active_id.to_string(), true); });
            }
            TokenAction::Submit { form_id, on_submit, on_invalid } => {
                leptos::logging::log!("[TOKEN_ACTION] Submit - form: {}, submit: {:?}, invalid: {:?}", form_id, on_submit, on_invalid);
                ActionRegistry::global().execute(&format!("submit:{}", form_id));
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    leptos::logging::log!("[TOKEN_ACTION_EXECUTOR] COMPLETE - {} actions executed in {}ms", executed_count, start_time.elapsed().as_millis());
    #[cfg(target_arch = "wasm32")]
    leptos::logging::log!("[TOKEN_ACTION_EXECUTOR] COMPLETE - {} actions executed", executed_count);
}

// ── Custom action handler (extracted from main executor loop) ───────────────

#[allow(unused_variables)]
fn execute_custom_action(name: &str, ctx: TokenCtx) {
    leptos::logging::log!("[TOKEN_ACTION] Custom - name: {}", name);
    if let Some(path) = name.strip_prefix("route:") {
        leptos::logging::log!("[TOKEN_ACTION] route -> navigating to {}", path);
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            let _ = win.location().set_href(path);
        }
    } else if let Some(url) = name.strip_prefix("open_url:") {
        leptos::logging::log!("[TOKEN_ACTION] open_url -> {}", url);
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            let _ = win.location().set_href(url);
        }
    } else if let Some(url) = name.strip_prefix("open_url_new:") {
        leptos::logging::log!("[TOKEN_ACTION] open_url_new -> {}", url);
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            let _ = win.open_with_url_and_target(url, "_blank");
        }
    } else if let Some(text) = name.strip_prefix("copy:") {
        leptos::logging::log!("[TOKEN_ACTION] copy -> {}", text);
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            if let Ok(nav) = win.navigator().dyn_into::<web_sys::Navigator>() {
                let clipboard = nav.clipboard();
                let _ = clipboard.write_text(text);
            }
        }
    } else if let Some(key) = name.strip_prefix("toggle:") {
        leptos::logging::log!("[TOKEN_ACTION] toggle -> {}", key);
        let key_str = key.to_string();
        ctx.visibility.update(|m| {
            let entry = m.entry(key_str.clone()).or_insert(false);
            *entry = !*entry;
        });
    } else if name == "dismiss_toast" {
        leptos::logging::log!("[TOKEN_ACTION] dismiss_toast");
        // Dismiss the most recent toast (or all if needed)
        ctx.toasts.update(|v| { v.pop(); });
    } else if name == "show_toast" {
        leptos::logging::log!("[TOKEN_ACTION] show_toast");
        ctx.show_toast("Toast notification");
    } else if let Some(rest) = name.strip_prefix("store_from_val:") {
        leptos::logging::log!("[TOKEN_ACTION] store_from_val -> {}", rest);
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() == 2 {
            let storage_key = parts[0];
            let val_key = parts[1];
            let val = ctx.strings.get().get(val_key).cloned().unwrap_or_default();
            #[cfg(target_arch = "wasm32")]
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
                let _ = storage.set_item(storage_key, &val);
            }
            leptos::logging::log!("[TOKEN_ACTION] stored '{}' -> {} = {}", val_key, storage_key, val);
        }
    } else if let Some(form_id) = name.strip_prefix("form:") {
        leptos::logging::log!("[TOKEN_ACTION] form submit -> {}", form_id);
        #[cfg(target_arch = "wasm32")]
        {
            let mut values = std::collections::HashMap::new();
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let collection = doc.get_elements_by_tag_name("input");
                for i in 0..collection.length() {
                    if let Some(el) = collection.item(i) {
                        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                            let name = input.name();
                            let val = input.value();
                            if !name.is_empty() {
                                values.insert(name, val);
                            }
                        }
                    }
                }
                let textareas = doc.get_elements_by_tag_name("textarea");
                for i in 0..textareas.length() {
                    if let Some(el) = textareas.item(i) {
                        if let Ok(ta) = el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                            let name = ta.name();
                            let val = ta.value();
                            if !name.is_empty() {
                                values.insert(name, val);
                            }
                        }
                    }
                }
            }
            leptos::logging::log!("[TOKEN_ACTION] form '{}' values: {:?}", form_id, values);
            ActionRegistry::global().execute(&format!("form:{}", form_id));
        }
        ActionRegistry::global().execute(name);
    } else if let Some(form_id) = name.strip_prefix("submit:") {
        leptos::logging::log!("[TOKEN_ACTION] submit -> {}", form_id);
        #[cfg(target_arch = "wasm32")]
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(form_el) = doc.get_element_by_id(form_id) {
                let mut values = std::collections::HashMap::new();
                let inputs = form_el.get_elements_by_tag_name("input");
                for i in 0..inputs.length() {
                    if let Some(el) = inputs.item(i) {
                        if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                            let n = input.name();
                            let v = input.value();
                            if !n.is_empty() { values.insert(n, v); }
                        }
                    }
                }
                let textareas = form_el.get_elements_by_tag_name("textarea");
                for i in 0..textareas.length() {
                    if let Some(el) = textareas.item(i) {
                        if let Ok(ta) = el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                            let n = ta.name();
                            let v = ta.value();
                            if !n.is_empty() { values.insert(n, v); }
                        }
                    }
                }
                leptos::logging::log!("[TOKEN_ACTION] submit '{}' values: {:?}", form_id, values);
            }
        }
        ActionRegistry::global().execute(&format!("submit:{}", form_id));
    } else if let Some(drag_id) = name.strip_prefix("drag:") {
        leptos::logging::log!("[TOKEN_ACTION] drag -> {}", drag_id);
        #[cfg(target_arch = "wasm32")]
        dom::trigger_file_input(Some("image/*"), false);
        ActionRegistry::global().execute(name);
    } else if let Some(rest) = name.strip_prefix("cycle:") {
        leptos::logging::log!("[TOKEN_ACTION] cycle -> {}", rest);
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0];
            let values: Vec<&str> = parts[1].split(',').collect();
            if !values.is_empty() {
                ctx.strings.update(|m| {
                    let current = m.get(key).cloned().unwrap_or_else(|| values[0].to_string());
                    let idx = values.iter().position(|&v| v == current).unwrap_or(0);
                    let next = values[(idx + 1) % values.len()];
                    m.insert(key.to_string(), next.to_string());
                });
            }
        }
    } else if let Some(val_key) = name.strip_prefix("val:") {
        leptos::logging::log!("[TOKEN_ACTION] val -> {}", val_key);
        #[cfg(target_arch = "wasm32")]
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.get_element_by_id(val_key) {
                if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                    let value = input.value();
                    ctx.set_string(val_key, value);
                }
            }
        }
    } else if let Some(query) = name.strip_prefix("search:") {
        leptos::logging::log!("[TOKEN_ACTION] search -> {}", query);
        ActionRegistry::global().execute(name);
    } else if let Some(handler) = name.strip_prefix("scroll:") {
        leptos::logging::log!("[TOKEN_ACTION] scroll -> {}", handler);
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            let handler = handler.to_string();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                ActionRegistry::global().execute(&format!("scroll:{}", handler));
            }) as Box<dyn FnMut(_)>);
            let _ = win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    } else if let Some(endpoint) = name.strip_prefix("inf:") {
        leptos::logging::log!("[TOKEN_ACTION] inf -> {}", endpoint);
        #[cfg(target_arch = "wasm32")]
        {
            let endpoint = endpoint.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let promise = window.fetch_with_str(&endpoint);
                    let resp = wasm_bindgen_futures::JsFuture::from(promise).await;
                    if let Ok(resp) = resp {
                        if let Ok(response) = resp.dyn_into::<web_sys::Response>() {
                            if let Ok(text_promise) = response.text() {
                                let text = wasm_bindgen_futures::JsFuture::from(text_promise).await;
                                if let Ok(text) = text {
                                    if let Some(text_str) = text.as_string() {
                                        ctx.set_string(&format!("inf:{}", endpoint), text_str);
                                        ctx.bump_list_rev();
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    } else if let Some(endpoint) = name.strip_prefix("fetch:get:") {
        leptos::logging::log!("[TOKEN_ACTION] fetch:get -> {}", endpoint);
        #[cfg(target_arch = "wasm32")]
        {
            let endpoint = endpoint.to_string();
            let ctx = ctx;
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let promise = window.fetch_with_str(&endpoint);
                    let resp = wasm_bindgen_futures::JsFuture::from(promise).await;
                    if let Ok(resp) = resp {
                        if let Ok(response) = resp.dyn_into::<web_sys::Response>() {
                            if let Ok(text_promise) = response.text() {
                                let text = wasm_bindgen_futures::JsFuture::from(text_promise).await;
                                if let Ok(text) = text {
                                    if let Some(text_str) = text.as_string() {
                                        leptos::logging::log!("[TOKEN_ACTION] fetch:get response: {}", text_str);
                                        ctx.set_string(&format!("fetch:{}", endpoint), text_str);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    } else {
        ActionRegistry::global().execute(name);
    }
}
