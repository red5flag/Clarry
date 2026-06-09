use std::env;
use std::fs;
use std::path::Path;

/// Automatically insert `.end()` calls into token DSL page files based on
/// indentation.  When a method-chain line has lower indentation than the
/// container that opened above it, we pop that container by emitting `.end()`.
/// Lines inside `.child(...)` blocks are left untouched.
fn preprocess_token_dsl(content: &str) -> String {
    let mut result = Vec::new();
    let mut container_stack: Vec<usize> = Vec::new();
    let mut paren_depth: i32 = 0;
    let mut in_string = false;
    let mut string_char = '\0';
    let mut escape_next = false;

    // Recognised container and leaf factory names for implicit-child DSL
    let container_names = [
        "col(", "row(", "block(", "grid(",
        "grid2(", "grid3(", "skeleton(", "skeleton_text(",
        "card(", "section_title(", "pill_chip(",
    ];
    let leaf_names = [
        "text(", "btn(", "img_block(", "text_input(", "input_number(",
        "input_password(", "checkbox(", "textarea(", "select(", "badge(",
        "chip(", "divider(", "spacer(", "skeleton(", "skeleton_text(",
        "copy_block(", "chat_bubble(", "qr_code(", "progress_bar(", "rating(",
        "video(", "video_ambient(", "audio_player(", "model_viewer(", "iframe(",
        "sr_only(", "skip_link(", "live_region(", "modal(", "tabs(", "accordion(",
        "overlay(", "portal(", "tooltip(", "drawer(", "terminal(", "log_view(",
        "hex_view(", "tree_view(", "status_bar(", "command_palette(", "shortcut(",
        "theme_provider(", "chat_ui(", "toast_container(",
        "card(", "section_title(", "pill_chip(",
    ];

    for line in content.lines() {
        let stripped = line.trim_start();
        let indent = line.len() - stripped.len();

        // Skip paren tracking for pure comment lines so unbalanced parens
        // inside comments do not corrupt multi-line .child() detection.
        let is_comment_line = stripped.starts_with("//");

        // ---- paren-depth tracking (ignoring strings and comments) ----
        let mut line_paren_delta = 0i32;
        let mut line_in_string = in_string;
        let mut line_string_char = string_char;
        let mut line_escape = escape_next;

        if !is_comment_line {
            for c in line.chars() {
                if line_escape {
                    line_escape = false;
                    continue;
                }
                if c == '\\' {
                    line_escape = true;
                    continue;
                }
                if line_in_string {
                    if c == line_string_char {
                        line_in_string = false;
                    }
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

        // If we are inside a `.child(...)` (or any) paren block, pass through.
        if was_inside_parens {
            result.push(line.to_string());
            continue;
        }

        // Skip blank / comment lines
        if stripped.is_empty() || is_comment_line {
            result.push(line.to_string());
            continue;
        }

        // ── Implicit child DSL: standalone factory calls at deeper indent ─
        let is_standalone = !stripped.starts_with('.');
        let is_container = is_standalone && container_names.iter().any(|n| stripped.starts_with(n));
        let is_leaf = is_standalone && leaf_names.iter().any(|n| stripped.starts_with(n));

        // Factories that have corresponding methods on Container (prepend `.`)
        let has_container_method = [
            "col(", "row(", "block(", "grid(",
            "grid2(", "grid3(", "skeleton(", "skeleton_text(",
            "text(", "btn(",
            "overlay(", "portal(", "split(", "aspect(",
            "tooltip(", "drawer(", "terminal(", "log_view(",
            "hex_view(", "tree_view(", "status_bar(", "command_palette(", "shortcut(",
            "modal(", "tabs(", "accordion(",
            "card(", "section_title(", "pill_chip(",
        ].iter().any(|n| stripped.starts_with(n));

        // Emit .end() for containers that have closed due to indent decrease
        if !container_stack.is_empty() {
            let parent_indent = *container_stack.last().unwrap();
            if indent <= parent_indent && !is_standalone_container(stripped) {
                let mut ends_needed = 0;
                while let Some(&last_indent) = container_stack.last() {
                    if indent <= last_indent {
                        container_stack.pop();
                        ends_needed += 1;
                    } else {
                        break;
                    }
                }
                for _ in 0..ends_needed {
                    result.push(" ".repeat(indent) + ".end()");
                }
            }
        }

        if (is_container || is_leaf) && !container_stack.is_empty() {
            let parent_indent = *container_stack.last().unwrap();
            if indent > parent_indent {
                if is_container && has_container_method {
                    let transformed = " ".repeat(indent) + "." + stripped;
                    result.push(transformed);
                    container_stack.push(indent);
                } else if is_leaf && has_container_method {
                    let transformed = " ".repeat(indent) + "." + stripped;
                    result.push(transformed);
                } else {
                    let transformed = " ".repeat(indent) + ".child(" + stripped + ")";
                    result.push(transformed);
                }
                continue;
            }
        }

        if is_standalone_container(stripped) {
            container_stack.push(indent);
            result.push(line.to_string());
            continue;
        }

        result.push(line.to_string());
    }

    result.join("\n")
}

fn is_standalone_container(stripped: &str) -> bool {
    (stripped.starts_with("col(") || stripped.starts_with("row(")
     || stripped.starts_with("block(") || stripped.starts_with("grid("))
    && !stripped.starts_with('.')
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
        assert!(out.contains(".text(\"hello\")"));
        assert!(out.contains(".btn(\"Click\")"));
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
        assert!(out.contains(".text(\"nested\")"));
        assert!(out.contains(".end()"));
    }

    #[test]
    fn test_four_level_nesting() {
        let input = r#"col()
    block()
        row()
            text("deep")
        .end()
    .end()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".col()"));
        assert!(out.contains(".block()"));
        assert!(out.contains(".row()"));
        assert!(out.contains(".text(\"deep\")"));
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
        assert!(out.contains(".text(\"Hello\")"));
        assert!(out.contains(".btn(\"Click\")"));
        assert!(out.contains(".end()"));
    }

    #[test]
    fn test_sibling_containers() {
        let input = r#"col()
    row()
        text("a")
    .end()
    block()
        text("b")
    .end()
"#;
        let out = preprocess_token_dsl(input);
        let ends: Vec<_> = out.lines().filter(|l| l.trim() == ".end()").collect();
        assert_eq!(ends.len(), 2);
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
        assert!(out.contains(".text(\"outside\")"));
    }

    #[test]
    fn test_escaped_quotes_in_string() {
        let input = r#"col()
    text("He said \"Hello\"")
        .bold()
"#;
        let out = preprocess_token_dsl(input);
        assert!(out.contains(".text(\"He said \\\"Hello\\\"\")"));
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
        // Verify key transformations
        assert!(out.contains(".row()"));
        assert!(out.contains(".text(\"Hello\")"));
        assert!(out.contains(".btn(\"Click\")"));
        assert!(out.contains(".col()"));
        assert!(out.contains(".block()"));
        assert!(out.contains(".text(\"Deep\")"));
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
                                    let content = fs::read_to_string(file_path).unwrap();

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
                                let content = fs::read_to_string(&path).unwrap();

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