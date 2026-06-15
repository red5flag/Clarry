// src/tokens/builders/types.rs
//
// Container and leaf element types.

use crate::tokens::node::{IntoToken, Layout, Str, TokenNode};
use crate::tokens::core::id::next_id;
use super::spec::TokenBuilder;

// ── Container (unified builder for Col, Row, Block, Grid) ─────────────────────

#[derive(Clone)]
pub struct Container {
    pub stack: Vec<TokenNode>,
}

impl Container {
    /// Pop the current builder context, moving the finished child into its parent.
    pub fn end(mut self) -> Self {
        if self.stack.len() > 1 {
            let child = self.stack.pop().unwrap();
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
        n.style.extra = "display:grid;grid-template-columns:repeat(2,1fr);gap:0.75rem;".into();
        let mut parent = self;
        parent.stack.push(n);
        parent
    }

    pub fn grid3(self) -> Self {
        let mut n = TokenNode::new(next_id());
        n.layout = Layout::Row;
        n.style.extra = "display:grid;grid-template-columns:repeat(3,1fr);gap:2px;padding:2px;".into();
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

    pub fn section_title(self, t: impl Into<crate::tokens::node::Str>) -> Self {
        let mut n = TokenNode::new(next_id());
        n.content = Some(t.into());
        n.class = "text-xl font-bold text-gray-800 mt-6".into();
        let mut parent = self;
        parent.node_mut().children.push(n);
        parent
    }
    pub fn section(self, t: impl Into<crate::tokens::node::Str>) -> Self {
        self.section_title(t)
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
