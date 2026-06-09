// src/tokens/render/element.rs
//
// Element rendering: converts RenderOp → Leptos views.

use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

use crate::tokens::action::TokenAction;
use crate::tokens::reactive::TokenCtx;
use crate::tokens::debug::inspector_log;

use super::pipeline::NodeMeta;
use super::executor::execute_token_action_reactive;

// ── Content interpolation ───────────────────────────────────────────────────────

/// Interpolate template markers in a content string with reactive state.
///
/// Recognised markers:
///   `{count:id}` — reactive counter value from `TokenCtx.counters`
///
/// Any text outside markers is rendered as static text.  If no context is
/// available, counter placeholders fall back to `"0"`.
pub(crate) fn interpolate_content(content: &str, ctx: Option<TokenCtx>) -> AnyView {
    let raw = content.to_string();

    // Fast path – no template markers at all.
    if !raw.contains("{count:") {
        return view! { {raw} }.into_any();
    }

    // Split on `{count:…}` markers and build a mixed static/reactive fragment.
    let mut parts: Vec<AnyView> = Vec::new();
    let mut rest = raw.as_str();

    while let Some(start) = rest.find("{count:") {
        // Emit any literal text before the marker.
        if start > 0 {
            let literal = rest[..start].to_string();
            parts.push(view! { {literal} }.into_any());
        }

        // Find the closing brace.
        if let Some(end_rel) = rest[start + 7..].find('}') {
            let counter_id = rest[start + 7..start + 7 + end_rel].to_string();
            rest = &rest[start + 7 + end_rel + 1..];

            // Build a reactive view that subscribes to the counter signal.
            match ctx {
                Some(c) => {
                    let id = counter_id.clone();
                    let text = move || {
                        let val = c.counters.get().get(&id).copied().unwrap_or(0);
                        format!("{val}")
                    };
                    parts.push(view! { {text} }.into_any());
                }
                None => {
                    parts.push(view! { {"0"} }.into_any());
                }
            }
        } else {
            // Malformed marker – emit the rest as literal text.
            parts.push(view! { {rest.to_string()} }.into_any());
            break;
        }
    }

    // Emit any trailing literal text after the last marker.
    if !rest.is_empty() {
        parts.push(view! { {rest.to_string()} }.into_any());
    }

    if parts.len() == 1 {
        parts.remove(0)
    } else {
        view! { <>{parts}</> }.into_any()
    }
}

// ── Build a single element ──────────────────────────────────────────────────────

