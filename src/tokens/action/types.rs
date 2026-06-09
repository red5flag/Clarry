// src/tokens/action/types.rs
//
// Core action types: TokenAction enum, event bindings, data flow primitives.


use serde::{Deserialize, Serialize};

use crate::tokens::node::Str;

// ── Into<TokenAction> implementations for string shortcuts ───────────────────

impl From<&str> for TokenAction {
    fn from(s: &str) -> Self {
        TokenAction::OpenUrl { url: s.into(), new_tab: false }
    }
}

impl From<String> for TokenAction {
    fn from(s: String) -> Self {
        TokenAction::OpenUrl { url: s.into(), new_tab: false }
    }
}

#[cfg(target_arch = "wasm32")]
use leptos::web_sys;

// ── Shared sub-types ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LogLevel { Debug, Info, Warn, Error }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod { Get, Post, Put, Delete, Patch }

/// Simplified data source - just key/value pairs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DataSource {
    Signal(Str),
    Literal(Str),
}

/// Simplified data target
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DataTarget {
    Signal(Str),
    Element(Str),
}

// ── TokenAction (unified action type) ─────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenAction {
    // DOM
    Show { show: Str, hide: Vec<Str> },
    Hide(Str),
    HideAllModals,
    ToggleClass { target: Str, class: Str },
    SetStyle { target: Str, property: Str, value: Str },
    SetActive { group: Str, active_id: Str, active_css: Str, inactive_css: Str },

    // Input
    TriggerFileInput { accept: Option<Str>, multiple: bool },
    Submit { form_id: Str, on_submit: Str, on_invalid: Option<Str> },

    // Logging
    Log { level: LogLevel, message: Str },

    // Navigation
    Navigate(Str),
    ScrollTo { target: Str, behavior: ScrollBehavior },
    OpenUrl { url: Str, new_tab: bool },

    // Storage - simplified, backend inferred from context
    StoreSet { key: Str, value: Str },
    StoreSetTtl { key: Str, value: Str, ttl_seconds: u64 },
    StoreGet { key: Str, target: DataTarget },
    StoreDelete { key: Str },
    Increment { key: Str, by: i32 },
    Decrement { key: Str, by: i32 },
    ToggleState { key: Str, on_state: Str, off_state: Str },

    // Async / real-time
    Preload { key: Str, endpoint: Str },
    Watch { key: Str },

    // Theme
    SetThemeVar { name: Str, value: Str },

    // System control
    RequestFullscreen,
    ExitFullscreen,
    RequestPointerLock,
    Vibrate { pattern: Vec<u32> },
    Notify { title: Str, body: Str, icon: Option<Str> },
    Share { title: Str, text: Str, url: Option<Str> },

    // Custom
    Custom(Str),
    Chain(Vec<TokenAction>),
    CopyToClipboard(Str),
}

// ── Event binding types ──────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Click,
    DoubleClick,
    Hover,
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Submit,
    Change,
    KeyDown,
    Scroll,
    Resize,
    IntersectEnter,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EventBinding {
    pub event: EventType,
    pub action: TokenAction,
    pub debounce_ms: Option<u32>,
    pub throttle_ms: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ScrollBehavior {
    Smooth,
    Instant,
    Auto,
}

// ── Async action spec ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AsyncActionSpec {
    pub endpoint: Str,
    pub method: HttpMethod,
    pub source: DataSource,
    pub target: DataTarget,
    pub on_success: Option<TokenAction>,
    pub on_error: Option<TokenAction>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AsyncTrigger {
    OnSubmit,
    OnChange,
    OnClick,
    OnScroll,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FieldSpec {
    pub name: Str,
    pub required: bool,
    pub validation: Option<Str>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FormSpec {
    pub id: Str,
    pub fields: Vec<FieldSpec>,
    pub submit_action: TokenAction,
}

// ── Cart operations ───────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CartOp {
    Add { item_id: Str, qty: u32 },
    Remove { item_id: Str },
    UpdateQty { item_id: Str, qty: u32 },
    Clear,
    Checkout,
}

// ── Action Registry ───────────────────────────────────────────────────────────
