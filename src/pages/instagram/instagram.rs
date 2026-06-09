pub fn page_token() -> impl IntoToken {
    let user = &crate::data::app_data::seed_users()[0];
    let posts = crate::data::app_data::seed_posts();

    col()
        .id("instagram_page")
        .css("min-h-screen bg-black text-white pb-20")
        // Header
        row()
            .css("sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between")
            text("Instagram").css("text-xl font-bold tracking-tight")
            row().css("gap-4")
                text("♡").css("text-xl")
                text("✉").css("text-xl")
            .end()
        .end()
        // Profile header
        row()
            .css("px-4 py-6 gap-4 sm:gap-6 items-center")
            img_block(user.avatar_url.as_str())
                .css("w-16 h-16 sm:w-20 sm:h-20 rounded-full border-2 border-white flex-shrink-0")
            col().css("flex-1 min-w-0")
                text_bind("ig_name").css("text-lg font-bold truncate")
                text_bind("ig_handle").css("text-sm text-gray-400 truncate")
                row().css("gap-2 sm:gap-4 mt-1")
                    text(format!("{} posts", user.posts_count)).css("text-xs sm:text-sm")
                    text(format!("{} followers", user.followers)).css("text-xs sm:text-sm")
                    text(format!("{} following", user.following)).css("text-xs sm:text-sm")
                .end()
            .end()
        .end()
        text_bind("ig_bio").css("px-4 text-sm text-gray-300")
        // Action buttons
        row().css("px-4 py-3 gap-2")
            btn("Edit Profile").variant("ghost").size_str("sm").css("flex-1 border border-gray-700 rounded-lg")
                .on_click_nav("instagram_edit")
            btn("Create").variant("ghost").size_str("sm").css("flex-1 border border-gray-700 rounded-lg")
                .on_click_nav("instagram_create")
        .end()
        // Stories
        row()
            .css("px-4 py-4 gap-4 overflow-x-auto scrollbar-none")
            story_ring("Your Story", "https://i.pravatar.cc/150?u=me")
            story_ring("Alice", "https://i.pravatar.cc/150?u=alice")
            story_ring("Bob", "https://i.pravatar.cc/150?u=bob")
        .end()
        // Posts grid - fixed: added grid-cols-3 and rounded corners
        grid(3).css("grid-cols-3 gap-1 px-0")
            .children(posts.iter().map(|p| {
                block()
                    .css("aspect-square bg-gray-800 relative overflow-hidden rounded-sm")
                    .child(img_block(p.image_url.as_deref().unwrap_or(""))
                        .css("w-full h-full object-cover")
                    )
            }))
        .end()
        // Bottom nav
        row()
            .css("fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 py-3 px-6 justify-between z-50")
            btn("🏠").variant("ghost").on_click_nav("instagram").css("text-xl")
            btn("🔍").variant("ghost").css("text-xl")
            btn("➕").variant("ghost").on_click_nav("instagram_create").css("text-xl")
            btn("♡").variant("ghost").css("text-xl")
            img_block(user.avatar_url.as_str()).css("w-6 h-6 rounded-full")
        .end()
}

fn story_ring(_name: &str, img: &str) -> impl IntoToken {
    col().css("items-center gap-1 min-w-[72px]")
        img_block(img)
            .css("w-14 h-14 sm:w-16 sm:h-16 rounded-full border-2 border-pink-500 p-0.5")
        text(_name).css("text-xs text-gray-300")
    .end()
}
