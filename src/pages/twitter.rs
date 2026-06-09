use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::{seed_users, seed_posts};

pub fn page_token() -> impl IntoToken {
    let user = &seed_users()[0];
    let posts = seed_posts();

    col()
        .id("twitter_page")
        .css("min-h-screen bg-black text-white")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-black/80 backdrop-blur border-b border-gray-800 px-4 py-3 items-center")
            .child(img_block(user.avatar_url.as_str()).css("w-8 h-8 rounded-full mr-4"))
            .child(text("Home").css("text-lg font-bold flex-1"))
            .child(text("⚙").css("text-xl"))
        )
        // Compose
        .child(row()
            .css("px-4 py-3 border-b border-gray-800 gap-3")
            .child(img_block(user.avatar_url.as_str()).css("w-10 h-10 rounded-full"))
            .child(block()
                .css("flex-1 bg-gray-900 rounded-xl px-4 py-2")
                .child(text("What's happening?").css("text-gray-500"))
            )
        )
        // Timeline
        .child(col().css("divide-y divide-gray-800")
            .children(posts.iter().map(|p| {
                tweet_card(p, user)
            }))
        )
        // Bottom nav
        .child(row()
            .css("fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 py-3 px-8 justify-between")
            .child(text("🏠").css("text-xl"))
            .child(text("🔍").css("text-xl"))
            .child(text("🔔").css("text-xl"))
            .child(text("✉").css("text-xl"))
        )
}

fn tweet_card(post: &crate::data::app_data::MediaPost, _me: &crate::data::app_data::UserProfile) -> impl IntoToken {
    row()
        .css("px-4 py-3 gap-3")
        .child(img_block(post.author_avatar.as_str())
            .css("w-10 h-10 rounded-full flex-shrink-0")
        )
        .child(col().css("flex-1 min-w-0")
            .child(row().css("gap-1 items-baseline")
                .child(text(post.author_name.as_str()).css("font-bold text-sm"))
                .child(text(format!("@{} · {}", post.author_id, post.timestamp)).css("text-gray-500 text-sm"))
            )
            .child(text(post.content.as_str()).css("text-sm mt-1"))
            .child_opt(post.image_url.as_ref().map(|url| {
                img_block(url.as_str()).css("w-full rounded-xl mt-2 max-h-64 object-cover")
            }))
            .child(row().css("mt-3 gap-6 text-gray-500")
                .child(text("💬").css("text-sm"))
                .child(text(format!("{}", post.comments)).css("text-sm"))
                .child(text("🔁").css("text-sm"))
                .child(text("♡").css("text-sm"))
                .child(text(format!("{}", post.likes)).css("text-sm"))
                .child(text("📤").css("text-sm"))
            )
        )
}
