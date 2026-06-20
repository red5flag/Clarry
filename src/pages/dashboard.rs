use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_metrics;

let metrics = seed_metrics();

col
        id dashboard_page
        css min-h-screen bg-gray-50
        row
            css sticky top-0 z-50 bg-white border-b px-6 py-4 items-center justify-between
            txt "Dashboard"
                css text-2xl font-bold
            row
                css gap-4
                txt "red@fedora:~/Carly$ cargo leptos watch
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling getrandom v0.2.17
   Compiling zerocopy v0.8.52
error: the wasm*-unknown-unknown targets are not supported by default, you may need to enable the "js" feature. For more information see: https://docs.rs/getrandom/#webassembly-support
   --> /home/red/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.17/src/lib.rs:346:9
    |
346 | /         compile_error!("the wasm*-unknown-unknown targets are not supported by \
347 | |                         default, you may need to enable the \"js\" feature. \
348 | |                         For more information see: \
349 | |                         https://docs.rs/getrandom/#webassembly-support");
    | |________________________________________________________________________^

   Compiling rand_core v0.6.4========> ] 307/318: getrandom, zerocopy(build.rs)                                                                               
error: could not compile `getrandom` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
   Compiling password-hash v0.5.0
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.6
   Compiling argon2 v0.5.3
   Compiling farley v0.1.0 (/home/red/Carly)
    Building [=======================> ] 450/451: farley(bin)                                                                                                 🔔"
                    css text-xl
                img_block "https://i.pravatar.cc/150?u=me"
                    css w-8 h-8 rounded-full
        grid2
            css gap-4 px-6 py-6
            .add_all(metrics.iter().map(|m| {
                col()
                    .set_class("bg-white rounded-xl p-5 shadow-sm gap-2")
                    .add(text(m.label.as_str()).set_class("text-sm text-gray-500"))
                    .add(text(m.value.as_str()).set_class("text-2xl font-bold"))
                    .add(row().set_class("items-center gap-1")
                        .add(text(if m.is_positive { "↑" } else { "↓" })
                            .set_class(if m.is_positive { "text-green-500 text-sm" } else { "text-red-500 text-sm" }))
                        .add(text(format!("{:.1}%", m.change.abs()))
                            .set_class(if m.is_positive { "text-green-600 text-sm" } else { "text-red-600 text-sm" }))
                    )
            }))
        block
            css mx-6 bg-white rounded-xl p-5 shadow-sm
            txt "Revenue Trend"
                css text-lg font-bold mb-4
            block
                css h-40 bg-gradient-to-r from-blue-100 to-blue-50 rounded-lg flex items-end px-4 pb-4 gap-2
                .add_all([40, 65, 45, 80, 55, 90, 70].iter().map(|&h| {
                    block().set_class("flex-1 bg-blue-500 rounded-t-sm")
                        .set_class(format!("height:{}%", h))
                }))
        row
            css fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between
            txt "📊"
                css text-xl text-blue-600
            txt "📋"
                css text-xl
            txt "👥"
                css text-xl
            txt "⚙"
                css text-xl
