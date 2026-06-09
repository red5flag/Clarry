// src/tokens/debug.rs
//
// Comprehensive debugging, validation, and inspection for TokenNode graphs.
// Use this to track node flow, validate structure, and visualize trees in-dev.

use crate::tokens::node::TokenNode;
use crate::tokens::action::TokenAction;
use std::fmt::Write;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct DebugConfig {
    pub enabled: bool,
    pub verbose: bool,
    pub show_styles: bool,
    pub show_actions: bool,
    pub validate_on_render: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self { enabled: true, verbose: false, show_styles: false, show_actions: true, validate_on_render: true }
    }
}

pub fn format_tree(node: &TokenNode, depth: usize, cfg: &DebugConfig) -> String {
    let indent = "│   ".repeat(depth);
    let connector = if depth == 0 { "" } else { "├── " };
    let mut out = String::new();
    writeln!(out, "{}{}{} [{}]{}", indent, connector, node.tag, node.id, if node.children.is_empty() { "" } else { " ▼" }).unwrap();

    if cfg.show_styles && !node.style.extra.is_empty() {
        writeln!(out, "{}   └─ style: \"{}\"", indent, truncate(&node.style.extra, 40)).unwrap();
    }
    if cfg.show_actions && !node.actions.is_empty() {
        let actions: Vec<&str> = node.actions.iter().map(|a| match a {
            TokenAction::Log { message, .. } => message,
            TokenAction::Show { show, .. } => show,
            TokenAction::Hide(id) => id,
            TokenAction::Custom(name) => name,
            _ => "?",
        }).collect();
        writeln!(out, "{}   └─ actions: [{}]", indent, actions.join(", ")).unwrap();
    }
    if let Some(content) = &node.content {
        writeln!(out, "{}   └─ content: \"{}\"", indent, truncate(content, 30)).unwrap();
    }
    if let Some(binding) = &node.data_binding {
        writeln!(out, "{}   └─ binding: {} → {}", indent, binding.key, binding.target_id.as_ref().map(|s| s.as_ref()).unwrap_or("?")).unwrap();
    }
    for child in &node.children {
        write!(out, "{}", format_tree(child, depth + 1, cfg)).unwrap();
    }
    out
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max { format!("{}...", &s[..max]) } else { s.to_string() }
}

pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub stats: TreeStats,
}
#[derive(Debug, Default)]
pub struct TreeStats { pub node_count: usize, pub depth: usize, pub action_count: usize, pub binding_count: usize }

pub fn validate_tree(root: &TokenNode) -> ValidationResult {
    let mut result = ValidationResult { is_valid: true, errors: Vec::new(), warnings: Vec::new(), stats: TreeStats::default() };
    let mut seen_ids = std::collections::HashSet::new();
    validate_node(root, &mut seen_ids, 1, &mut result);
    result.is_valid = result.errors.is_empty();
    result
}

fn validate_node(node: &TokenNode, seen_ids: &mut std::collections::HashSet<String>, depth: usize, result: &mut ValidationResult) {
    result.stats.node_count += 1; result.stats.depth = result.stats.depth.max(depth);
    result.stats.action_count += node.actions.len();
    if node.data_binding.is_some() { result.stats.binding_count += 1; }
    if !seen_ids.insert(node.id.to_string()) { result.errors.push(format!("Duplicate ID: '{}' in '{}'", node.id, node.tag)); }
    if node.id.is_empty() && (!node.actions.is_empty() || node.on_click.is_some()) {
        result.warnings.push(format!("Interactive node '{}' has empty ID", node.tag));
    }
    for child in &node.children { validate_node(child, seen_ids, depth + 1, result); }
}

pub fn log_lifecycle(stage: &str, node: &TokenNode, cfg: &DebugConfig) {
    if !cfg.enabled { return; }
    leptos::logging::log!("[TOKEN_DEBUG] ── Lifecycle: {} ──", stage);
    leptos::logging::log!("[TOKEN_DEBUG] Root: {} ({})", node.tag, node.id);
    let v = validate_tree(node);
    if v.is_valid {
        leptos::logging::log!("[TOKEN_DEBUG] Valid | Nodes: {} | Depth: {} | Actions: {}", v.stats.node_count, v.stats.depth, v.stats.action_count);
    } else {
        leptos::logging::warn!("[TOKEN_DEBUG] Invalid | Errors: {:?}", v.errors);
    }
}

