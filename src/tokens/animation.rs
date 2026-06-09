// src/tokens/animation.rs
//
// Declarative animation system.
//
// Three levels:
//
//   AnimSpec         — a single serializable animation descriptor.
//                      Maps to one CSS `animation:` declaration.
//
//   TransitionSpec   — serializable CSS transition descriptor.
//
//   AnimationBuilder — fluent API that compiles to a CSS string fragment
//                      appended to `StyleToken::extra`.
//
// Keyframe library
// ────────────────
//   All keyframes are emitted into a single <style> tag injected by
//   `AnimStyleInjector` (a Leptos component).  The names are stable so
//   multiple nodes using the same keyframe share the rule.
//
// Scroll-trigger
// ──────────────
//   `AnimationBuilder::on_scroll_enter` attaches an IntersectionObserver
//   via a small inline JS snippet.  No external library required.
//   On SSR the snippet is inert; the element starts in its initial state
//   and the observer fires on first client-side paint.
//
// Stagger
// ───────
//   `stagger_children(delay_step_ms)` returns a CSS string that adds
//   `animation-delay` to each direct child via nth-child selectors.
//   Call it on a container and all children animate in sequence.

use std::borrow::Cow;
use serde::{Deserialize, Serialize};

// ── Easing constants ──────────────────────────────────────────────────────────

pub const EASE:         &str = "cubic-bezier(0.25,0.1,0.25,1)";
pub const EASE_IN:      &str = "cubic-bezier(0.42,0,1,1)";
pub const EASE_OUT:     &str = "cubic-bezier(0,0,0.58,1)";
pub const EASE_IN_OUT:  &str = "cubic-bezier(0.42,0,0.58,1)";
pub const SPRING:       &str = "cubic-bezier(0.34,1.56,0.64,1)";
pub const SHARP:        &str = "cubic-bezier(0.4,0,0.6,1)";
pub const LINEAR:       &str = "linear";

// ── Named keyframes ───────────────────────────────────────────────────────────

/// Known keyframe names emitted by `keyframe_css()`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Keyframe {
    FadeIn,
    FadeOut,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
    ScaleIn,
    ScaleOut,
    Pulse,
    Bounce,
    Shake,
    Spin,
    Ping,
    HeartBeat,
    /// Custom name — caller is responsible for injecting the `@keyframes` rule.
    Custom(Cow<'static, str>),
}

impl Keyframe {
    pub fn name(&self) -> &str {
        match self {
            Keyframe::FadeIn    => "tok-fade-in",
            Keyframe::FadeOut   => "tok-fade-out",
            Keyframe::SlideUp   => "tok-slide-up",
            Keyframe::SlideDown => "tok-slide-down",
            Keyframe::SlideLeft => "tok-slide-left",
            Keyframe::SlideRight=> "tok-slide-right",
            Keyframe::ScaleIn   => "tok-scale-in",
            Keyframe::ScaleOut  => "tok-scale-out",
            Keyframe::Pulse     => "tok-pulse",
            Keyframe::Bounce    => "tok-bounce",
            Keyframe::Shake     => "tok-shake",
            Keyframe::Spin      => "tok-spin",
            Keyframe::Ping      => "tok-ping",
            Keyframe::HeartBeat => "tok-heartbeat",
            Keyframe::Custom(n) => n.as_ref(),
        }
    }
}

/// Returns the full `@keyframes` CSS block for all built-in animations.
/// Inject once via `AnimStyleInjector`.
pub fn keyframe_css() -> &'static str {
    r#"
@keyframes tok-fade-in    { from { opacity:0 }                  to { opacity:1 } }
@keyframes tok-fade-out   { from { opacity:1 }                  to { opacity:0 } }
@keyframes tok-slide-up   { from { opacity:0;transform:translateY(1rem) }  to { opacity:1;transform:translateY(0) } }
@keyframes tok-slide-down { from { opacity:0;transform:translateY(-1rem) } to { opacity:1;transform:translateY(0) } }
@keyframes tok-slide-left { from { opacity:0;transform:translateX(1rem) }  to { opacity:1;transform:translateX(0) } }
@keyframes tok-slide-right{ from { opacity:0;transform:translateX(-1rem) } to { opacity:1;transform:translateX(0) } }
@keyframes tok-scale-in   { from { opacity:0;transform:scale(0.9) } to { opacity:1;transform:scale(1) } }
@keyframes tok-scale-out  { from { opacity:1;transform:scale(1) }   to { opacity:0;transform:scale(0.9) } }
@keyframes tok-pulse      { 0%,100%{transform:scale(1);opacity:1} 50%{transform:scale(1.05);opacity:0.8} }
@keyframes tok-bounce     { 0%,100%{transform:translateY(0)}       40%{transform:translateY(-0.5rem)} 60%{transform:translateY(-0.25rem)} }
@keyframes tok-shake      { 0%,100%{transform:translateX(0)} 20%,60%{transform:translateX(-0.3rem)} 40%,80%{transform:translateX(0.3rem)} }
@keyframes tok-spin       { from{transform:rotate(0deg)} to{transform:rotate(360deg)} }
@keyframes tok-ping       { 0%{transform:scale(1);opacity:1} 75%,100%{transform:scale(1.5);opacity:0} }
@keyframes tok-heartbeat  { 0%,100%{transform:scale(1)} 14%{transform:scale(1.1)} 28%{transform:scale(1)} 42%{transform:scale(1.1)} 70%{transform:scale(1)} }

