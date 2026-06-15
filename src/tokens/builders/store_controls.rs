// src/tokens/builders/store_controls.rs
//
// Inline storage control widgets — pre-wired UI blocks for reading, writing,
// adding to, and removing from storage keys, all in one expression.

use crate::tokens::node::{Layout, Str, TokenNode};
use crate::tokens::core::id::next_id;
use crate::tokens::core::types::IntoToken;
use crate::tokens::action::{
    TokenAction,
    store_set_input, store_push, store_remove, store_write_to_path,
    preload,
};
use super::types::{Container, Block};
use super::factory::{text_read, text_dynamic};

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Derive a stable input element ID from a storage key.
/// e.g. "notes.draft" → "notes_draft_input"
fn input_id(storage_key: &str) -> String {
    format!("{}_input", storage_key.replace('.', "_"))
}

/// Derive a stable display element ID from a storage key.
/// e.g. "notes.draft" → "notes_draft_display"
fn display_id(storage_key: &str) -> String {
    format!("{}_display", storage_key.replace('.', "_"))
}

fn label_node(s: impl Into<Str>) -> TokenNode {
    let mut n = TokenNode::new(next_id());
    n.content = Some(s.into());
    n.class = "text-xs font-medium text-gray-500 uppercase tracking-wide mb-1".into();
    n
}

fn key_badge(key: &str) -> TokenNode {
    let mut n = TokenNode::new(next_id());
    n.content = Some(format!("storage: {key}").into());
    n.class = "text-xs font-mono bg-gray-100 text-gray-600 px-2 py-0.5 rounded border border-gray-200 mb-2 self-start".into();
    n
}

fn row_node(css: &str) -> TokenNode {
    let mut n = TokenNode::new(next_id());
    n.layout = Layout::Row;
    n.class = css.into();
    n
}

fn col_node(css: &str) -> TokenNode {
    let mut n = TokenNode::new(next_id());
    n.layout = Layout::Col;
    n.class = css.into();
    n
}

fn input_node(placeholder: &str, id: &str) -> TokenNode {
    let mut n = TokenNode::new(id);
    n.tag = "input".into();
    n.input_type = Some("text".into());
    n.placeholder = Some(placeholder.into());
    n.class = "flex-1 px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-l bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-400".into();
    n
}

fn action_btn(label: &str, css: &str, action: TokenAction) -> TokenNode {
    let mut n = TokenNode::new(next_id());
    n.tag = "button".into();
    n.content = Some(label.into());
    n.class = css.into();
    n.actions.push(action);
    n
}

/// Plain scalar display — shows raw stored value.
fn display_node(storage_key: &str) -> TokenNode {
    let key = storage_key.to_string();
    let mut wrapper = col_node("text-sm font-mono bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded px-3 py-2 min-h-[2rem] break-all text-gray-700 dark:text-gray-300");
    wrapper.children.push(text_read(storage_key).into_node());
    // Fallback hint when empty
    let hint = {
        let key2 = key.clone();
        text_dynamic(move || {
            use leptos::prelude::*;
            use crate::tokens::reactive::TokenCtx;
            use crate::tokens::storage::primitive::Store;
            let empty = use_context::<TokenCtx>()
                .map(|ctx| {
                    let _rev = ctx.list_rev.get();
                    ctx.strings.get().get(&key2).cloned()
                        .or_else(|| Store::read(&key2))
                        .unwrap_or_default()
                        .is_empty()
                })
                .unwrap_or(true);
            if empty { "(empty)".to_string() } else { String::new() }
        })
    };
    let mut hint_node = hint.into_node();
    hint_node.class = "text-gray-400 dark:text-gray-600 italic text-xs".into();
    wrapper.children.push(hint_node);
    wrapper
}

