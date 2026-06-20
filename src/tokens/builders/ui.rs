// src/tokens/builders/ui.rs
//
// Advanced UI primitives: modal, tabs, accordion, media, overlays, etc.

use std::sync::Arc;
use leptos::prelude::*;
use crate::tokens::node::{IntoToken, Layout, Str, TokenNode};
use crate::tokens::core::id::next_id;
use crate::tokens::action::TokenAction;
use super::spec::TokenBuilder;
use super::types::{Container, Row, Block, Col, Text};
use super::factory::{row, col, block, text, btn};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

// ── Layout / overlay primitives ───────────────────────────────────────────

pub fn grid2() -> Row {
    row().set_class("display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:0.75rem;")
}

pub fn grid3() -> Row {
    row().set_class("display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:2px;padding:2px;")
}

pub fn skeleton(w: f32, h: f32) -> Block {
    block()
        .w(w).h(h)
        .set_class("bg-gray-200 rounded animate-pulse")
}

pub fn overlay() -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "fixed inset-0 z-50 flex items-center justify-center".into();
    n.style.extra = "background:rgba(0,0,0,0.5);".into();
    Container { stack: vec![n] }
}

pub fn portal(target_id: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(target_id);
    n.tag = "div".into();
    n.style.extra = "position:absolute;".into();
    Container { stack: vec![n] }
}

pub fn split(ratio: f32) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.layout = Layout::Row;
    n.style.extra = format!("display:flex;--split-ratio:{};", ratio).into();
    Container { stack: vec![n] }
}

pub fn aspect(w: u16, h: u16) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.style.extra = format!("aspect-ratio:{}/{};", w, h).into();
    Container { stack: vec![n] }
}

pub fn tooltip(_target_id: impl Into<Str>, content: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "absolute z-40 px-2 py-1 text-xs bg-gray-900 text-white rounded opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none".into();
    n.content = Some(content.into());
    Container { stack: vec![n] }
}

pub fn drawer(id: impl Into<Str>, side: impl Into<Str>, content: impl Into<Str>) -> Block {
    use crate::tokens::action::types::TokenAction;
    let id_str: Str = id.into();
    let side = side.into().to_string();
    let mut n = TokenNode::new(id_str.clone());
    n.tag = "div".into();
    n.class = "fixed z-50 bg-white shadow-lg p-4 flex flex-col gap-2".into();
    let (position, size) = match side.as_str() {
        "left" => ("left:0;top:0;bottom:0;", "width:16rem;"),
        "right" => ("right:0;top:0;bottom:0;", "width:16rem;"),
        "top" => ("top:0;left:0;right:0;", "height:16rem;"),
        "bottom" => ("bottom:0;left:0;right:0;", "height:16rem;"),
        _ => ("right:0;top:0;bottom:0;", "width:16rem;"),
    };
    n.style.extra = format!("{}{}display:none;", position, size).into();

    // Auto-inject close button
    let mut close_btn = TokenNode::new(format!("{}_close", id_str));
    close_btn.tag = "button".into();
    close_btn.content = Some("✕".into());
    close_btn.class = "self-end text-gray-500 hover:text-gray-800 text-lg leading-none mb-1 cursor-pointer".into();
    close_btn.actions.push(TokenAction::Hide(id_str.clone()));
    n.children.push(close_btn);

    // Content wrapper
    let mut content_node = TokenNode::new(format!("{}_content", id_str));
    content_node.tag = "div".into();
    content_node.content = Some(content.into());
    n.children.push(content_node);

    Container { stack: vec![n] }
}

// ── Modal / Tabs / Accordion ──────────────────────────────────────────────

// Helpers for tab/accordion item DSL
pub fn tab(title: impl Into<Str>, content: impl IntoToken) -> (Str, TokenNode) {
    (title.into(), content.into_node())
}

pub fn section(title: impl Into<Str>, content: impl IntoToken) -> (Str, TokenNode) {
    (title.into(), content.into_node())
}

// ── Media primitives ───────────────────────────────────────────────────────

pub fn video(src: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "video".into();
    n.content = Some(src.into());
    n.style.extra = "width:100%;height:auto;".into();
    n.attributes.insert("controls".into(), "".into());
    Container { stack: vec![n] }
}