/* Scroll-trigger entrance animations (pure CSS, no JS) */
@keyframes tok-fade-in-up { from { opacity:0; transform:translateY(1rem); } to { opacity:1; transform:translateY(0); } }
@keyframes tok-fade-in-scale { from { opacity:0; transform:scale(0.95); } to { opacity:1; transform:scale(1); } }

/* Scroll-trigger initial states (hidden) */
.tok-hidden { opacity:0; transform:translateY(1rem); }
.tok-hidden-scale { opacity:0; transform:scale(0.95); }

/* Scroll-trigger visible states (animated in) */
.tok-visible { animation:tok-fade-in-up 0.6s cubic-bezier(0,0,0.58,1) forwards; }
.tok-visible-scale { animation:tok-fade-in-scale 0.6s cubic-bezier(0,0,0.58,1) forwards; }
"#
}

// ── AnimSpec ──────────────────────────────────────────────────────────────────

/// Serializable descriptor for a single `animation:` declaration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnimSpec {
    pub keyframe:   Keyframe,
    pub duration_ms: u32,
    pub delay_ms:   u32,
    pub easing:     Cow<'static, str>,
    pub iterations: AnimIterations,
    pub fill_mode:  FillMode,
    pub direction:  AnimDirection,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AnimIterations { Count(f32), Infinite }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FillMode    { None, Forwards, Backwards, Both }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AnimDirection { Normal, Reverse, Alternate, AlternateReverse }

impl AnimSpec {
    pub fn new(keyframe: Keyframe) -> Self {
        Self {
            keyframe,
            duration_ms: 300,
            delay_ms:    0,
            easing:      EASE_OUT.into(),
            iterations:  AnimIterations::Count(1.0),
            fill_mode:   FillMode::Both,
            direction:   AnimDirection::Normal,
        }
    }

    /// Compile to a CSS `animation:` value string (without the property name).
    pub fn to_css_value(&self) -> String {
        let iter = match &self.iterations {
            AnimIterations::Count(n) => format!("{n}"),
            AnimIterations::Infinite => "infinite".into(),
        };
        let fill = match self.fill_mode {
            FillMode::None      => "none",
            FillMode::Forwards  => "forwards",
            FillMode::Backwards => "backwards",
            FillMode::Both      => "both",
        };
        let dir = match self.direction {
            AnimDirection::Normal           => "normal",
            AnimDirection::Reverse          => "reverse",
            AnimDirection::Alternate        => "alternate",
            AnimDirection::AlternateReverse => "alternate-reverse",
        };
        format!(
            "{name} {dur}ms {ease} {delay}ms {iter} {fill} {dir}",
            name  = self.keyframe.name(),
            dur   = self.duration_ms,
            ease  = self.easing,
            delay = self.delay_ms,
            iter  = iter,
            fill  = fill,
            dir   = dir,
        )
    }

    pub fn duration(mut self, ms: u32) -> Self { self.duration_ms = ms; self }
    pub fn delay(mut self, ms: u32) -> Self    { self.delay_ms = ms; self }
    pub fn ease(mut self, e: &'static str) -> Self { self.easing = e.into(); self }
    pub fn infinite(mut self) -> Self { self.iterations = AnimIterations::Infinite; self }
    pub fn loop_n(mut self, n: f32) -> Self { self.iterations = AnimIterations::Count(n); self }
    pub fn reverse(mut self) -> Self { self.direction = AnimDirection::Reverse; self }
    pub fn alternate(mut self) -> Self { self.direction = AnimDirection::Alternate; self }
}

// ── TransitionSpec ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransitionSpec {
    pub property:    Cow<'static, str>,
    pub duration_ms: u32,
    pub easing:      Cow<'static, str>,
    pub delay_ms:    u32,
}

