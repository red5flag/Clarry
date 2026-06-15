use std::env;
use std::fs;
use std::path::Path;

/// Automatically transform indented token DSL into method-chain Rust.
///
/// Rules:
///   - A standalone factory that is a *real container* (col/row/block/grid/…) opens an
///     indent level.  When indentation returns to or below the opener's level, `.end()`
///     is automatically emitted.  Explicit `.end()` is still accepted but no longer needed.
///   - A standalone factory that is a *leaf node* (btn/text/img_block/…) at deeper indent
///     becomes `.method()` on the current container.
///   - Lines already starting with `.` (method chains) are passed through unchanged.
///   - Lines inside open parentheses (multi-line Rust expressions) are passed through.
///   - **Bare modifier keywords** are expanded to `.method(rest)` chain calls:
///       css "classes"           →  .css("classes")
///       act navigate("home")    →  .act(navigate("home"))
///       id "page-id"            →  .id("page-id")
///       variant "primary"       →  .variant("primary")
///       size_str "sm"           →  .size_str("sm")
///       on_click_nav "route"    →  .on_click_nav("route")
///       bold  (no arg)          →  .bold()
///     Modifiers are checked BEFORE close-container logic so a modifier at the same
///     indent as its node never accidentally closes the parent.
fn preprocess_token_dsl(content: &str) -> String {
    // ── Container lists ───────────────────────────────────────────────────
    // Real layout containers: push indent level, emit .end() on close
    let real_containers: &[&str] = &[
        "col(", "row(", "block(", "grid(", "grid2(", "grid3(",
        "stack(", "split(", "aspect(", "overlay(", "portal(",
        "drawer(", "card(", "tooltip(",
    ];

    // Leaf-type nodes: accept modifier lines but do NOT emit .end()
    let leaf_containers: &[&str] = &[
        "btn(", "text(", "txt(",
        "text_input(", "txtinp(", "input_number(", "innum(",
        "input_password(", "inpsw(", "checkbox(", "textarea(", "txtarea(", "select(",
        "img_block(", "video(", "video_ambient(", "audio_player(", "audio(",
        "model_viewer(", "model(", "iframe(",
        "badge(", "chip(", "qr_code(", "progress_bar(", "rating(",
        "skeleton(", "skeleton_text(", "copy_block(",
        "text_bind(", "txtbnd(", "text_read(", "counter_text(",
        "bold(", "muted(", "uppercase(", "center(", "h1(", "h2(", "h3(",
        "caption(", "label(", "mono(", "italic(", "strike(", "underline(", "color(",
        "loading(", "disabled(",
        "sr(", "sr_only(", "skip_link(", "live(", "live_region(",
        "terminal(", "log_view(", "hex_view(", "tree_view(", "status_bar(",
        "command_palette(", "shortcut(", "toast_container(", "chat_bubble(",
        "chat_ui(", "divider(", "spacer(",
        "modal(", "tabs(", "accordion(", "theme_provider(", "theme!(",
        "pill_chip(",
        "write_to(", "read_from(", "add_to(", "remove_from(", "clear_key(",
        "load_from(", "storage_panel(", "list_panel(", "file_storage_panel(",
    ];

    let is_real_container   = |s: &str| real_containers.iter().any(|n| s.starts_with(n));
    let is_leaf_container   = |s: &str| leaf_containers.iter().any(|n| s.starts_with(n));
    let is_container_method = |s: &str| is_real_container(s) || is_leaf_container(s);

    // ── Bare modifier keywords ────────────────────────────────────────────
    // Lines of the form:   <keyword> <rest>
    // are emitted as:      .<keyword>(<rest>)
    // where <rest> is already a valid Rust expression (string literal, fn call, etc.)
    //
    // Single-token keywords (no argument):  bold  muted  italic  etc.
    // are emitted as:  .<keyword>()
    //
    // The modifier line does NOT open a new indent level and does NOT push the stack.
    let modifier_keywords: &[&'static str] = &[
        "css", "act", "id", "variant", "size", "size_str",
        "on_click_nav", "placeholder", "href", "src", "alt",
        "min", "max", "step", "rows", "cols",
        "name", "for_id",
        "aria_label", "aria_hidden", "aria_live",
        // Shorthand method aliases (emitted as .method(...) chain, not .child())
        "var", "sz", "inc", "dec", "tog", "cyc", "in_",
        "on_nav", "use_var", "append_css",
        "anim_pulse", "section", "section_title",
    ];
    // Zero-arg modifier flags (just `.flag()`)
    let modifier_flags: &[&'static str] = &[
        "bold", "muted", "italic", "strike", "underline", "uppercase",
        "center", "mono", "disabled", "loading",
        "on_scroll_enter", "on_scroll_enter_scale",
    ];

    // Returns (keyword, rest_expression) if line is a bare modifier, else None.
    // rest borrows from `s`; keyword is a &'static str from the lists above.
    fn check_modifier<'a>(
        s: &'a str,
        modifier_keywords: &[&'static str],
        modifier_flags: &[&'static str],
    ) -> Option<(&'static str, &'a str)> {
        for &kw in modifier_flags {
            if s == kw || s == &format!("{}()", kw) {
                return Some((kw, ""));
            }
        }
        for &kw in modifier_keywords {
            let prefix_len = kw.len() + 1; // "keyword "
            if s.len() > prefix_len - 1
                && s.starts_with(kw)
                && s.as_bytes().get(kw.len()) == Some(&b' ')
            {
                let rest = s[prefix_len..].trim();
                return Some((kw, rest));
            }
            // Also match old-style keyword(...) calls: css("..."), act(...), id("...")
            // These appear at deeper indent and should become .keyword(...) chains, not .child()
            if s.starts_with(kw) && s.as_bytes().get(kw.len()) == Some(&b'(') {
                // Return entire call as-is; the emitter will prefix with `.`
                return Some((kw, &s[kw.len()..]));
            }
        }
        None
    }

    let mut result: Vec<String> = Vec::new();
    // Stack of (indent_level, needs_end)
    let mut container_stack: Vec<(usize, bool)> = Vec::new();
    let mut paren_depth: i32 = 0;
    let mut in_string = false;
    let mut string_char = '\0';
    let mut escape_next = false;
    // When wrapping a multi-line expression in .child(...), track the depth at
    // which the .child( was opened so we can close it when paren_depth returns.
    let mut pending_child_close_at: i32 = -1;
    // When a leaf is wrapped as .child(leaf_call), modifiers indented under it
    // must be folded INTO that .child() call rather than chained on the parent.
    // pending_leaf_indent = Some(leaf_indent) while we are inside such a leaf.
    // Any modifier whose indent > leaf_indent is appended to the last result line
    // (stripping its trailing `)`) instead of being pushed as a new line.
    let mut pending_leaf_indent: Option<usize> = None;

    for line in content.lines() {
        let stripped = line.trim_start();
        let indent = line.len() - stripped.len();
        let is_comment_line = stripped.starts_with("//");

        // ── Paren-depth tracking ──────────────────────────────────────────
        let mut line_paren_delta = 0i32;
        let mut line_in_string = in_string;
        let mut line_string_char = string_char;
        let mut line_escape = escape_next;

        if !is_comment_line {
            for c in line.chars() {
                if line_escape { line_escape = false; continue; }
                if c == '\\' { line_escape = true; continue; }
                if line_in_string {
                    if c == line_string_char { line_in_string = false; }
                } else if c == '"' || c == '\'' {
                    line_in_string = true;
                    line_string_char = c;
                } else if c == '(' {
                    line_paren_delta += 1;
                } else if c == ')' {
                    line_paren_delta -= 1;
                }
            }
        }

        let was_inside_parens = paren_depth > 0;
        paren_depth += line_paren_delta;
        in_string = line_in_string;
        string_char = line_string_char;
        escape_next = line_escape;

        // Inside an open paren block → pass through unchanged.
        // If we opened a .child( wrapper and paren_depth just returned to 0,
        // append the closing ) to the last line.
        if was_inside_parens {
            if paren_depth == 0 && pending_child_close_at == 0 {
                // This line closed the multi-line expression; append closing )
                result.push(format!("{}) ", line));
                pending_child_close_at = -1;
            } else {
                result.push(line.to_string());
            }
            continue;
        }

        // Blank or comment lines → pass through
        if stripped.is_empty() || is_comment_line {
            result.push(line.to_string());
            continue;
        }

        let is_standalone = !stripped.starts_with('.');

        // Explicit .end() → pop stack and pass through
        if stripped == ".end()" {
            container_stack.pop();
            result.push(line.to_string());
            continue;
        }

        // ── Bare modifier keyword: css / act / id / variant / ... ─────────
        // MUST be checked BEFORE close-container logic so that a modifier at
        // the same indent as its parent node does not accidentally close it.
        // Modifiers attach as method calls to the most-recently opened node.
        // They do NOT push the stack, do NOT close any container.
        //
        // Special case: if pending_leaf_indent is set and this modifier is
        // indented deeper than the leaf, fold it INTO the .child() call by
        // appending to the last result line (stripping trailing `)` first).
        if is_standalone {
            if let Some((kw, rest)) = check_modifier(stripped, modifier_keywords, modifier_flags) {
                let modifier_str = if rest.is_empty() {
                    format!(".{}()", kw)
                } else if rest.starts_with('(') {
                    format!(".{}{}", kw, rest)
                } else {
                    format!(".{}({})", kw, rest)
                };

                if let Some(leaf_indent) = pending_leaf_indent {
                    if indent > leaf_indent && line_paren_delta == 0 {
                        // Fold into the pending .child(leaf...) call —
                        // only when the modifier value is single-line (no open parens)
                        if let Some(last) = result.last_mut() {
                            // Strip trailing ) and space, append modifier, re-close
                            let trimmed = last.trim_end_matches(|c: char| c == ' ');
                            if trimmed.ends_with(')') {
                                *last = format!("{}{})", &trimmed[..trimmed.len()-1], modifier_str);
                            } else {
                                last.push_str(&modifier_str);
                            }
                        }
                        continue;
                    } else if indent <= leaf_indent || line_paren_delta != 0 {
                        // Multi-line modifier or dedented — end leaf folding window
                        pending_leaf_indent = None;
                    }
                }

                result.push(format!("{}{}", " ".repeat(indent), modifier_str));
                continue;
            }
        }

        // Any non-modifier, non-comment standalone line at or below leaf indent
        // ends the pending-leaf modifier window.
        if is_standalone && !stripped.starts_with("//") {
            if let Some(leaf_indent) = pending_leaf_indent {
                if indent <= leaf_indent {
                    pending_leaf_indent = None;
                }
            }
        }

        // ── Close containers whose indent ≥ current line ─────────────────
        if is_standalone && !container_stack.is_empty() {
            let parent_indent = container_stack.last().unwrap().0;
            if indent <= parent_indent {
                while let Some(&(last_indent, needs_end)) = container_stack.last() {
                    if indent <= last_indent {
                        container_stack.pop();
                        if needs_end {
                            result.push(" ".repeat(indent) + ".end()");
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        // ── Transform standalone calls inside a container context ─────────
        // Macro invocations (theme!(...), vec![...]) inside a container get wrapped
        // in .child() — they cannot be turned into .method() calls directly.
        let is_macro_call = {
            let s = stripped.trim_start_matches(|c: char| c.is_alphanumeric() || c == '_');
            s.starts_with("!(") || s.starts_with("![")
        };
        if is_standalone && is_macro_call && !container_stack.is_empty() {
            let parent_indent = container_stack.last().unwrap().0;
            if indent > parent_indent {
                if line_paren_delta > 0 {
                    result.push(format!("{}.child({}", " ".repeat(indent), stripped));
                    pending_child_close_at = 0;
                } else {
                    result.push(format!("{}.child({})", " ".repeat(indent), stripped));
                }
                continue;
            }
        }
        if is_standalone && !is_macro_call && !container_stack.is_empty() {
            let parent_indent = container_stack.last().unwrap().0;
            if indent > parent_indent {
                if is_real_container(stripped) {
                    // Real containers: .row(), .col(), .block() etc. — method on Container, push stack
                    result.push(" ".repeat(indent) + "." + stripped);
                    container_stack.push((indent, true));
                    pending_leaf_indent = None; // real container opens a new scope
                } else if is_leaf_container(stripped) || true {
                    // Leaf factories and other standalone calls: wrap in .child()
                    if line_paren_delta > 0 {
                        // Multi-line expression: open .child( without closing )
                        // The closing ) will be appended when paren_depth returns to 0
                        result.push(format!("{}.child({}", " ".repeat(indent), stripped));
                        pending_child_close_at = 0;
                        // Multi-line leaves can't use pending_leaf_indent folding
                        pending_leaf_indent = None;
                    } else {
                        result.push(format!("{}.child({})", " ".repeat(indent), stripped));
                        // Record leaf indent so subsequent modifier lines fold in
                        pending_leaf_indent = Some(indent);
                    }
                } else {
                    unreachable!()
                }
                continue;
            }
        }

        // ── Top-level standalone container factory → open indent level ────
        if is_standalone && is_container_method(stripped) {
            let needs_end = is_real_container(stripped);
            container_stack.push((indent, needs_end));
            result.push(line.to_string());
            continue;
        }

        result.push(line.to_string());
    }

    // Close any remaining open real-containers at EOF
    while container_stack.len() > 1 {
        let (_, needs_end) = container_stack.pop().unwrap();
        if needs_end {
            result.push(".end()".to_string());
        }
    }

    result.join("\n")
}

/// Rewrite bare `[...]` array literals (not preceded by `vec!`) to `vec![...]`.
/// Operates as a pure text pre-pass before DSL tokenisation.
/// Skips `[` inside string literals, raw strings, and character literals.
fn rewrite_vec_literals(content: &str) -> String {
    let mut out = String::with_capacity(content.len() + 64);
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Raw string: r#"..."# or r"..."
        if c == 'r' && i + 1 < len && (chars[i + 1] == '"' || chars[i + 1] == '#') {
            // Count leading #
            let mut hashes = 0;
            let mut j = i + 1;
            while j < len && chars[j] == '#' { hashes += 1; j += 1; }
            if j < len && chars[j] == '"' {
                // Emit raw string as-is until closing "###...#
                let closing: String = std::iter::once('"').chain(std::iter::repeat('#').take(hashes)).collect();
                let start = i;
                i = j + 1; // skip opening "
                loop {
                    if i >= len { break; }
                    // Check for closing sequence
                    let remaining: String = chars[i..].iter().collect();
                    if remaining.starts_with(&closing) {
                        i += closing.len();
                        break;
                    }
                    i += 1;
                }
                let raw_str: String = chars[start..i].iter().collect();
                out.push_str(&raw_str);
                continue;
            }
            // Not a raw string — fall through
        }

        // Regular string literal
        if c == '"' {
            out.push('"');
            i += 1;
            while i < len {
                let sc = chars[i];
                out.push(sc);
                if sc == '\\' && i + 1 < len {
                    i += 1;
                    out.push(chars[i]);
                } else if sc == '"' {
                    break;
                }
                i += 1;
            }
            i += 1;
            continue;
        }

        // Character literal
        if c == '\'' {
            out.push('\'');
            i += 1;
            while i < len {
                let sc = chars[i];
                out.push(sc);
                if sc == '\\' && i + 1 < len {
                    i += 1;
                    out.push(chars[i]);
                } else if sc == '\'' {
                    break;
                }
                i += 1;
            }
            i += 1;
            continue;
        }

        // `[` — rewrite to `vec![` unless preceded by `vec!` or `]` (index expr)
        if c == '[' {
            let preceded_by_vec = out.trim_end().ends_with("vec!");
            // Also skip if it looks like an index: preceded by ident char or `]` or `)`
            let last_non_ws = out.trim_end().chars().last();
            let is_index = matches!(last_non_ws,
                Some(ch) if ch == ']' || ch == ')' || ch.is_alphanumeric() || ch == '_'
            );
            if preceded_by_vec || is_index {
                out.push('[');
            } else {
                out.push_str("vec![");
            }
            i += 1;
            continue;
        }

        out.push(c);
        i += 1;
    }
    out
}

/// Wrap a bare page-body (no fn signature) in `pub fn page_token() -> impl IntoToken`.
/// If the file already contains `fn page_token`, pass through unchanged.
fn wrap_page_fn(content: &str) -> String {
    if content.contains("fn page_token") {
        return content.to_string();
    }
    format!("pub fn page_token() -> impl IntoToken {{\n{}\n}}", content)
}


#[cfg(test)]
mod tests {
    use super::preprocess_token_dsl;

    #[test]
    fn test_implicit_leaf_child() {
        let input = r#"col()
    text("hello")
    btn("Click")
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".child(text(\"hello\"))"));
        assert!(out.contains(".child(btn(\"Click\"))"));
    }

    #[test]
    fn test_implicit_nested_container() {
        let input = r#"col()
    row()
        text("nested")
    .end()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".row()"));
        assert!(out.contains(".child(text(\"nested\"))"));
        assert!(out.contains(".end()"));
    }

    #[test]
    fn test_four_level_nesting() {
        let input = r#"col()
    block()
        row()
            text("deep")
"#;
        let out = preprocess_token_dsl(input);
        // top-level col() stays as-is; nested ones get a dot prefix
        assert!(out.contains("col()"));
        assert!(out.contains(".block()"));
        assert!(out.contains(".row()"));
        assert!(out.contains(".child(text(\"deep\"))"));
        let ends: Vec<_> = out.lines().filter(|l| l.trim() == ".end()").collect();
        assert_eq!(ends.len(), 2);
    }

    #[test]
    fn test_child_parens_untouched() {
        let input = r#"col()
    .child(
        text("hello")
    )
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains("        text(\"hello\")"));
    }

    #[test]
    fn test_mixed_containers_and_leaves() {
        let input = r#"col()
    row()
        text("Hello")
            .bold()
        btn("Click")
            .variant("primary")
            .act(increment("counter"))
    .end()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".row()"));
        assert!(out.contains(".child(text(\"Hello\"))"));
        assert!(out.contains(".child(btn(\"Click\"))"));
        assert!(out.contains(".end()"));
    }

    #[test]
    fn test_sibling_containers() {
        // Implicit-close via indentation: no explicit .end() needed.
        // col > row > text("a"), then block > text("b"), then EOF.
        // Generates: .end() for row (when block seen at same indent),
        //            .end() for block (EOF), .end() for col (EOF) = 3 total.
        let input = r#"col()
    row()
        text("a")
    block()
        text("b")
"#;
        let out = preprocess_token_dsl(input);
        let ends: Vec<_> = out.lines().filter(|l| l.trim() == ".end()").collect();
        // row gets .end() when block() is encountered at same indent
        // block gets .end() at EOF (len > 1 loop)
        // col (root, len==1) does NOT get .end() — it is the return value
        assert_eq!(ends.len(), 2, "got:\n{}", out);
    }

    #[test]
    fn test_empty_container() {
        let input = r#"col()
    block()
    .end()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".block()"));
        assert!(out.contains(".end()"));
    }

    #[test]
    fn test_multiline_child_call() {
        let input = r#"col()
    .child(
        text("line1")
            .bold()
    )
    text("outside")
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains("        text(\"line1\")"));
        assert!(out.contains(".bold()"));
        assert!(out.contains(".child(text(\"outside\"))"));
    }

    #[test]
    fn test_escaped_quotes_in_string() {
        let input = r#"col()
    text("He said \"Hello\"")
        .bold()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".child(text(\"He said \\\"Hello\\\"\"))"));
    }

    #[test]
    fn test_dsl_example_from_spec() {
        let input = r#"col()
    .id("page")
    .css("min-h-screen p-6")
    row()
        .css("gap-4")
        text("Hello")
            .bold()
        btn("Click")
            .variant("primary")
            .act(increment("counter"))
    col()
        .css("gap-2")
        text("Nested")
        block()
            .css("p-4")
            text("Deep")
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".row()"));
        assert!(out.contains(".child(text(\"Hello\"))"));
        assert!(out.contains(".child(btn(\"Click\"))"));
        assert!(out.contains(".col()"));
        assert!(out.contains(".block()"));
        assert!(out.contains(".child(text(\"Deep\"))"));
    }

    // ── New bare-modifier syntax tests ────────────────────────────────────

    #[test]
    fn test_css_bare_modifier() {
        let input = r#"col()
    css "min-h-screen bg-black"
    row()
        css "gap-4 items-center"
        text("Hello")
            css "text-sm font-bold"
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".css(\"min-h-screen bg-black\")"));
        assert!(out.contains(".css(\"gap-4 items-center\")"));
        assert!(out.contains(".css(\"text-sm font-bold\")"));
    }

    #[test]
    fn test_act_bare_modifier() {
        let input = r#"col()
    btn("Save")
        act store_set("key", "val")
    btn("Nav")
        act navigate("home")
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".act(store_set(\"key\", \"val\"))"));
        assert!(out.contains(".act(navigate(\"home\"))"));
    }

    #[test]
    fn test_id_and_variant_modifiers() {
        let input = r#"col()
    id "my-page"
    btn("Click")
        variant "primary"
        css "w-full"
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".id(\"my-page\")"));
        assert!(out.contains(".variant(\"primary\")"));
        assert!(out.contains(".css(\"w-full\")"));
    }

    #[test]
    fn test_bare_flag_modifier() {
        let input = r#"col()
    text("Hello")
        bold
        italic
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".bold()"));
        assert!(out.contains(".italic()"));
    }

    #[test]
    fn test_modifier_does_not_push_stack() {
        // css modifier between two sibling containers should not affect nesting
        let input = r#"col()
    css "gap-4"
    row()
        css "gap-2"
        text("A")
    block()
        text("B")
"#;
        let out = preprocess_token_dsl(input);
        // col gets css
        assert!(out.contains(".css(\"gap-4\")"));
        // row gets css
        assert!(out.contains(".css(\"gap-2\")"));
        // both row and block are children of col
        assert!(out.contains(".row()"));
        assert!(out.contains(".block()"));
        // text A is child of row, text B is child of block
        assert!(out.contains(".child(text(\"A\"))"));
        assert!(out.contains(".child(text(\"B\"))"));
        // two .end() calls for row and block
        let ends: Vec<_> = out.lines().filter(|l| l.trim() == ".end()").collect();
        assert_eq!(ends.len(), 2, "Expected 2 .end() calls, got: {}", out);
    }

    #[test]
    fn test_on_click_nav_modifier() {
        let input = r#"col()
    btn("Home")
        on_click_nav "instagram_home"
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".on_click_nav(\"instagram_home\")"));
    }

    #[test]
    fn test_mixed_old_and_new_syntax() {
        // Old .css() / .act() style should still pass through unchanged
        let input = r#"col()
    row()
        .css("gap-4")
        text("Hello")
            css "text-sm"
            .bold()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".css(\"gap-4\")"));
        assert!(out.contains(".css(\"text-sm\")"));
        assert!(out.contains(".bold()"));
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/pages/");
    println!("cargo:rerun-if-changed=src/tokens/");

    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| "target".to_string());
    let pages_out_dir = Path::new(&out_dir).join("pages");

    // Create pages output directory
    fs::create_dir_all(&pages_out_dir).unwrap();

    // Generate module declarations for all pages
    let mut pages = Vec::new();
    let mut page_modules = Vec::new();
    let pages_dir = Path::new("src/pages");
    
    if pages_dir.exists() {
        println!("🔍 Pages directory exists: {}", pages_dir.display());
        
        // Check for .rs files in pages directory and subdirectories
        if let Ok(entries) = fs::read_dir(pages_dir) {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                
                if path.is_dir() {
                    // Handle subdirectory with multiple .rs files (module split)
                    if let Some(dir_name) = path.file_name() {
                        if let Some(dir_str) = dir_name.to_str() {
                            let page_name = dir_str.to_string();
                            let mut rs_files = Vec::new();
                            
                            // Collect all .rs files in the subdirectory
                            if let Ok(sub_entries) = fs::read_dir(&path) {
                                for sub_entry in sub_entries {
                                    let sub_path = sub_entry.unwrap().path();
                                    if let Some(file_name) = sub_path.file_name() {
                                        if let Some(file_str) = file_name.to_str() {
                                            if file_str.ends_with(".rs") {
                                                rs_files.push((file_str.to_string(), sub_path));
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if !rs_files.is_empty() {
                                println!("Found page module in subdirectory: {} ({} files)", page_name, rs_files.len());
                                pages.push(page_name.clone());
                                page_modules.push(page_name.clone());
                                
                                // Create a directory for this page's modules
                                let page_out_dir = pages_out_dir.join(&page_name);
                                fs::create_dir_all(&page_out_dir).unwrap();
                                
                                // Process each .rs file
                                let mut module_decls = String::new();
                                let mut main_file_content = String::new();
                                
                                for (file_str, file_path) in &rs_files {
                                    let module_name = file_str.trim_end_matches(".rs");
                                    let raw = fs::read_to_string(file_path).unwrap();
                                    let content = rewrite_vec_literals(&wrap_page_fn(&raw));

                                    // 1. Auto-insert .end() calls based on indentation
                                    let preprocessed = preprocess_token_dsl(&content);

                                    // 2. Strip old single-line use statements (but preserve data imports) and comments
                                    let lines: Vec<&str> = preprocessed.lines().collect();
                                    let filtered: Vec<String> = lines
                                        .into_iter()
                                        .filter(|line| {
                                            let trimmed = line.trim();
                                            let is_use = trimmed.starts_with("use ") && trimmed.ends_with(";");
                                            let is_data_import = is_use && trimmed.contains("crate::data");
                                            (!is_use || is_data_import) && !trimmed.starts_with("//")
                                        })
                                        .map(|line| line.to_string())
                                        .collect();
                                    let cleaned = filtered.join("\n");

                                    let fixed_content = format!(
                                        "use leptos::prelude::*;\n\
                                         use leptos::wasm_bindgen::JsCast;\n\
                                         use crate::tokens::prelude::*;\n\
                                         use crate::tokens::debug::inspector_log;\n\
                                         use crate::tokens::builders::reset_id_counter;\n\
                                         \n{}",
                                        cleaned
                                    );
                                    
                                    // Check if this is the main file (same name as dir)
                                    if file_str.as_str() == format!("{}.rs", dir_str) {
                                        // Store main file content for mod.rs
                                        main_file_content = fixed_content;
                                    } else {
                                        // Write component file
                                        let dest_file = page_out_dir.join(file_str);
                                        fs::write(&dest_file, fixed_content).unwrap();
                                        // Add module declaration
                                        module_decls.push_str(&format!("pub mod {};\n", module_name));
                                    }
                                }
                                
                                // Create mod.rs with main file content + module declarations
                                let mod_content = format!(
                                    "// Auto-generated module for {}\n\
                                     // Do not edit manually - regenerated by build.rs\n\n\
                                     {}\n\n\
                                     {}\n",
                                    page_name,
                                    main_file_content,
                                    module_decls
                                );
                                let mod_file = page_out_dir.join("mod.rs");
                                fs::write(&mod_file, mod_content).unwrap();
                            }
                        }
                    }
                } else if path.is_file() {
                    // Handle direct .rs files in pages directory
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_str) = file_name.to_str() {
                            println!("Processing file: {}", file_str);
                            if file_str.ends_with(".rs") {
                                if file_str == "mod.rs" {
                                    println!("Skipping mod.rs file");
                                    continue;
                                }
                                let page_name = file_str.trim_end_matches(".rs");
                                println!("Found page: {}", page_name);
                                pages.push(page_name.to_string());
                                
                                // Read, preprocess, fix imports, and write to output directory
                                let raw = fs::read_to_string(&path).unwrap();
                                let content = rewrite_vec_literals(&wrap_page_fn(&raw));

                                // 1. Auto-insert .end() calls based on indentation
                                let preprocessed = preprocess_token_dsl(&content);

                                // 2. Strip old single-line use statements (but preserve data imports) and comments
                                let lines: Vec<&str> = preprocessed.lines().collect();
                                let filtered: Vec<String> = lines
                                    .into_iter()
                                    .filter(|line| {
                                        let trimmed = line.trim();
                                        let is_use = trimmed.starts_with("use ") && trimmed.ends_with(";");
                                        let is_data_import = is_use && trimmed.contains("crate::data");
                                        (!is_use || is_data_import) && !trimmed.starts_with("//")
                                    })
                                    .map(|line| line.to_string())
                                    .collect();
                                let cleaned = filtered.join("\n");

                                let fixed_content = format!(
                                    "use leptos::prelude::*;\n\
                                     use leptos::wasm_bindgen::JsCast;\n\
                                     use crate::tokens::prelude::*;\n\
                                     use crate::tokens::debug::inspector_log;\n\
                                     use crate::tokens::builders::reset_id_counter;\n\
                                     \n{}",
                                    cleaned
                                );
                                
                                let dest_file = pages_out_dir.join(format!("{}.rs", page_name));
                                fs::write(&dest_file, fixed_content).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Generate mod.rs file
    let mut mod_code = String::new();
    mod_code.push_str("// Auto-generated page module declarations\n");
    mod_code.push_str("// Do not edit manually - regenerated by build.rs\n\n");
    
    // Add module declarations for subdirectories using include!
    for module in &page_modules {
        mod_code.push_str("#[allow(unused_imports)]\n");
        mod_code.push_str(&format!("pub mod {} {{\n", module));
        mod_code.push_str(&format!("    include!(concat!(env!(\"OUT_DIR\"), \"/pages/{}/mod.rs\"));\n", module));
        mod_code.push_str("}\n");
    }

    // Add module declarations for direct .rs files using include!
    // (Files were already preprocessed and written to output directory above)
    for page in &pages {
        if page != "mod" && !page_modules.contains(page) {
            let dest_file = pages_out_dir.join(format!("{}.rs", page));
            if dest_file.exists() {
                mod_code.push_str("#[allow(unused_imports)]\n");
                mod_code.push_str(&format!("pub mod {} {{\n", page));
                mod_code.push_str(&format!("    include!(concat!(env!(\"OUT_DIR\"), \"/pages/{}.rs\"));\n", page));
                mod_code.push_str("}\n");
            }
        }
    }
    
    // Add the list_pages function
    mod_code.push_str("\n// List all discovered pages\n");
    mod_code.push_str("pub fn list_pages() -> Vec<String> {\n");
    mod_code.push_str("    vec![\n");
    for (i, page) in pages.iter().enumerate() {
        let comma = if i == pages.len() - 1 { "" } else { "," };
        mod_code.push_str(&format!("        \"{}\".to_string(){}\n", page, comma));
    }
    mod_code.push_str("    ]\n");
    mod_code.push_str("}\n");
    
    // Add the dispatch_page function
    // mod_code.push_str("\n// Dispatch to specific page component\n");
    // mod_code.push_str("pub fn dispatch_page(page_name: &str) -> impl IntoView {\n");
    // mod_code.push_str("    match page_name {\n");
    // for page in &pages {
    //     match page.as_str() {
    //         "smolvlm" => mod_code.push_str("        \"smolvlm\" => view! { { smolvlm::SmolVlmPage() } },\n"),
    //         "test_simple" => mod_code.push_str("        \"test_simple\" => view! { { test_simple::test_simple_page() } },\n"),
    //         "builder" => mod_code.push_str("        \"builder\" => view! { <div class=\"p-8\"><h1>Builder Page</h1><p>Builder page content coming soon...</p></div> },\n"),
    //         _ => mod_code.push_str(&format!("        \"{}\" => view! {{ <div class=\"p-8\"><h1>Page {}</h1><p>Page content coming soon...</p></div> }},\n", page, page)),
    //     }
    // }
    // mod_code.push_str("        _ => view! { <div class=\"p-8 text-center\">\n");
    // mod_code.push_str("            <h1 class=\"text-2xl font-bold text-red-600\">Page Not Found</h1>\n");
    // mod_code.push_str("            <p class=\"text-gray-600\">The page \"{0}\" was not found.</p>\n");
    // mod_code.push_str("            <a href=\"/\" class=\"mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600\">Go Home</a>\n");
    // mod_code.push_str("        </div> },\n");
    // mod_code.push_str("    }\n");
    // mod_code.push_str("}\n");
    
    let mod_file = pages_out_dir.join("mod.rs");
    fs::write(&mod_file, mod_code).unwrap();
    
    // Generate include file for main.rs
    let include_code = format!("include!(\"{}/pages/mod.rs\");", out_dir);
    let dest_path = Path::new(&out_dir).join("pages_modules.rs");
    fs::write(&dest_path, include_code).unwrap();
    
    println!("✅ Generated page loader system with {} pages", pages.len());
    for page in &pages {
        println!("  📄 {}", page);
    }

    // ── Build Artifacts Verification ─────────────────────────────────────
    println!("cargo:warning=✅ Build artifacts check:");
    let pkg_dir = Path::new("target/site/pkg");
    if pkg_dir.exists() {
        if let Ok(entries) = fs::read_dir(pkg_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.ends_with(".wasm") || name_str.ends_with(".js") {
                    let meta = entry.metadata().ok();
                    let size = meta.map(|m| m.len()).unwrap_or(0);
                    println!("cargo:warning=  📦 {} ({:.1} KB)", name_str, size as f64 / 1024.0);
                }
            }
        }
    } else {
        println!("cargo:warning=  ⚠️ pkg directory not found at target/site/pkg");
    }
}