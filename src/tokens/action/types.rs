// src/tokens/action/types.rs
//
// Core action types: TokenAction enum, event bindings, data flow primitives.

use serde::{Deserialize, Serialize};

use crate::tokens::node::Str;

// ── Into<TokenAction> implementations for string shortcuts ───────────────────

impl From<&str> for TokenAction {
    fn from(s: &str) -> Self {
        TokenAction::OpenUrl {
            url: s.into(),
            new_tab: false,
        }
    }
}

impl From<String> for TokenAction {
    fn from(s: String) -> Self {
        TokenAction::OpenUrl {
            url: s.into(),
            new_tab: false,
        }
    }
}

// ── Shared sub-types ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

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
    Show {
        show: Str,
        hide: Vec<Str>,
    },
    Hide(Str),
    HideAllModals,
    ToggleClass {
        target: Str,
        class: Str,
    },
    SetStyle {
        target: Str,
        property: Str,
        value: Str,
    },
    SetActive {
        group: Str,
        active_id: Str,
        active_css: Str,
        inactive_css: Str,
    },

    // Input
    TriggerFileInput {
        accept: Option<Str>,
        multiple: bool,
    },
    Submit {
        form_id: Str,
        on_submit: Str,
        on_invalid: Option<Str>,
    },

    // Logging
    Log {
        level: LogLevel,
        message: Str,
    },

    // Navigation
    Navigate(Str),
    ScrollTo {
        target: Str,
        behavior: ScrollBehavior,
    },
    OpenUrl {
        url: Str,
        new_tab: bool,
    },

    // Storage - simplified, backend inferred from context
    StoreSet {
        key: Str,
        value: Str,
    },
    StoreSetTtl {
        key: Str,
        value: Str,
        ttl_seconds: u64,
    },
    StoreGet {
        key: Str,
        target: DataTarget,
    },
    StoreDelete {
        key: Str,
    },
    StorePush {
        key: Str,
        input_key: Str,
    },
    StoreRemove {
        key: Str,
        input_key: Str,
    },
    StoreWriteToPath {
        path_input: Str,
        val_input: Str,
    },
    Increment {
        key: Str,
        by: i32,
    },
    Decrement {
        key: Str,
        by: i32,
    },
    ToggleState {
        key: Str,
        on_state: Str,
        off_state: Str,
    },

    // Async / real-time
    Preload {
        key: Str,
        endpoint: Str,
    },
    Watch {
        key: Str,
    },

    // Theme
    SetThemeVar {
        name: Str,
        value: Str,
    },

    // System control
    RequestFullscreen,
    ExitFullscreen,
    RequestPointerLock,
    Vibrate {
        pattern: Vec<u32>,
    },
    Notify {
        title: Str,
        body: Str,
        icon: Option<Str>,
    },
    Share {
        title: Str,
        text: Str,
        url: Option<Str>,
    },

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

// ── Action Registry ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_creates_open_url() {
        let action: TokenAction = "https://example.com".into();
        if let TokenAction::OpenUrl { url, new_tab } = action {
            assert_eq!(&*url, "https://example.com");
            assert!(!new_tab);
        } else {
            panic!("expected OpenUrl");
        }
    }

    #[test]
    fn from_string_creates_open_url() {
        let action: TokenAction = String::from("https://example.com").into();
        if let TokenAction::OpenUrl { url, new_tab } = action {
            assert_eq!(&*url, "https://example.com");
            assert!(!new_tab);
        } else {
            panic!("expected OpenUrl");
        }
    }

    #[test]
    fn token_action_equality() {
        let a = TokenAction::Navigate("home".into());
        let b = TokenAction::Navigate("home".into());
        assert_eq!(a, b);
    }

    #[test]
    fn token_action_inequality() {
        let a = TokenAction::Navigate("home".into());
        let b = TokenAction::Navigate("about".into());
        assert_ne!(a, b);
    }

    #[test]
    fn chain_action() {
        let chain = TokenAction::Chain(vec![
            TokenAction::Navigate("a".into()),
            TokenAction::HideAllModals,
        ]);
        if let TokenAction::Chain(actions) = chain {
            assert_eq!(actions.len(), 2);
        } else {
            panic!("expected Chain");
        }
    }

    #[test]
    fn event_binding_fields() {
        let binding = EventBinding {
            event: EventType::Click,
            action: TokenAction::HideAllModals,
            debounce_ms: Some(200),
            throttle_ms: None,
        };
        assert_eq!(binding.event, EventType::Click);
        assert_eq!(binding.debounce_ms, Some(200));
        assert!(binding.throttle_ms.is_none());
    }

    #[test]
    fn async_action_spec_fields() {
        let spec = AsyncActionSpec {
            endpoint: "/api/data".into(),
            method: HttpMethod::Post,
            source: DataSource::Literal("body".into()),
            target: DataTarget::Signal("result".into()),
            on_success: Some(TokenAction::HideAllModals),
            on_error: None,
        };
        assert_eq!(&*spec.endpoint, "/api/data");
        assert_eq!(spec.method, HttpMethod::Post);
    }

    #[test]
    fn form_spec_fields() {
        let form = FormSpec {
            id: "login".into(),
            fields: vec![
                FieldSpec {
                    name: "email".into(),
                    required: true,
                    validation: None,
                },
                FieldSpec {
                    name: "pass".into(),
                    required: true,
                    validation: Some("min:8".into()),
                },
            ],
            submit_action: TokenAction::Custom("submit".into()),
        };
        assert_eq!(form.fields.len(), 2);
        assert!(form.fields[0].required);
    }

    #[test]
    fn cart_op_variants() {
        let add = CartOp::Add {
            item_id: "sku-1".into(),
            qty: 2,
        };
        let clear = CartOp::Clear;
        assert_ne!(add, clear);
        if let CartOp::Add { item_id, qty } = add {
            assert_eq!(&*item_id, "sku-1");
            assert_eq!(qty, 2);
        }
    }

    #[test]
    fn store_set_action() {
        let action = TokenAction::StoreSet {
            key: "user.name".into(),
            value: "Alice".into(),
        };
        if let TokenAction::StoreSet { key, value } = action {
            assert_eq!(&*key, "user.name");
            assert_eq!(&*value, "Alice");
        }
    }

    #[test]
    fn increment_decrement_actions() {
        let inc = TokenAction::Increment {
            key: "count".into(),
            by: 1,
        };
        let dec = TokenAction::Decrement {
            key: "count".into(),
            by: 5,
        };
        if let TokenAction::Increment { by, .. } = inc {
            assert_eq!(by, 1);
        }
        if let TokenAction::Decrement { by, .. } = dec {
            assert_eq!(by, 5);
        }
    }

    #[test]
    fn serialization_roundtrip() {
        let action = TokenAction::Navigate("test".into());
        let json = serde_json::to_string(&action).unwrap();
        let back: TokenAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, back);
    }
}
