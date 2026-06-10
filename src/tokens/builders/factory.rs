// src/tokens/builders/factory.rs
//
// Basic layout, text, action constructors, and storage bindings.

use std::sync::Arc;
use crate::tokens::node::{Layout, Str, TokenNode};
use crate::tokens::core::id::next_id;
pub use crate::tokens::core::id::reset_id_counter;
use crate::tokens::action::{TokenAction, LogLevel, ScrollBehavior};
use super::types::{Container, Row, Col, Block, Grid, Text, Btn, Img};

pub fn row() -> Row {
    let mut n = TokenNode::new(next_id());
    n.layout = Layout::Row;
    Container { stack: vec![n] }
}
pub fn col() -> Col {
    let mut n = TokenNode::new(next_id());
    n.layout = Layout::Col;
    Container { stack: vec![n] }
}
pub fn grid(cols: u8) -> Grid {
    let mut n = TokenNode::new(next_id());
    n.layout = Layout::Grid { cols };
    Container { stack: vec![n] }
}
pub fn block() -> Block {
    let n = TokenNode::new(next_id());
    Container { stack: vec![n] }
}
pub fn stack() -> Col {
    let mut n = TokenNode::new(next_id());
    n.layout = Layout::Col;
    n.style.extra = "display:flex;flex-direction:column;".into();
    Container { stack: vec![n] }
}
pub fn inf(endpoint: impl Into<Str>) -> TokenAction {
    TokenAction::Custom(format!("inf:{}", endpoint.into()).into())
}
pub fn text(content: impl Into<Str>) -> Text {
    let mut n = TokenNode::new(next_id());
    n.content = Some(content.into());
    Text(n)
}

/// Create text with dynamic/reactive content using a Leptos signal
pub fn text_dynamic<F>(f: F) -> Text
where
    F: Fn() -> String + Send + Sync + 'static
{
    use leptos::prelude::*;
    let mut n = TokenNode::new(next_id());
    let f = std::sync::Arc::new(f);
    n.dynamic_content = Some(Arc::new(move || {
        let f2 = f.clone();
        view! { <span>{move || f2()}</span> }.into_any()
    }));
    Text(n)
}
pub fn btn(label: impl Into<Str>) -> Btn {
    let mut n = TokenNode::new(next_id()); 
    n.tag = "button".into();
    n.content = Some(label.into());
    n.style.extra = "cursor:pointer;user-select:none;".into(); 
    Btn(n)
}
pub fn img_block(src: impl Into<Str>) -> Img {
    let src: Str = src.into();
    let mut n = TokenNode::new(next_id());
    n.tag = "img".into();
    n.attributes.insert("src".into(), src);
    n.attributes.insert("alt".into(), "".into());
    n.class = "w-full h-full object-cover".into();
    Img(n)
}

// ── Fluent standalone action constructors ───────────────────────────────────

pub fn chain(actions: Vec<TokenAction>) -> TokenAction {
    TokenAction::Chain(actions)
}

pub fn store(key: impl Into<Str>, value: impl Into<Str>) -> TokenAction {
    TokenAction::StoreSet { key: key.into(), value: value.into() }
}

pub fn hide(id: impl Into<Str>) -> TokenAction {
    TokenAction::Hide(id.into())
}

pub fn show(id: impl Into<Str>) -> TokenAction {
    TokenAction::Show { show: id.into(), hide: vec![] }
}

pub fn show_hiding(show_id: impl Into<Str>, hide_ids: Vec<impl Into<Str>>) -> TokenAction {
    TokenAction::Show {
        show: show_id.into(),
        hide: hide_ids.into_iter().map(|s| s.into()).collect(),
    }
}

pub fn log(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log { level: LogLevel::Info, message: msg.into() }
}

pub fn debug(msg: impl Into<Str>) -> TokenAction {
    TokenAction::Log { level: LogLevel::Debug, message: msg.into() }
}

