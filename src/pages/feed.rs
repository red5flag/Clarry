use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_feed;

pub fn page_token() -> impl IntoToken {
    let items = seed_feed();

    col()
        .id("feed_page")
        .css("min-h-screen bg-white")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-white border-b px-4 py-3 items-center justify-between")
            .child(text("Feed").css("text-xl font-bold"))
            .child(text("🔍").css("text-xl"))
        )
        // Feed items
        .child(col().css("divide-y divide-gray-100")
            .children(items.iter().map(|item| {
                row()
                    .css("px-4 py-4 gap-4")
                    .child(col().css("flex-1 gap-2")
                        .child(row().css("gap-2 items-center")
                            .child(text(item.source.as_str()).css("text-xs font-semibold text-blue-600 uppercase"))
                            .child(text("·").css("text-xs text-gray-400"))
                            .child(text(item.timestamp.as_str()).css("text-xs text-gray-400"))
                        )
                        .child(text(item.title.as_str()).css("text-base font-bold leading-snug"))
                        .child(text(item.summary.as_str()).css("text-sm text-gray-600 line-clamp-2"))
                        .child(row().css("items-center gap-2 mt-1")
                            .child(text("⏱").css("text-xs"))
                            .child(text(format!("{} min read", item.read_time)).css("text-xs text-gray-400"))
                        )
                    )
                    .child_opt(item.image_url.as_ref().map(|url| {
                        img_block(url.as_str()).css("w-24 h-24 rounded-lg object-cover flex-shrink-0")
                    }))
            }))
        )
        // Bottom nav
        .child(row()
            .css("fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between")
            .child(text("🏠").css("text-xl"))
            .child(text("🔍").css("text-xl"))
            .child(text("📑").css("text-xl text-blue-600"))
            .child(text("👤").css("text-xl"))
        )
}
