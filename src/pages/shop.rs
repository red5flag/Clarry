use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_products;

pub fn page_token() -> impl IntoToken {
    let products = seed_products();

    col()
        .id("shop_page")
        .css("min-h-screen bg-gray-50")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-white border-b px-4 py-3 items-center justify-between")
            .child(text("Shop").css("text-xl font-bold"))
            .child(row().css("gap-4")
                .child(text("🔍").css("text-xl"))
                .child(text("🛒").css("text-xl"))
            )
        )
        // Categories
        .child(row()
            .css("px-4 py-3 gap-2 overflow-x-auto scrollbar-none")
            .child(pill_chip("All"))
            .child(pill_chip("Home"))
            .child(pill_chip("Tech"))
            .child(pill_chip("Fashion"))
            .child(pill_chip("Beauty"))
        )
        // Products grid
        .child(grid(2).css("gap-2 px-2 py-2 sm:gap-4 sm:px-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4")
            .children(products.iter().map(|p| {
                col()
                    .css("bg-white rounded-xl overflow-hidden shadow-sm")
                    .child(img_block(p.image_url.as_str())
                        .css("w-full aspect-square object-cover")
                    )
                    .child(col().css("p-3 gap-1")
                        .child(text(p.name.as_str()).css("text-sm font-medium line-clamp-2"))
                        .child(row().css("items-center gap-1")
                            .child(text("★").css("text-yellow-500 text-xs"))
                            .child(text(format!("{:.1}", p.rating)).css("text-xs text-gray-600"))
                            .child(text(format!("({})", p.reviews)).css("text-xs text-gray-400"))
                        )
                        .child(text(format!("${:.2}", p.price)).css("text-lg font-bold"))
                    )
            }))
        )
        // Bottom nav
        .child(row()
            .css("fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between")
            .child(text("🏠").css("text-xl"))
            .child(text("♡").css("text-xl"))
            .child(text("🛒").css("text-xl text-blue-600"))
            .child(text("👤").css("text-xl"))
        )
}

fn pill_chip(label: &str) -> impl IntoToken {
    block()
        .css("px-4 py-1.5 bg-gray-900 text-white rounded-full text-sm whitespace-nowrap")
        .child(text(label).css("text-sm"))
}