/// List display — parses JSON array and shows one item per line.
fn list_display_node(storage_key: &str) -> TokenNode {
    let key = storage_key.to_string();
    let mut wrapper = col_node("bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded px-3 py-2 min-h-[2rem] gap-1");
    let node = {
        let key2 = key.clone();
        text_dynamic(move || {
            use leptos::prelude::*;
            use crate::tokens::reactive::TokenCtx;
            use crate::tokens::storage::primitive::Store;
            let raw = use_context::<TokenCtx>()
                .map(|ctx| {
                    let _rev = ctx.list_rev.get();
                    ctx.strings.get().get(&key2).cloned()
                        .or_else(|| Store::read(&key2))
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            if raw.is_empty() {
                return "(empty)".to_string();
            }
            // Parse as JSON array of objects with "text" field
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&raw) {
                if arr.is_empty() {
                    return "(empty)".to_string();
                }
                arr.iter()
                    .filter_map(|v| v.get("text").and_then(|t| t.as_str()).map(|s| format!("• {}", s)))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                raw
            }
        })
    };
    let mut item_node = node.into_node();
    item_node.class = "text-sm font-mono text-gray-700 dark:text-gray-300 whitespace-pre-line".into();
    wrapper.children.push(item_node);
    wrapper
}

fn wrapper(key: &str) -> TokenNode {
    let mut n = col_node("flex flex-col gap-1.5 p-3 bg-white rounded-lg border border-gray-200 shadow-sm");
    n.attributes.insert("data-storage-key".into(), key.into());
    n
}

// ── Public factory functions ─────────────────────────────────────────────────

/// **write_to("key")** — Inline save control.  
/// Renders: `[key badge] [label] [input ________] [Save]`  
/// Clicking Save writes the input value to the storage key.
///
/// ```dsl
/// write_to("user.name")
/// ```
pub fn write_to(key: &'static str) -> Block {
    let iid = next_id();
    let btn_css = "px-3 py-1.5 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-r font-medium transition-colors";

    let mut root = wrapper(key);
    root.children.push(key_badge(key));
    root.children.push(label_node("Write"));

    let mut row = row_node("flex items-center");
    row.children.push(input_node("New value…", &iid));
    row.children.push(action_btn("Save", btn_css, store_set_input(key, iid.clone())));
    root.children.push(row);

    Container { stack: vec![root] }
}

/// **read_from("key")** — Inline live display.  
/// Renders: `[key badge] [label] [live value display]`  
/// The display auto-updates whenever the storage key changes.
///
/// ```dsl
/// read_from("user.name")
/// ```
pub fn read_from(key: &'static str) -> Block {
    let mut root = wrapper(key);
    root.children.push(key_badge(key));
    root.children.push(label_node("Read"));
    root.children.push(display_node(key));
    Container { stack: vec![root] }
}

/// **add_to("key")** — Inline list-append control.  
/// Renders: `[key badge] [label] [input ________] [Add]`  
/// Clicking Add appends the typed value to the JSON array at this key.
///
/// ```dsl
/// add_to("tasks.list")
/// ```
pub fn add_to(key: &'static str) -> Block {
    let iid = next_id();
    let btn_css = "px-3 py-1.5 text-sm bg-green-500 hover:bg-green-600 text-white rounded-r font-medium transition-colors";

    let mut root = wrapper(key);
    root.children.push(key_badge(key));
    root.children.push(label_node("Add to list"));

    let mut row = row_node("flex items-center");
    row.children.push(input_node("Item to add…", &iid));
    row.children.push(action_btn("Add", btn_css, store_push(key, iid.clone())));
    root.children.push(row);
    root.children.push(list_display_node(key));

    Container { stack: vec![root] }
}

/// **remove_from("key")** — Inline list-remove control.  
/// Renders: `[key badge] [label] [input ________] [Remove]`  
/// Clicking Remove deletes all entries matching the typed value from the array.
///
/// ```dsl
/// remove_from("tasks.list")
/// ```
pub fn remove_from(key: &'static str) -> Block {
    let iid = next_id();
    let btn_css = "px-3 py-1.5 text-sm bg-red-500 hover:bg-red-600 text-white rounded-r font-medium transition-colors";

    let mut root = wrapper(key);
    root.children.push(key_badge(key));
    root.children.push(label_node("Remove from list (type exact text)"));

    let mut row = row_node("flex items-center");
    row.children.push(input_node("Exact text to remove…", &iid));
    row.children.push(action_btn("Remove", btn_css, store_remove(key, iid.clone())));
    root.children.push(row);
    root.children.push(list_display_node(key));

    Container { stack: vec![root] }
}

/// **clear_key("key")** — One-click clear button.  
/// Renders: `[key badge] [Delete key ✕]`
///
/// ```dsl
/// clear_key("user.session")
/// ```
pub fn clear_key(key: &'static str) -> Block {
    let btn_css = "px-3 py-1.5 text-sm bg-gray-200 hover:bg-red-100 text-gray-700 hover:text-red-700 rounded font-medium transition-colors self-start";

    let mut root = wrapper(key);
    root.children.push(key_badge(key));
    root.children.push(action_btn(&format!("Delete key  ✕"), btn_css, TokenAction::StoreDelete { key: key.into() }));

    Container { stack: vec![root] }
}

/// **load_from("key", "/endpoint")** — Fetch + display control.  
/// Renders: `[key badge] [Load from /endpoint] [live display]`  
/// Clicking Load fetches the endpoint and stores the result under `key`.
///
/// ```dsl
/// load_from("api.users", "/api/users")
/// ```
pub fn load_from(key: &'static str, endpoint: &'static str) -> Block {
    let btn_css = "px-3 py-1.5 text-sm bg-purple-500 hover:bg-purple-600 text-white rounded font-medium transition-colors self-start";

    let mut root = wrapper(key);
    root.children.push(key_badge(key));
    root.children.push(action_btn(&format!("Load from  {endpoint}"), btn_css, preload(key, endpoint)));
    root.children.push(display_node(key));
    Container { stack: vec![root] }
}

/// **storage_panel("key")** — Full CRUD panel for a single key in one call.  
/// Renders write + read + clear controls stacked vertically.
///
/// ```dsl
/// storage_panel("user.profile")
/// ```
pub fn storage_panel(key: &'static str) -> Block {
    let iid = next_id();
    let _did = display_id(key);

    let save_css  = "px-3 py-1.5 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-r font-medium transition-colors";
    let clear_css = "px-2 py-1 text-xs bg-gray-100 hover:bg-red-100 text-gray-600 hover:text-red-600 rounded font-medium transition-colors";

    let mut root = wrapper(key);
    root.class = "flex flex-col gap-2 p-3 bg-white rounded-lg border border-gray-200 shadow-sm".into();

    // Header: key badge + clear button in same row
    let mut header = row_node("flex items-center justify-between mb-1");
    header.children.push(key_badge(key));
    header.children.push(action_btn("Clear ✕", clear_css, TokenAction::StoreDelete { key: key.into() }));
    root.children.push(header);

    // Write row
    root.children.push(label_node("Write"));
    let mut write_row = row_node("flex items-center");
    write_row.children.push(input_node("New value…", &iid));
    write_row.children.push(action_btn("Save", save_css, store_set_input(key, iid.clone())));
    root.children.push(write_row);

    // Read display — reactive via text_read
    root.children.push(label_node("Current value"));
    root.children.push(display_node(key));
    Container { stack: vec![root] }
}

/// **list_panel("key")** — Full add/remove/display panel for a JSON array key.  
/// Combines add_to + remove_from + live list display in one widget.
///
/// ```dsl
/// list_panel("tasks.list")
/// ```
pub fn list_panel(key: &'static str) -> Block {
    let add_iid    = next_id();
    let remove_iid = next_id();
    let _did       = display_id(key);

    let add_css    = "px-3 py-1.5 text-sm bg-green-500 hover:bg-green-600 text-white rounded-r font-medium transition-colors";
    let remove_css = "px-3 py-1.5 text-sm bg-red-500 hover:bg-red-600 text-white rounded-r font-medium transition-colors";
    let clear_css  = "px-2 py-1 text-xs bg-gray-100 hover:bg-red-100 text-gray-600 hover:text-red-600 rounded font-medium transition-colors";

    let mut root = wrapper(key);
    root.class = "flex flex-col gap-2 p-3 bg-white rounded-lg border border-gray-200 shadow-sm".into();

    // Header
    let mut header = row_node("flex items-center justify-between mb-1");
    header.children.push(key_badge(key));
    header.children.push(action_btn("Clear ✕", clear_css, TokenAction::StoreDelete { key: key.into() }));
    root.children.push(header);

    // Add row
    root.children.push(label_node("Add item"));
    let mut add_row = row_node("flex items-center");
    add_row.children.push(input_node("Item to add…", &add_iid));
    add_row.children.push(action_btn("Add", add_css, store_push(key, add_iid.clone())));
    root.children.push(add_row);

    // Remove row
    root.children.push(label_node("Remove item"));
    let mut rem_row = row_node("flex items-center");
    rem_row.children.push(input_node("Item to remove…", &remove_iid));
    rem_row.children.push(action_btn("Remove", remove_css, store_remove(key, remove_iid.clone())));
    root.children.push(rem_row);

    // Live display — list items parsed from JSON array
    root.children.push(label_node("Contents"));
    root.children.push(list_display_node(key));
    Container { stack: vec![root] }
}

/// **file_storage_panel("key")** — Write a value to a path under /storage/<dir>/<key>.json.
/// User types a subdirectory and a value; saves via store to the given path and displays it.
///
/// ```dsl
/// file_storage_panel("files.mydata")
/// ```
pub fn file_storage_panel(_key: &'static str) -> Block {
    let path_iid = next_id();
    let val_iid  = next_id();
    let save_css = "px-3 py-1.5 text-sm bg-indigo-500 hover:bg-indigo-600 text-white rounded-r font-medium transition-colors";

    let mut root = col_node("flex flex-col gap-2 p-3 bg-white rounded-lg border border-indigo-200 shadow-sm");

    // Instructions
    let mut hint = label_node("Type a dot-notation key path and a value, then Save. The value persists in /storage.");
    hint.class = "text-xs text-gray-500 mb-1".into();
    root.children.push(hint);

    // Path input
    root.children.push(label_node("Storage key path (e.g. storage.notes.draft)"));
    let mut path_row = row_node("flex items-center");
    path_row.children.push(input_node("storage.notes.draft", &path_iid));
    root.children.push(path_row);

    // Value input + Save
    root.children.push(label_node("Value to write"));
    let mut val_row = row_node("flex items-center");
    val_row.children.push(input_node("Value…", &val_iid));
    val_row.children.push(action_btn("Save", save_css, store_write_to_path(path_iid.clone(), val_iid.clone())));
    root.children.push(val_row);

    // Feedback display
    root.children.push(label_node("Last written"));
    root.children.push(display_node("storage.last_written"));

    Container { stack: vec![root] }
}
