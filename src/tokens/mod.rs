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
pub mod structural; // ← Core structural primitives (foreach, bind, component, etc.)

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
    store_push, store_remove, store_inc, store_tog,
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
    row, row_named, row_ref, col, col_named, col_ref, block, block_named, block_ref,
    grid, grid_named, grid_ref, grid2, grid2_named, grid3, grid3_named, img_block,
    btn, text, txt, text_dynamic,
    bold, muted, uppercase, center, h1, h2, h3, caption, label, mono, italic, strike, underline, color,
    loading, disabled,
    text_input, txtinp, input_number, innum, input_password, inpsw, checkbox, textarea, txtarea, select,
    overlay, portal, split, aspect, tooltip, drawer,
    terminal, log_view, hex_view, tree_view, shortcut,
    tab, section,
    stack, inf, instagram_sidebar, scroll_to, text_bind, txtbnd, text_read, img_bind, data_list, chat_messages, chat_bubble_messages,
    json_list, json_count, json_field,
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

// ── Structural Primitives ─────────────────────────────────────────────────────
pub use structural::{
    // Collection iteration
    foreach, ForeachBuilder, Foreach,
    // Conditionals
    if_true, if_false, if_eq, if_exists, if_gt, if_lt,
    ConditionalBuilder, Conditional, Condition,
    // Collection operations
    count, filter, sort, find, limit,
    FilterBuilder, SortBuilder, FindBuilder, LimitBuilder,
    CollectionView, CollectionOp, FilterPredicate,
    // Components
    component, use_component, slot,
    ComponentDefBuilder, ComponentInstanceBuilder,
    Template, ComponentInstance, SlotDef,
    // Bindings
    bind, global, local, literal, computed,
    BindingSource,
    // Relations
    relation, RelationBuilder, Relation,
    // Queries
    query, QueryBuilder, QueryDef, JoinDef,
    // Scopes
    local_scope, ScopeBuilder, ScopeBinding,
};