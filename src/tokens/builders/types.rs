// src/tokens/builders/types.rs
//
// Container and leaf element types.

use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::tokens::node::{IntoToken, Layout, Str, TokenNode};
use crate::tokens::core::id::next_id;
use super::spec::TokenBuilder;

// ── Named-container registry ─────────────────────────────────────────────────
/// Maps a DSL name (e.g. "Parent") to the finalized TokenNode so it can be
/// referenced later by `col_ref` / `row_ref` / `block_ref` / `grid_ref`.
static NAMED_REGISTRY: Lazy<Mutex<HashMap<String, TokenNode>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_named(name: &str, node: &TokenNode) {
    if let Ok(mut reg) = NAMED_REGISTRY.lock() {
        reg.insert(name.to_string(), node.clone());
    }
}

pub fn lookup_named(name: &str) -> Option<TokenNode> {
    NAMED_REGISTRY.lock().ok()?.get(name).cloned()
}

// ── Container (unified builder for Col, Row, Block, Grid) ─────────────────────

#[derive(Clone)]
pub struct Container {
    pub stack: Vec<TokenNode>,
}

impl Container {
    /// Add a child node to the current container.
    pub fn add(mut self, node: impl IntoToken) -> Self {
        self.node_mut().children.push(node.into_node());
        self
    }

    /// Add multiple child nodes from an iterator.
    pub fn add_all(mut self, nodes: impl IntoIterator<Item = impl IntoToken>) -> Self {
        for node in nodes {
            self.node_mut().children.push(node.into_node());
        }
        self
    }

    /// Add an optional child node.
    pub fn add_opt(mut self, node: Option<impl IntoToken>) -> Self {
        if let Some(node) = node {
            self.node_mut().children.push(node.into_node());
        }
        self
    }

