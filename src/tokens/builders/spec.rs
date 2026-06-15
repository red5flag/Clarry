// src/tokens/builders/spec.rs
//
// TokenBuilder trait and ActionChain — the core fluent API specification.

use std::sync::Arc;
use leptos::prelude::IntoView;

use crate::tokens::action::{
    TokenAction, EventBinding, EventType, LogLevel,
    DataTarget, ScrollBehavior,
};
use crate::tokens::animation::AnimationBuilder;
use crate::tokens::node::{DataBinding, IntoToken, Str, StyleMode, TokenNode};
use crate::tokens::render::render_dom;

// ── TokenBuilder trait ────────────────────────────────────────────────────────

pub trait TokenBuilder: Sized + IntoToken {
    fn node_mut(&mut self) -> &mut TokenNode;

    // ── Identity ─────────────────────────────────────────────────────────
    fn id(mut self, id: impl Into<Str>) -> Self {
        self.node_mut().id = id.into(); self
    }

    // ── Background ───────────────────────────────────────────────────────
    fn bg(mut self, r: u8, g: u8, b: u8) -> Self {
        self.node_mut().style.bg = Some([r, g, b, 255]); self
    }
    fn bg_a(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.node_mut().style.bg = Some([r, g, b, a]); self
    }
    fn bg_hsl(self, h: u16, s: u8, l: u8) -> Self {
        self.append_css(format!("background:hsl({h},{s}%,{l}%);"))
    }
    fn bg_w(self)        -> Self { self.bg(255, 255, 255) }
    fn bg_muted(self)    -> Self { self.bg(239, 239, 239) }
    fn bg_accent(self)   -> Self { self.bg(102, 126, 234) }
    fn bg_surface(self)  -> Self { self.bg(250, 250, 250) }
    fn bg_black(self)    -> Self { self.bg(0, 0, 0) }
    fn bg_success(self)  -> Self { self.bg(34, 197, 94)  }
    fn bg_warning(self)  -> Self { self.bg(234, 179, 8)  }
    fn bg_danger(self)   -> Self { self.bg(239, 68, 68)  }

    // ── Size / position ───────────────────────────────────────────────────
    fn size(mut self, w: f32, h: f32) -> Self {
        self.node_mut().style.w = Some(w); self.node_mut().style.h = Some(h); self
    }
    fn w(mut self, v: f32) -> Self { self.node_mut().style.w = Some(v); self }
    fn h(mut self, v: f32) -> Self { self.node_mut().style.h = Some(v); self }
    fn pos(mut self, top: f32, left: f32) -> Self {
        self.node_mut().style.top = Some(top); self.node_mut().style.left = Some(left); self
    }
    fn max_w(self, v: f32) -> Self { self.append_css(format!("max-width:{v:.2}rem;")) }
    fn min_h(self, v: f32) -> Self { self.append_css(format!("min-height:{v:.2}rem;")) }

    // ── Shape ────────────────────────────────────────────────────────────
    fn radius(mut self, r: f32) -> Self { self.node_mut().style.radius = Some(r); self }
    fn circle(self, d: f32) -> Self { self.size(d, d).radius(d / 2.0) }
    fn pill(self)   -> Self { self.bg_accent().radius(9999.0).append_css("color:#fff;font-weight:600;") }
    fn ghost(self)  -> Self { self.bg_muted().radius(0.4).append_css("font-weight:600;") }
    fn outline(self)-> Self { self.append_css("border:1.5px solid #dbdbdb;background:transparent;border-radius:0.4rem;") }
    fn shadow(mut self, d: f32) -> Self { self.node_mut().style.shadow = Some(d); self }
    fn card(self)   -> Self { self.bg_w().radius(0.75).shadow(0.3) }

    // ── Spacing ───────────────────────────────────────────────────────────
    fn pad(mut self, p: f32) -> Self { self.node_mut().style.pad = Some(p); self }
    fn gap(mut self, g: f32) -> Self { self.node_mut().style.gap = Some(g); self }
    fn pad_x(self, v: f32)   -> Self { self.append_css(format!("padding-left:{v:.2}rem;padding-right:{v:.2}rem;")) }
    fn pad_y(self, v: f32)   -> Self { self.append_css(format!("padding-top:{v:.2}rem;padding-bottom:{v:.2}rem;")) }
    fn margin(self, v: f32)  -> Self { self.append_css(format!("margin:{v:.2}rem;")) }

    // ── Typography ────────────────────────────────────────────────────────
    fn size_rem(self, rem: f32)   -> Self { self.append_css(format!("font-size:{rem:.2}rem;")) }
    fn bold(self)                 -> Self { self.append_css("font-weight:700;") }
    fn semibold(self)             -> Self { self.append_css("font-weight:600;") }
    fn medium(self)               -> Self { self.append_css("font-weight:500;") }
    fn muted(self)                -> Self { self.append_css("color:#8e8e8e;") }
    fn white(self)                -> Self { self.append_css("color:#fff;") }
    fn center(self)               -> Self { self.append_css("text-align:center;") }
    fn line_clamp(self, n: u8)    -> Self {
        self.append_css(format!(
            "display:-webkit-box;-webkit-line-clamp:{n};-webkit-box-orient:vertical;overflow:hidden;"
        ))
    }
    fn truncate(self) -> Self {
        self.append_css("overflow:hidden;text-overflow:ellipsis;white-space:nowrap;")
    }
    fn letter_spacing(self, v: f32) -> Self {
        self.append_css(format!("letter-spacing:{v:.2}em;"))
    }
    fn uppercase(self) -> Self { self.append_css("text-transform:uppercase;") }

