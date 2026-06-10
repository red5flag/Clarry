// src/tokens/builders/page_config.rs
//
// Global page configuration: Palette, Media, and Store presets.
// Use `page_config()` to set up global styles, responsive breakpoints,
// and default storage values before your main layout.
//
// Example:
//   page_config()
//       .palette(Palette::dark())
//       .media(Media::mobile_first())
//       .store_preset("user", json!({"name":"Guest"}))
//       .page("demo")
//           col()
//               text("Hello")

use crate::tokens::node::{IntoToken, Str, TokenNode};
use crate::tokens::core::id::next_id;
use super::spec::TokenBuilder;
use super::types::Container;

// ── PageConfig builder ─────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct PageConfig {
    pub root: Container,
    pub palette: Palette,
    pub media: Media,
    pub store_presets: Vec<(Str, Str)>,
}

impl PageConfig {
    pub fn new() -> Self {
        Self {
            root: Container { stack: vec![TokenNode::new(next_id())] },
            palette: Palette::default(),
            media: Media::default(),
            store_presets: Vec::new(),
        }
    }

    /// Apply a palette to the page.
    pub fn palette(mut self, p: Palette) -> Self {
        self.palette = p;
        self
    }

    /// Apply media/responsive configuration.
    pub fn media(mut self, m: Media) -> Self {
        self.media = m;
        self
    }

    /// Add a store preset. `key` is the storage key, `json` is the default value.
    pub fn store_preset(mut self, key: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.store_presets.push((key.into(), value.into()));
        self
    }

    /// Set the page id on the root container.
    pub fn page_id(mut self, id: impl Into<Str>) -> Self {
        self.root = self.root.id(id);
        self
    }

    /// Attach the main page layout as a child of the config root.
    pub fn page(mut self, content: impl IntoToken) -> Container {
        let mut root = self.root;
        // Inject palette CSS variables as a style block child
        let palette_css = self.palette.to_css_vars();
        if !palette_css.is_empty() {
            let mut style_node = TokenNode::new(next_id());
            style_node.tag = "style".into();
            style_node.content = Some(palette_css.into());
            root.node_mut().children.push(style_node);
        }
        // Inject media query CSS as a style block child
        let media_css = self.media.to_css();
        if !media_css.is_empty() {
            let mut style_node = TokenNode::new(next_id());
            style_node.tag = "style".into();
            style_node.content = Some(media_css.into());
            root.node_mut().children.push(style_node);
        }
        // Add store presets as data attributes on root (consumed by TokenCtx)
        for (key, value) in &self.store_presets {
            root.node_mut().attributes.insert(
                format!("data-preset-{}", key).into(),
                value.clone(),
            );
        }
        // Attach the actual page content
        root.node_mut().children.push(content.into_node());
        root
    }
}

impl Default for PageConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Shortcut: `page_config()`
pub fn page_config() -> PageConfig {
    PageConfig::new()
}

// ── Palette ──────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Palette {
    pub primary: Str,
    pub secondary: Str,
    pub background: Str,
    pub surface: Str,
    pub text: Str,
    pub text_muted: Str,
    pub success: Str,
    pub warning: Str,
    pub danger: Str,
    pub font_base: Str,
    pub font_heading: Str,
    pub size_scale: Str, // "sm" | "md" | "lg"
}

impl Palette {
    pub fn light() -> Self {
        Self {
            primary: "#2563eb".into(),
            secondary: "#64748b".into(),
            background: "#f8fafc".into(),
            surface: "#ffffff".into(),
            text: "#0f172a".into(),
            text_muted: "#64748b".into(),
            success: "#22c55e".into(),
            warning: "#f59e0b".into(),
            danger: "#ef4444".into(),
            font_base: "ui-sans-serif, system-ui, sans-serif".into(),
            font_heading: "ui-sans-serif, system-ui, sans-serif".into(),
            size_scale: "md".into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: "#60a5fa".into(),
            secondary: "#94a3b8".into(),
            background: "#0f172a".into(),
            surface: "#1e293b".into(),
            text: "#f8fafc".into(),
            text_muted: "#94a3b8".into(),
            success: "#4ade80".into(),
            warning: "#fbbf24".into(),
            danger: "#f87171".into(),
            font_base: "ui-sans-serif, system-ui, sans-serif".into(),
            font_heading: "ui-sans-serif, system-ui, sans-serif".into(),
            size_scale: "md".into(),
        }
    }

    pub fn size_str(&self) -> &str {
        &self.size_scale
    }

    fn to_css_vars(&self) -> String {
        format!(
            ":root {{ \
            --tok-primary: {}; \
            --tok-secondary: {}; \
            --tok-bg: {}; \
            --tok-surface: {}; \
            --tok-text: {}; \
            --tok-text-muted: {}; \
            --tok-success: {}; \
            --tok-warning: {}; \
            --tok-danger: {}; \
            --tok-font-base: {}; \
            --tok-font-heading: {}; \
            font-family: {}; \
            }}",
            self.primary, self.secondary, self.background, self.surface,
            self.text, self.text_muted, self.success, self.warning, self.danger,
            self.font_base, self.font_heading, self.font_base,
        )
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
    }
}

// ── Media (responsive breakpoints) ───────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Media {
    pub mobile_breakpoint: u16,
    pub desktop_breakpoint: u16,
    pub compact: bool, // true = mobile-first, false = desktop-first
}

impl Media {
    pub fn mobile_first() -> Self {
        Self { mobile_breakpoint: 640, desktop_breakpoint: 1024, compact: true }
    }

    pub fn desktop_first() -> Self {
        Self { mobile_breakpoint: 640, desktop_breakpoint: 1024, compact: false }
    }

    fn to_css(&self) -> String {
        if self.compact {
            format!(
                "@media (max-width: {}px) {{ .tok-compact {{ display:none !important; }} }}
                 @media (min-width: {}px) {{ .tok-expanded {{ display:none !important; }} }}",
                self.mobile_breakpoint, self.desktop_breakpoint
            )
        } else {
            format!(
                "@media (min-width: {}px) {{ .tok-desktop {{ display:none !important; }} }}
                 @media (max-width: {}px) {{ .tok-mobile {{ display:none !important; }} }}",
                self.mobile_breakpoint, self.desktop_breakpoint
            )
        }
    }
}

impl Default for Media {
    fn default() -> Self {
        Self::mobile_first()
    }
}
