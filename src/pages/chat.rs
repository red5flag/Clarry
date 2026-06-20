use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_messages;

let messages = seed_messages();

col
        id chat_page
        css min-h-screen bg-gray-900 text-white
        row
            css sticky top-0 z-50 bg-gray-800 px-4 py-3 items-center gap-3 border-b border-gray-700
            txt "←"
                css text-xl
            img_block "https://i.pravatar.cc/150?u=alice"
                css w-8 h-8 rounded-full
            col
                css flex-1
                txt "Alice Chen"
                    css font-bold text-sm
                txt "Online"
                    css text-xs text-green-400
            txt "📞"
                css text-xl
            txt "📹"
                css text-xl
        col
            css flex-1 px-4 py-4 gap-3
            .add_all(messages.iter().map(|m| {
                if m.is_me {
                    row().set_class("justify-end")
                        .add(block()
                            .set_class("bg-blue-600 text-white rounded-2xl rounded-tr-sm px-4 py-2 max-w-[75%]")
                            .add(text(m.text.as_str()).set_class("text-sm"))
                            .add(text(m.timestamp.as_str()).set_class("text-xs text-blue-200 mt-1 text-right"))
                        )
                } else {
                    row().set_class("justify-start gap-2")
                        .add(img_block(m.sender_avatar.as_str())
                            .set_class("w-6 h-6 rounded-full self-end")
                        )
                        .add(block()
                            .set_class("bg-gray-700 text-white rounded-2xl rounded-tl-sm px-4 py-2 max-w-[75%]")
                            .add(text(m.text.as_str()).set_class("text-sm"))
                            .add(text(m.timestamp.as_str()).set_class("text-xs text-gray-400 mt-1 text-right"))
                        )
                }
            }))
        row
            css sticky bottom-0 bg-gray-800 px-4 py-3 gap-3 items-center border-t border-gray-700
            txt "➕"
                css text-xl text-gray-400
            block
                css flex-1 bg-gray-700 rounded-full px-4 py-2
                txt "Message..."
                    css text-gray-400 text-sm
            txt "🎤"
                css text-xl text-gray-400
            txt "➤"
                css text-xl text-blue-500
