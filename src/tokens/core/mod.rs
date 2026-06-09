// src/tokens/core/mod.rs
//
// Core token definitions: types, traits, and ID generation.
// The foundational layer that all other token modules depend on.

pub mod types;
pub mod id;

pub use types::{
    BindMode, DataBinding, IntoToken, Layout, StyleMode, StyleToken, Str, TokenNode, s,
};
pub use id::{next_id, reset_id_counter};
