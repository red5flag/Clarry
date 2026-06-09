// src/tokens/core/types.rs
//
// Core token tree types and the `IntoToken` trait.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::tokens::action::{TokenAction, EventBinding};
// Animation types imported when needed by consumers

// ── Type aliases ─────────────────────────────────────────────────────────────

pub type Str = Arc<str>;

pub fn s(text: &str) -> Str {
    Arc::from(text)
}

// ── Style sub-structs ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StyleToken {
    pub bg: Option<[u8; 4]>,
    pub w: Option<f32>,
    pub h: Option<f32>,
    pub top: Option<f32>,
    pub left: Option<f32>,
    pub radius: Option<f32>,
    pub shadow: Option<f32>,
    pub pad: Option<f32>,
    pub gap: Option<f32>,
    pub grid_cols: Option<u8>,
    pub extra: Str,
}

impl StyleToken {
    /// Compute a hash key from all style fields for caching.
    pub fn hash_key(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.bg.hash(&mut hasher);
        self.w.map(|f| f.to_bits()).hash(&mut hasher);
        self.h.map(|f| f.to_bits()).hash(&mut hasher);
        self.top.map(|f| f.to_bits()).hash(&mut hasher);
        self.left.map(|f| f.to_bits()).hash(&mut hasher);
        self.radius.map(|f| f.to_bits()).hash(&mut hasher);
        self.shadow.map(|f| f.to_bits()).hash(&mut hasher);
        self.pad.map(|f| f.to_bits()).hash(&mut hasher);
        self.gap.map(|f| f.to_bits()).hash(&mut hasher);
        self.grid_cols.hash(&mut hasher);
        self.extra.hash(&mut hasher);
        hasher.finish()
    }

    /// Compile the style token to a CSS string.
    /// Uses rem units for layout properties to match the builder API convention.
    pub fn compile(&self) -> String {
        let mut css = String::new();
        if let Some(bg) = self.bg {
            css.push_str(&format!(
                "background: rgba({}, {}, {}, {});",
                bg[0], bg[1], bg[2], bg[3] as f32 / 255.0
            ));
        }
        if let Some(w) = self.w {
            css.push_str(&format!("width: {:.2}rem;", w));
        }
        if let Some(h) = self.h {
            css.push_str(&format!("height: {:.2}rem;", h));
        }
        if let Some(top) = self.top {
            css.push_str(&format!("top: {:.2}rem;", top));
        }
        if let Some(left) = self.left {
            css.push_str(&format!("left: {:.2}rem;", left));
        }
        if let Some(radius) = self.radius {
            css.push_str(&format!("border-radius: {:.2}rem;", radius));
        }
        if let Some(shadow) = self.shadow {
            css.push_str(&format!("box-shadow: 0 0 {:.2}rem rgba(0,0,0,0.3);", shadow));
        }
        if let Some(pad) = self.pad {
            css.push_str(&format!("padding: {:.2}rem;", pad));
        }
        if let Some(gap) = self.gap {
            css.push_str(&format!("gap: {:.2}rem;", gap));
        }
        if !self.extra.is_empty() {
            css.push_str(&self.extra);
        }
        css
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum StyleMode {
    #[default]
    Inline,
    Class,
    Static,
    Computed(Str),
}

// ── Layout enum ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Layout {
    #[default]
    Col,
    Row,
    Grid { cols: u8 },
    Absolute { top: f32, left: f32 },
}

// ── Data binding ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum BindMode {
    #[default]
    OneWay,
    TwoWay,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DataBinding {
    pub key: Str,
    pub target_id: Option<Str>,
    pub mode: BindMode,
}

impl DataBinding {
    pub fn one_way(key: impl Into<Str>) -> Self {
        Self {
            key: key.into(),
            target_id: None,
            mode: BindMode::OneWay,
        }
    }

