// src/tokens/render/hydration.rs
//
// Tree hydration, data interpolation, and toggle panel linking.

use crate::tokens::action::TokenAction;
use crate::tokens::node::{TokenNode, StyleMode, DataBinding};
use crate::tokens::storage::ListStore;

// ── Bind toggle targets ───────────────────────────────────────────────────────

pub(crate) fn bind_toggle_ids(node: &mut TokenNode) {
    let toggle_key = node.actions.iter().find_map(|a| {
        if let TokenAction::Custom(s) = a {
            s.strip_prefix("toggle:").map(|k| k.to_string())
        } else {
            None
        }
    });

    if let Some(key) = toggle_key {
        let class_str = node.class.to_string();
        if class_str.split_whitespace().any(|c| c == "hidden") {
            node.id = key.as_str().into();
            let stripped = class_str
                .split_whitespace()
                .filter(|c| *c != "hidden")
                .collect::<Vec<_>>()
                .join(" ");
            node.class = stripped.as_str().into();
            let mut extra = node.style.extra.to_string();
            if !extra.contains("display:none") {
                extra.push_str("display:none;");
                node.style.extra = extra.as_str().into();
            }
        }
    }

    for child in node.children.iter_mut() {
        bind_toggle_ids(child);
    }
}

/// After bind_toggle_ids, scan for toggle buttons (nodes with toggle: but no hidden
/// class) and link them to sibling panel nodes that have hidden class.
///
/// Heuristic: for each toggle key (e.g. "details_panel") we strip the suffix and look
/// for a hidden direct child whose data_binding key or class contains that stem.
pub(crate) fn link_toggle_panels(node: &mut TokenNode) {
    // First recurse so leaves are processed bottom-up.
    for child in node.children.iter_mut() {
        link_toggle_panels(child);
    }

    // Collect toggle keys from the entire subtree (not just direct children).
    let mut toggle_keys: Vec<String> = Vec::new();
    collect_toggle_keys(node, &mut toggle_keys);

    if toggle_keys.is_empty() {
        return;
    }

    // For each toggle key, try to find a matching hidden direct child.
    for key in toggle_keys {
        // Skip keys that are already bound to a modal/panel by bind_toggle_ids.
        // We only want keys whose target is still missing an ID.
        let stem = key
            .trim_end_matches("_panel")
            .trim_end_matches("_modal");

        for child in node.children.iter_mut() {
            let sib_class = child.class.to_string();
            if !sib_class.split_whitespace().any(|c| c == "hidden") {
                continue;
            }
            if !child.id.is_empty() {
                continue;
            }

            // Match by data_binding key or class content.
            let binding_str = child
                .data_binding
                .as_ref()
                .map(|b| b.key.to_string())
                .unwrap_or_default();
            let matches = (!stem.is_empty())
                && (binding_str.contains(stem)
                    || sib_class.contains(stem)
                    || child.content.as_ref().map(|c| c.contains(stem)).unwrap_or(false));

            if matches {
                child.id = key.as_str().into();
                let stripped = sib_class
                    .split_whitespace()
                    .filter(|c| *c != "hidden")
                    .collect::<Vec<_>>()
                    .join(" ");
                child.class = stripped.as_str().into();
                let mut extra = child.style.extra.to_string();
                if !extra.contains("display:none") {
                    extra.push_str("display:none;");
                    child.style.extra = extra.as_str().into();
                }
                break;
            }
        }
    }
}

pub(crate) fn collect_toggle_keys(node: &TokenNode, out: &mut Vec<String>) {
    let has_toggle = node.actions.iter().any(|a| {
        matches!(a, TokenAction::Custom(s) if s.starts_with("toggle:"))
    });
    let has_hidden = node.class.split_whitespace().any(|c| c == "hidden");
    if has_toggle && !has_hidden {
        if let Some(key) = node.actions.iter().find_map(|a| {
            if let TokenAction::Custom(s) = a {
                s.strip_prefix("toggle:").map(|k| k.to_string())
            } else {
                None
            }
        }) {
            if !out.contains(&key) {
                out.push(key);
            }
        }
    }
    for child in &node.children {
        collect_toggle_keys(child, out);
    }
}

// ── Deep panel linker ────────────────────────────────────────────────────────
//
// Scans the entire tree so toggle buttons and their panels can live in
// separate sibling subtrees (e.g. tabs in one container, panels in another).

