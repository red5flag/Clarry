use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_messages;

pub fn page_token() -> impl IntoToken {
    let messages = seed_messages();

    col()
        .id("chat_page")
        .css("min-h-screen bg-gray-900 text-white")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-gray-800 px-4 py-3 items-center gap-3 border-b border-gray-700")
            .child(text("←").css("text-xl"))
            .child(img_block("https://i.pravatar.cc/150?u=alice")
                .css("w-8 h-8 rounded-full")
            )
            .child(col().css("flex-1")
                .child(text("Alice Chen").css("font-bold text-sm"))
                .child(text("Online").css("text-xs text-green-400"))
            )
            .child(text("📞").css("text-xl"))
            .child(text("📹").css("text-xl"))
        )
        // Messages
        .child(col().css("flex-1 px-4 py-4 gap-3")
            .children(messages.iter().map(|m| {
                if m.is_me {
                    row().css("justify-end")
                        .child(block()
                            .css("bg-blue-600 text-white rounded-2xl rounded-tr-sm px-4 py-2 max-w-[75%]")
                            .child(text(m.text.as_str()).css("text-sm"))
                            .child(text(m.timestamp.as_str()).css("text-xs text-blue-200 mt-1 text-right"))
                        )
                } else {
                    row().css("justify-start gap-2")
                        .child(img_block(m.sender_avatar.as_str())
                            .css("w-6 h-6 rounded-full self-end")
                        )
                        .child(block()
                            .css("bg-gray-700 text-white rounded-2xl rounded-tl-sm px-4 py-2 max-w-[75%]")
                            .child(text(m.text.as_str()).css("text-sm"))
                            .child(text(m.timestamp.as_str()).css("text-xs text-gray-400 mt-1 text-right"))
                        )
                }
            }))
        )
        // Input
        .child(row()
            .css("sticky bottom-0 bg-gray-800 px-4 py-3 gap-3 items-center border-t border-gray-700")
            .child(text("➕").css("text-xl text-gray-400"))
            .child(block()
                .css("flex-1 bg-gray-700 rounded-full px-4 py-2")
                .child(text("Message...").css("text-gray-400 text-sm"))
            )
            .child(text("🎤").css("text-xl text-gray-400"))
            .child(text("➤").css("text-xl text-blue-500"))
        )
}
