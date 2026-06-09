pub fn page_token() -> impl IntoToken {
    let user = &crate::data::app_data::seed_users()[0];

    col()
        .id("instagram_edit_page")
        .css("min-h-screen bg-black text-white pb-20")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between")
            .child(btn("← Back").variant("ghost").size_str("sm").act(navigate("instagram")))
            .child(text("Edit Profile").css("text-lg font-bold"))
            .child(block().css("w-12"))
        )
        .child(col().css("px-4 py-6 gap-6")
            // Avatar preview
            .child(col().css("items-center gap-2")
                .child(img_block(user.avatar_url.as_str())
                    .css("w-24 h-24 rounded-full border-2 border-white"))
                .child(btn("Change Photo").variant("ghost").size_str("sm").css("text-blue-400"))
            )
            // Name
            .child(col().css("gap-1")
                .child(text("Name").css("text-sm text-gray-400"))
                .child(block().css("bg-gray-900 border border-gray-700 rounded-lg p-3")
                    .act(in_("ig_name_input")))
            )
            // Username
            .child(col().css("gap-1")
                .child(text("Username").css("text-sm text-gray-400"))
                .child(block().css("bg-gray-900 border border-gray-700 rounded-lg p-3")
                    .act(in_("ig_handle_input")))
            )
            // Bio
            .child(col().css("gap-1")
                .child(text("Bio").css("text-sm text-gray-400"))
                .child(block().css("bg-gray-900 border border-gray-700 rounded-lg p-3 min-h-[80px]")
                    .act(in_("ig_bio_input")))
            )
            // Save button
            .child(btn("Save").variant("primary").size_str("md").css("w-full rounded-lg")
                .act(chain(vec![
                    store_from_val("ig_name", "ig_name_input"),
                    store_from_val("ig_handle", "ig_handle_input"),
                    store_from_val("ig_bio", "ig_bio_input"),
                    navigate("instagram"),
                ]))
            )
        )
        // Bottom nav
        .child(row()
            .css("fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 py-3 px-6 justify-between z-50")
            .child(btn("🏠").variant("ghost").act(navigate("instagram")).css("text-xl"))
            .child(btn("🔍").variant("ghost").css("text-xl"))
            .child(btn("➕").variant("ghost").act(navigate("instagram_create")).css("text-xl"))
            .child(btn("♡").variant("ghost").css("text-xl"))
            .child(img_block(user.avatar_url.as_str()).css("w-6 h-6 rounded-full"))
        )
}
