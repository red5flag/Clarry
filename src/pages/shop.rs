use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_products;

pub fn page_token() -> impl IntoToken {
    let products = seed_products();
    col
        id shop_page
        css min-h-screen bg-gray-50
        row
            css sticky top-0 z-50 bg-white border-b px-4 py-3 items-center justify-between
            txt "Shop"
                css text-xl font-bold
            row
                css gap-4
                txt "🔍"
                    css text-xl
                txt "🛒"
                    css text-xl
        row
            css px-4 py-3 gap-2 overflow-x-auto scrollbar-none
            pill_chip "All"
            pill_chip "Home"
            pill_chip "Tech"
            pill_chip "Fashion"
            pill_chip "Beauty"
        grid2
            css gap-2 px-2 py-2 sm:gap-4 sm:px-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4
            .add_all(products.iter().map(|p| {
                col()
                    .set_class("bg-white rounded-xl overflow-hidden shadow-sm")
                    .add(img_block(p.image_url.as_str())
                        .set_class("w-full aspect-square object-cover")
                    )
                    .add(col().set_class("p-3 gap-1")
                        .add(text(p.name.as_str()).set_class("text-sm font-medium line-clamp-2"))
                        .add(row().set_class("items-center gap-1")
                            .add(text("★").set_class("text-yellow-500 text-xs"))
                            .add(text(format!("{:.1}", p.rating)).set_class("text-xs text-gray-600"))
                            .add(text(format!("({})", p.reviews)).set_class("text-xs text-gray-400"))
                        )
                        .add(text(format!("${:.2}", p.price)).set_class("text-lg font-bold"))
                    )
            }))
        row
            css fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between
            txt "🏠"
                css text-xl
            txt "♡"
                css text-xl
            txt "🛒"
                css text-xl text-blue-600
            txt "👤"
                css text-xl
.end()
}

fn pill_chip(label: &str) -> impl IntoToken {
    block
        css px-4 py-1.5 bg-gray-900 text-white rounded-full text-sm whitespace-nowrap
        txt(label)
            css text-sm
    .end()
}
