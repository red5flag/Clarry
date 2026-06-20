// src/tokens/render.rs
//
// Reactive rendering engine for token trees.
// Transforms TokenNode → RenderOp list → Leptos views.
// All UI state is managed via TokenCtx signals (no imperative DOM).

use leptos::prelude::*;
use serde_json;

use crate::tokens::debug::{log_lifecycle, validate_tree, DebugConfig, inspector_log};

use crate::tokens::action::TokenAction;
use crate::tokens::node::TokenNode;
use crate::tokens::reactive::TokenCtx;

use super::hydration::{
    ensure_tab_panels, bind_toggle_ids, link_toggle_panels,
    resolve_toggle_panels, make_panels_mutually_exclusive,
    normalize_layout, hydrate_tree,
};
use super::element::build_div;

// ── Public entry point ────────────────────────────────────────────────────────

/// Wraps the render pipeline in a Leptos component to guarantee context
/// boundaries across SSR/Hydrate.
#[component]
pub fn TokenRenderer(mut root: TokenNode) -> impl IntoView {
    // NOTE: Do NOT reset the ID counter here.  IDs are assigned when the token
    // tree is constructed (before this component runs).  The reset belongs in
    // the page component, before calling page_token().

    #[cfg(feature = "ssr")]
    {
        inspector_log("[HYDRATE_TRACE] 🖥️ SSR: <TokenRenderer> Component Mounting".to_string());
        leptos::logging::log!("[HYDRATION] 🖥️ SSR: Starting server-side render");
    }
    #[cfg(not(feature = "ssr"))]
    {
        // ── PROMINENT CSR DETECTION LOGGING ───────────────────────────────────
        leptos::logging::log!("═══════════════════════════════════════════════════════════");
        leptos::logging::log!("[CSR_DETECT] 🌐🌐🌐 CSR: <TokenRenderer> Component Mounting on CLIENT!");
        leptos::logging::log!("═══════════════════════════════════════════════════════════");
        inspector_log("[HYDRATE_TRACE] 🌐 CSR/Hydrate: <TokenRenderer> Component Mounting".to_string());
        leptos::logging::log!("[HYDRATION] 🌐 CSR: Starting client-side hydration");

        // Log DOM state before hydration for mismatch detection
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let body_children = body.child_nodes().length();
                    leptos::logging::log!("[HYDRATION] 🌐 CSR: Body has {} child nodes before hydration", body_children);
                    inspector_log(format!("[HYDRATION] 🌐 CSR: Body has {} child nodes before hydration", body_children));

                    // Check for common hydration mismatch signs
                    if body_children == 0 {
                        leptos::logging::error!("[HYDRATION] ⛔ CSR ERROR: Body has no children - SSR may have failed or returned empty HTML");
                        inspector_log("[HYDRATION] ⛔ CSR ERROR: Body has no children - SSR may have failed or returned empty HTML".to_string());
                    }
                }
            }
        }
    }

    // ── Hydration Timing Metrics ─────────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    let hydration_start = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    leptos::logging::log!("[TOKEN_RENDER] 1️⃣ STARTING RENDER PIPELINE");
    let debug_cfg = DebugConfig::default();
    let validation = validate_tree(&root);
    if !validation.is_valid {
        leptos::logging::error!("[TOKEN_RENDER] ⛔ TREE VALIDATION FAILED: {:?}", validation.errors);
        return view! { <div style="color:red;padding:1rem;">Token Tree Invalid. Check Console.</div> }.into_any();
    }
    leptos::logging::log!("[TOKEN_RENDER] ✅ Validation Passed");
    log_lifecycle("SSR_INPUT", &root, &debug_cfg);

    // 🟢 HYDRATION TRACE: Context Creation
    #[cfg(feature = "ssr")]
    inspector_log("[HYDRATE_TRACE] 🖥️ SSR: Creating TokenCtx".to_string());
    #[cfg(not(feature = "ssr"))]
    inspector_log("[HYDRATE_TRACE] 🌐 CSR/Hydrate: Creating TokenCtx".to_string());

    let ctx = TokenCtx::new();
    ensure_tab_panels(&mut root);
    bind_toggle_ids(&mut root);
    link_toggle_panels(&mut root);
    resolve_toggle_panels(&mut root);
    make_panels_mutually_exclusive(&mut root);
    normalize_layout(&mut root);
    hydrate_tree(&mut root);
    ctx.seed_from_tree(&root);
    ctx.hide_prefixed("modal_");
    // 🟢 HYDRATION TRACE: Context Provisioning
    #[cfg(feature = "ssr")]
    {
        inspector_log("[HYDRATE_TRACE] 🖥️ SSR: Context Seeded & Ready".to_string());
        leptos::logging::log!("[HYDRATION] 🖥️ SSR: Context seeded with {} hidden elements", ctx.visibility.get().len());
    }
    #[cfg(not(feature = "ssr"))]
    {
        inspector_log("[HYDRATE_TRACE] 🌐 CSR/Hydrate: Context Seeded & Ready".to_string());
        leptos::logging::log!("[HYDRATION] 🌐 CSR: Context seeded with {} hidden elements", ctx.visibility.get().len());
    }

    inspector_log("[HYDRATE_TRACE] 📦 Calling provide_context(TokenCtx)...".to_string());
    provide_context(ctx);
    inspector_log("[HYDRATE_TRACE] ✅ Context Provided to Component Tree".to_string());

    // ── Context Ready Timing Metric ───────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        let context_ready = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        inspector_log(format!(
            "[HYDRATE_METRIC] Context ready: {:.2}ms after render start",
            context_ready - hydration_start
        ));
    }

    let ops = flatten(root.clone());
    let ops_len = ops.len();
    leptos::logging::log!("[TOKEN_RENDER] 📦 Flattened {} ops", ops_len);
    log_lifecycle("CSR_OUTPUT", &root, &debug_cfg);

    let result = build_views(ops);

    // ── Final Hydration Timing Metric ───────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        let hydration_end = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        let total_time = hydration_end - hydration_start;
        inspector_log(format!(
            "[HYDRATE_METRIC] ✅ Full hydration completed in {:.2}ms | nodes:{} ops:{}",
            total_time,
            validation.stats.node_count,
            ops_len
        ));

        // Flag slow hydration
        if total_time > 1000.0 {
            inspector_log(format!(
                "[HYDRATE_WARN] ⚠️ Hydration took {:.2}ms (>1s) - consider optimizing token tree",
                total_time
            ));
        }
    }

    #[cfg(feature = "ssr")]
    {
        leptos::logging::log!("[HYDRATION] 🖥️ SSR: Render pipeline completed successfully");
        inspector_log("[HYDRATE_TRACE] 🖥️ SSR: Render pipeline completed".to_string());
    }
    #[cfg(not(feature = "ssr"))]
    {
        leptos::logging::log!("[HYDRATION] 🌐 CSR: Hydration pipeline completed successfully");
        inspector_log("[HYDRATE_TRACE] 🌐 CSR: Hydration pipeline completed".to_string());

        // ── PROMINENT CSR COMPLETION LOGGING ───────────────────────────────────
        leptos::logging::log!("═══════════════════════════════════════════════════════════");
        leptos::logging::log!("[CSR_DETECT] 🌐🌐🌐 CSR: TokenRenderer hydration pipeline COMPLETED!");
        leptos::logging::log!("═══════════════════════════════════════════════════════════");

        // Log DOM state after hydration for mismatch detection
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let body_children_after = body.child_nodes().length();
                    leptos::logging::log!("[HYDRATION] 🌐 CSR: Body has {} child nodes after hydration", body_children_after);
                    inspector_log(format!("[HYDRATION] 🌐 CSR: Body has {} child nodes after hydration", body_children_after));

                    // Check if hydration modified the DOM (sign of mismatch)
                    if body_children_after == 0 {
                        leptos::logging::error!("[HYDRATION] ⛔ CSR ERROR: Body still empty after hydration - complete hydration failure");
                        inspector_log("[HYDRATION] ⛔ CSR ERROR: Body still empty after hydration - complete hydration failure".to_string());
                    } else {
                        leptos::logging::log!("[HYDRATION] ✅ CSR: Hydration appears successful - DOM has content");
                        inspector_log("[HYDRATION] ✅ CSR: Hydration appears successful - DOM has content".to_string());
                    }
                }
            }
        }
    }

    result.into_any()
}