impl TransitionSpec {
    pub fn new(property: &'static str) -> Self {
        Self { property: property.into(), duration_ms: 200, easing: EASE_OUT.into(), delay_ms: 0 }
    }
    pub fn duration(mut self, ms: u32) -> Self { self.duration_ms = ms; self }
    pub fn delay(mut self, ms: u32) -> Self    { self.delay_ms = ms; self }
    pub fn ease(mut self, e: &'static str) -> Self { self.easing = e.into(); self }

    pub fn to_css_value(&self) -> String {
        format!("{} {}ms {} {}ms", self.property, self.duration_ms, self.easing, self.delay_ms)
    }
}

// ── AnimationBuilder ──────────────────────────────────────────────────────────
//
// Fluent builder that accumulates animation / transition / scroll-trigger
// CSS and compiles to a string that can be appended to `StyleToken::extra`.

#[derive(Clone, Debug, Default)]
pub struct AnimationBuilder {
    animations:  Vec<AnimSpec>,
    transitions: Vec<TransitionSpec>,
    scroll_trigger: Option<ScrollTrigger>,
    hover_scale:    Option<f32>,
    hover_opacity:  Option<f32>,
    hover_lift:     Option<f32>,
    press_scale:    Option<f32>,
}

#[derive(Clone, Debug)]
pub struct ScrollTrigger {
    pub enter_class: &'static str,
    pub initial_class: &'static str,
    pub threshold: f32,
}

impl AnimationBuilder {
    pub fn new() -> Self { Self::default() }

    // ── Animations ───────────────────────────────────────────────────────

    pub fn play(mut self, spec: AnimSpec) -> Self {
        self.animations.push(spec); self
    }