pub(crate) fn resolve_toggle_panels(node: &mut TokenNode) {
    let mut toggle_keys: Vec<String> = Vec::new();
    collect_toggle_keys(node, &mut toggle_keys);
    // Only process panel toggles (not modals)
    let panel_keys: Vec<String> = toggle_keys.into_iter().filter(|k| k.ends_with("_panel")).collect();
    if panel_keys.is_empty() {
        return;
    }
    for key in &panel_keys {
        let stem = key.trim_end_matches("_panel").trim_end_matches("_modal");
        if stem.is_empty() {
            continue;
        }
        find_and_link_panel(node, key, stem);
    }
}

pub(crate) fn find_and_link_panel(node: &mut TokenNode, key: &str, stem: &str) -> bool {
    if node.id.to_string() == key {
        return true;
    }
    let binding_str = node.data_binding.as_ref().map(|b| b.key.to_string()).unwrap_or_default();
    let class_str = node.class.to_string();
    let matches = binding_str.contains(stem) || class_str.contains(stem);
    let looks_like_panel = node.data_binding.is_some() || !node.children.is_empty();
    if matches && looks_like_panel && node.id.is_empty() {
        node.id = key.into();
        let mut extra = node.style.extra.to_string();
        if !extra.contains("display:none") {
            extra.push_str("display:none;");
            node.style.extra = extra.as_str().into();
        }
        if class_str.split_whitespace().any(|c| c == "hidden") {
            let stripped = class_str.split_whitespace().filter(|c| *c != "hidden").collect::<Vec<_>>().join(" ");
            node.class = stripped.as_str().into();
        }
        return true;
    }
    for child in node.children.iter_mut() {
        if find_and_link_panel(child, key, stem) {
            return true;
        }
    }
    false
}

// ── Tab-panel mutual exclusivity ─────────────────────────────────────────────
//
// When a container has multiple toggle:*_panel buttons, replace each toggle
// with a Show action that reveals the selected panel and hides siblings.

pub(crate) fn make_panels_mutually_exclusive(node: &mut TokenNode) {
    for child in node.children.iter_mut() {
        make_panels_mutually_exclusive(child);
    }
    // A tab row is a Row whose direct children each have a toggle:*_panel action
    let panel_keys: Vec<String> = node.children.iter().filter_map(|child| {
        child.actions.iter().find_map(|a| {
            if let TokenAction::Custom(s) = a {
                s.strip_prefix("toggle:").filter(|k| k.ends_with("_panel")).map(|k| k.to_string())
            } else {
                None
            }
        })
    }).collect();
    if panel_keys.len() < 2 {
        return;
    }
    for child in node.children.iter_mut() {
        if let Some(idx) = child.actions.iter().position(|a| {
            if let TokenAction::Custom(s) = a {
                s.starts_with("toggle:") && s.ends_with("_panel")
            } else {
                false
            }
        }) {
            if let TokenAction::Custom(s) = &child.actions[idx] {
                let show_key = s.strip_prefix("toggle:").unwrap().to_string();
                let hide_keys: Vec<crate::tokens::node::Str> = panel_keys.iter()
                    .filter(|k| *k != &show_key)
                    .map(|k| k.as_str().into())
                    .collect();
                child.actions[idx] = TokenAction::Show { show: show_key.clone().into(), hide: hide_keys };
                // Give the tab button an ID so we can style it when active
                let tab_id = show_key.trim_end_matches("_panel").to_string();
                if child.id.is_empty() {
                    child.id = format!("{}_tab", tab_id).as_str().into();
                }
            }
        }
    }
}

// ── Generic layout normalization ───────────────────────────────────────────────
//
// Walks the token tree and applies generic formatting corrections so the
// DSL author doesn't have to specify every Tailwind detail.

