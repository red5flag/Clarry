// src/tokens/render.rs
//
// Rendering pipeline — orchestrates submodules.
// Refactored from monolithic file into focused responsibilities.

pub mod pipeline;
pub mod hydration;
pub mod element;
pub mod executor;
pub mod dom;

pub use pipeline::{TokenRenderer, render_dom};
