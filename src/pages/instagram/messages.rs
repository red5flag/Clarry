use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_messages_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            btn "← Back"
                var ghost
                sz sm
                act nav instagram_home
            txtbnd ig/me/name
                css text-base font-semibold
            btn "✏"
                var ghost
                css text-xl

        row
            css px-4 py-3 gap-4 overflow-x-auto scrollbar-none border-b border-gray-800
            col
                css items-center gap-1 min-w-[64px]
                img_block "https://i.pravatar.cc/150?u=me"
                    css w-14 h-14 rounded-full object-cover border border-gray-700
                txt "Your note"
                    css text-xs text-gray-400 truncate text-center
            col
                css items-center gap-1 min-w-[64px]
                img_block "https://i.pravatar.cc/150?u=bob"
                    css w-14 h-14 rounded-full object-cover border border-gray-700
                txt "Bob"
                    css text-xs text-gray-400 truncate text-center
            col
                css items-center gap-1 min-w-[64px]
                img_block "https://i.pravatar.cc/150?u=diana"
                    css w-14 h-14 rounded-full object-cover border border-gray-700
                txt "Diana"
                    css text-xs text-gray-400 truncate text-center
            col
                css items-center gap-1 min-w-[64px]
                img_block "https://i.pravatar.cc/150?u=charlie"
                    css w-14 h-14 rounded-full object-cover border border-gray-700
                txt "Charlie"
                    css text-xs text-gray-400 truncate text-center

        row
            css px-4 pt-4 pb-2 justify-between items-center
            txt "Messages"
                css text-sm font-semibold
            txt "Requests"
                css text-sm text-gray-400

        row
            css px-4 py-3 gap-3 items-center cursor-pointer active:bg-gray-900
            act store_set ig/viewing/dm_id, dm_bob
            act store_set ig/viewing/user_id, bob
            act nav instagram_messages_detail
            block
                css relative flex-shrink-0
                block
                    css w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://i.pravatar.cc/150?u=bob"
                        css w-full h-full rounded-full border-2 border-black object-cover
                block
                    css absolute bottom-0.5 right-0.5 w-3.5 h-3.5 rounded-full bg-green-500 border-2 border-black
            col
                css flex-1 min-w-0 gap-0.5
                txt "Bob Smith"
                    css text-sm font-semibold
                row
                    css gap-1 items-center
                    txt "Golden hour was amazing!"
                        css text-xs text-gray-500 truncate max-w-[200px]
                    txt "·"
                        css text-gray-600 text-xs
                    txt "2h"
                        css text-xs text-gray-500 flex-shrink-0
            txt "📷"
                css text-xl text-gray-600

        row
            css px-4 py-3 gap-3 items-center cursor-pointer active:bg-gray-900
            act store_set ig/viewing/dm_id, dm_diana
            act store_set ig/viewing/user_id, diana
            act nav instagram_messages_detail
            block
                css relative flex-shrink-0
                block
                    css w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://i.pravatar.cc/150?u=diana"
                        css w-full h-full rounded-full border-2 border-black object-cover
                block
                    css absolute bottom-0.5 right-0.5 w-3.5 h-3.5 rounded-full bg-green-500 border-2 border-black
            col
                css flex-1 min-w-0 gap-0.5
                txt "Diana Prince"
                    css text-sm font-semibold
                row
                    css gap-1 items-center
                    txt "Love that new post!"
                        css text-xs font-semibold text-white truncate max-w-[200px]
                    txt "·"
                        css text-gray-600 text-xs
                    txt "5h"
                        css text-xs text-gray-500 flex-shrink-0
            block
                css w-5 h-5 rounded-full bg-blue-500 flex items-center justify-center flex-shrink-0
                txt "3"
                    css text-white text-xs font-bold

        row
            css px-4 py-3 gap-3 items-center cursor-pointer active:bg-gray-900
            act store_set ig/viewing/dm_id, dm_charlie
            act store_set ig/viewing/user_id, charlie
            act nav instagram_messages_detail
            block
                css relative flex-shrink-0
                block
                    css w-14 h-14 rounded-full p-0.5 bg-gray-700
                    img_block "https://i.pravatar.cc/150?u=charlie"
                        css w-full h-full rounded-full border-2 border-black object-cover
            col
                css flex-1 min-w-0 gap-0.5
                txt "Charlie Day"
                    css text-sm text-gray-200
                row
                    css gap-1 items-center
                    txt "That mountain trail was wild 🏔️"
                        css text-xs text-gray-500 truncate max-w-[200px]
                    txt "·"
                        css text-gray-600 text-xs
                    txt "1d"
                        css text-xs text-gray-500 flex-shrink-0
            txt "📷"
                css text-xl text-gray-600

        row
            css fixed bottom-0 left-0 right-0 bg-black/95 backdrop-blur border-t border-gray-800 py-2 px-4 justify-around items-center z-50 max-w-lg mx-auto
            btn "🏠"
                var ghost
                css text-2xl opacity-60
                act nav instagram_home
            btn "🔍"
                var ghost
                css text-2xl opacity-60
                act nav instagram_explore
            btn "➕"
                var ghost
                css text-2xl opacity-60
                act nav instagram_create
            btn "🎬"
                var ghost
                css text-2xl opacity-60
                act nav instagram_reels
            btn "♡"
                var ghost
                css text-2xl opacity-60
                act nav instagram_notifications
            btn "👤"
                var ghost
                css text-2xl opacity-60
                act nav instagram