pub fn navigate(page: impl Into<Str>) -> TokenAction {
    TokenAction::Navigate(page.into())
}

pub fn scroll_to(target: impl Into<Str>) -> TokenAction {
    TokenAction::ScrollTo { target: target.into(), behavior: ScrollBehavior::Smooth }
}

pub fn copy(text: impl Into<Str>) -> TokenAction {
    TokenAction::CopyToClipboard(text.into())
}

pub fn open_url(url: impl Into<Str>) -> TokenAction {
    TokenAction::OpenUrl { url: url.into(), new_tab: false }
}

pub fn hide_all_modals() -> TokenAction {
    TokenAction::HideAllModals
}

pub fn toggle_class(target: impl Into<Str>, class: impl Into<Str>) -> TokenAction {
    TokenAction::ToggleClass { target: target.into(), class: class.into() }
}

pub fn increment(key: impl Into<Str>) -> TokenAction {
    TokenAction::Increment { key: key.into(), by: 1 }
}

pub fn decrement(key: impl Into<Str>) -> TokenAction {
    TokenAction::Decrement { key: key.into(), by: 1 }
}

pub fn counter_text(id: impl Into<String>, prefix: &str) -> Text {
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    let id = id.into();
    let prefix = prefix.to_string();
    text_dynamic(move || {
        use_context::<TokenCtx>()
            .map(|ctx| format!("{} {}", prefix, ctx.counter_fn(id.clone())()))
            .unwrap_or(format!("{} 0", prefix))
    })
}

// ── Storage read/write factories ──────────────────────────────────────────────

pub fn text_bind(key: impl Into<String>) -> Text {
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    let key = key.into();
    text_dynamic(move || {
        use_context::<TokenCtx>()
            .map(|ctx| ctx.string_fn(key.clone())())
            .unwrap_or_default()
    })
}