pub fn video_ambient(src: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "video".into();
    n.content = Some(src.into());
    n.style.extra = "width:100%;height:100%;object-fit:cover;".into();
    n.attributes.insert("autoplay".into(), "".into());
    n.attributes.insert("loop".into(), "".into());
    n.attributes.insert("muted".into(), "".into());
    n.attributes.insert("playsinline".into(), "".into());
    Container { stack: vec![n] }
}

pub fn audio(src: impl Into<Str>) -> Block { audio_player(src) }
pub fn audio_player(src: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "audio".into();
    n.content = Some(src.into());
    n.attributes.insert("controls".into(), "".into());
    n.style.extra = "width:100%;".into();
    Container { stack: vec![n] }
}

pub fn model(src: impl Into<Str>) -> Block { model_viewer(src) }
pub fn model_viewer(src: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "model-viewer".into();
    n.content = Some(src.into());
    n.style.extra = "width:100%;height:300px;".into();
    n.attributes.insert("camera-controls".into(), "".into());
    n.attributes.insert("auto-rotate".into(), "".into());
    Container { stack: vec![n] }
}

pub fn iframe(src: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "iframe".into();
    n.content = Some(src.into());
    n.style.extra = "width:100%;height:100%;border:none;".into();
    Container { stack: vec![n] }
}

// ── Chat primitives ────────────────────────────────────────────────────────

pub fn chat_bubble(text_content: impl Into<Str>, mine: bool) -> Block {
    let (bg, align) = if mine {
        ("background:#2563eb;color:#fff;", "align-self:flex-end;")
    } else {
        ("background:#f3f4f6;color:#111827;", "align-self:flex-start;")
    };
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.content = Some(text_content.into());
    n.style.extra = format!("{bg}{align}padding:0.5rem 1rem;border-radius:1rem;max-width:70%;margin:0.25rem 0;").into();
    Container { stack: vec![n] }
}

pub fn chat_ui(messages_key: impl Into<Str>, _send_action: TokenAction) -> Block {
    use crate::tokens::reactive::TokenCtx;
    let key: Str = messages_key.into();
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "flex flex-col h-full border rounded-lg overflow-hidden".into();
    n.dynamic_content = Some(Arc::new(move || {
        let key_clone = key.clone();
        let _key_clone2 = key.clone();
        view! {
            <div class="flex flex-col h-full">
                <div class="flex-1 overflow-y-auto p-4 space-y-2">
                    {move || {
                        let ctx = use_context::<TokenCtx>();
                        let msgs = ctx.as_ref().map(|c| c.room_messages(key_clone.to_string()).get()).unwrap_or_default();
                        msgs.into_iter().map(|msg| view! {
                            <div class="bg-gray-100 rounded-lg px-3 py-2 text-sm">{msg}</div>
                        }.into_any()).collect::<Vec<_>>()
                    }}
                </div>
                <div class="border-t p-3 gap-2 flex items-center">
                    <input id="chat_input" class="flex-1 p-2 border rounded-full" placeholder="Type a message..." />
                    <button
                        class="bg-blue-600 text-white px-4 py-2 rounded-full text-sm"
                        on:click=move |_| {
                            #[cfg(target_arch = "wasm32")]
                            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                if let Some(el) = doc.get_element_by_id("chat_input") {
                                    if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                                        let val = input.value();
                                        if !val.is_empty() {
                                            if let Some(c) = use_context::<TokenCtx>() {
                                                c.add_message(&_key_clone2.to_string(), val);
                                            }
                                            input.set_value("");
                                        }
                                    }
                                }
                            }
                        }
                    >
                        "Send"
                    </button>
                </div>
            </div>
        }.into_any()
    }));
    Container { stack: vec![n] }
}

// ── Data display primitives ─────────────────────────────────────────────────

pub fn qr_code(data: impl Into<Str>, id: impl Into<Str>) -> Block {
    let id_str: Str = id.into();
    let data_str: Str = data.into();
    crate::tokens::islands::island(move || {
        view! {
            <div id={id_str.as_ref()} style="display:inline-block;" data-qr={data_str.as_ref()}>
                <canvas id={format!("{}_canvas", id_str)}></canvas>
            </div>
        }.into_any()
    })
}

