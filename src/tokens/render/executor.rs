// src/tokens/render/executor.rs
//
// Reactive token action executor.
// Iterative; no recursion. Updates TokenCtx signals instead of poking the DOM.

use leptos::prelude::*;

use crate::tokens::action::{TokenAction, LogLevel, ActionRegistry};
use crate::tokens::reactive::TokenCtx;
use crate::tokens::debug::inspector_log;
use crate::tokens::storage::Store;

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
                    // Update reactive classes so the tab styling is driven by the same signal
                    ctx.classes.update(|m| {
                        let active_entry = m.entry(active_tab.clone()).or_default();
                        active_entry.retain(|c| c != "text-gray-400");
                        for cls in ["border-t-2", "border-white", "font-semibold"] {
                            if !active_entry.contains(&cls.to_string()) {
                                active_entry.push(cls.to_string());
                            }
                        }
                        for tab in &inactive_tabs {
                            let inactive_entry = m.entry(tab.clone()).or_default();
                            inactive_entry.retain(|c| c != "border-t-2" && c != "border-white" && c != "font-semibold");
                            if !inactive_entry.contains(&"text-gray-400".to_string()) {
                                inactive_entry.push("text-gray-400".to_string());
                            }
                        }
                    });
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
                let val = value.to_string();
                Store::write(key, &val);
                ctx.set_string(key, val);
                ctx.bump_list_rev();
            }
            TokenAction::StoreSetTtl { key, value, ttl_seconds } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreSetTtl - key: {}, ttl: {}s", key, ttl_seconds);
                let val = value.to_string();
                Store::write(key, &val);
                ctx.set_string(key, val);
                ctx.bump_list_rev();
                // TTL is handled at cache layer; backend persists regardless
                let _ttl = *ttl_seconds;
            }
            TokenAction::Preload { key, endpoint } => {
                leptos::logging::log!("[TOKEN_ACTION] Preload - key: {}, endpoint: {}", key, endpoint);
                #[cfg(target_arch = "wasm32")]
                {
                    let endpoint = endpoint.to_string();
                    let key_str = key.to_string();
                    let strings_signal = ctx.strings;
                    let list_rev_signal = ctx.list_rev;
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            let promise = window.fetch_with_str(&endpoint);
                            match wasm_bindgen_futures::JsFuture::from(promise).await {
                                Ok(resp) => {
                                    if let Ok(response) = resp.dyn_into::<web_sys::Response>() {
                                        if response.ok() {
                                            if let Ok(text_promise) = response.text() {
                                                match wasm_bindgen_futures::JsFuture::from(text_promise).await {
                                                    Ok(text) => {
                                                        if let Some(text_str) = text.as_string() {
                                                            leptos::logging::log!("[TOKEN_ACTION] Preload success key={} len={}", key_str, text_str.len());
                                                            Store::write(&key_str, &text_str);
                                                            strings_signal.update(|m| { m.insert(key_str.clone(), text_str); });
                                                            list_rev_signal.update(|n| *n += 1);
                                                        }
                                                    }
                                                    Err(e) => leptos::logging::log!("[TOKEN_ACTION] Preload text() failed: {:?}", e),
                                                }
                                            }
                                        } else {
                                            let status = response.status();
                                            leptos::logging::log!("[TOKEN_ACTION] Preload HTTP error: {}", status);
                                            let msg = format!("HTTP {}", status);
                                            strings_signal.update(|m| { m.insert(key_str.clone(), msg); });
                                            list_rev_signal.update(|n| *n += 1);
                                        }
                                    }
                                }
                                Err(e) => leptos::logging::log!("[TOKEN_ACTION] Preload fetch failed: {:?}", e),
                            }
                        }
                    });
                }
            }
            TokenAction::Watch { key } => {
                leptos::logging::log!("[TOKEN_ACTION] Watch - key: {}", key);
                if let Some(val) = Store::read(key) {
                    leptos::logging::log!("[TOKEN_ACTION] Watch updated key: {} -> {}", key, &val);
                    ctx.set_string(key, val);
                    ctx.bump_list_rev();
                }
            }
            TokenAction::StoreGet { key, target } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreGet - key: {}", key);
                if let Some(val) = Store::read(key) {
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
            TokenAction::StoreDelete { key } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreDelete - key: {}", key);
                // Write empty string to the specific key, not delete_root which would
                // wipe all sibling keys sharing the same root (e.g. demo.note would
                // destroy demo.items, demo.tags etc.)
                Store::write(key, "");
                ctx.set_string(key, "");
                ctx.bump_list_rev();
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
                let id = key.to_string();
                // Toggle visibility: if not explicitly set, default is visible
                // so first click hides.  If already hidden (seeded), first click shows.
                let current = ctx.visibility.get().get(&id).copied();
                let new_val = match current {
                    Some(v) => !v,
                    None    => false, // visible → hide
                };
                ctx.visibility.update(|m| { m.insert(id.clone(), new_val); });
                // Sync extra classes so Tailwind "hidden" tracks the signal.
                if new_val {
                    ctx.classes.update(|m| {
                        if let Some(list) = m.get_mut(&id) {
                            list.retain(|c| c != "hidden");
                        }
                    });
                } else {
                    ctx.classes.update(|m| {
                        m.entry(id.clone()).or_default().retain(|c| c != "hidden");
                        m.entry(id).or_default().push("hidden".to_string());
                    });
                }
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
            TokenAction::StorePush { key, input_key } => {
                leptos::logging::log!("[TOKEN_ACTION] StorePush - key: {}, input_key: {}", key, input_key);
                let input_val = ctx.strings.get().get(input_key.as_ref()).cloned()
                    .or_else(|| Store::read(input_key));
                if let Some(val) = input_val.filter(|v| !v.is_empty()) {
                    // Read array from ctx.strings first (most up-to-date), then Store
                    let arr_json = ctx.strings.get().get(key.as_ref()).cloned()
                        .filter(|s| !s.is_empty())
                        .or_else(|| Store::read(key))
                        .unwrap_or_else(|| "[]".to_string());
                    let mut arr: Vec<serde_json::Value> = serde_json::from_str(&arr_json).unwrap_or_default();
                    // Always push — duplicates allowed
                    arr.push(serde_json::json!({"text": val}));
                    let new_json = serde_json::to_string(&arr).unwrap_or_default();
                    Store::write(key, &new_json);
                    ctx.set_string(key, new_json);
                    ctx.bump_list_rev();
                    // Clear the input signal AND the DOM element value
                    ctx.set_string(input_key, "".to_string());
                    #[cfg(target_arch = "wasm32")]
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(el) = doc.get_element_by_id(input_key.as_ref()) {
                            if let Ok(inp) = el.dyn_into::<web_sys::HtmlInputElement>() {
                                let _ = inp.set_value("");
                            }
                        }
                    }
                }
            }
            TokenAction::StoreRemove { key, input_key } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreRemove - key: {}, input_key: {}", key, input_key);
                let input_val = ctx.strings.get().get(input_key.as_ref()).cloned()
                    .or_else(|| Store::read(input_key));
                if let Some(val) = input_val.filter(|v| !v.is_empty()) {
                    // Read array from ctx.strings first (most up-to-date), then Store
                    let arr_json = ctx.strings.get().get(key.as_ref()).cloned()
                        .filter(|s| !s.is_empty())
                        .or_else(|| Store::read(key))
                        .unwrap_or_else(|| "[]".to_string());
                    let mut arr: Vec<serde_json::Value> = serde_json::from_str(&arr_json).unwrap_or_default();
                    // Remove only the FIRST matching item (not all duplicates)
                    let mut removed = false;
                    arr.retain(|item| {
                        if !removed && item.get("text").and_then(|v| v.as_str()) == Some(&val) {
                            removed = true;
                            false
                        } else {
                            true
                        }
                    });
                    let new_json = serde_json::to_string(&arr).unwrap_or_default();
                    Store::write(key, &new_json);
                    ctx.set_string(key, new_json);
                    ctx.bump_list_rev();
                    // Do NOT clear the input — DOM doesn't sync from signal,
                    // so clearing signal only causes silent failure on re-add
                }
            }
            TokenAction::StoreWriteToPath { path_input, val_input } => {
                leptos::logging::log!("[TOKEN_ACTION] StoreWriteToPath - path_input: {}, val_input: {}", path_input, val_input);
                let strings = ctx.strings.get();
                let path = strings.get(path_input.as_ref()).cloned()
                    .or_else(|| Store::read(path_input))
                    .unwrap_or_default();
                let val = strings.get(val_input.as_ref()).cloned()
                    .or_else(|| Store::read(val_input))
                    .unwrap_or_default();
                if !path.is_empty() {
                    Store::write(&path, &val);
                    ctx.set_string(&path, val.clone());
                    ctx.set_string("storage.last_written", format!("{} = {}", path, val));
                    ctx.bump_list_rev();
                    ctx.set_string(path_input, "".to_string());
                    ctx.set_string(val_input, "".to_string());
                    #[cfg(target_arch = "wasm32")]
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        for eid in [path_input.as_ref(), val_input.as_ref()] {
                            if let Some(el) = doc.get_element_by_id(eid) {
                                if let Ok(inp) = el.dyn_into::<web_sys::HtmlInputElement>() {
                                    let _ = inp.set_value("");
                                }
                            }
                        }
                    }
                }
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
    } else if let Some(encoded) = name.strip_prefix("copy_with_toast:") {
        let text = encoded.replace("\x3A", ":").replace("\x0A", "\n");
        leptos::logging::log!("[TOKEN_ACTION] copy_with_toast -> {} chars", text.len());
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            // Copy to clipboard
            if let Ok(nav) = win.navigator().dyn_into::<web_sys::Navigator>() {
                let _ = nav.clipboard().write_text(&text);
            }
            // Spawn cursor-following "Copied!" toast via eval
            if let Some(doc) = win.document() {
                if let Ok(el) = doc.create_element("div") {
                    let _ = el.set_attribute("style",
                        "position:fixed;z-index:9999;pointer-events:none;\
                         background:#1f2937;color:#fff;font-size:12px;font-weight:600;\
                         padding:4px 10px;border-radius:6px;white-space:nowrap;\
                         transform:translate(-50%,-130%);transition:opacity 0.3s;opacity:1;");
                    let _ = el.set_text_content(Some("Copied!"));
                    // Position at center of viewport initially; mousemove handler will update
                    let _ = el.set_attribute("id", "_tok_copy_toast");
                    if let Some(body) = doc.body() {
                        let _ = body.append_child(&el);
                        // Use setTimeout to remove after 1.5s
                        let win2 = win.clone();
                        let cb = wasm_bindgen::closure::Closure::once(move || {
                            if let Some(doc2) = win2.document() {
                                if let Some(toast) = doc2.get_element_by_id("_tok_copy_toast") {
                                    let _ = toast.remove();
                                }
                            }
                        });
                        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(), 1500
                        );
                        cb.forget();
                    }
                }
                // Track mouse to follow cursor
                let js_code = "
                    (function() {
                        var t = document.getElementById('_tok_copy_toast');
                        if (!t) return;
                        var h = function(e) {
                            t.style.left = e.clientX + 'px';
                            t.style.top  = e.clientY + 'px';
                        };
                        document.addEventListener('mousemove', h, {passive: true});
                        setTimeout(function() {
                            document.removeEventListener('mousemove', h);
                        }, 1600);
                    })();
                ";
                let _ = js_sys::eval(js_code);
            }
        }
    } else if let Some(key) = name.strip_prefix("toggle:") {
        leptos::logging::log!("[TOKEN_ACTION] toggle -> {}", key);
        let key_str = key.to_string();
        let current = ctx.visibility.get().get(&key_str).copied();
        let new_val = match current {
            Some(v) => !v,
            None    => false,
        };
        ctx.visibility.update(|m| { m.insert(key_str.clone(), new_val); });
        if new_val {
            ctx.classes.update(|m| {
                if let Some(list) = m.get_mut(&key_str) {
                    list.retain(|c| c != "hidden");
                }
            });
        } else {
            ctx.classes.update(|m| {
                m.entry(key_str.clone()).or_default().retain(|c| c != "hidden");
                m.entry(key_str).or_default().push("hidden".to_string());
            });
        }
    } else if name == "dismiss_toast" {
        leptos::logging::log!("[TOKEN_ACTION] dismiss_toast");
        // Dismiss the most recent toast (or all if needed)
        ctx.toasts.update(|v| { v.pop(); });
    } else if name == "show_toast" {
        leptos::logging::log!("[TOKEN_ACTION] show_toast");
        ctx.show_toast("Toast notification");
    } else if let Some(path) = name.strip_prefix("store_inc:") {
        leptos::logging::log!("[TOKEN_ACTION] store_inc -> {}", path);
        use crate::tokens::storage::primitive::Store;
        let current = Store::read(path).unwrap_or_else(|| "0".to_string());
        let count: i32 = current.parse().unwrap_or(0);
        let next = (count + 1).to_string();
        Store::write(path, &next);
        ctx.set_string(path, next.clone());
        ctx.bump_list_rev();
    } else if let Some(path) = name.strip_prefix("store_toggle:") {
        leptos::logging::log!("[TOKEN_ACTION] store_toggle -> {}", path);
        use crate::tokens::storage::primitive::Store;
        let current = Store::read(path).unwrap_or_else(|| "false".to_string());
        let next = if current == "true" { "false" } else { "true" };
        Store::write(path, next);
        ctx.set_string(path, next.to_string());
        ctx.bump_list_rev();
    } else if let Some(rest) = name.strip_prefix("chat_send:") {
        leptos::logging::log!("[TOKEN_ACTION] chat_send -> {}", rest);
        let parts: Vec<&str> = rest.splitn(3, ':').collect();
        if parts.len() >= 2 {
            let input_key = parts[0];
            let storage_key = parts[1];
            let sender = parts.get(2).unwrap_or(&"me");
            let text = ctx.strings.get().get(input_key).cloned().unwrap_or_default();
            if !text.is_empty() {
                use crate::tokens::storage::primitive::Store;
                // Write to user.message.# → auto-append under storage_key root
                let root = storage_key.split('.').next().unwrap_or(storage_key);
                let array_path = format!("{}.#", storage_key);
                let count = Store::read(storage_key)
                    .and_then(|s| serde_json::from_str::<Vec<serde_json::Value>>(&s).ok())
                    .map(|v| v.len())
                    .unwrap_or(0);
                let msg_json = format!(
                    r#"{{"id":"msg_{}","text":"{}","sender":"{}","timestamp":"now"}}"#,
                    count + 1,
                    text.replace('"', "\\\""),
                    sender
                );
                Store::write(&array_path, &msg_json);
                // Update reactive strings so text_bind / chat_messages refresh
                if let Some(updated_array) = Store::read(storage_key) {
                    ctx.set_string(storage_key, updated_array);
                }
                if let Some(updated_root) = Store::read(root) {
                    ctx.set_string(root, updated_root);
                }
                ctx.bump_list_rev();
                // Clear input
                ctx.set_string(input_key, "");
            }
        }
    } else if let Some(rest) = name.strip_prefix("store_set_input:") {
        leptos::logging::log!("[TOKEN_ACTION] store_set_input -> {}", rest);
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0];
            let input_key = parts[1];
            // Read from live ctx.strings first (set by input_handler), fallback to Store
            let val = ctx.strings.get().get(input_key).cloned()
                .or_else(|| Store::read(input_key))
                .unwrap_or_default();
            Store::write(key, &val);
            ctx.set_string(key, val.clone());
            ctx.bump_list_rev();
            // Clear the input signal AND the DOM element value
            ctx.set_string(input_key, "".to_string());
            #[cfg(target_arch = "wasm32")]
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(el) = doc.get_element_by_id(input_key) {
                    if let Ok(inp) = el.dyn_into::<web_sys::HtmlInputElement>() {
                        let _ = inp.set_value("");
                    }
                }
            }
        }
    } else if let Some(id) = name.strip_prefix("toggle_drawer:") {
        leptos::logging::log!("[TOKEN_ACTION] toggle_drawer -> {}", id);
        let id_str = id.to_string();
        let current = ctx.visibility.get().get(&id_str).copied();
        let new_val = match current {
            Some(v) => !v,
            None    => false,
        };
        ctx.visibility.update(|m| { m.insert(id_str.clone(), new_val); });
        if new_val {
            ctx.classes.update(|m| {
                if let Some(list) = m.get_mut(&id_str) {
                    list.retain(|c| c != "hidden");
                }
            });
        } else {
            ctx.classes.update(|m| {
                m.entry(id_str.clone()).or_default().retain(|c| c != "hidden");
                m.entry(id_str).or_default().push("hidden".to_string());
            });
        }
    } else if let Some(ids_str) = name.strip_prefix("cycle_drawer:") {
        leptos::logging::log!("[TOKEN_ACTION] cycle_drawer -> {}", ids_str);
        let ids: Vec<String> = ids_str.split(',').map(|s| s.to_string()).collect();
        if !ids.is_empty() {
            let vis = ctx.visibility.get();
            let current_idx = ids.iter().position(|id| vis.get(id).copied().unwrap_or(false));
            // Hide all first
            for id in &ids {
                ctx.visibility.update(|m| { m.insert(id.clone(), false); });
            }
            // Show next (or none if at end)
            if let Some(idx) = current_idx {
                let next_idx = (idx + 1) % ids.len();
                if next_idx != idx || ids.len() == 1 {
                    ctx.visibility.update(|m| { m.insert(ids[next_idx].clone(), true); });
                }
            } else {
                // None open → open first
                ctx.visibility.update(|m| { m.insert(ids[0].clone(), true); });
            }
        }
    } else if let Some(rest) = name.strip_prefix("store_from_val:") {
        leptos::logging::log!("[TOKEN_ACTION] store_from_val -> {}", rest);
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() == 2 {
            let storage_key = parts[0];
            let val_key = parts[1];
            let val = ctx.strings.get().get(val_key).cloned().unwrap_or_default();
            Store::write(storage_key, &val);
            ctx.set_string(storage_key, &val);
            ctx.bump_list_rev();
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