    pub fn two_way(key: impl Into<Str>, target_id: impl Into<Str>) -> Self {
        Self {
            key: key.into(),
            target_id: Some(target_id.into()),
            mode: BindMode::TwoWay,
        }
    }
}

// ── TokenNode ─────────────────────────────────────────────────────────────────

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TokenNode {
    pub id: Str,
    pub tag: Str,
    pub style: StyleToken,
    pub style_mode: StyleMode,
    pub class: Str,
    pub content: Option<Str>,
    pub children: Vec<TokenNode>,
    pub layout: Layout,
    pub data_binding: Option<DataBinding>,
    pub actions: Vec<TokenAction>,
    pub event_bindings: Vec<EventBinding>,
    pub on_nav: Option<Str>,
    #[serde(skip)]
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Dynamic content for reactive rendering with Leptos signals
    #[serde(skip)]
    pub dynamic_content: Option<Arc<dyn Fn() -> leptos::prelude::AnyView + Send + Sync>>,
    /// Input element attributes (type, placeholder, name)
    pub input_type: Option<Str>,
    pub placeholder: Option<Str>,
    pub name: Option<Str>,
    /// UI variant (e.g. "primary", "secondary", "danger", "ghost")
    pub variant: Option<Str>,
    /// Size variant (e.g. "sm", "md", "lg")
    pub size: Option<Str>,
    /// Loading state (shows spinner, disables interaction)
    pub loading: bool,
    /// Disabled state (HTML disabled attribute)
    pub disabled: bool,
    /// Generic HTML attributes (key → value)
    pub attributes: HashMap<Str, Str>,
}

impl std::fmt::Debug for TokenNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenNode")
            .field("id", &self.id)
            .field("tag", &self.tag)
            .field("style", &self.style)
            .field("style_mode", &self.style_mode)
            .field("class", &self.class)
            .field("content", &self.content)
            .field("children", &self.children)
            .field("layout", &self.layout)
            .field("data_binding", &self.data_binding)
            .field("actions", &self.actions)
            .field("event_bindings", &self.event_bindings)
            .field("on_nav", &self.on_nav)
            .field("on_click", &self.on_click.is_some())
            .field("dynamic_content", &self.dynamic_content.is_some())
            .field("input_type", &self.input_type)
            .field("placeholder", &self.placeholder)
            .field("name", &self.name)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("loading", &self.loading)
            .field("disabled", &self.disabled)
            .field("attributes", &self.attributes)
            .finish()
    }
}

impl PartialEq for TokenNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.tag == other.tag
            && self.style == other.style
            && self.style_mode == other.style_mode
            && self.class == other.class
            && self.content == other.content
            && self.children == other.children
            && self.layout == other.layout
            && self.data_binding == other.data_binding
            && self.actions == other.actions
            && self.event_bindings == other.event_bindings
            && self.on_nav == other.on_nav
            && self.on_click.is_some() == other.on_click.is_some()
            && self.dynamic_content.is_some() == other.dynamic_content.is_some()
            && self.input_type == other.input_type
            && self.placeholder == other.placeholder
            && self.name == other.name
            && self.variant == other.variant
            && self.size == other.size
            && self.loading == other.loading
            && self.disabled == other.disabled
            && self.attributes == other.attributes
    }
}

impl TokenNode {
    pub fn new(id: impl Into<Str>) -> Self {
        Self {
            id: id.into(),
            tag: "div".into(),
            dynamic_content: None,
            style: StyleToken::default(),
            style_mode: StyleMode::Inline,
            class: "".into(),
            content: None,
            children: Vec::new(),
            layout: Layout::default(),
            data_binding: None,
            actions: Vec::new(),
            event_bindings: Vec::new(),
            on_nav: None,
            on_click: None,
            input_type: None,
            placeholder: None,
            name: None,
            variant: None,
            size: None,
            loading: false,
            disabled: false,
            attributes: HashMap::new(),
        }
    }

    /// Clone the node without its children (shallow clone for indexing).
    pub fn clone_shallow(&self) -> Self {
        Self {
            id: Arc::clone(&self.id),
            tag: Arc::clone(&self.tag),
            style: self.style.clone(),
            style_mode: self.style_mode.clone(),
            class: Arc::clone(&self.class),
            content: self.content.as_ref().map(Arc::clone),
            children: Vec::new(),
            layout: self.layout.clone(),
            data_binding: self.data_binding.clone(),
            actions: self.actions.clone(),
            event_bindings: self.event_bindings.clone(),
            on_nav: self.on_nav.clone(),
            on_click: self.on_click.as_ref().map(Arc::clone),
            dynamic_content: self.dynamic_content.as_ref().map(Arc::clone),
            input_type: self.input_type.clone(),
            placeholder: self.placeholder.clone(),
            name: self.name.clone(),
            variant: self.variant.clone(),
            size: self.size.clone(),
            loading: self.loading,
            disabled: self.disabled,
            attributes: self.attributes.clone(),
        }
    }
}

// ── IntoToken trait ───────────────────────────────────────────────────────────

pub trait IntoToken {
    fn into_node(self) -> TokenNode;
}

impl IntoToken for TokenNode {
    fn into_node(self) -> TokenNode { self }
}