pub fn progress_bar(value_key: impl Into<Str>, _max: u32) -> Block {
    use crate::tokens::reactive::TokenCtx;
    let key: Str = value_key.into();
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "w-full bg-gray-200 rounded-full h-2".into();
    n.dynamic_content = Some(Arc::new(move || {
        let ctx = use_context::<TokenCtx>();
        let pct = ctx.as_ref().and_then(|c| c.strings.get().get(key.as_ref()).cloned().unwrap_or_default().parse::<u32>().ok()).unwrap_or(0);
        let width = format!("{}%", pct.min(100));
        view! {
            <div class="w-full bg-gray-200 rounded-full h-2">
                <div class="bg-blue-600 h-2 rounded-full transition-all duration-300" style=format!("width:{}", width)></div>
            </div>
        }.into_any()
    }));
    Container { stack: vec![n] }
}

pub fn rating(id: impl Into<Str>, max: u8) -> Block {
    use crate::tokens::reactive::TokenCtx;
    let id_str: Str = id.into();
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "flex gap-1".into();
    n.dynamic_content = Some(Arc::new(move || {
        let ctx = use_context::<TokenCtx>();
        let current = ctx.as_ref().and_then(|c| c.strings.get().get(id_str.as_ref()).cloned().unwrap_or_default().parse::<u8>().ok()).unwrap_or(0);
        let stars: Vec<AnyView> = (1..=max).map(|i| {
            let id_clone = id_str.clone();
            let is_selected = i <= current;
            view! {
                <button
                    class=move || if is_selected { "text-2xl text-yellow-400 cursor-pointer transition-colors" } else { "text-2xl text-gray-300 hover:text-yellow-400 cursor-pointer transition-colors" }
                    on:click=move |_| {
                        if let Some(c) = use_context::<TokenCtx>() {
                            c.set_string(id_clone.as_ref(), i.to_string());
                        }
                    }
                >
                    "★"
                </button>
            }.into_any()
        }).collect();
        view! { <div class="flex gap-1">{stars}</div> }.into_any()
    }));
    Container { stack: vec![n] }
}

pub fn skeleton_text(lines: u8) -> Col {
    col().set_class("space-y-2").add_all((0..lines).map(|i| {
        skeleton(if i == lines - 1 { 16.0 } else { 24.0 }, 1.0)
    }))
}

pub fn badge(label: impl Into<Str>, color: &str) -> Text {
    text(label).append_css(format!(
        "display:inline-flex;align-items:center;padding:0.125rem 0.5rem;\
         border-radius:9999px;font-size:0.75rem;font-weight:600;\
         background:{color};color:#fff;"
    ))
}

pub fn chip(label: impl Into<Str>, on_remove: Option<TokenAction>) -> Block {
    let mut c = block()
        .set_class("inline-flex items-center gap-1 px-3 py-1 rounded-full bg-gray-100 text-sm");
    c = c.add(text(label));
    if let Some(action) = on_remove {
        c = c.add(btn("×").set_class("text-gray-500 hover:text-gray-700 ml-1").push_action(action));
    }
    c
}

pub fn divider() -> Block {
    block().set_class("w-full h-px bg-gray-200 my-2")
}

pub fn spacer(rem: f32) -> Block {
    block().append_css(format!("height:{rem:.2}rem;flex-shrink:0;"))
}

pub fn copy_block(text_content: impl Into<Str> + Clone) -> Block {
    let t: Str = text_content.into();
    // Encode text for safe embedding in the custom action string (replace : with \x3A)
    let safe = t.replace(':', "\x3A").replace('\n', "\x0A");
    let copy_toast_action = TokenAction::Custom(
        format!("copy_with_toast:{}", safe).into()
    );
    block()
        .set_class("relative group")
        .add(
            block()
                .set_class("bg-gray-900 text-green-400 font-mono text-sm p-4 rounded-lg overflow-x-auto whitespace-pre")
                .add(text(t))
        )
        .add(
            btn("Copy")
                .set_class("absolute top-2 right-2 text-xs bg-gray-700 hover:bg-gray-600 text-white px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-all")
                .push_action(copy_toast_action)
        )
}