// Global log storage for inspector console
use parking_lot::Mutex;
static INSPECTOR_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn inspector_log(msg: String) {
    leptos::logging::log!("[INSPECTOR] {}", msg);
    let mut logs = INSPECTOR_LOGS.lock();
    logs.push(msg);
    if logs.len() > 100 {
        logs.remove(0);
    }
}

pub fn get_inspector_logs() -> Vec<String> {
    INSPECTOR_LOGS.lock().clone()
}

pub fn clear_inspector_logs() {
    INSPECTOR_LOGS.lock().clear();
}

// ── Enhanced Hydration Issue Detection Utilities ─────────────────────

/// Compare SSR vs CSR DOM attributes for a given element ID
#[cfg(target_arch = "wasm32")]
pub fn log_dom_comparison(element_id: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(element_id) {
                // Log critical attributes
                let tag = element.tag_name();
                let class = element.get_attribute("class").unwrap_or_default();
                let style = element.get_attribute("style").unwrap_or_default();
                let data_attrs = element.get_attribute("data-ssr").unwrap_or_default();

                inspector_log(format!(
                    "[DOM_COMPARE] {}#{} | class='{}' | style='{}' | data-ssr='{}'",
                    tag, element_id, class, style, data_attrs
                ));

                // Check for common hydration mismatches
                if !data_attrs.is_empty() && style.contains("display:none") {
                    inspector_log(format!(
                        "[DOM_WARN] {}#{} has display:none but data-ssr='{}' - may indicate visibility mismatch",
                        tag, element_id, data_attrs
                    ));
                }
            }
        }
    }
}

/// Log current TokenCtx state for debugging
pub fn log_token_ctx_state(ctx: &crate::tokens::reactive::TokenCtx, label: &str) {
    let visibility_count = ctx.visibility.get().len();
    let counter_count = ctx.counters.get().len();
    let class_count = ctx.classes.get().len();

    inspector_log(format!(
        "[TOKEN_CTX_SNAPSHOT] {} | visibility:{} counters:{} classes:{}",
        label, visibility_count, counter_count, class_count
    ));

    // Log specific problematic keys
    for (key, visible) in ctx.visibility.get().iter() {
        if key.starts_with("modal_") && !visible {
            inspector_log(format!("[TOKEN_CTX] Modal '{}' is hidden (expected)", key));
        }
    }
}

