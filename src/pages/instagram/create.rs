pub fn page_token() -> impl IntoToken {
    let user = &crate::data::app_data::seed_users()[0];

    col()
        .id("instagram_create_page")
        .css("min-h-screen bg-black text-white pb-20")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between")
            .child(btn("← Back").variant("ghost").size_str("sm").act(navigate("instagram")))
            .child(text("New Post").css("text-lg font-bold"))
            .child(block().css("w-12"))
        )
        .child(col().css("px-4 py-6 gap-6")
            // Media upload placeholder
            .child(block().css("aspect-square bg-gray-900 border-2 border-dashed border-gray-700 rounded-xl flex items-center justify-center")
                .child(col().css("items-center gap-2")
                    .child(text("📷").css("text-4xl"))
                    .child(text("Tap to upload photo").css("text-sm text-gray-400"))
                )
            )
            // Caption
            .child(col().css("gap-1")
                .child(text("Caption").css("text-sm text-gray-400"))
                .child(block().css("bg-gray-900 border border-gray-700 rounded-lg p-3 min-h-[80px]")
                    .act(in_("ig_caption_input")))
            )
            // Story toggle
            .child(row().css("items-center justify-between py-2")
                .child(text("Share to Story").css("text-sm"))
                .child(block().css("w-12 h-6 bg-gray-700 rounded-full relative")
                    .child(block().css("absolute left-1 top-1 w-4 h-4 bg-white rounded-full")))
            )
            // Post button
            .child(btn("Share").variant("primary").size_str("md").css("w-full rounded-lg")
                .act(chain(vec![
                    store_from_val("ig_last_caption", "ig_caption_input"),
                    navigate("instagram"),
                ]))
            )
        )
        // Bottom nav
        .child(row()
            .css("fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 py-3 px-6 justify-between z-50")
            .child(btn("🏠").variant("ghost").act(navigate("instagram")).css("text-xl"))
            .child(btn("🔍").variant("ghost").css("text-xl"))
            .child(btn("➕").variant("ghost").act(navigate("instagram_create")).css("text-xl text-blue-400"))
            .child(btn("♡").variant("ghost").css("text-xl"))
            .child(img_block(user.avatar_url.as_str()).css("w-6 h-6 rounded-full"))
        )
}
