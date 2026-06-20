use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_feed;

let items = seed_feed();

col
        id feed_page
        css min-h-screen bg-white
        row
            css sticky top-0 z-50 bg-white border-b px-4 py-3 items-center justify-between
            txt "Feed"
                css text-xl font-bold
            txt "🔍"
                css text-xl
        col
            css divide-y divide-gray-100
            .add_all(items.iter().map(|item| {
                row()
                    .set_class("px-4 py-4 gap-4")
                    .add(col().set_class("flex-1 gap-2")
                        .add(row().set_class("gap-2 items-center")
                            .add(text(item.source.as_str()).set_class("text-xs font-semibold text-blue-600 uppercase"))
                            .add(text("·").set_class("text-xs text-gray-400"))
                            .add(text(item.timestamp.as_str()).set_class("text-xs text-gray-400"))
                        )
                        .add(text(item.title.as_str()).set_class("text-base font-bold leading-snug"))
                        .add(text(item.summary.as_str()).set_class("text-sm text-gray-600 line-clamp-2"))
                        .add(row().set_class("items-center gap-2 mt-1")
                            .add(text("⏱").set_class("text-xs"))
                            .add(text(format!("{} min read", item.read_time)).set_class("text-xs text-gray-400"))
                        )
                    )
                    .add_opt(item.image_url.as_ref().map(|url| {
                        img_block(url.as_str()).set_class("w-24 h-24 rounded-lg object-cover flex-shrink-0")
                    }))
            }))
        row
            css fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between
            txt "🏠"
                css text-xl
            txt "🔍"
                css text-xl
            txt "📑"
                css text-xl text-blue-600
            txt "👤"
                css text-xl