pub(crate) fn build_div(meta: NodeMeta, children: Vec<AnyView>) -> AnyView {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();
    leptos::logging::log!("[TOKEN_BUILD_ELEMENT] START - tag: {}, id: {}, children: {}", meta.tag, meta.id, children.len());

    // 🟢 HYDRATION TRACE: Context Retrieval Check
    let ctx: Option<TokenCtx> = use_context::<TokenCtx>();
    match ctx {
        Some(_) => inspector_log(format!("[HYDRATE_TRACE] ✅ use_context SUCCESS - Context found for element '{}'", meta.id)),
        None    => {
            leptos::logging::error!("[HYDRATE_TRACE] ❌ use_context FAILED - Context missing for element '{}'! Boundary broken.", meta.id);
            inspector_log(format!("[HYDRATE_TRACE] ❌ use_context FAILED - Context missing for element '{}'! Boundary broken.", meta.id));
        }
    }

    // 🟢 CLIENT DETECTION: Log whether element is built on client or server
    #[cfg(target_arch = "wasm32")]
    {
        inspector_log(format!("[HYDRATION_TRACE] 🌐 CLIENT: Element '{}' built", meta.id));

        // Check if element exists in DOM (for pure CSR, elements won't exist initially)
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(_element) = document.get_element_by_id(&meta.id) {
                    leptos::logging::log!("[HYDRATION] 🌐 CSR: Element '{}' found in DOM - hydration should succeed", meta.id);
                } else {
                    // For pure CSR, elements not found is expected - not an error
                    leptos::logging::log!("[HYDRATION] 🌐 CSR: Element '{}' not found in DOM (pure CSR - creating new)", meta.id);
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        inspector_log(format!("[HYDRATION_TRACE] 🖥️ SERVER: Element '{}' built", meta.id));
        leptos::logging::log!("[HYDRATION] 🖥️ SSR: Element '{}' rendered with ID: {}", meta.id, meta.id);
    }

    let tag             = meta.tag.clone();
    let id              = meta.id.clone();
    let id_for_style    = id.clone();
    let id_for_class    = id.clone();
    let base_style      = meta.style.clone();
    let base_class      = meta.class.clone().unwrap_or_default();
    let actions         = meta.actions.clone();
    let on_click_fn     = meta.on_click.clone();
    let on_nav          = meta.on_nav;
    let ctx_for_handler = ctx; // Capture for closure
    let is_disabled     = meta.disabled || meta.loading;

    // ── Variant / size class mapping ──────────────────────────────────────
    // Compose Tailwind classes from variant + size fields.
    let variant_classes: String = {
        let mut parts = Vec::new();
        if let Some(v) = meta.variant.as_deref() {
            parts.push(match v {
                "primary" => "bg-blue-600 text-white hover:bg-blue-700",
                "secondary" => "bg-gray-200 text-gray-800 hover:bg-gray-300",
                "danger" => "bg-red-600 text-white hover:bg-red-700",
                "ghost" => "bg-transparent text-gray-700 hover:bg-gray-100",
                "success" => "bg-green-600 text-white hover:bg-green-700",
                _ => "",
            });
        }
        if let Some(s) = meta.size.as_deref() {
            parts.push(match s {
                "sm" => "px-2 py-1 text-sm",
                "md" => "px-4 py-2 text-base",
                "lg" => "px-6 py-3 text-lg",
                _ => "",
            });
        }
        if meta.loading {
            parts.push("opacity-70 cursor-wait");
        }
        if meta.disabled {
            parts.push("opacity-50 cursor-not-allowed");
        }
        parts.join(" ")
    };
    let base_class = if variant_classes.is_empty() {
        base_class
    } else {
        format!("{} {}", base_class, variant_classes)
    };

    // Detect keyboard actions (key:)
    let key_actions: Vec<TokenAction> = actions.iter().filter(|a| {
        if let TokenAction::Custom(s) = a { s.starts_with("key:") } else { false }
    }).cloned().collect();

    // ── Keydown handler (for key: actions on inputs / buttons) ────────────
    let actions_for_keydown = actions.clone();
    let id_for_keydown = id.clone();
    let keydown_handler = move |ev: leptos::ev::KeyboardEvent| {
        if key_actions.is_empty() { return; }
        let pressed = ev.key();
        let mut matched = false;
        for action in &key_actions {
            if let TokenAction::Custom(s) = action {
                if let Some(key) = s.strip_prefix("key:") {
                    if pressed.eq_ignore_ascii_case(key) {
                        matched = true;
                    }
                }
            }
        }
        if matched {
            inspector_log(format!("[HYDRATE_TRACE] ⌨️  Key '{}' pressed on '{}'", pressed, id_for_keydown));
            if let Some(ctx) = ctx_for_handler {
                execute_token_action_reactive(&actions_for_keydown, ctx);
            }
        }
    };

    // ── Reactive style closure ─────────────────────────────────────────────
    //
    // Visibility signal logic:
    //   Some(false) → append "display:none;" (overrides any existing display)
    //   Some(true)  → strip "display:none;" if present; element uses whatever
    //                 display its CSS declares (flex, block, etc.)
    //   None        → return base style unchanged
    //
    // Elements that use display:flex for centering (like test_modal) should have
    // "display:flex;" in their CSS.  We override it hidden with an appended
    // "display:none;" (last declaration wins in CSS), and restore it by removing
    // that suffix when shown.

    let style_closure = move || {
        let vis = ctx.and_then(|c| c.visibility.get().get(&id_for_style).copied());
        match vis {
            Some(false) => {
                // Strip any existing display:none to avoid duplication, then append.
                let clean = base_style.replace("display:none;", "");
                format!("{}display:none;", clean)
            }
            Some(true) => {
                // Strip any display:none that was hard-coded or previously appended.
                base_style.replace("display:none;", "")
            }
            None => base_style.clone(),
        }
    };

    // ── Click handler ─────────────────────────────────────────────────
    let id_for_log = id.clone();
    let id_for_closure = id_for_log.clone();
    let has_click_handler = !actions.is_empty() || on_click_fn.is_some() || on_nav.is_some();
    leptos::logging::log!("[TOKEN_BUILD_ELEMENT] Creating click handler for id: {}, actions: {}, has_on_click_fn: {}, has_click_handler: {}", id_for_log, actions.len(), on_click_fn.is_some(), has_click_handler);

    let actions_for_click = actions.clone();
    let class_for_click = base_class.clone();

    // ── Reactive class (hydration-safe closure) ────────────────────────
    let base_class_clean: Vec<String> = base_class.split_whitespace().filter(|&c| c != "hidden").map(|s| s.to_string()).collect();
    let base_class_no_hidden = base_class_clean.join(" ");
    let class_closure = move || {
        let extra = ctx.map(|c| c.classes.get().get(&id_for_class).map(|v| v.join(" ")).unwrap_or_default()).unwrap_or_default();
        let vis = ctx.and_then(|c| c.visibility.get().get(&id_for_class).copied());
        let hidden = matches!(vis, Some(false));
        let mut parts: Vec<String> = Vec::new();
        if !base_class_no_hidden.is_empty() { parts.push(base_class_no_hidden.clone()); }
        if !extra.is_empty() { parts.push(extra); }
        if hidden { parts.push("hidden".to_string()); }
        parts.join(" ")
    };
    #[allow(unused_variables)]
    let click_handler = move |ev: leptos::ev::MouseEvent| {
        inspector_log(format!("[HYDRATE_TRACE] 🔘 Clicked: '{}'", id_for_closure));

        // Modal backdrop guard: if this element is a modal backdrop (fixed overlay),
        // only execute the toggle when the click target IS the backdrop itself.
        // Clicks on modal content children must not bubble up and close the modal.
        let is_modal_backdrop = class_for_click.contains("fixed")
            && class_for_click.contains("inset-0")
            && actions_for_click.iter().any(|a| {
                match a {
                    TokenAction::Custom(s) => s.starts_with("toggle:") && (s.ends_with("_modal") || s.contains("modal")),
                    TokenAction::Hide(_) => true,
                    _ => false,
                }
            });
        if is_modal_backdrop {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(target) = ev.target() {
                    if let Some(current) = ev.current_target() {
                        if !target.eq(&current) {
                            // Click came from a child element inside the modal — ignore
                            return;
                        }
                    }
                }
            }
        }

        if let Some(ctx) = ctx_for_handler {
            inspector_log("[HYDRATE_TRACE] ⚡ Executing actions with valid context".to_string());
            execute_token_action_reactive(&actions_for_click, ctx);
        } else {
            leptos::logging::error!("[HYDRATE_TRACE] ⛔ Click event fired but context is None! Event detached.");
            inspector_log("[HYDRATE_TRACE] ⛔ Click event fired but context is None! Event detached.".to_string());
        }
        if let Some(f) = &on_click_fn { f(); }
        if let Some(page) = &on_nav {
            inspector_log(format!("[HYDRATE_TRACE] 🧭 Navigate to page: {}", page));
            #[cfg(target_arch = "wasm32")]
            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(&format!("/{}", page));
            }
        }
    };

    leptos::logging::log!("[TOKEN_BUILD_ELEMENT] Click handler created for id: {}", id_for_log);

    let content_view = meta.content.clone().map(|c| {
        let interpolated = interpolate_content(&c, ctx);
        view! { <span>{interpolated}</span> }.into_any()
    })
    .or_else(|| meta.dynamic_content.map(|f| f()));
    let content_placeholder = meta.content.clone().unwrap_or_default();

    // Detect `in:NAME` action — forces element to render as <input type="text" name=NAME>
    let in_name = actions.iter().find_map(|a| {
        if let TokenAction::Custom(s) = a {
            s.strip_prefix("in:").map(|n| n.to_string())
        } else { None }
    });
    let tag = if in_name.is_some() { "input".to_string() } else { tag };
    let in_name_clone = in_name.clone();

    // Render different HTML elements based on tag / special actions
    let result = if tag == "input" {
        // ── Input element ──────────────────────────────────────────────────────
        let input_name: Option<String> = meta.name.as_deref().map(|s| s.to_string())
            .or_else(|| in_name_clone.clone());
        let input_type = meta.input_type.as_deref().unwrap_or("text");
        let placeholder = meta.placeholder.as_deref().unwrap_or("");

        let disabled_flag = is_disabled;
        if has_click_handler {
            view! {
                <input
                    id=id
                    style=style_closure
                    class=class_closure
                    on:click=click_handler
                    on:keydown=keydown_handler
                    data-binding=meta.data_binding
                    type=input_type
                    name=input_name
                    placeholder=placeholder
                    disabled=disabled_flag
                />
            }.into_any()
        } else {
            view! {
                <input
                    id=id
                    style=style_closure
                    class=class_closure
                    on:keydown=keydown_handler
                    data-binding=meta.data_binding
                    type=input_type
                    name=input_name
                    placeholder=placeholder
                    disabled=disabled_flag
                />
            }.into_any()
        }
    } else {
        match tag.as_str() {
            "button" => {
                let disabled_flag = is_disabled;
                let btn_content = if meta.loading {
                    view! {
                        <span class="inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2"></span>
                        <span>Loading...</span>
                    }.into_any()
                } else {
                    content_view.unwrap_or_else(|| view! { <span></span> }.into_any())
                };
                if has_click_handler && !is_disabled {
                    view! {
                        <button
                            id=id
                            style=style_closure
                            class=class_closure
                            on:click=click_handler
                            data-binding=meta.data_binding
                            type="button"
                            disabled=disabled_flag
                        >
                            {btn_content}
                            {children}
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <button
                            id=id
                            style=style_closure
                            class=class_closure
                            data-binding=meta.data_binding
                            type="button"
                            disabled=disabled_flag
                        >
                            {btn_content}
                            {children}
                        </button>
                    }.into_any()
                }
            }
            "span" => {
                if has_click_handler {
                    view! {
                        <span
                            id=id
                            style=style_closure
                            class=class_closure
                            on:click=click_handler
                            data-binding=meta.data_binding
                        >
                            {content_view}
                            {children}
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <span
                            id=id
                            style=style_closure
                            class=class_closure
                            data-binding=meta.data_binding
                        >
                            {content_view}
                            {children}
                        </span>
                    }.into_any()
                }
            }
            "img" => {
                view! {
                    <img
                        id=id
                        style=style_closure
                        class=class_closure
                        src=content_placeholder.clone()
                        alt=content_placeholder
                    />
                }.into_any()
            }
            "textarea" => {
                view! {
                    <textarea
                        id=id
                        style=style_closure
                        class=class_closure
                        data-binding=meta.data_binding
                        disabled=is_disabled
                    >
                        {content_placeholder}
                    </textarea>
                }.into_any()
            }
            "select" => {
                view! {
                    <select
                        id=id
                        style=style_closure
                        class=class_closure
                        data-binding=meta.data_binding
                        disabled=is_disabled
                    >
                        {children}
                    </select>
                }.into_any()
            }
            "pre" => {
                view! {
                    <pre
                        id=id
                        style=style_closure
                        class=class_closure
                        data-binding=meta.data_binding
                    >
                        {content_view}
                        {children}
                    </pre>
                }.into_any()
            }
            "video" | "audio" => {
                let src = meta.content.clone().unwrap_or_default();
                let has_controls = meta.attributes.contains_key("controls");
                let has_autoplay = meta.attributes.contains_key("autoplay");
                let has_loop = meta.attributes.contains_key("loop");
                let has_muted = meta.attributes.contains_key("muted");
                let has_playsinline = meta.attributes.contains_key("playsinline");
                let preload = meta.attributes.get("preload").cloned().unwrap_or_default().to_string();
                if tag == "video" {
                    view! {
                        <video
                            id=id
                            style=style_closure
                            class=class_closure
                            src=src
                            controls=has_controls
                            autoplay=has_autoplay
                            loop=has_loop
                            muted=has_muted
                            playsinline=has_playsinline
                            preload=preload
                        />
                    }.into_any()
                } else {
                    view! {
                        <audio
                            id=id
                            style=style_closure
                            class=class_closure
                            src=src
                            controls=has_controls
                            autoplay=has_autoplay
                            loop=has_loop
                            muted=has_muted
                            preload=preload
                        />
                    }.into_any()
                }
            }
            "iframe" => {
                view! {
                    <iframe
                        id=id
                        style=style_closure
                        class=class_closure
                        src=content_placeholder
                    />
                }.into_any()
            }
            "model-viewer" => {
                let src = meta.content.clone().unwrap_or_default();
                view! {
                    <model-viewer
                        id=id
                        style=style_closure
                        class=class_closure
                        src=src
                        camera-controls
                        auto-rotate
                    />
                }.into_any()
            }
            "style" => {
                view! {
                    <style>{content_placeholder}</style>
                }.into_any()
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = tag.chars().nth(1).unwrap_or('1');
                match level {
                    '1' => view! { <h1 id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</h1> }.into_any(),
                    '2' => view! { <h2 id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</h2> }.into_any(),
                    '3' => view! { <h3 id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</h3> }.into_any(),
                    '4' => view! { <h4 id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</h4> }.into_any(),
                    '5' => view! { <h5 id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</h5> }.into_any(),
                    _   => view! { <h6 id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</h6> }.into_any(),
                }
            }
            "p" => {
                view! { <p id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</p> }.into_any()
            }
            "a" => {
                let href = meta.attributes.get("href").cloned().unwrap_or_default().to_string();
                let target = meta.attributes.get("target").cloned().unwrap_or_default().to_string();
                let rel = if target == "_blank" { "noopener noreferrer" } else { "" };
                if has_click_handler {
                    view! {
                        <a id=id style=style_closure class=class_closure data-binding=meta.data_binding
                           href=href target=target rel=rel on:click=click_handler>
                            {content_view}{children}
                        </a>
                    }.into_any()
                } else {
                    view! {
                        <a id=id style=style_closure class=class_closure data-binding=meta.data_binding
                           href=href target=target rel=rel>
                            {content_view}{children}
                        </a>
                    }.into_any()
                }
            }
            "ul" => {
                view! { <ul id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</ul> }.into_any()
            }
            "ol" => {
                view! { <ol id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</ol> }.into_any()
            }
            "li" => {
                view! { <li id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</li> }.into_any()
            }
            "label" => {
                let label_for = meta.attributes.get("for").cloned().unwrap_or_default().to_string();
                view! { <label id=id style=style_closure class=class_closure data-binding=meta.data_binding for=label_for>{content_view}{children}</label> }.into_any()
            }
            "form" => {
                let _form_id = meta.id.clone();
                let actions_for_form = actions.clone();
                let ctx_for_form = ctx_for_handler;
                let submit_handler = move |ev: leptos::ev::SubmitEvent| {
                    ev.prevent_default();
                    if let Some(ctx) = ctx_for_form {
                        execute_token_action_reactive(&actions_for_form, ctx);
                    }
                };
                view! {
                    <form id=id style=style_closure class=class_closure data-binding=meta.data_binding
                          on:submit=submit_handler>
                        {children}
                    </form>
                }.into_any()
            }
            "table" => {
                view! { <table id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</table> }.into_any()
            }
            "thead" => {
                view! { <thead id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</thead> }.into_any()
            }
            "tbody" => {
                view! { <tbody id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</tbody> }.into_any()
            }
            "tr" => {
                view! { <tr id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</tr> }.into_any()
            }
            "th" => {
                view! { <th id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</th> }.into_any()
            }
            "td" => {
                view! { <td id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</td> }.into_any()
            }
            "nav" | "header" | "footer" | "main" | "section" | "article" | "aside" => {
                if has_click_handler {
                    match tag.as_str() {
                        "nav"     => view! { <nav     id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</nav>     }.into_any(),
                        "header"  => view! { <header  id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</header>  }.into_any(),
                        "footer"  => view! { <footer  id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</footer>  }.into_any(),
                        "main"    => view! { <main    id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</main>    }.into_any(),
                        "section" => view! { <section id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</section> }.into_any(),
                        "article" => view! { <article id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</article> }.into_any(),
                        "aside"   => view! { <aside   id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</aside>   }.into_any(),
                        _         => view! { <div     id=id style=style_closure class=class_closure data-binding=meta.data_binding on:click=click_handler>{content_view}{children}</div>     }.into_any(),
                    }
                } else {
                    match tag.as_str() {
                        "nav"     => view! { <nav     id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</nav>     }.into_any(),
                        "header"  => view! { <header  id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</header>  }.into_any(),
                        "footer"  => view! { <footer  id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</footer>  }.into_any(),
                        "main"    => view! { <main    id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</main>    }.into_any(),
                        "section" => view! { <section id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</section> }.into_any(),
                        "article" => view! { <article id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</article> }.into_any(),
                        "aside"   => view! { <aside   id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</aside>   }.into_any(),
                        _         => view! { <div     id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</div>     }.into_any(),
                    }
                }
            }
            "canvas" => {
                let width = meta.attributes.get("width").cloned().unwrap_or_default().to_string();
                let height = meta.attributes.get("height").cloned().unwrap_or_default().to_string();
                view! {
                    <canvas id=id style=style_closure class=class_closure data-binding=meta.data_binding
                            width=width height=height>
                        {children}
                    </canvas>
                }.into_any()
            }
            "svg" => {
                let raw_html = format!(
                    "<svg id=\"{}\" style=\"{}\" class=\"{}\">{}</svg>",
                    meta.id, meta.style, meta.class.unwrap_or_default(), content_placeholder
                );
                view! { <div inner_html=raw_html /> }.into_any()
            }
            "kbd" | "code" | "blockquote" => {
                match tag.as_str() {
                    "kbd"        => view! { <kbd        id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</kbd>        }.into_any(),
                    "code"       => view! { <code       id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</code>       }.into_any(),
                    "blockquote" => view! { <blockquote id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</blockquote> }.into_any(),
                    _ => unreachable!(),
                }
            }
            "hr" => {
                view! { <hr id=id style=style_closure class=class_closure data-binding=meta.data_binding /> }.into_any()
            }
            "details" => {
                view! { <details id=id style=style_closure class=class_closure data-binding=meta.data_binding>{children}</details> }.into_any()
            }
            "summary" => {
                view! { <summary id=id style=style_closure class=class_closure data-binding=meta.data_binding>{content_view}{children}</summary> }.into_any()
            }
            "script" => {
                view! { <script>{content_placeholder}</script> }.into_any()
            }
            "picture" | "source" => {
                let raw_tag = meta.tag.clone();
                let mut attrs = String::new();
                for (k, v) in &meta.attributes {
                    if v.is_empty() {
                        attrs.push_str(&format!(" {}", k));
                    } else {
                        attrs.push_str(&format!(" {}=\"{}\"", k, v));
                    }
                }
                let raw_html = format!("<{} id=\"{}\" style=\"{}\" class=\"{}\" {} />", raw_tag, meta.id, meta.style, meta.class.unwrap_or_default(), attrs);
                view! { <div inner_html=raw_html /> }.into_any()
            }
            _ => {
                if has_click_handler {
                    view! {
                        <div
                            id=id
                            style=style_closure
                            class=class_closure
                            on:click=click_handler
                            data-binding=meta.data_binding
                        >
                            {content_view}
                            {children}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div
                            id=id
                            style=style_closure
                            class=class_closure
                            data-binding=meta.data_binding
                        >
                            {content_view}
                            {children}
                        </div>
                    }.into_any()
                }
            }
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    leptos::logging::log!("[TOKEN_BUILD_ELEMENT] COMPLETE - tag: {}, id: {}, duration: {}ms", meta.tag, meta.id, start_time.elapsed().as_millis());
    #[cfg(target_arch = "wasm32")]
    leptos::logging::log!("[TOKEN_BUILD_ELEMENT] COMPLETE - tag: {}, id: {}", meta.tag, meta.id);

    // ── Event bindings (WASM-only via raw DOM) ─────────────────────────
    // Leptos view! macros only support static event attributes, so we attach
    // programmatically after the element is rendered.  Each binding creates a
    // leaked Closure so the listener stays alive for the element's lifetime.
    // Debounce / throttle are applied here using setTimeout and performance.now().
    //
    // NOTE: closure.forget() leaks memory.  A static HashSet prevents duplicate
    // listeners if this Effect ever re-runs (e.g. component re-mount).  Full
    // cleanup requires storing closures and removing listeners on unmount.
    #[cfg(target_arch = "wasm32")]
    {
        use std::collections::HashSet;
        use parking_lot::Mutex;
        use once_cell::sync::Lazy;
        static LISTENER_IDS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

        let id_for_events = id_for_log.clone();
        let bindings = meta.event_bindings.clone();
        let ctx_for_events = ctx;
        leptos::prelude::Effect::new(move |_| {
            // Prevent duplicate listeners on the same element ID
            {
                let mut set = LISTENER_IDS.lock();
                if set.contains(&id_for_events) {
                    return;
                }
                set.insert(id_for_events.clone());
            }
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(el) = doc.get_element_by_id(&id_for_events) {
                    for binding in &bindings {
                        let event_name = match binding.event {
                            crate::tokens::action::EventType::Click => "click",
                            crate::tokens::action::EventType::DoubleClick => "dblclick",
                            crate::tokens::action::EventType::MouseEnter => "mouseenter",
                            crate::tokens::action::EventType::MouseLeave => "mouseleave",
                            crate::tokens::action::EventType::Focus => "focus",
                            crate::tokens::action::EventType::Blur => "blur",
                            crate::tokens::action::EventType::Change => "change",
                            crate::tokens::action::EventType::KeyDown => "keydown",
                            crate::tokens::action::EventType::Scroll => "scroll",
                            crate::tokens::action::EventType::Resize => "resize",
                            _ => continue,
                        };
                        let action = binding.action.clone();
                        let ctx = ctx_for_events;
                        let debounce_ms = binding.debounce_ms;
                        let throttle_ms = binding.throttle_ms;

                        let closure: Box<dyn FnMut(web_sys::Event)> = if let Some(ms) = debounce_ms {
                            use std::rc::Rc;
                            use std::cell::Cell;
                            let timeout_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
                            Box::new(move |_: web_sys::Event| {
                                if let Some(id) = timeout_id.get() {
                                    if let Some(win) = web_sys::window() {
                                        win.clear_timeout_with_handle(id);
                                    }
                                }
                                let action = action.clone();
                                let timeout = timeout_id.clone();
                                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                                    timeout.set(None);
                                    if let Some(c) = ctx {
                                        execute_token_action_reactive(&[action], c);
                                    }
                                });
                                if let Some(win) = web_sys::window() {
                                    if let Ok(id) = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                                        cb.as_ref().unchecked_ref(),
                                        ms as i32,
                                    ) {
                                        timeout_id.set(Some(id));
                                    }
                                }
                            })
                        } else if let Some(ms) = throttle_ms {
                            use std::rc::Rc;
                            use std::cell::Cell;
                            let last_run: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
                            let ms_f64 = ms as f64;
                            Box::new(move |_: web_sys::Event| {
                                let now = web_sys::window()
                                    .and_then(|w| w.performance())
                                    .map(|p| p.now())
                                    .unwrap_or(0.0);
                                if now - last_run.get() >= ms_f64 {
                                    last_run.set(now);
                                    if let Some(c) = ctx {
                                        execute_token_action_reactive(&[action.clone()], c);
                                    }
                                }
                            })
                        } else {
                            Box::new(move |_: web_sys::Event| {
                                if let Some(c) = ctx {
                                    execute_token_action_reactive(&[action.clone()], c);
                                }
                            })
                        };

                        let closure = wasm_bindgen::closure::Closure::wrap(closure);
                        let _ = el.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
                        closure.forget(); // keep listener alive for element lifetime
                    }
                }
            }
        });
    }

    result
}