/// Render a token tree as a fully-reactive Leptos view.
///
/// * Creates a `TokenCtx`, pre-seeds it from any `display:none` nodes.
/// * Provides the context so every node's click handler can update signals.
/// * Every Show/Hide/ToggleClass action updates Leptos signals → no
///   imperative DOM manipulation needed.
pub fn render_dom(root: TokenNode) -> AnyView {
    view! { <TokenRenderer root=root /> }.into_any()
}
// ── Flatten tree → linear op list ────────────────────────────────────────────

fn flatten(root: TokenNode) -> Vec<RenderOp> {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();
    leptos::logging::log!("[TOKEN_FLATTEN] START - root_id: {}", root.id);

    let mut ops: Vec<RenderOp> = Vec::with_capacity(256);
    let mut stack: Vec<(TokenNode, bool)> = vec![(root, false)];
    let mut node_count = 0;

    while let Some((mut node, visited)) = stack.pop() {
        if visited {
            ops.push(RenderOp::Close);
            continue;
        }

        node_count += 1;

        let style_str = node.style.compile();
        let class_str = {
            let layout_prefix = match &node.layout {
                crate::tokens::node::Layout::Row => Some("flex "),
                crate::tokens::node::Layout::Col => Some("flex flex-col "),
                crate::tokens::node::Layout::Grid { .. } => Some("grid "),
                _ => None,
            };
            if node.class.is_empty() {
                layout_prefix.map(|p| p.trim().to_string())
            } else {
                match layout_prefix {
                    Some(p) => Some(format!("{}{}", p, node.class)),
                    None => Some(node.class.to_string()),
                }
            }
        };
        let binding = node.data_binding.as_ref()
            .and_then(|b| serde_json::to_string(b).ok());

        ops.push(RenderOp::Open {
            tag:           node.tag.to_string(),
            id:            node.id.to_string(),
            style:         style_str,
            class:         class_str,
            content:       node.content.as_ref().map(|c| c.to_string()),
            data_binding:  binding,
            actions:       node.actions.clone(),
            event_bindings: node.event_bindings.clone(),
            on_nav:        node.on_nav.as_ref().map(|s| s.to_string()),
            on_click:      node.on_click.clone(),
            dynamic_content: node.dynamic_content.clone(),
            input_type:    node.input_type.as_ref().map(|s| s.to_string()),
            placeholder: node.placeholder.as_ref().map(|s| s.to_string()),
            name:          node.name.as_ref().map(|s| s.to_string()),
            variant:       node.variant.as_ref().map(|s| s.to_string()),
            size:          node.size.as_ref().map(|s| s.to_string()),
            loading:       node.loading,
            disabled:      node.disabled,
            attributes:    node.attributes.clone(),
        });

        stack.push((TokenNode::new(""), true));

        for child in node.children.drain(..).rev() {
            stack.push((child, false));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    leptos::logging::log!("[TOKEN_FLATTEN] COMPLETE - {} nodes processed in {}ms", node_count, start_time.elapsed().as_millis());
    #[cfg(target_arch = "wasm32")]
    leptos::logging::log!("[TOKEN_FLATTEN] COMPLETE - {} nodes processed", node_count);
    ops
}
// ── Build views from op list ──────────────────────────────────────────────────

fn build_views(ops: Vec<RenderOp>) -> impl IntoView {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();
    leptos::logging::log!("[TOKEN_BUILD_VIEWS] START - {} operations", ops.len());

    let mut view_stack: Vec<Vec<AnyView>> = vec![vec![]];
    let mut meta_stack: Vec<NodeMeta>     = vec![];
    let mut built_count = 0;

    for op in ops {
        match op {
            RenderOp::Open {
                tag, id, style, class, content, data_binding,
                actions, event_bindings, on_nav, on_click, dynamic_content,
                input_type, placeholder, name,
                variant, size, loading, disabled, attributes,
            } => {
                view_stack.push(vec![]);
                meta_stack.push(NodeMeta {
                    tag, id, style, class, content, data_binding,
                    actions, event_bindings, on_nav, on_click, dynamic_content,
                    input_type, placeholder, name,
                    variant, size, loading, disabled, attributes,
                });
            }
            RenderOp::Close => {
                let children = view_stack.pop().unwrap_or_default();
                let meta     = meta_stack.pop().unwrap();
                let view     = build_div(meta, children);
                view_stack.last_mut().unwrap().push(view);
                built_count += 1;
            }
        }
    }

    let mut root = view_stack.pop().unwrap_or_default();
    let result = if root.is_empty() {
        view! { <div /> }.into_any()
    } else {
        root.remove(0)
    };

    #[cfg(not(target_arch = "wasm32"))]
    leptos::logging::log!("[TOKEN_BUILD_VIEWS] COMPLETE - {} views built in {}ms", built_count, start_time.elapsed().as_millis());
    #[cfg(target_arch = "wasm32")]
    leptos::logging::log!("[TOKEN_BUILD_VIEWS] COMPLETE - {} views built", built_count);
    result
}

// ── Op / Meta types ───────────────────────────────────────────────────────────

#[allow(clippy::large_enum_variant)]
pub enum RenderOp {
    Open {
        tag:           String,
        id:            String,
        style:         String,
        class:         Option<String>,
        content:       Option<String>,
        data_binding:  Option<String>,
        actions:       Vec<TokenAction>,
        event_bindings: Vec<crate::tokens::action::EventBinding>,
        on_nav:        Option<String>,
        on_click:      Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
        dynamic_content: Option<std::sync::Arc<dyn Fn() -> AnyView + Send + Sync>>,
        input_type:    Option<String>,
        placeholder:   Option<String>,
        name:            Option<String>,
        variant:       Option<String>,
        size:          Option<String>,
        loading:       bool,
        disabled:      bool,
        attributes:    std::collections::HashMap<crate::tokens::node::Str, crate::tokens::node::Str>,
    },
    Close,
}

pub struct NodeMeta {
    pub tag:           String,
    pub id:            String,
    pub style:         String,
    pub class:         Option<String>,
    pub content:       Option<String>,
    pub data_binding:  Option<String>,
    pub actions:       Vec<TokenAction>,
    pub event_bindings: Vec<crate::tokens::action::EventBinding>,
    pub on_nav:        Option<String>,
    pub on_click:      Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    pub dynamic_content: Option<std::sync::Arc<dyn Fn() -> AnyView + Send + Sync>>,
    pub input_type:    Option<String>,
    pub placeholder:   Option<String>,
    pub name:          Option<String>,
    pub variant:       Option<String>,
    pub size:          Option<String>,
    pub loading:       bool,
    pub disabled:      bool,
    pub attributes:    std::collections::HashMap<crate::tokens::node::Str, crate::tokens::node::Str>,
}
