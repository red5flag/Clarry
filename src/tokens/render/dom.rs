// src/tokens/render/dom.rs
//
// DOM fallback helpers (WASM-only).

#![allow(unused_variables)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

pub fn show_element(_show: &str, _hide: &[&str]) {
    #[cfg(target_arch = "wasm32")] {
        let doc = web_sys::window().unwrap().document().unwrap();
        for id in _hide {
            if let Some(el) = doc.get_element_by_id(id) {
                let _ = el.unchecked_into::<web_sys::HtmlElement>()
                    .style().set_property("display", "none");
            }
        }
        if let Some(el) = doc.get_element_by_id(_show) {
            let _ = el.unchecked_into::<web_sys::HtmlElement>()
                .style().set_property("display", "");
        }
    }
}

pub fn set_element_text(id: &str, text: &str) {
    #[cfg(target_arch = "wasm32")] {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.get_element_by_id(id) {
                el.set_text_content(Some(text));
            }
        }
    }
}

pub fn trigger_file_input(accept: Option<&str>, multiple: bool) {
    #[cfg(target_arch = "wasm32")] {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(input) = doc.create_element("input").ok().and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                let _ = input.set_attribute("type", "file");
                if let Some(a) = accept {
                    let _ = input.set_attribute("accept", a);
                }
                input.set_multiple(multiple);
                let _ = input.click();
            }
        }
    }
}

pub fn set_style(target: &str, property: &str, value: &str) {
    #[cfg(target_arch = "wasm32")] {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.get_element_by_id(target) {
                let _ = el.dyn_into::<web_sys::HtmlElement>()
                    .map(|e| e.style().set_property(property, value));
            }
        }
    }
}

pub fn update_tab_visuals(active_tab: &str, inactive_tabs: &[&str]) {
    #[cfg(target_arch = "wasm32")] {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.get_element_by_id(active_tab) {
                let class_list = el.unchecked_into::<web_sys::HtmlElement>().class_list();
                let active_arr = web_sys::js_sys::Array::new();
                let _ = active_arr.push(&"border-t-2".into());
                let _ = active_arr.push(&"border-white".into());
                let _ = active_arr.push(&"font-semibold".into());
                let _ = class_list.add(&active_arr);
                let inactive_arr = web_sys::js_sys::Array::new();
                let _ = inactive_arr.push(&"text-gray-400".into());
                let _ = class_list.remove(&inactive_arr);
            }
            for id in inactive_tabs {
                if let Some(el) = doc.get_element_by_id(id) {
                    let class_list = el.unchecked_into::<web_sys::HtmlElement>().class_list();
                    let active_arr = web_sys::js_sys::Array::new();
                    let _ = active_arr.push(&"border-t-2".into());
                    let _ = active_arr.push(&"border-white".into());
                    let _ = active_arr.push(&"font-semibold".into());
                    let _ = class_list.remove(&active_arr);
                    let inactive_arr = web_sys::js_sys::Array::new();
                    let _ = inactive_arr.push(&"text-gray-400".into());
                    let _ = class_list.add(&inactive_arr);
                }
            }
        }
    }
}