/// Read a dot-notation storage path reactively.
/// Bumps list_rev so any storage write causes re-render.
pub fn text_read(path: impl Into<String>) -> Text {
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    use crate::tokens::storage::primitive::Store;
    let path = path.into();
    text_dynamic(move || {
        use_context::<TokenCtx>()
            .map(|ctx| {
                let _rev = ctx.list_rev.get();
                ctx.strings.get().get(&path).cloned()
                    .or_else(|| Store::read(&path))
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    })
}

pub fn img_bind(key: impl Into<String>, fallback: impl Into<Str>) -> Block {
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    let key = key.into();
    let fallback = fallback.into().to_string();
    let mut n = TokenNode::new(next_id());
    n.tag = "img".into();
    n.class = "w-full h-auto object-cover rounded-lg".into();
    n.dynamic_content = Some(Arc::new(move || {
        let src = use_context::<TokenCtx>()
            .map(|ctx| ctx.string_fn(key.clone())())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| fallback.clone());
        let src_clone = src.clone();
        view! { <img src=src_clone class="w-full h-auto object-cover rounded-lg" alt="" /> }.into_any()
    }));
    Container { stack: vec![n] }
}

pub fn data_list<F>(key: impl Into<String>, render_item: F) -> Block
where
    F: Fn(usize, &str) -> Container + 'static,
{
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    let key = key.into();
    let _render_item = Arc::new(render_item);
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "flex flex-col gap-2".into();
    n.dynamic_content = Some(Arc::new(move || {
        let rev = use_context::<TokenCtx>().map(|ctx| ctx.list_rev.get()).unwrap_or(0);
        let key_clone = key.clone();
        let key_clone2 = key.clone();
        view! {
            <div data-list-rev=rev data-list-key=key_clone>
                { "List bound to " } { key_clone2 }
            </div>
        }.into_any()
    }));
    Container { stack: vec![n] }
}

/// Read a JSON array from a storage root key and render each object as a
/// styled chat row.  Expects `{"text": "...", "sender": "..."}` shape.
pub fn chat_messages(storage_root: impl Into<String>) -> Block {
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    use crate::tokens::storage::primitive::Store;
    use serde_json::Value;
    let root = storage_root.into();
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "flex flex-col gap-2 max-h-48 overflow-y-auto".into();
    n.dynamic_content = Some(Arc::new(move || {
        let has_ctx = use_context::<TokenCtx>().is_some();
        let raw = use_context::<TokenCtx>()
            .map(|ctx| {
                let rev = ctx.list_rev.get();
                let from_strings = ctx.strings.get().get(&root).cloned();
                let from_store = Store::read(&root);
                let has_strings = from_strings.is_some();
                let has_store = from_store.is_some();
                let chosen = from_strings.or_else(|| from_store).unwrap_or_default();
                leptos::logging::log!("[CHAT_MSGS] rev={} root={} strings={:?} store={:?} chosen_len={}", rev, root, has_strings, has_store, chosen.len());
                chosen
            })
            .unwrap_or_default();
        leptos::logging::log!("[CHAT_MSGS] has_ctx={} raw_len={}", has_ctx, raw.len());
        let items: Vec<Value> = serde_json::from_str(&raw).unwrap_or_default();
        let rows = items.into_iter().map(|item| {
            let text = item.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let sender = item.get("sender").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let is_user = sender == "user" || sender == "me";
            let align = if is_user { "self-end bg-blue-500 text-white" } else { "self-start bg-gray-200 text-gray-800" };
            view! {
                <div class={format!("px-3 py-2 rounded-lg text-sm max-w-[80%] {}", align)}>
                    <span class="font-semibold text-xs opacity-75 block mb-0.5">{sender}</span>
                    {text}
                </div>
            }
        }).collect::<Vec<_>>();
        view! {
            <div class="flex flex-col gap-2 p-2">
                {rows}
            </div>
        }.into_any()
    }));
    Container { stack: vec![n] }
}

pub fn chat_bubble_messages(storage_root: impl Into<String>) -> Block {
    use leptos::prelude::*;
    use crate::tokens::reactive::TokenCtx;
    use crate::tokens::storage::primitive::Store;
    use serde_json::Value;
    let root = storage_root.into();
    let mut n = TokenNode::new(next_id());
    n.tag = "div".into();
    n.class = "flex flex-col gap-1 max-h-48 overflow-y-auto p-2".into();
    n.dynamic_content = Some(Arc::new(move || {
        let raw = use_context::<TokenCtx>()
            .map(|ctx| {
                let _rev = ctx.list_rev.get();
                ctx.strings.get().get(&root).cloned()
                    .or_else(|| Store::read(&root))
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let items: Vec<Value> = serde_json::from_str(&raw).unwrap_or_default();
        let bubbles = items.into_iter().map(|item| {
            let text = item.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let sender = item.get("sender").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let is_user = sender == "user" || sender == "me";
            let (bg_class, align_class) = if is_user {
                ("bg-blue-600 text-white", "self-end")
            } else {
                ("bg-gray-100 text-gray-900", "self-start")
            };
            view! {
                <div class={format!("px-4 py-2 rounded-2xl text-sm max-w-[70%] my-1 {} {}", bg_class, align_class)}>
                    <span class="font-semibold text-xs opacity-75 block mb-0.5">{sender}</span>
                    {text}
                </div>
            }
        }).collect::<Vec<_>>();
        view! {
            <div class="flex flex-col gap-1">
                {bubbles}
            </div>
        }.into_any()
    }));
    Container { stack: vec![n] }
}

// ── Extension trait for chaining actions fluently ───────────────────────────

pub trait ActionChain {
    fn then(self, next: TokenAction) -> TokenAction;
}

impl ActionChain for TokenAction {
    fn then(self, next: TokenAction) -> TokenAction {
        match self {
            TokenAction::Chain(mut actions) => {
                actions.push(next);
                TokenAction::Chain(actions)
            }
            _ => TokenAction::Chain(vec![self, next]),
        }
    }
}
