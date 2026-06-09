// src/tokens/action.rs
//
// Token action system — declarative UI effects and custom handlers.
// Refactored into focused submodules.

pub mod types;
pub mod registry;
pub mod builder;
pub mod web;

pub use types::*;
pub use registry::*;
pub use builder::*;
pub use web::*;
