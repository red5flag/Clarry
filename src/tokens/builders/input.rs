// src/tokens/builders/input.rs
//
// Form and input primitives.

use crate::tokens::node::{Str, TokenNode};
use crate::tokens::core::id::next_id;
use super::types::Container;

/// Common input reset style to avoid duplication.
const INPUT_RESET: &str = "border:none;outline:none;background:transparent;width:100%;font-size:0.9rem;";

pub fn txtinp(placeholder: impl Into<Str>, id: impl Into<Str>) -> Container { text_input(placeholder, id) }
pub fn innum(id: impl Into<Str>) -> Container { input_number(id) }
pub fn inpsw(id: impl Into<Str>) -> Container { input_password(id) }
pub fn txtarea(id: impl Into<Str>, rows: u8) -> Container { textarea(id, rows) }

/// Text input field
pub fn text_input(placeholder: impl Into<Str>, id: impl Into<Str>) -> Container {
    let mut n = TokenNode::new(id);
    n.tag = "input".into();
    n.input_type = Some("text".into());
    n.placeholder = Some(placeholder.into());
    n.style.extra = format!("{}cursor:text;", INPUT_RESET).into();
    Container { stack: vec![n] }
}

/// Number input field
pub fn input_number(id: impl Into<Str>) -> Container {
    let mut n = TokenNode::new(id);
    n.tag = "input".into();
    n.input_type = Some("number".into());
    n.style.extra = INPUT_RESET.into();
    Container { stack: vec![n] }
}

/// Password input field
pub fn input_password(id: impl Into<Str>) -> Container {
    let mut n = TokenNode::new(id);
    n.tag = "input".into();
    n.input_type = Some("password".into());
    n.style.extra = INPUT_RESET.into();
    Container { stack: vec![n] }
}

/// Checkbox with label
pub fn checkbox(id: impl Into<Str>, label: impl Into<Str>) -> Container {
    let mut n = TokenNode::new(id);
    n.tag = "input".into();
    n.input_type = Some("checkbox".into());
    n.content = Some(label.into());
    Container { stack: vec![n] }
}

/// Multi-line text area
pub fn textarea(id: impl Into<Str>, rows: u8) -> Container {
    let mut n = TokenNode::new(id);
    n.tag = "textarea".into();
    n.style.extra = format!("{}resize:vertical;rows:{};", INPUT_RESET, rows).into();
    Container { stack: vec![n] }
}

/// Select dropdown with options
pub fn select(id: impl Into<Str>, options: Vec<impl Into<Str>>) -> Container {
    let mut n = TokenNode::new(id);
    n.tag = "select".into();
    for opt in options {
        let mut child = TokenNode::new(next_id());
        child.tag = "option".into();
        child.content = Some(opt.into());
        n.children.push(child);
    }
    Container { stack: vec![n] }
}