    // ── Layout helpers ────────────────────────────────────────────────────
    fn align_center(self)    -> Self { self.append_css("align-items:center;") }
    fn align_start(self)     -> Self { self.append_css("align-items:flex-start;") }
    fn align_end(self)       -> Self { self.append_css("align-items:flex-end;") }
    fn justify_center(self)  -> Self { self.append_css("justify-content:center;") }
    fn justify_between(self) -> Self { self.append_css("justify-content:space-between;") }
    fn justify_end(self)     -> Self { self.append_css("justify-content:flex-end;") }
    fn center_content(self)  -> Self {
        self.append_css("display:flex;align-items:center;justify-content:center;")
    }
    fn flex1(self)    -> Self { self.append_css("flex:1;min-height:0;") }
    fn flex_shrink0(self) -> Self { self.append_css("flex-shrink:0;") }
    fn full_w(self)   -> Self { self.append_css("width:100%;") }
    fn relative(self) -> Self { self.append_css("position:relative;") }
    fn inset(self)    -> Self { self.append_css("position:absolute;inset:0;") }
    fn square(self)   -> Self { self.append_css("aspect-ratio:1;") }
    fn tall(self)     -> Self { self.append_css("aspect-ratio:9/16;") }
    fn z(self, v: i32)-> Self { self.append_css(format!("z-index:{v};")) }
    fn wrap(self)     -> Self { self.append_css("flex-wrap:wrap;") }

    // ── Borders ───────────────────────────────────────────────────────────
    fn border_b(self)  -> Self { self.append_css("border-bottom:1px solid #efefef;") }
    fn border_t(self)  -> Self { self.append_css("border-top:1px solid #dbdbdb;") }
    fn border(self)    -> Self { self.append_css("border:1px solid #dbdbdb;") }
    fn ring(self)      -> Self {
        self.append_css("border:2px solid #fff;box-shadow:0 0 0 2px #dbdbfa;")
    }
    fn ring_accent(self) -> Self {
        self.append_css("border:2px solid #667eea;")
    }

    // ── Scroll / overflow ─────────────────────────────────────────────────
    fn scroll_x(self) -> Self {
        self.append_css("overflow-x:auto;overflow-y:hidden;scrollbar-width:none;")
    }
    fn scroll_y(self) -> Self { self.append_css("overflow-y:auto;") }
    fn clip(self)     -> Self { self.append_css("overflow:hidden;") }

    // ── Page / sticky ─────────────────────────────────────────────────────
    fn page_bg(self) -> Self {
        self.append_css(
            "min-height:100vh;background:#fafafa;\
             font-family:system-ui,sans-serif;color:#262626;"
        )
    }
    fn sticky_top(self) -> Self {
        self.append_css("position:sticky;top:0;z-index:10;background:#fff;\
                         border-bottom:1px solid #efefef;")
    }
    fn fixed_bottom(self) -> Self {
        self.append_css("position:fixed;bottom:0;left:0;right:0;z-index:10;\
                         background:#fff;border-top:1px solid #efefef;")
    }

    // ── Overlay helpers ───────────────────────────────────────────────────
    fn overlay_bottom(self) -> Self {
        self.append_css(
            "position:absolute;bottom:0;left:0;right:0;\
             background:linear-gradient(to top,rgba(0,0,0,0.7),transparent);\
             padding:0.5rem;"
        )
    }
    fn overlay_tag(self) -> Self {
        self.append_css(
            "position:absolute;top:0.5rem;left:0.5rem;\
             background:rgba(0,0,0,0.7);color:#fff;\
             padding:0.2rem 0.5rem;border-radius:0.25rem;"
        )
    }
    fn frosted(self) -> Self {
        self.append_css("backdrop-filter:blur(12px);background:rgba(255,255,255,0.75);")
    }

    // ── Raw CSS / Tailwind Classes ────────────────────────────────────────
    // Always targets SELF. Use .child_css() to style the last-added child.
    fn css(mut self, extra: impl Into<Str>) -> Self {
        let extra_str = extra.into();
        self.node_mut().class = extra_str;
        self.node_mut().style_mode = StyleMode::Class;
        self
    }
    /// Style the most recently added child (for chained child styling).
    fn child_css(mut self, extra: impl Into<Str>) -> Self {
        let extra_str = extra.into();
        if let Some(last) = self.node_mut().children.last_mut() {
            last.class = extra_str;
            last.style_mode = StyleMode::Class;
        }
        self
    }
    fn append_css(mut self, extra: impl AsRef<str>) -> Self {
        let cur = self.node_mut().style.extra.to_string();
        self.node_mut().style.extra = format!("{}{}", cur, extra.as_ref()).into();
        self
    }
    fn style_mode(mut self, mode: StyleMode) -> Self {
        self.node_mut().style_mode = mode; self
    }

