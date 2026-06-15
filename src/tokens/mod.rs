// src/tokens/mod.rs

pub mod core;
pub mod node;   // backward-compat re-export
pub mod store;
pub mod storage;
pub mod action;
pub mod animation;
pub mod render;
pub mod builders;
pub mod islands;   // ← Leptos island constructors
pub mod reactive; // ← TokenCtx signal-driven rendering
pub mod debug;    // ← Debugging, validation, and inspection
pub mod async_resource;
pub mod prelude;
pub mod schema;

#[cfg(feature = "turboquant")]
pub mod kv_cache;

// ── Core ──────────────────────────────────────────────────────────────────────
pub use node::{
    BindMode, DataBinding, IntoToken, Layout, StyleMode, StyleToken, Str, TokenNode, s,
};
pub use store::{FlyCache, TokenArena, TokenStore};
pub use reactive::TokenCtx;
pub use async_resource::{AsyncResource, ResourceState};

// ── Debug ─────────────────────────────────────────────────────────────────────
pub use debug::{
    DebugConfig, format_tree, validate_tree, ValidationResult,
    TreeStats, log_lifecycle, TokenTreeInspector, TokenInspectorStyles
};

// ── Island ────────────────────────────────────────────────────────────────────
pub use islands::{
    island, style_inject, fmt_n,
};

// ── Storage ───────────────────────────────────────────────────────────────────
pub use storage::{
    MemoryStore, LocalStore, SessionStore, StoreManager, NestedStore, Store,
    StoreEntry, subscribe, store_json, load_json,
};

// ── Actions ───────────────────────────────────────────────────────────────────
pub use action::{
    ActionRegistry, CartOp,
    DataSource, DataTarget, HttpMethod, LogLevel,
    EventBinding, EventType, AsyncActionSpec, AsyncTrigger, FormSpec, FieldSpec,
    ScrollBehavior, TokenAction,
    log, debug, warn, chain,
    show, show_hiding, hide, hide_all_modals,
    toggle_class, add_class, remove_class, set_style, set_attr,
    navigate, nav, open_url, url, open_url_new_tab, route,
    trigger_upload, copy_to_clipboard,
    store_set, store_set_input, store_get, store_delete, store_set_ttl, store_from_val,
    store_push, store_remove,
    toggle_state, cycle_state, increment, increment_by, decrement, decrement_by,
    tog, cyc, inc, dec,
    submit_form, fetch_get, preload, store_watch, set_theme_var,
    toggle_drawer, cycle_drawer, chat_send,
    form, toggle, drag, val, search, scroll, key, resize, intersect, in_,
};

// ── Animation ─────────────────────────────────────────────────────────────────
pub use animation::{
    AnimSpec, AnimationBuilder, TransitionSpec, Keyframe,
    EASE, EASE_IN, EASE_OUT, EASE_IN_OUT, SPRING, SHARP, LINEAR,
    keyframe_css, stagger, stagger_children,
};

// ── Builders ──────────────────────────────────────────────────────────────────
pub use builders::{
    row, col, block, btn, text, txt, text_dynamic, grid, grid2, grid3, img_block,
    bold, muted, uppercase, center, h1, h2, h3, caption, label, mono, italic, strike, underline, color,
    loading, disabled,
    text_input, txtinp, input_number, innum, input_password, inpsw, checkbox, textarea, txtarea, select,
    overlay, portal, split, aspect, tooltip, drawer,
    terminal, log_view, hex_view, tree_view, status_bar, command_palette, shortcut,
    modal, tabs, accordion,
    stack, inf, scroll_to, text_bind, txtbnd, text_read, img_bind, data_list, chat_messages, chat_bubble_messages,
    video, video_ambient, audio_player, audio, model_viewer, model, iframe,
    chat_bubble, chat_ui, qr_code,
    progress_bar, rating,
    skeleton, skeleton_text,
    badge, chip, divider, spacer,
    copy_block, toast_container,
    sr_only, sr, live_region, live, skip_link,
    theme_provider,
    write_to, read_from, add_to, remove_from, clear_key, load_from,
    storage_panel, list_panel, file_storage_panel,
    Row, Col, Block, Btn, Text, Grid, Img,
    TokenBuilder, ActionChain,
    store,  // fluent API store function
    counter_text,  // reactive counter text
};

pub use schema::{Schema, FieldDef, FieldKind, PreloadStrategy, StorageFormat, schema_registry, register_schema, get_schema, schema_for_key, parse_key};

pub use render::render_dom;