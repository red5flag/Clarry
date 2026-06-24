// src/main.rs
// Leptos 0.8 SSR + Hydration entry point

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use l8_loader::api;
use std::path::Path as StdPath;
use tokio::fs::File;
use tower_http::services::ServeDir;

// ── Custom WASM handler with correct MIME type ───────────────────────────────
async fn serve_wasm(Path(path): Path<String>, pkg_path: String) -> impl IntoResponse {
    use axum::body::Body;
    use axum::http::HeaderValue;

    let full_path = StdPath::new(&pkg_path).join(&path);

    match File::open(&full_path).await {
        Ok(file) => {
            let metadata = file.metadata().await.ok();

            // Read file into memory for better WebAssembly compatibility
            let bytes = tokio::fs::read(&full_path).await;

            match bytes {
                Ok(contents) => {
                    let mut response = Response::new(Body::from(contents));

                    // Set correct MIME type for WASM and JS files
                    if let Some(meta) = metadata {
                        if meta.is_file() {
                            if path.ends_with(".wasm") {
                                response.headers_mut().insert(
                                    CONTENT_TYPE,
                                    HeaderValue::from_static("application/wasm"),
                                );
                            } else if path.ends_with(".js") {
                                response.headers_mut().insert(
                                    CONTENT_TYPE,
                                    HeaderValue::from_static("application/javascript; charset=utf-8"),
                                );
                            }
                        }
                    }
                    response
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Debug builds need larger stack - Axum+hyper+tower consume significant stack
    let stack_bytes = 8 * 1024 * 1024;

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_stack_size(stack_bytes)
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime")
        .block_on(async_main());
}

#[cfg(not(target_arch = "wasm32"))]
async fn async_main() {
    let addr = std::env::var("CLARRY_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let pkg_path = "target/site/pkg".to_string();
    let pkg_path_clone = pkg_path.clone();

    let app = Router::new()
        // Catch-all: serve HTML shell with route injected into window.__INITIAL_ROUTE__
        .fallback(get({
            let pkg = pkg_path_clone.clone();
            move |axum::extract::OriginalUri(uri): axum::extract::OriginalUri| async move {
                let route = uri.path().trim_start_matches('/').to_string();
                let route = if route.is_empty() { "demo".to_string() } else { route };
                let html = minimal_html_shell(pkg, &route);
                axum::response::Html(html)
            }
        }))
        // Serve WASM with custom handler for correct MIME type
        .route("/pkg/l8-loader.wasm", get({
            let pkg = pkg_path_clone.clone();
            move || serve_wasm(Path("l8-loader.wasm".to_string()), pkg.clone())
        }))
        // Add alias for l8-loader_bg.wasm (wasm-bindgen default naming)
        .route("/pkg/l8-loader_bg.wasm", get({
            let pkg = pkg_path_clone.clone();
            move || serve_wasm(Path("l8-loader.wasm".to_string()), pkg.clone())
        }))
        .route("/pkg/l8-loader.js", get({
            let pkg = pkg_path_clone.clone();
            move || serve_wasm(Path("l8-loader.js".to_string()), pkg.clone())
        }))
        // Serve other static files in /pkg
        .nest_service("/pkg", ServeDir::new(&pkg_path).precompressed_gzip())
        // Log forwarding endpoints
        .nest("/api/logs", api::create_log_router().with_state(()));

    println!("🚀 Server running on http://{}", addr);
    println!("📦 pkg path: {}", pkg_path);
    println!("🌐 CSR: Ready to serve requests (pure client-side rendering)");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🌐 CSR: Successfully bound to address {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// ── Minimal HTML shell for pure CSR ──────────────────────────────────────────────
//
// This function returns a minimal HTML shell with no SSR content.
// The WASM bundle will render everything client-side via mount_to_body.

/// Escape a string for safe embedding inside a JavaScript double-quoted literal.
fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('<', "\\x3c")
     .replace('>', "\\x3e")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
}

fn minimal_html_shell(_pkg_path: String, route: &str) -> String {
    let js_file = "/pkg/l8-loader.js".to_string();
    let keyframes = l8_loader::tokens::animation::keyframe_css();
    let safe_route = escape_js_string(route);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>L8 Token DSL</title>
    <script src="https://cdn.tailwindcss.com" crossorigin="anonymous"></script>
    <script type="module" src="https://unpkg.com/@google/model-viewer/dist/model-viewer.min.js" crossorigin="anonymous"></script>
    <style id="tok-keyframes">{}</style>
    <script>
        window.__INITIAL_ROUTE__ = "{}";
    </script>
    <script type="module">
        import init, {{ hydrate }} from "{}";
        try {{
            await init();
            hydrate();
        }} catch(e) {{
            console.error('WASM init failed:', e);
        }}
    </script>
</head>
<body>
</body>
</html>"#,
        keyframes, safe_route, js_file
    )
}

// ── App component — reactive content only (no HTML shell) ─────────────────────
//
// The App component is defined in src/lib.rs and exported from the l8_loader crate.
// The server calls shell() → App() from lib.rs.
// The client calls mount_to_body(App) which renders the App component from scratch.
// Note: Token rendering system is incompatible with Leptos hydration markers,
// so we use pure CSR (mount_to_body) instead of hydration.