pub fn toast_container(_signal_key: impl Into<Str>) -> Block {
    use crate::tokens::reactive::TokenCtx;
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "fixed bottom-4 right-4 z-50 flex flex-col gap-2".into();
    n.dynamic_content = Some(Arc::new(move || {
        let ctx = use_context::<TokenCtx>();
        let toasts = ctx.map(|c| c.toast_fn()()).unwrap_or_default();
        if toasts.is_empty() {
            return view! { <div></div> }.into_any();
        }
        let views: Vec<AnyView> = toasts.into_iter().map(|(id, msg)| {
            let id_clone = id.clone();
            let _id_clone2 = id.clone();
            view! {
                <div class="bg-white shadow-lg rounded-lg px-4 py-3 flex items-center gap-3 min-w-64 border-l-4 border-blue-500">
                    <span class="flex-1 text-sm">{msg}</span>
                    <button
                        class="text-gray-400 hover:text-gray-600"
                        on:click=move |_| {
                            if let Some(c) = use_context::<TokenCtx>() {
                                c.dismiss_toast(&id_clone);
                            }
                        }
                    >
                        "×"
                    </button>
                </div>
            }.into_any()
        }).collect();
        view! { <div>{views}</div> }.into_any()
    }));
    Container { stack: vec![n] }
}

// ── Terminal / embedded primitives ────────────────────────────────────────

pub fn terminal(id: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(id);
    n.tag = "pre".into();
    n.class = "font-mono text-sm bg-gray-900 text-green-400 p-4 overflow-auto".into();
    Container { stack: vec![n] }
}

pub fn log_view(source: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "pre".into();
    n.class = "font-mono text-xs bg-black text-gray-300 p-3 overflow-auto max-h-48".into();
    n.data_binding = Some(crate::tokens::node::DataBinding::one_way(source.into()));
    Container { stack: vec![n] }
}

pub fn hex_view(bytes_key: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "pre".into();
    n.class = "font-mono text-xs bg-gray-100 text-gray-800 p-2 overflow-auto".into();
    n.data_binding = Some(crate::tokens::node::DataBinding::one_way(bytes_key.into()));
    Container { stack: vec![n] }
}

pub fn tree_view(data_key: impl Into<Str>) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "ul".into();
    n.class = "text-sm space-y-1".into();
    n.data_binding = Some(crate::tokens::node::DataBinding::one_way(data_key.into()));
    Container { stack: vec![n] }
}

pub fn shortcut(keys: impl Into<Str>, action: TokenAction) -> Block {
    let mut n = TokenNode::new(next_id());
    n.tag = "kbd".into();
    n.content = Some(keys.into());
    n.class = "inline-flex items-center gap-1 px-2 py-0.5 text-xs font-mono text-gray-600 bg-gray-100 border border-gray-300 rounded".into();
    n.actions.push(action);
    Container { stack: vec![n] }
}

// ── Accessibility primitives ────────────────────────────────────────────────

pub fn sr(text_content: impl Into<Str>) -> Text { sr_only(text_content) }
pub fn sr_only(text_content: impl Into<Str>) -> Text {
    text(text_content).append_css(
        "position:absolute;width:1px;height:1px;padding:0;margin:-1px;\
         overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0;"
    )
}

pub fn live(key: impl Into<Str>, politeness: &str) -> Block { live_region(key, politeness) }
pub fn live_region(key: impl Into<Str>, politeness: &str) -> Block {
    block()
        .attr("aria-live", politeness)
        .attr("aria-atomic", "true")
        .append_css("position:absolute;width:1px;height:1px;overflow:hidden;")
        .read(key)
}

pub fn skip_link(target: impl Into<Str>) -> Block {
    let href = format!("#{}", target.into());
    block()
        .set_class("sr-only focus:not-sr-only focus:absolute focus:top-0 focus:left-0 bg-white px-4 py-2 z-50")
        .add(text("Skip to content").attr("href", href))
}

// ── Theme provider ──────────────────────────────────────────────────────────

/// Flat key-value theme: `theme(vec![("primary", "#3b82f6"), ...], content)`
/// Pairs of (key, value) strings followed by a content token.
/// Kept as a free helper for non-DSL usage.
pub fn theme_provider(vars: Vec<(&str, &str)>, content: impl IntoToken) -> Block {
    let css = vars.iter()
        .map(|(k, v)| format!("  --{}: {};", k, v))
        .collect::<Vec<_>>()
        .join("\n");
    let mut style_node = TokenNode::new(next_id());
    style_node.tag = "style".into();
    style_node.content = Some(format!(":root {{\n{}\n}}", css).into());

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.class = "p-4 rounded-lg border".into();
    root.children.push(style_node);
    root.children.push(content.into_node());
    Container { stack: vec![root] }
}
