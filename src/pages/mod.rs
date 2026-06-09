// src/pages/mod.rs
//
// Dynamic Page Loader Entry Point
// Includes the auto-generated dispatcher and module definitions from build.rs

// The build script generates OUT_DIR/pages/mod.rs which contains:
// 1. Module declarations for every page found in src/pages/
// 2. A list_pages() function
// 3. A dispatch_page() function to render the correct component

include!(concat!(env!("OUT_DIR"), "/pages/mod.rs"));