    /// Pop the current builder context, moving the finished child into its parent.
    pub fn end(mut self) -> Self {
        if self.stack.len() > 1 {
            let child = self.stack.pop().unwrap();
            // If the finished container has a data-name, register it for reference.
            if let Some(name) = child.attributes.get("data-name").map(|s| s.to_string()) {
                register_named(&name, &child);
            }
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(child);
            }
        }
        self
    }

    pub fn col(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Col;
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn row(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Row;
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn block(self) -> Self {
        let n = TokenNode::new(next_id());
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn text(self, content: impl Into<Str>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.content = Some(content.into());
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }

    pub fn btn(self, content: impl Into<Str>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "button".into();
        n.content = Some(content.into());
        n.class = "inline-flex items-center justify-center gap-1 rounded font-medium transition-colors whitespace-nowrap".into();
        n.style.extra = "cursor:pointer;user-select:none;".into();
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }

    pub fn grid(self, cols: u8) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Grid { cols };
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn grid2(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Row;
        n.style.extra = "display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:0.75rem;".into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn grid3(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Row;
        n.style.extra = "display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:2px;padding:2px;".into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn stack(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Col;
        n.style.extra = "display:flex;flex-direction:column;".into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn split(self, ratio: f32) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.layout = Layout::Row;
        n.style.extra = format!("display:flex;--split-ratio:{};", ratio).into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn aspect(self, w: u16, h: u16) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.style.extra = format!("aspect-ratio:{}/{};", w, h).into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn overlay(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.class = "fixed inset-0 z-50 flex items-center justify-center".into();
        n.style.extra = "background:rgba(0,0,0,0.5);".into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn portal(self, target_id: impl Into<crate::tokens::node::Str>) -> Self {
        let mut n = TokenNode::new(target_id);
        n.tag = "div".into();
        n.style.extra = "position:absolute;".into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn tooltip(self, _target_id: impl Into<crate::tokens::node::Str>, content: impl Into<crate::tokens::node::Str>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.class = "absolute z-40 px-2 py-1 text-xs bg-gray-900 text-white rounded opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none".into();
        n.content = Some(content.into());
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn card(self, title: impl Into<crate::tokens::node::Str>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.class = "p-4 bg-white rounded-lg shadow-sm space-y-2".into();
        let mut title_n = TokenNode::new(next_id());
        title_n.content = Some(title.into());
        title_n.class = "text-sm font-semibold text-gray-700 mb-1".into();
        n.children.push(title_n);
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    /// Card with inline storage: creates a card titled with the key name,
    /// with a write_to control and read_from display embedded automatically.
    /// Children indented under it are appended after the storage controls.
    pub fn card_store(self, key: &'static str) -> Self {
        use crate::tokens::action::store_set_input;
        use crate::tokens::core::id::next_id as gen_id;

        let input_id = gen_id();
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.class = "p-4 bg-white rounded-lg shadow-sm space-y-2".into();

        // Title = key name
        let mut title_n = TokenNode::new(next_id());
        title_n.content = Some(key.into());
        title_n.class = "text-sm font-semibold text-gray-700 mb-1".into();
        n.children.push(title_n);

        // Key badge
        let mut badge = TokenNode::new(next_id());
        badge.content = Some(format!("storage: {key}").into());
        badge.class = "text-xs font-mono bg-gray-100 text-gray-600 px-2 py-0.5 rounded border border-gray-200 mb-2".into();
        n.children.push(badge);

        // Write row: input + Save button
        let mut write_row = TokenNode::new(next_id());
        write_row.layout = Layout::Row;
        write_row.class = "flex items-center gap-1".into();

        let mut input = TokenNode::new(input_id.clone());
        input.tag = "input".into();
        input.input_type = Some("text".into());
        input.placeholder = Some("New value…".into());
        input.class = "flex-1 px-3 py-1.5 text-sm border border-gray-300 rounded bg-white text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-400".into();
        write_row.children.push(input);

        let mut save_btn = TokenNode::new(next_id());
        save_btn.tag = "button".into();
        save_btn.content = Some("Save".into());
        save_btn.class = "px-3 py-1.5 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded font-medium transition-colors".into();
        save_btn.actions.push(store_set_input(key, input_id));
        write_row.children.push(save_btn);
        n.children.push(write_row);

        // Read display — reactive text_read
        let read_block = super::factory::text_read(key);
        let mut read_n = read_block.into_node();
        read_n.class = "text-sm font-mono bg-gray-50 border border-gray-200 rounded px-3 py-2 min-h-[2rem] break-all text-gray-700".into();
        n.children.push(read_n);

        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn card_named(self, name: impl Into<crate::tokens::node::Str>, title: impl Into<crate::tokens::node::Str>) -> Self {
        let name = name.into().to_string();
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.class = "p-4 bg-white rounded-lg shadow-sm space-y-2".into();
        n.attributes.insert("data-name".into(), name.clone().into());
        let mut title_n = TokenNode::new(next_id());
        title_n.content = Some(title.into());
        title_n.class = "text-sm font-semibold text-gray-700 mb-1".into();
        n.children.push(title_n);
        register_named(&name, &n);
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn section_title(self, t: impl Into<crate::tokens::node::Str>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.content = Some(t.into());
        n.class = "text-xl font-bold text-gray-800 mt-6".into();
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }

    pub fn section_named(self, name: impl Into<crate::tokens::node::Str>, t: impl Into<crate::tokens::node::Str>) -> Self {
        let name = name.into().to_string();
        let mut n = TokenNode::new(next_id());
        n.content = Some(t.into());
        n.class = "text-xl font-bold text-gray-800 mt-6".into();
        n.attributes.insert("data-name".into(), name.clone().into());
        register_named(&name, &n);
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }

    pub fn drawer(self, id: impl Into<crate::tokens::node::Str>, side: impl Into<crate::tokens::node::Str>, content: impl Into<crate::tokens::node::Str>) -> Self {
        use crate::tokens::core::id::next_id;
        use crate::tokens::action::TokenAction;
        let id_str: crate::tokens::node::Str = id.into();
        let side = side.into().to_string();
        let mut n = TokenNode::new(id_str.clone());
        n.tag = "div".into();
        n.class = "fixed z-50 bg-white shadow-lg flex flex-col".into();
        let (position, size) = match side.as_str() {
            "left"   => ("left:0;top:0;bottom:0;", "width:20rem;"),
            "right"  => ("right:0;top:0;bottom:0;", "width:20rem;"),
            "bottom" => ("bottom:0;left:0;right:0;", "height:20rem;"),
            _        => ("top:0;left:0;right:0;", "height:20rem;"),
        };
        n.style.extra = format!("{}{}{}", position, size, "display:none;").into();

        // Header row: title + close button
        let mut header = TokenNode::new(next_id());
        header.layout = crate::tokens::node::Layout::Row;
        header.class = "flex items-center justify-between px-4 py-3 border-b border-gray-200 flex-shrink-0".into();

        let mut title = TokenNode::new(next_id());
        title.content = Some(content.into());
        title.class = "font-semibold text-gray-800 text-sm".into();
        header.children.push(title);

        let mut close_btn = TokenNode::new(next_id());
        close_btn.tag = "button".into();
        close_btn.content = Some("✕".into());
        close_btn.class = "text-gray-400 hover:text-gray-700 text-lg leading-none p-1 rounded hover:bg-gray-100 transition-colors".into();
        close_btn.actions.push(TokenAction::Hide(id_str));
        header.children.push(close_btn);

        n.children.push(header);

        // Content area
        let mut body = TokenNode::new(next_id());
        body.layout = crate::tokens::node::Layout::Col;
        body.class = "flex-1 overflow-y-auto p-4".into();
        n.children.push(body);

        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn command_palette(self, actions: Vec<crate::tokens::action::TokenAction>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "button".into();
        n.content = Some("⌘".into());
        n.class = "text-sm px-2 py-1 rounded bg-gray-200 hover:bg-gray-300".into();
        n.actions = actions;
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }

    pub fn status_bar(self, items: Vec<impl IntoToken>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.tag = "div".into();
        n.layout = Layout::Row;
        n.class = "text-xs text-gray-500 bg-gray-100 px-3 py-1".into();
        for item in items {
            let mut child = TokenNode::new(next_id());
            child.tag = "span".into();
            child.content = Some(item.into_node().content.unwrap_or_default());
            n.children.push(child);
        }
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }

    pub fn modal(self, id: impl Into<crate::tokens::node::Str>, title: impl Into<crate::tokens::node::Str>, items: Vec<TokenNode>) -> Self {
        use crate::tokens::action::TokenAction;
        let id_str: crate::tokens::node::Str = id.into();
        let title_str: crate::tokens::node::Str = title.into();

        let mut backdrop = TokenNode::new(id_str.clone());
        backdrop.tag = "div".into();
        backdrop.class = "fixed inset-0 z-50 bg-black/50 flex items-center justify-center".into();
        backdrop.style.extra = "background:rgba(0,0,0,0.5);display:none;".into();

        let mut card = TokenNode::new(format!("{}_card", id_str));
        card.tag = "div".into();
        card.class = "bg-white rounded-lg shadow-lg max-w-md w-full mx-4 p-6 relative".into();

        let mut header = TokenNode::new(format!("{}_header", id_str));
        header.tag = "div".into();
        header.class = "flex items-center justify-between mb-4".into();
        let mut title_node = TokenNode::new(format!("{}_title", id_str));
        title_node.tag = "h3".into();
        title_node.content = Some(title_str);
        title_node.class = "text-lg font-semibold text-gray-900".into();
        header.children.push(title_node);

        let mut close_btn = TokenNode::new(format!("{}_close", id_str));
        close_btn.tag = "button".into();
        close_btn.content = Some("✕".into());
        close_btn.class = "text-gray-400 hover:text-gray-600 text-xl leading-none".into();
        close_btn.actions.push(TokenAction::Hide(id_str));
        header.children.push(close_btn);

        card.children.push(header);
        for item in items {
            card.children.push(item);
        }
        backdrop.children.push(card);

        let mut parent = self;
        parent.node_mut().children.push(backdrop);
        parent
    }

    pub fn tabs(self, active_signal: impl Into<crate::tokens::node::Str>, items: Vec<(crate::tokens::node::Str, TokenNode)>) -> Self {
        use crate::tokens::action::TokenAction;
        let signal = active_signal.into();
        let mut tab_bar = TokenNode::new(next_id());
        tab_bar.tag = "div".into();
        tab_bar.class = "flex border-b border-gray-200 mb-4".into();

        let mut panels = TokenNode::new(next_id());
        panels.tag = "div".into();

        let panel_ids: Vec<String> = (0..items.len())
            .map(|idx| format!("{}_{}", signal, idx))
            .collect();

        for (idx, (label, content)) in items.into_iter().enumerate() {
            let tab_key = format!("{}_{}", signal, idx);

            let hide_ids: Vec<crate::tokens::node::Str> = panel_ids
                .iter()
                .filter(|id| id.as_str() != tab_key)
                .map(|id| id.as_str().into())
                .collect();

            let mut tab = TokenNode::new(next_id());
            tab.tag = "button".into();
            tab.content = Some(label);
            tab.class = "px-4 py-2 text-sm font-medium text-gray-500 border-b-2 border-transparent hover:text-gray-700".into();
            tab.actions.push(TokenAction::Custom(format!("cycle:{}:{}", signal, tab_key).into()));
            tab.actions.push(TokenAction::Show {
                show: tab_key.clone().into(),
                hide: hide_ids,
            });
            tab_bar.children.push(tab);

            let mut panel = TokenNode::new(tab_key);
            panel.tag = "div".into();
            if idx > 0 {
                panel.class = "hidden".into();
            }
            panel.children.push(content);
            panels.children.push(panel);
        }

        let mut root = TokenNode::new(next_id());
        root.tag = "div".into();
        root.children.push(tab_bar);
        root.children.push(panels);

        let mut parent = self;
        parent.node_mut().children.push(root);
        parent
    }

    pub fn accordion(self, items: Vec<(crate::tokens::node::Str, TokenNode)>) -> Self {
        use crate::tokens::action::TokenAction;
        let mut root = TokenNode::new(next_id());
        root.tag = "div".into();
        root.class = "space-y-2".into();

        for (idx, (title, content)) in items.into_iter().enumerate() {
            let section_id = format!("accordion_{}", idx);

            let mut header = TokenNode::new(next_id());
            header.tag = "button".into();
            header.content = Some(title);
            header.class = "w-full text-left px-4 py-3 bg-gray-100 rounded-lg font-medium flex justify-between items-center".into();
            header.actions.push(TokenAction::ToggleState {
                key: section_id.clone().into(),
                on_state: "true".into(),
                off_state: "false".into(),
            });

            let mut panel = TokenNode::new(section_id.clone());
            panel.tag = "div".into();
            panel.class = "hidden px-4 py-2".into();
            panel.children.push(content);

            let mut section = TokenNode::new(next_id());
            section.tag = "div".into();
            section.children.push(header);
            section.children.push(panel);
            root.children.push(section);
        }

        let mut parent = self;
        parent.node_mut().children.push(root);
        parent
    }

    pub fn theme(self, vars: Vec<(&str, &str)>, items: Vec<TokenNode>) -> Self {
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
        for item in items {
            root.children.push(item);
        }

        let mut parent = self;
        parent.node_mut().children.push(root);
        parent
    }
}

impl TokenBuilder for Container {
    fn node_mut(&mut self) -> &mut TokenNode {
        self.stack.last_mut().unwrap()
    }
}

impl IntoToken for Container {
    fn into_node(mut self) -> TokenNode {
        while self.stack.len() > 1 {
            let child = self.stack.pop().unwrap();
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(child);
            }
        }
        self.stack.into_iter().next().unwrap()
    }
}

// ── Type aliases for backward-compatible DSL ───────────────────────────────────

pub type Col = Container;
pub type Row = Container;
pub type Block = Container;
pub type Grid = Container;

// ── Leaf element types ───────────────────────────────────────────────────────

pub struct Text(pub TokenNode);
pub struct Btn(pub TokenNode);
pub struct Img(pub TokenNode);

impl Clone for Text { fn clone(&self) -> Self { Self(self.0.clone()) } }
impl Clone for Btn  { fn clone(&self) -> Self { Self(self.0.clone()) } }
impl Clone for Img  { fn clone(&self) -> Self { Self(self.0.clone()) } }

impl TokenBuilder for Text {
    fn node_mut(&mut self) -> &mut TokenNode { &mut self.0 }
}
impl IntoToken for Text { fn into_node(self) -> TokenNode { self.0 } }

impl TokenBuilder for Btn {
    fn node_mut(&mut self) -> &mut TokenNode { &mut self.0 }
}
impl IntoToken for Btn { fn into_node(self) -> TokenNode { self.0 } }

impl TokenBuilder for Img {
    fn node_mut(&mut self) -> &mut TokenNode { &mut self.0 }
}
impl IntoToken for Img { fn into_node(self) -> TokenNode { self.0 } }