pub(crate) fn normalize_layout(node: &mut TokenNode) {
    for child in node.children.iter_mut() {
        normalize_layout(child);
    }

    let class = node.class.to_string();

    // 1. Text nodes with flex-1 + vertical padding but no horizontal padding
    //    get a sensible default so text isn't crammed at the edges.
    if node.content.is_some()
        && class.contains("flex-1")
        && class.contains("py-")
        && !class.contains("px-")
    {
        node.class = format!("{} px-4", class).as_str().into();
    }

    // 2. Row containers with alignment helpers but no gap get a small default gap
    //    so flex items don't butt up against each other.
    //    Only apply if no gap class AND no inline gap style is present.
    if matches!(node.layout, crate::tokens::node::Layout::Row)
        && (class.contains("justify-") || class.contains("items-center"))
        && !class.contains("gap-")
        && node.style.gap.is_none()
    {
        node.class = format!("{} gap-4", class).as_str().into();
    }

    // 3. Flex-direction conflict: when the CSS class contains `flex-row` but the
    //    inline style has `flex-direction:column` (from Layout::Col), the inline
    //    style wins and breaks the layout. Remove the conflicting inline style.
    if class.contains("flex-row") {
        let extra = node.style.extra.to_string();
        if extra.contains("flex-direction:column") {
            node.style.extra = extra.replace("flex-direction:column;", "").replace("flex-direction:column", "").as_str().into();
        }
    }

    // 8. Chat/message containers that use flex-row for alignment but lack
    //    min-h-0 can cause overflow issues in flex parents.
    if class.contains("flex-row")
        && (class.contains("overflow-y-auto") || class.contains("overflow-y-auto"))
        && !class.contains("min-h-")
        && !class.contains("min-h-0")
    {
        node.class = format!("{} min-h-0", class).as_str().into();
    }
}

// ── Assign display IDs for DOM updating ────────────────────────────────────────
//
// ── Ensure tab panels exist ─────────────────────────────────────────────────
//
// When a tab row references panels that don't exist in the tree (e.g. home
// profile modal only has a posts grid), create placeholder panels so the
// Show actions have something to reveal.

pub(crate) fn ensure_tab_panels(node: &mut TokenNode) {
    for child in node.children.iter_mut() {
        ensure_tab_panels(child);
    }

    // Find tab containers among direct children (blocks that contain a tab row)
    let tab_container_indices: Vec<usize> = node.children.iter().enumerate()
        .filter(|(_, child)| {
            child.children.iter().any(|c| {
                c.actions.iter().any(|a| {
                    if let TokenAction::Custom(s) = a {
                        s.starts_with("toggle:") && s.ends_with("_panel")
                    } else { false }
                })
            })
        })
        .map(|(i, _)| i)
        .collect();

    if tab_container_indices.is_empty() {
        return;
    }

    // Collect all panel toggle keys from tab rows inside tab containers
    let mut all_panel_keys: Vec<String> = Vec::new();
    for &idx in &tab_container_indices {
        let container = &node.children[idx];
        for child in &container.children {
            for action in &child.actions {
                if let TokenAction::Custom(s) = action {
                    if let Some(key) = s.strip_prefix("toggle:") {
                        if key.ends_with("_panel") && !all_panel_keys.contains(&key.to_string()) {
                            all_panel_keys.push(key.to_string());
                        }
                    }
                }
            }
        }
    }

    if all_panel_keys.len() < 2 {
        return;
    }

    // Find existing panel IDs and assign IDs to content blocks that have matching bindings.
    // Convention: a panel named "foo_panel" matches a data binding named "foo".
    let mut existing_ids: Vec<String> = Vec::new();
    for child in node.children.iter_mut() {
        let id = child.id.to_string();
        if id.ends_with("_panel") {
            existing_ids.push(id);
            continue;
        }
        // Try to match by stripping "_panel" suffix from panel keys
        if let Some(binding) = &child.data_binding {
            let bkey = binding.key.as_ref();
            for panel_key in &all_panel_keys {
                if existing_ids.contains(panel_key) {
                    continue;
                }
                let stem = panel_key.trim_end_matches("_panel");
                if bkey == stem || bkey == panel_key.as_str() {
                    child.id = panel_key.as_str().into();
                    existing_ids.push(panel_key.clone());
                    break;
                }
            }
        }
        // Also check children one level deep for data bindings (e.g. grid inside block)
        if child.id.is_empty() {
            for grandchild in child.children.iter_mut() {
                if let Some(binding) = &grandchild.data_binding {
                    let bkey = binding.key.as_ref();
                    for panel_key in &all_panel_keys {
                        if existing_ids.contains(panel_key) {
                            continue;
                        }
                        let stem = panel_key.trim_end_matches("_panel");
                        if bkey == stem || bkey == panel_key.as_str() {
                            child.id = panel_key.as_str().into();
                            existing_ids.push(panel_key.clone());
                            break;
                        }
                    }
                }
            }
        }
    }

    for key in &all_panel_keys {
        if !existing_ids.contains(key) {
            let mut placeholder = TokenNode::new(key.as_str());
            placeholder.tag = "div".into();
            placeholder.class = "flex items-center justify-center min-h-[200px]".into();
            placeholder.style_mode = StyleMode::Class;
            let mut extra = placeholder.style.extra.to_string();
            extra.push_str("display:none;");
            placeholder.style.extra = extra.as_str().into();

            let mut text = TokenNode::new(format!("{}_placeholder_text", key));
            text.tag = "span".into();
            text.content = Some("No content yet".into());
            text.class = "text-gray-400 text-sm".into();
            placeholder.children.push(text);

            node.children.push(placeholder);
        }
    }
}