    // ── Children ─────────────────────────────────────────────────────────
    fn child(mut self, c: impl IntoToken) -> Self {
        self.node_mut().children.push(c.into_node()); self
    }
    fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: IntoToken {
        self.node_mut().children.extend(iter.into_iter().map(|c| c.into_node())); self
    }
    fn child_if(self, cond: bool, c: impl IntoToken) -> Self {
        if cond { self.child(c) } else { self }
    }
    fn child_opt(self, c: Option<impl IntoToken>) -> Self {
        match c { Some(c) => self.child(c), None => self }
    }

    // ── Interaction ───────────────────────────────────────────────────────
    fn on_nav(mut self, page: impl Into<Str>) -> Self { self.node_mut().on_nav = Some(page.into()); self }
    fn on_click(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.node_mut().on_click = Some(Arc::new(f)); self
    }
    fn on_action(mut self, action: TokenAction) -> Self {
        self.node_mut().actions.push(action); self
    }
    fn on_event(mut self, binding: EventBinding) -> Self {
        self.node_mut().event_bindings.push(binding); self
    }
    fn on_hover(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::MouseEnter, action, debounce_ms: None, throttle_ms: None })
    }
    fn on_leave(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::MouseLeave, action, debounce_ms: None, throttle_ms: None })
    }
    fn on_focus(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::Focus, action, debounce_ms: None, throttle_ms: None })
    }
    fn on_blur(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::Blur, action, debounce_ms: None, throttle_ms: None })
    }
    fn on_change(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::Change, action, debounce_ms: None, throttle_ms: None })
    }
    fn on_intersect(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::IntersectEnter, action, debounce_ms: None, throttle_ms: None })
    }
    fn on_double_click(self, action: TokenAction) -> Self {
        self.on_event(EventBinding { event: EventType::DoubleClick, action, debounce_ms: None, throttle_ms: None })
    }

    // ── Dedicated click event helpers ─────────────────────────────────────
    fn on_click_show(self, id: impl Into<Str>) -> Self {
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::Show { show: id.into(), hide: vec![] },
            debounce_ms: None,
            throttle_ms: None,
        })
    }
    fn on_click_hide(self, id: impl Into<Str>) -> Self {
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::Hide(id.into()),
            debounce_ms: None,
            throttle_ms: None,
        })
    }
    fn on_click_toggle(self, id: impl Into<Str>) -> Self {
        let id = id.into();
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::ToggleClass { target: id.clone(), class: "hidden".into() },
            debounce_ms: None,
            throttle_ms: None,
        })
    }
    fn on_click_nav(self, page: impl Into<Str>) -> Self {
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::Navigate(page.into()),
            debounce_ms: None,
            throttle_ms: None,
        })
    }
    fn on_click_store_set(self, key: impl Into<Str>, val: impl Into<Str>) -> Self {
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::StoreSet { key: key.into(), value: val.into() },
            debounce_ms: None,
            throttle_ms: None,
        })
    }
    fn on_click_inc(self, key: impl Into<Str>) -> Self {
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::Increment { key: key.into(), by: 1 },
            debounce_ms: None,
            throttle_ms: None,
        })
    }
    fn on_click_dec(self, key: impl Into<Str>) -> Self {
        self.on_event(EventBinding {
            event: EventType::Click,
            action: TokenAction::Decrement { key: key.into(), by: 1 },
            debounce_ms: None,
            throttle_ms: None,
        })
    }

    // ── Declarative actions ───────────────────────────────────────────────
    // Always binds to SELF. Use .child_act() to attach to the last-added child.
    fn act(mut self, action: impl Into<TokenAction>) -> Self {
        self.node_mut().actions.push(action.into()); self
    }
    /// Attach an action to the most recently added child.
    fn child_act(mut self, action: impl Into<TokenAction>) -> Self {
        let action = action.into();
        if let Some(last) = self.node_mut().children.last_mut() {
            last.actions.push(action);
        }
        self
    }
    fn tap(self, action: TokenAction) -> Self {
        self.append_css("cursor:pointer;").on_action(action)
    }
    fn taps(self, actions: Vec<TokenAction>) -> Self {
        self.append_css("cursor:pointer;").on_action(TokenAction::Chain(actions))
    }
    fn log(self, msg: impl Into<Str>) -> Self {
        self.act(TokenAction::Log { level: LogLevel::Info, message: msg.into() })
    }
    fn open_modal(self, id: impl Into<Str>) -> Self {
        let id_str = id.into();
        self.act(TokenAction::Show { show: id_str, hide: vec![] }).log("🪟 modal opened")
    }
    fn close_modal(self) -> Self {
        self.act(TokenAction::HideAllModals).log("✕ modal closed")
    }
    fn trigger_upload(self, accept: impl Into<Str>) -> Self {
        self.act(TokenAction::TriggerFileInput { accept: Some(accept.into()), multiple: false })
    }
    fn show(self, id: impl Into<Str>) -> Self {
        self.act(TokenAction::Show { show: id.into(), hide: vec![] })
    }
    fn hide_el(self, id: impl Into<Str>) -> Self {
        self.act(TokenAction::Hide(id.into()))
    }
    fn toggle_class_act(self, target: impl Into<Str>, class: impl Into<Str>) -> Self {
        self.act(TokenAction::ToggleClass { target: target.into(), class: class.into() })
    }
    fn scroll_to(self, target: impl Into<Str>) -> Self {
        self.act(TokenAction::ScrollTo { target: target.into(), behavior: ScrollBehavior::Smooth })
    }
    fn copy(self, text: impl Into<Str>) -> Self {
        self.act(TokenAction::CopyToClipboard(text.into()))
    }
    fn open_url(self, url: impl Into<Str>) -> Self {
        self.act(TokenAction::OpenUrl { url: url.into(), new_tab: false })
    }
    fn navigate(self, page: impl Into<Str>) -> Self {
        self.act(TokenAction::Navigate(page.into()))
    }

    // ── Storage shortcuts ─────────────────────────────────────────────────
    fn store_set(self, key: impl Into<Str>, val: impl Into<Str>) -> Self {
        self.act(TokenAction::StoreSet {
            key: key.into(),
            value: val.into(),
        })
    }
    fn store_get(self, key: impl Into<Str>, target: DataTarget) -> Self {
        self.act(TokenAction::StoreGet {
            key: key.into(), target,
        })
    }
    fn store_delete(self, key: impl Into<Str>) -> Self {
        self.act(TokenAction::StoreDelete {
            key: key.into(),
        })
    }
    fn increment(self, key: impl Into<Str>) -> Self {
        self.act(TokenAction::Increment { key: key.into(), by: 1 })
    }
    fn decrement(self, key: impl Into<Str>) -> Self {
        self.act(TokenAction::Decrement { key: key.into(), by: 1 })
    }
    // Short aliases for common state mutations
    fn inc(self, key: impl Into<Str>) -> Self { self.increment(key) }
    fn dec(self, key: impl Into<Str>) -> Self { self.decrement(key) }
    fn toggle(self, key: impl Into<Str>) -> Self {
        self.act(TokenAction::ToggleState { key: key.into(), on_state: "true".into(), off_state: "false".into() })
    }
    fn toggle_state(self, key: impl Into<Str>, on: impl Into<Str>, off: impl Into<Str>) -> Self {
        self.act(TokenAction::ToggleState { key: key.into(), on_state: on.into(), off_state: off.into() })
    }
    fn tog(self, key: impl Into<Str>) -> Self { self.toggle(key) }
    fn cyc(self, key: impl Into<Str>, states: Vec<impl Into<Str>>) -> Self { self.cycle(key, states) }
    fn cycle(self, key: impl Into<Str>, states: Vec<impl Into<Str>>) -> Self {
        let states: Vec<Str> = states.into_iter().map(|s| s.into()).collect();
        self.act(TokenAction::Custom(format!("cycle:{}:{}", key.into(), states.join(",")).into()))
    }
    fn store_set_ttl(self, key: impl Into<Str>, val: impl Into<Str>, ttl_seconds: u64) -> Self {
        self.act(TokenAction::StoreSetTtl { key: key.into(), value: val.into(), ttl_seconds })
    }
    fn preload(self, key: impl Into<Str>, endpoint: impl Into<Str>) -> Self {
        self.act(TokenAction::Preload { key: key.into(), endpoint: endpoint.into() })
    }
    fn store_watch(self, key: impl Into<Str>) -> Self {
        self.act(TokenAction::Watch { key: key.into() })
    }

    // ── System control ────────────────────────────────────────────────────
    fn fullscreen(self) -> Self {
        self.act(TokenAction::RequestFullscreen)
    }
    fn exit_fullscreen(self) -> Self {
        self.act(TokenAction::ExitFullscreen)
    }
    fn pointer_lock(self) -> Self {
        self.act(TokenAction::RequestPointerLock)
    }
    fn vibrate(self, pattern: Vec<u32>) -> Self {
        self.act(TokenAction::Vibrate { pattern })
    }
    fn notify(self, title: impl Into<Str>, body: impl Into<Str>) -> Self {
        self.act(TokenAction::Notify { title: title.into(), body: body.into(), icon: None })
    }
    fn notify_with_icon(self, title: impl Into<Str>, body: impl Into<Str>, icon: impl Into<Str>) -> Self {
        self.act(TokenAction::Notify { title: title.into(), body: body.into(), icon: Some(icon.into()) })
    }
    fn share(self, title: impl Into<Str>, text: impl Into<Str>) -> Self {
        self.act(TokenAction::Share { title: title.into(), text: text.into(), url: None })
    }
    fn share_url(self, title: impl Into<Str>, text: impl Into<Str>, url: impl Into<Str>) -> Self {
        self.act(TokenAction::Share { title: title.into(), text: text.into(), url: Some(url.into()) })
    }

    // ── Drawer helpers ────────────────────────────────────────────────────
    fn toggle_drawer(self, id: impl Into<Str>) -> Self {
        self.act(TokenAction::Custom(format!("toggle_drawer:{}", id.into()).into()))
    }
    fn cycle_drawer(self, ids: Vec<impl Into<Str>>) -> Self {
        let ids: Vec<String> = ids.into_iter().map(|s| s.into().to_string()).collect();
        self.act(TokenAction::Custom(format!("cycle_drawer:{}", ids.join(",")).into()))
    }

    // ── File storage (dot-notation paths) ─────────────────────────────────
    fn file_store_set(self, key: impl Into<Str>, _val: impl Into<Str>) -> Self {
        self.act(TokenAction::Custom(format!("file_store_set:{}", key.into()).into()))
    }
    fn file_store_get(self, key: impl Into<Str>, _target: DataTarget) -> Self {
        self.act(TokenAction::Custom(format!("file_store_get:{}", key.into()).into()))
    }
    fn file_store_delete(self, key: impl Into<Str>) -> Self {
        self.act(TokenAction::Custom(format!("file_store_delete:{}", key.into()).into()))
    }

    // ── Variant / size / state ────────────────────────────────────────────
    fn variant(mut self, v: impl Into<Str>) -> Self {
        self.node_mut().variant = Some(v.into()); self
    }
    fn var(mut self, v: impl Into<Str>) -> Self {
        self.node_mut().variant = Some(v.into()); self
    }
    fn size_str(mut self, v: impl Into<Str>) -> Self {
        self.node_mut().size = Some(v.into()); self
    }
    fn sz(mut self, v: impl Into<Str>) -> Self {
        self.node_mut().size = Some(v.into()); self
    }
    fn loading(mut self, v: bool) -> Self {
        self.node_mut().loading = v; self
    }
    fn disabled(mut self, v: bool) -> Self {
        self.node_mut().disabled = v; self
    }

    // ── Data binding ──────────────────────────────────────────────────────
    // Always binds to SELF. Use .child_bind() for the last-added child.
    fn bind(mut self, signal: impl Into<Str>, token_id: impl Into<Str>) -> Self {
        self.node_mut().data_binding = Some(DataBinding::two_way(signal, token_id));
        self
    }
    fn child_bind(mut self, signal: impl Into<Str>, token_id: impl Into<Str>) -> Self {
        let binding = DataBinding::two_way(signal, token_id);
        if let Some(last) = self.node_mut().children.last_mut() {
            last.data_binding = Some(binding);
        }
        self
    }

    // ── Data reading / infinite scroll ──────────────────────────────────────
    // Always reads into SELF. Use .child_read() for the last-added child.
    fn read(mut self, endpoint: impl Into<Str>) -> Self {
        self.node_mut().data_binding = Some(DataBinding::one_way(endpoint.into()));
        self
    }
    fn child_read(mut self, endpoint: impl Into<Str>) -> Self {
        let ep = endpoint.into();
        if let Some(last) = self.node_mut().children.last_mut() {
            last.data_binding = Some(DataBinding::one_way(ep));
        }
        self
    }
    fn inf(self, endpoint: impl Into<Str>) -> Self {
        self.act(TokenAction::Custom(format!("inf:{}", endpoint.into()).into()))
    }

    // ── List rendering ────────────────────────────────────────────────────
    // Always lists on SELF. Use .child_list() for the last-added child.
    fn list(mut self, key: impl Into<Str>) -> Self {
        self.node_mut().data_binding = Some(DataBinding::one_way(format!("list:{}", key.into())));
        self
    }
    fn child_list(mut self, key: impl Into<Str>) -> Self {
        let ep = format!("list:{}", key.into());
        if let Some(last) = self.node_mut().children.last_mut() {
            last.data_binding = Some(DataBinding::one_way(ep));
        }
        self
    }

    // ── Stack layout ──────────────────────────────────────────────────────
    fn stack(self) -> Self {
        self.append_css("display:flex;flex-direction:column;")
    }

    // ── Chart rendering ────────────────────────────────────────────────────
    // Always charts on SELF. Use .child_chart() for the last-added child.
    fn chart(mut self, chart_type: impl Into<Str>) -> Self {
        self.node_mut().data_binding = Some(DataBinding::one_way(format!("chart:{}", chart_type.into())));
        self
    }
    fn child_chart(mut self, chart_type: impl Into<Str>) -> Self {
        let ep = format!("chart:{}", chart_type.into());
        if let Some(last) = self.node_mut().children.last_mut() {
            last.data_binding = Some(DataBinding::one_way(ep));
        }
        self
    }

    // ── Form helpers ──────────────────────────────────────────────────────
    fn form_id(self, id: impl Into<Str>) -> Self {
        self.append_css(format!("--tok-form-id:{};", id.into()))
    }
    fn on_submit(self, handler_name: impl Into<Str>) -> Self {
        self.act(TokenAction::Submit {
            form_id:    "".into(),
            on_submit:  handler_name.into(),
            on_invalid: None,
        })
    }

    // ── Animation — direct shortcuts ──────────────────────────────────────
    fn anim(self, builder: AnimationBuilder) -> Self {
        let css = builder.to_css();
        let initial = builder.initial_class();
        let mut s = self.append_css(css);
        if let Some(cls) = initial {
            // Add initial class by appending to extra as a data attr shim.
            s = s.append_css(format!("--tok-initial-class:{cls};"));
        }
        s
    }
    // Preset shortcut methods — common patterns without building AnimationBuilder
    fn fade_in(self, ms: u32) -> Self {
        self.append_css(format!(
            "animation:tok-fade-in {ms}ms cubic-bezier(0,0,0.58,1) both;"
        ))
    }
    fn fade_in_delay(self, ms: u32, delay_ms: u32) -> Self {
        self.append_css(format!(
            "animation:tok-fade-in {ms}ms cubic-bezier(0,0,0.58,1) {delay_ms}ms both;"
        ))
    }
    fn slide_up(self, ms: u32) -> Self {
        self.append_css(format!("animation:tok-slide-up {ms}ms cubic-bezier(0,0,0.58,1) both;"))
    }
    fn slide_down(self, ms: u32) -> Self {
        self.append_css(format!("animation:tok-slide-down {ms}ms cubic-bezier(0,0,0.58,1) both;"))
    }
    fn scale_in(self, ms: u32) -> Self {
        self.append_css(format!("animation:tok-scale-in {ms}ms cubic-bezier(0.34,1.56,0.64,1) both;"))
    }
    fn anim_pulse(self) -> Self {
        self.append_css("animation:tok-pulse 1.5s ease-in-out infinite;")
    }
    fn anim_spin(self) -> Self {
        self.append_css("animation:tok-spin 1s linear infinite;")
    }
    fn anim_ping(self) -> Self {
        self.append_css("animation:tok-ping 1s ease-in-out infinite;")
    }
    fn anim_bounce(self) -> Self {
        self.append_css("animation:tok-bounce 0.8s ease-in-out infinite;")
    }
    fn anim_heartbeat(self) -> Self {
        self.append_css("animation:tok-heartbeat 1.3s ease-in-out infinite;")
    }
    fn anim_shake(self) -> Self {
        self.append_css("animation:tok-shake 0.5s ease both;")
    }

    // Transition shortcuts
    fn transition_all(self, ms: u32) -> Self {
        self.append_css(format!("transition:all {ms}ms cubic-bezier(0,0,0.58,1);"))
    }
    fn transition_transform(self, ms: u32) -> Self {
        self.append_css(format!("transition:transform {ms}ms cubic-bezier(0,0,0.58,1);"))
    }
    fn transition_opacity(self, ms: u32) -> Self {
        self.append_css(format!("transition:opacity {ms}ms cubic-bezier(0,0,0.58,1);"))
    }
    fn transition_colors(self, ms: u32) -> Self {
        self.append_css(format!(
            "transition:background-color {ms}ms ease, color {ms}ms ease, border-color {ms}ms ease;"
        ))
    }

    // Hover/press micro-interactions (JS-assisted via animation_js())
    fn hover_scale(self, factor: f32) -> Self {
        self.append_css(format!(
            "--tok-hover-scale:{factor};transition:transform 200ms cubic-bezier(0,0,0.58,1);"
        ))
    }
    fn hover_lift(self, rem: f32) -> Self {
        self.append_css(format!(
            "--tok-hover-lift:{rem:.2}rem;transition:transform 200ms cubic-bezier(0,0,0.58,1);"
        ))
    }
    fn hover_dim(self, opacity: f32) -> Self {
        self.append_css(format!(
            "--tok-hover-opacity:{opacity:.2};transition:opacity 150ms ease;"
        ))
    }
    fn press(self, scale: f32) -> Self {
        self.append_css(format!(
            "--tok-press-scale:{scale};transition:transform 100ms ease;"
        ))
    }
    fn press_default(self) -> Self { self.press(0.95) }

    /// Animate element into view when scrolled to (uses IntersectionObserver).
    fn on_scroll_enter(mut self) -> Self {
        self.node_mut().class = "tok-hidden".into();
        self
    }
    fn on_scroll_enter_scale(mut self) -> Self {
        self.node_mut().class = "tok-hidden-scale".into();
        self
    }

    // ── Navigation ─────────────────────────────────────────────────────────
    fn open_url_new_tab(self, url: impl Into<Str>) -> Self {
        let url_str = url.into();
        self.act(TokenAction::OpenUrl { url: url_str, new_tab: true })
    }

    fn submit_form(self, form_id: impl Into<Str>) -> Self {
        let id = form_id.into();
        self.act(TokenAction::Submit { form_id: id, on_submit: "handle_form_submit".into(), on_invalid: None })
    }

    // ── Fluent English-like API for non-programmers ─────────────────────────

    /// Bind an action to an event using English-like syntax.
    /// Events: "click", "hover", "leave", "focus", "blur", "change", "doubleclick"
    fn on(self, event: &str, action: TokenAction) -> Self {
        let event_type = match event.to_lowercase().as_str() {
            "click" => EventType::Click,
            "hover" | "mouseenter" => EventType::MouseEnter,
            "leave" | "mouseleave" => EventType::MouseLeave,
            "focus" => EventType::Focus,
            "blur" => EventType::Blur,
            "change" => EventType::Change,
            "doubleclick" | "double" => EventType::DoubleClick,
            "scroll" => EventType::Scroll,
            "resize" => EventType::Resize,
            "intersect" | "enter" => EventType::IntersectEnter,
            _ => EventType::Click,
        };
        self.on_event(EventBinding { event: event_type, action, debounce_ms: None, throttle_ms: None })
    }

    // then() is intentionally NOT on TokenBuilder. Use ActionChain on TokenAction values:
    //   store("key", "val").then(hide("modal")).then(log("Done!"))
    // See factory.rs for the ActionChain impl.

    // ── Convenience methods (closure-free) ─────────────────────────────────

    /// Store a value with automatic backend inference (localStorage on web, memory otherwise)
    fn store(self, key: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.store_set(key, value)
    }

    /// Hide an element by id
    fn hide(self, id: impl Into<Str>) -> Self {
        self.hide_el(id)
    }

    /// Show an element by id (optionally hiding others)
    fn show_only(self, id: impl Into<Str>) -> Self {
        self.show(id)
    }

    /// Log a message to console
    fn console(self, msg: impl Into<Str>) -> Self {
        self.log(msg)
    }

    // ── Attributes ───────────────────────────────────────────────────────
    fn attr(mut self, key: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.node_mut().attributes.insert(key.into(), value.into());
        self
    }

    // ── Theme ─────────────────────────────────────────────────────────────
    fn theme_var(self, name: &str, value: &str) -> Self {
        self.append_css(format!("--{name}:{value};"))
    }
    fn use_var(self, property: &str, var_name: &str) -> Self {
        self.append_css(format!("{property}:var(--{var_name});"))
    }

    // ── Typography shortcuts ──────────────────────────────────────────────
    fn h1(self) -> Self { self.append_css("font-size:2rem;font-weight:700;") }
    fn h2(self) -> Self { self.append_css("font-size:1.5rem;font-weight:600;") }
    fn h3(self) -> Self { self.append_css("font-size:1.25rem;font-weight:600;") }
    fn body_text(self) -> Self { self.append_css("font-size:1rem;") }
    fn caption(self) -> Self { self.append_css("font-size:0.75rem;color:#6b7280;") }
    fn label(self) -> Self {
        self.append_css("font-size:0.75rem;font-weight:600;text-transform:uppercase;letter-spacing:0.05em;color:#6b7280;")
    }
    fn mono(self) -> Self { self.append_css("font-family:monospace;") }
    fn italic(self) -> Self { self.append_css("font-style:italic;") }
    fn strike(self) -> Self { self.append_css("text-decoration:line-through;") }
    fn underline(self) -> Self { self.append_css("text-decoration:underline;") }
    fn color(self, hex: &str) -> Self { self.append_css(format!("color:{hex};")) }
    fn bg_hex(self, hex: &str) -> Self { self.append_css(format!("background:{hex};")) }
    fn opacity(self, v: f32) -> Self { self.append_css(format!("opacity:{v};")) }
    fn blur(self, px: u32) -> Self { self.append_css(format!("filter:blur({px}px);")) }
    fn cursor(self, c: &str) -> Self { self.append_css(format!("cursor:{c};")) }
    fn pointer(self) -> Self { self.append_css("cursor:pointer;") }
    fn select_none(self) -> Self { self.append_css("user-select:none;") }
    fn overflow(self, v: &str) -> Self { self.append_css(format!("overflow:{v};")) }
    fn display(self, v: &str) -> Self { self.append_css(format!("display:{v};")) }
    fn font(self, family: &str) -> Self { self.append_css(format!("font-family:{family};")) }
    fn line_height(self, v: f32) -> Self { self.append_css(format!("line-height:{v};")) }
    fn word_break(self, v: &str) -> Self { self.append_css(format!("word-break:{v};")) }
    fn max_h(self, v: f32) -> Self { self.append_css(format!("max-height:{v:.2}rem;")) }
    fn min_w(self, v: f32) -> Self { self.append_css(format!("min-width:{v:.2}rem;")) }
    fn aspect_ratio(self, w: u16, h: u16) -> Self { self.append_css(format!("aspect-ratio:{w}/{h};")) }
    fn col_span(self, n: u8) -> Self { self.append_css(format!("grid-column:span {n};")) }
    fn row_span(self, n: u8) -> Self { self.append_css(format!("grid-row:span {n};")) }
    fn will_change(self, v: &str) -> Self { self.append_css(format!("will-change:{v};")) }
    fn css_var(self, name: &str, value: &str) -> Self { self.append_css(format!("--{name}:{value};")) }
    fn border_color(self, hex: &str) -> Self { self.append_css(format!("border-color:{hex};")) }
    fn border_width(self, px: u32) -> Self { self.append_css(format!("border-width:{px}px;")) }
    fn border_style(self, s: &str) -> Self { self.append_css(format!("border-style:{s};")) }
    fn outline_none(self) -> Self { self.append_css("outline:none;") }
    fn pointer_events_none(self) -> Self { self.append_css("pointer-events:none;") }
    fn bg_gradient(self, stops: &str) -> Self { self.append_css(format!("background:linear-gradient({stops});")) }
    fn transform(self, t: &str) -> Self { self.append_css(format!("transform:{t};")) }

    // ── Conditional visibility ────────────────────────────────────────────
    fn show_when(self, id: impl Into<Str>) -> Self {
        self.act(TokenAction::Show { show: id.into(), hide: vec![] })
    }
    fn hide_when(self, id: impl Into<Str>) -> Self {
        self.act(TokenAction::Hide(id.into()))
    }

    // ── Content helpers ───────────────────────────────────────────────────
    fn text_content(mut self, text: impl Into<Str>) -> Self {
        self.node_mut().content = Some(text.into()); self
    }
    fn src(mut self, url: impl Into<Str>) -> Self {
        self.node_mut().attributes.insert("src".into(), url.into()); self
    }
    fn href(mut self, url: impl Into<Str>) -> Self {
        self.node_mut().attributes.insert("href".into(), url.into()); self
    }
    fn target_blank(mut self) -> Self {
        self.node_mut().attributes.insert("target".into(), "_blank".into()); self
    }

    // ── Accessibility ─────────────────────────────────────────────────────
    fn aria_label(mut self, label: impl Into<Str>) -> Self {
        self.node_mut().attributes.insert("aria-label".into(), label.into()); self
    }
    fn role(mut self, role: impl Into<Str>) -> Self {
        self.node_mut().attributes.insert("role".into(), role.into()); self
    }
    fn tabindex(mut self, idx: i32) -> Self {
        self.node_mut().attributes.insert("tabindex".into(), idx.to_string().into()); self
    }

    // ── Data attributes ───────────────────────────────────────────────────
    fn data(mut self, key: impl Into<Str>, value: impl Into<Str>) -> Self {
        let k = format!("data-{}", key.into());
        self.node_mut().attributes.insert(k.into(), value.into()); self
    }

    // ── Conditional classes ───────────────────────────────────────────────
    fn classes_when(mut self, class: impl Into<Str>, _condition: bool) -> Self {
        if _condition {
            let cls = class.into().to_string();
            let cur = self.node_mut().class.to_string();
            self.node_mut().class = format!("{} {}", cur, cls).into();
        }
        self
    }

    // ── Number animation ──────────────────────────────────────────────────
    fn animate_number(self, _duration_ms: u32) -> Self {
        self.append_css("transition:all 300ms cubic-bezier(0,0,0.58,1);")
    }

    // ── Loading / image helpers ───────────────────────────────────────────
    fn lazy_load(mut self) -> Self {
        self.node_mut().attributes.insert("loading".into(), "lazy".into()); self
    }
    fn placeholder_blur(self) -> Self {
        self.append_css("filter:blur(10px);transition:filter 300ms ease;")
    }

    // ── Shape / image shortcuts ───────────────────────────────────────────
    fn aspect_square(self) -> Self {
        self.append_css("aspect-ratio:1;")
    }
    fn rounded_full(self) -> Self {
        self.append_css("border-radius:9999px;")
    }
    fn object_cover(self) -> Self {
        self.append_css("object-fit:cover;")
    }

    // ── Responsive / breakpoint helpers ───────────────────────────────────
    // These append Tailwind-style responsive classes (e.g. sm:bg-blue-200)
    // instead of emitting invalid inline @media queries.
    fn sm(mut self, css: &str) -> Self {
        let cls = format!("sm:{}", css.split(';').next().unwrap_or(css));
        let cur = self.node_mut().class.to_string();
        self.node_mut().class = format!("{} {}", cur, cls).into();
        self
    }
    fn md_bp(mut self, css: &str) -> Self {
        let cls = format!("md:{}", css.split(';').next().unwrap_or(css));
        let cur = self.node_mut().class.to_string();
        self.node_mut().class = format!("{} {}", cur, cls).into();
        self
    }
    fn lg_bp(mut self, css: &str) -> Self {
        let cls = format!("lg:{}", css.split(';').next().unwrap_or(css));
        let cur = self.node_mut().class.to_string();
        self.node_mut().class = format!("{} {}", cur, cls).into();
        self
    }
    fn xl_bp(mut self, css: &str) -> Self {
        let cls = format!("xl:{}", css.split(';').next().unwrap_or(css));
        let cur = self.node_mut().class.to_string();
        self.node_mut().class = format!("{} {}", cur, cls).into();
        self
    }
    fn dark(mut self, css: &str) -> Self {
        let cls = format!("dark:{}", css.split(';').next().unwrap_or(css));
        let cur = self.node_mut().class.to_string();
        self.node_mut().class = format!("{} {}", cur, cls).into();
        self
    }
    fn print_only(mut self, css: &str) -> Self {
        let cls = format!("print:{}", css.split(';').next().unwrap_or(css));
        let cur = self.node_mut().class.to_string();
        self.node_mut().class = format!("{} {}", cur, cls).into();
        self
    }

    // ── Rendering ────────────────────────────────────────────────────────────
    fn render(self) -> impl IntoView {
        render_dom(self.into_node())
    }

}