    // Preset shortcuts
    pub fn fade_in(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::FadeIn).duration(ms))
    }
    pub fn fade_out(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::FadeOut).duration(ms))
    }
    pub fn slide_up(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::SlideUp).duration(ms))
    }
    pub fn slide_down(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::SlideDown).duration(ms))
    }
    pub fn slide_left(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::SlideLeft).duration(ms))
    }
    pub fn slide_right(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::SlideRight).duration(ms))
    }
    pub fn scale_in(self, ms: u32) -> Self {
        self.play(AnimSpec::new(Keyframe::ScaleIn).duration(ms))
    }
    pub fn pulse(self) -> Self {
        self.play(AnimSpec::new(Keyframe::Pulse).duration(1500).infinite())
    }
    pub fn bounce(self) -> Self {
        self.play(AnimSpec::new(Keyframe::Bounce).duration(800).infinite())
    }
    pub fn spin(self) -> Self {
        self.play(AnimSpec::new(Keyframe::Spin).duration(1000).infinite().ease(LINEAR))
    }
    pub fn ping(self) -> Self {
        self.play(AnimSpec::new(Keyframe::Ping).duration(1000).infinite())
    }
    pub fn shake(self) -> Self {
        self.play(AnimSpec::new(Keyframe::Shake).duration(500))
    }
    pub fn heartbeat(self) -> Self {
        self.play(AnimSpec::new(Keyframe::HeartBeat).duration(1300).infinite())
    }

    // ── Transitions ──────────────────────────────────────────────────────

    pub fn transition(mut self, spec: TransitionSpec) -> Self {
        self.transitions.push(spec); self
    }
    pub fn transition_all(self, ms: u32) -> Self {
        self.transition(TransitionSpec::new("all").duration(ms))
    }
    pub fn transition_transform(self, ms: u32) -> Self {
        self.transition(TransitionSpec::new("transform").duration(ms))
    }
    pub fn transition_opacity(self, ms: u32) -> Self {
        self.transition(TransitionSpec::new("opacity").duration(ms))
    }
    pub fn transition_colors(self, ms: u32) -> Self {
        self.transition(TransitionSpec::new("background-color, color, border-color").duration(ms))
    }

    // ── Hover / press micro-interactions (pure CSS, no JS) ─────────────────

    pub fn hover_scale(mut self, factor: f32) -> Self { self.hover_scale = Some(factor); self }
    pub fn hover_lift(mut self, rem: f32) -> Self     { self.hover_lift = Some(rem); self }
    pub fn hover_dim(mut self, opacity: f32) -> Self  { self.hover_opacity = Some(opacity); self }
    pub fn press(mut self, scale: f32) -> Self        { self.press_scale = Some(scale); self }

    // ── Scroll trigger ───────────────────────────────────────────────────

    /// Element fades/slides in when it enters the viewport.
    pub fn on_scroll_enter(mut self) -> Self {
        self.scroll_trigger = Some(ScrollTrigger {
            initial_class: "tok-hidden",
            enter_class:   "tok-visible",
            threshold:     0.15,
        });
        self
    }

    pub fn on_scroll_enter_scale(mut self) -> Self {
        self.scroll_trigger = Some(ScrollTrigger {
            initial_class: "tok-hidden-scale",
            enter_class:   "tok-visible-scale",
            threshold:     0.15,
        });
        self
    }

    // ── Compile ──────────────────────────────────────────────────────────

    /// Compile to a CSS fragment string (appended to `StyleToken::extra`).
    pub fn to_css(&self) -> String {
        let mut css = String::with_capacity(256);

        // animation:
        if !self.animations.is_empty() {
            let vals: Vec<String> = self.animations.iter().map(|a| a.to_css_value()).collect();
            css.push_str(&format!("animation:{};", vals.join(",")));
        }

        // transition:
        if !self.transitions.is_empty() {
            let vals: Vec<String> = self.transitions.iter().map(|t| t.to_css_value()).collect();
            css.push_str(&format!("transition:{};", vals.join(",")));
        }

        // hover / press via pure CSS :hover and :active pseudo-classes (no JS)
        let has_hover = self.hover_scale.is_some() || self.hover_opacity.is_some() || self.hover_lift.is_some();
        let has_press = self.press_scale.is_some();
        
        if has_hover || has_press {
            css.push_str(&format!("transition:transform 200ms {EASE_OUT},opacity 200ms {EASE_OUT};cursor:pointer;"));
        }
        
        // Build :hover transform
        if has_hover {
            let mut hover_transforms = Vec::new();
            if let Some(s) = self.hover_scale {
                hover_transforms.push(format!("scale({s})"));
            }
            if let Some(r) = self.hover_lift {
                hover_transforms.push(format!("translateY(-{r}rem)"));
            }
            if !hover_transforms.is_empty() {
                css.push_str(&format!(":hover{{transform:{};}}", hover_transforms.join(" ")));
            }
            if let Some(o) = self.hover_opacity {
                css.push_str(&format!(":hover{{opacity:{o};}}"));
            }
        }
        
        // Build :active (press) transform
        if let Some(s) = self.press_scale {
            css.push_str(&format!(":active{{transform:scale({s});}}"));
        }

        // scroll-trigger: output CSS that works with IntersectionObserver
        // The element starts hidden and transitions to visible when class is toggled
        if let Some(ref _st) = self.scroll_trigger {
            css.push_str("transition:opacity 0.3s ease,transform 0.3s ease;");
        }

        css
    }

    /// Class string that must be added to the element for scroll-trigger to work.
    pub fn initial_class(&self) -> Option<&'static str> {
        self.scroll_trigger.as_ref().map(|st| st.initial_class)
    }
}

// ── Stagger helper ────────────────────────────────────────────────────────────

/// Returns a CSS string that staggers `animation-delay` across up to `max_children`
/// direct children using nth-child selectors.
/// Append to the *container's* `extra` field.
pub fn stagger_children(delay_step_ms: u32, max_children: usize) -> String {
    let mut css = String::new();
    for i in 1..=max_children {
        css.push_str(&format!(
            "> :nth-child({i}) {{ animation-delay: {}ms; }}",
            (i as u32 - 1) * delay_step_ms
        ));
    }
    css
}

/// Stagger with a default of 12 children at 60 ms steps.
pub fn stagger(delay_step_ms: u32) -> String {
    stagger_children(delay_step_ms, 12)
}

// ── Inline JS helper script (DEPRECATED - now using pure CSS) ─────────────────
//
// Kept for backward compatibility but returns empty string.
// All animation effects now use pure CSS :hover/:active or Leptos native events.

pub fn animation_js() -> &'static str {
    "" // No JS needed - pure CSS + Leptos native events
}

// ── Leptos injector component ─────────────────────────────────────────────────

/// Inject all keyframe CSS once per page (no JS needed).
/// Place in the root layout, before other token components.
#[cfg(feature = "ssr")]
pub fn anim_style_injector_html() -> String {
    format!(
        "<style id=\"tok-keyframes\">{}</style>",
        keyframe_css()
    )
}