// ── Data hydration ──────────────────────────────────────────────────────────
//
// Walks the token tree and injects real data from ListStore for list bindings.
// Replaces {{context.field}} placeholders with actual values.

use regex::Regex;
use once_cell::sync::Lazy;

static PLACEHOLDER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\{\{([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)(?:\s*\?\s*'([^']*)'\s*:\s*'([^']*)')?\}\}"
    ).unwrap()
});

pub(crate) fn get_json_field<'a>(data: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut current = data;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

pub(crate) fn evaluate_ternary(value: Option<&serde_json::Value>, true_val: &str, false_val: &str) -> String {
    let truthy = match value {
        Some(serde_json::Value::Bool(b)) => *b,
        Some(serde_json::Value::String(s)) => !s.is_empty() && s != "false" && s != "0",
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0) != 0.0,
        Some(_) => true,
        None => false,
    };
    if truthy { true_val.to_string() } else { false_val.to_string() }
}

pub(crate) fn interpolate_placeholders(s: &str, data: &serde_json::Value) -> String {
    let mut result = s.to_string();
    while let Some(caps) = PLACEHOLDER_RE.captures(&result) {
        let m = caps.get(0).unwrap();
        let field_path = caps.get(1).unwrap().as_str();
        let replacement = if let Some(true_val) = caps.get(2) {
            let false_val = caps.get(3).unwrap().as_str();
            evaluate_ternary(get_json_field(data, field_path), true_val.as_str(), false_val)
        } else {
            match get_json_field(data, field_path) {
                Some(serde_json::Value::String(s)) => s.clone(),
                Some(serde_json::Value::Number(n)) => n.to_string(),
                Some(serde_json::Value::Bool(b)) => b.to_string(),
                _ => String::new(),
            }
        };
        result.replace_range(m.start()..m.end(), &replacement);
    }
    result
}

pub(crate) fn interpolate_node(node: &mut TokenNode, data: &serde_json::Value) {
    if let Some(content) = &node.content {
        let new_content = interpolate_placeholders(content, data);
        if new_content != content.as_ref() {
            node.content = Some(new_content.as_str().into());
        }
    }
    if let Some(binding) = &node.data_binding {
        let new_key = interpolate_placeholders(&binding.key, data);
        if new_key != binding.key.as_ref() {
            node.data_binding = Some(DataBinding {
                key: new_key.as_str().into(),
                target_id: binding.target_id.clone(),
                mode: binding.mode.clone(),
            });
        }
    }
    let new_class = interpolate_placeholders(&node.class, data);
    if new_class != node.class.as_ref() {
        node.class = new_class.as_str().into();
    }
    for child in node.children.iter_mut() {
        interpolate_node(child, data);
    }
}

pub(crate) fn infer_data_source(list_type: &str, parent: &TokenNode) -> Option<String> {
    // If parent has a non-list data binding, use it directly
    if let Some(binding) = &parent.data_binding {
        let key = binding.key.as_ref();
        if !key.starts_with("list:") && !key.is_empty() {
            return Some(key.to_string());
        }
    }
    // Otherwise, use the list_type as the data source key
    Some(list_type.to_string())
}

pub(crate) fn hydrate_tree(node: &mut TokenNode) {
    // First, recursively process existing children (non-list items)
    for child in node.children.iter_mut() {
        hydrate_tree(child);
    }

    // Then, scan for list templates and expand them
    let mut i = 0;
    while i < node.children.len() {
        let is_list = node.children[i].data_binding.as_ref()
            .map(|b| b.key.starts_with("list:"))
            .unwrap_or(false);

        if is_list {
            let list_type = node.children[i].data_binding.as_ref().unwrap()
                .key.strip_prefix("list:").unwrap().to_string();
            let source_key = infer_data_source(&list_type, node);

            if let Some(source) = source_key {
                let items = ListStore::global().get_list(&source);
                if !items.is_empty() {
                    let template = node.children.remove(i);
                    for item in items {
                        let mut instance = template.clone();
                        interpolate_node(&mut instance, &item);
                        instance.data_binding = None;
                        node.children.insert(i, instance);
                        i += 1;
                    }
                    continue; // skip i += 1 since we already advanced
                }
            }
        }
        i += 1;
    }
}