#[component]
pub fn TokenTreeInspector(token: TokenNode) -> impl IntoView {
    use leptos::prelude::*;

    let tree_text = format_tree(&token, 0, &DebugConfig {
        verbose: true,
        show_styles: true,
        show_actions: true,
        ..Default::default()
    });
    let validation = validate_tree(&token);
    let status_color = if validation.is_valid { "#a6e3a1" } else { "#f38ba8" };
    let status_text = if validation.is_valid { "Valid" } else { "Invalid" };

    // ── Hydration-Specific Diagnostics ─────────────────────────────────
    let hydration_info = {
        #[cfg(feature = "ssr")]
        { "🖥️ Server-Side Rendering (SSR)" }
        #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
        { "🌐 Client-Side Hydration (CSR/WASM)" }
        #[cfg(all(not(feature = "ssr"), not(target_arch = "wasm32")))]
        { "⚠️ Unknown Environment" }
    };

    let id_counter_note = {
        // Note about ID generation for hydration matching
        "🔢 IDs auto-generated: Ensure reset_id_counter() called before tree construction"
    };

    // Console log signal with hydration context
    let (logs, set_logs) = leptos::prelude::signal(get_inspector_logs());

    // Add hydration entry log automatically
    {
        let env = hydration_info;
        let msg = format!("[INSPECTOR_INIT] Environment: {} | Nodes: {}", env, validation.stats.node_count);
        if !logs.get().iter().any(|l| l.contains("[INSPECTOR_INIT]")) {
            inspector_log(msg);
            set_logs.set(get_inspector_logs());
        }
    }

    let add_test_log = move |_| {
        inspector_log("[TEST] Manual log entry".to_string());
        set_logs.set(get_inspector_logs());
    };

    let clear_logs = move |_| {
        clear_inspector_logs();
        set_logs.set(get_inspector_logs());
    };

    // ── Hydration Mismatch Detector ─────────────────────────────────────
    let mismatch_warnings = {
        let mut warnings: Vec<String> = Vec::new();

        // Check for common hydration issues
        if validation.stats.node_count == 0 {
            warnings.push("⚠️ Empty token tree - nothing to hydrate".to_string());
        }

        // Check for duplicate IDs (hydration killer)
        let mut seen_ids = std::collections::HashSet::new();
        let mut dupes = Vec::new();
        fn check_dupes(node: &TokenNode, seen: &mut std::collections::HashSet<String>, dupes: &mut Vec<String>) {
            if !seen.insert(node.id.to_string()) {
                dupes.push(node.id.to_string());
            }
            for child in &node.children {
                check_dupes(child, seen, dupes);
            }
        }
        check_dupes(&token, &mut seen_ids, &mut dupes);

        if !dupes.is_empty() {
            warnings.push(format!("⛔ Duplicate IDs detected: {:?} - hydration will fail", dupes));
        }

        // Check for nodes with actions but no stable ID
        let mut unstable_action_nodes = Vec::new();
        fn check_actions(node: &TokenNode, unstable: &mut Vec<String>) {
            if !node.actions.is_empty() && node.id.starts_with("t") && node.id.len() <= 3 {
                unstable.push(format!("{} ({})", node.id, node.tag));
            }
            for child in &node.children {
                check_actions(child, unstable);
            }
        }
        check_actions(&token, &mut unstable_action_nodes);

        if !unstable_action_nodes.is_empty() {
            warnings.push(format!("⚠️ Action nodes with auto-IDs may cause hydration issues: {:?}", unstable_action_nodes));
        }

        warnings
    };

    view! {
        <div class="token-inspector">
            <h3 class="token-inspector-title">"🔍 Token Graph Inspector"</h3>

            // ── Environment Badge ───────────────────────────────────────
            <div class="token-inspector-env">
                <span class="token-inspector-env-badge"
                      style="background:#6366f1;color:#fff;padding:2px 8px;border-radius:4px;font-size:11px;">
                    {hydration_info}
                </span>
            </div>

            // ── Status Panel ────────────────────────────────────────────
            <div class="token-inspector-status">
                <strong>"Status: "</strong>
                <span class="token-inspector-status-text" style=format!("color:{}", status_color)>
                    {status_text}
                </span>
                <span class="token-inspector-node-count">
                    {format!("Nodes: {} | Depth: {} | Actions: {}",
                        validation.stats.node_count,
                        validation.stats.depth,
                        validation.stats.action_count)}
                </span>
            </div>

            // ── Hydration Warnings ──────────────────────────────────────
            {move || if !mismatch_warnings.is_empty() {
                view! {
                    <div class="token-inspector-warnings"
                         style="background:#452727;padding:0.5rem;margin:0.5rem 0;border-radius:4px;">
                        <strong style="color:#f38ba8;">"⚠️ Hydration Risks Detected:"</strong>
                        {mismatch_warnings.iter().map(|w| {
                            let warning = w.clone();
                            view! { <div class="token-inspector-warning" style="color:#fab387;font-size:11px;margin:2px 0;">{warning}</div> }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="token-inspector-ok"
                         style="background:#224433;padding:0.5rem;margin:0.5rem 0;border-radius:4px;color:#a6e3a1;font-size:11px;">
                        "✅ No obvious hydration mismatches detected"
                    </div>
                }.into_any()
            }}

            // ── ID Counter Note ─────────────────────────────────────────
            <div class="token-inspector-id-note"
                 style="background:#313244;padding:0.3rem 0.5rem;margin:0.3rem 0;border-radius:3px;font-size:10px;color:#89b4fa;">
                {id_counter_note}
            </div>

            // ── Errors Display ──────────────────────────────────────────
            {move || if !validation.is_valid {
                view! {
                    <div class="token-inspector-errors">
                        {validation.errors.iter().map(|e| {
                            let err = e.clone();
                            view! { <div class="token-inspector-error">"⛔ "{err}</div> }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            // ── Tree Visualization ──────────────────────────────────────
            <pre class="token-inspector-tree">{tree_text}</pre>

            // ── Console Panel with Hydration Context ────────────────────
            <div class="token-inspector-console">
                <div class="token-inspector-console-header">
                    <strong>"📋 Console Log"</strong>
                    <div style="display:flex;gap:4px;">
                        <button class="token-inspector-console-clear" on:click=clear_logs>"Clear"</button>
                        <button class="token-inspector-console-test" on:click=add_test_log>"Test"</button>
                    </div>
                </div>
                <div class="token-inspector-console-output">
                    {move || logs.get().iter().map(|log| {
                        let log_clone = log.clone();
                        let is_error = log.contains("ERROR") || log.contains("FAILED") || log.contains("⛔");
                        let is_warn = log.contains("WARN") || log.contains("⚠️");
                        let is_success = log.contains("✅") || log.contains("SUCCESS") || log.contains("COMPLETE");
                        let is_hydrate = log.contains("HYDRATE") || log.contains("CSR") || log.contains("hydrate");

                        view! {
                            <div class="token-inspector-log"
                                 class:token-inspector-log-error=is_error
                                 class:token-inspector-log-warn=is_warn
                                 class:token-inspector-log-success=is_success
                                 class:token-inspector-log-hydrate=is_hydrate>
                                {log_clone}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TokenInspectorStyles() -> impl IntoView {
    view! {
        <style>
        ".token-inspector { position:fixed; top:0; right:0; width:400px; height:100vh; background:#1e1e2e; color:#cdd6f4; font-family:monospace; font-size:12px; overflow:auto; z-index:9999; border-left:1px solid #45475a; padding:1rem; box-sizing:border-box; display:flex; flex-direction:column; }
         .token-inspector-title { margin:0 0 0.5rem 0; color:#89b4fa; flex-shrink:0; }
         .token-inspector-env { margin-bottom: 0.5rem; }
         .token-inspector-env-badge { display: inline-block; }
         .token-inspector-warnings { border-left: 3px solid #fab387; }
         .token-inspector-warning { margin: 2px 0; }
         .token-inspector-ok { border-left: 3px solid #a6e3a1; }
         .token-inspector-id-note { font-family: monospace; border-left: 3px solid #89b4fa; }
         .token-inspector-status { padding:0.5rem; background:#313244; border-radius:4px; margin-bottom:0.5rem; flex-shrink:0; }
         .token-inspector-node-count { float:right; }
         .token-inspector-errors { background:#452727; padding:0.5rem; margin-bottom:0.5rem; border-radius:4px; flex-shrink:0; }
         .token-inspector-error { color:#f38ba8; }
         .token-inspector-tree { white-space:pre-wrap; word-break:break-all; margin:0; opacity:0.9; flex-shrink:0; max-height:40vh; overflow:auto; }
         .token-inspector-console { margin-top:1rem; border-top:1px solid #45475a; padding-top:0.5rem; flex:1; display:flex; flex-direction:column; min-height:0; }
         .token-inspector-console-header { display:flex; justify-content:space-between; align-items:center; margin-bottom:0.5rem; flex-shrink:0; }
         .token-inspector-console-clear, .token-inspector-console-test { background:#313244; color:#cdd6f4; border:1px solid #45475a; padding:0.25rem 0.5rem; border-radius:4px; cursor:pointer; font-size:11px; }
         .token-inspector-console-clear:hover, .token-inspector-console-test:hover { background:#45475a; }
         .token-inspector-console-output { flex:1; overflow:auto; background:#181825; border-radius:4px; padding:0.5rem; font-size:11px; min-height:200px; max-height:40vh; }
         .token-inspector-log { padding:0.25rem 0; border-bottom:1px solid #313244; }
         .token-inspector-log-error { color:#f38ba8; }
         .token-inspector-log-warn { color:#fab387; }
         .token-inspector-log-success { color:#a6e3a1; }
         .token-inspector-log-info { color:#89b4fa; }
         .token-inspector-log-hydrate { border-left: 2px solid #89b4fa; padding-left: 0.5rem; background: rgba(137, 180, 250, 0.1); }"
        </style>
    }
}
