use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_post_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            btn "← Back"
                var ghost
                sz sm
                act nav instagram_home
            txt "Post"
                css text-base font-semibold
            btn "•••"
                var ghost
                css text-lg

        row
            css px-4 py-3 gap-3 items-center
            block
                css w-9 h-9 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0 cursor-pointer
                act nav instagram_profile
                img_block "https://i.pravatar.cc/150?u=bob"
                    css w-full h-full rounded-full border-2 border-black object-cover
            col
                css flex-1 gap-0
                txtbnd ig/viewing/post_author_name
                    css text-sm font-semibold
                txtbnd ig/viewing/post_author_handle
                    css text-xs text-gray-500
            btn "Follow"
                var ghost
                sz sm
                css text-blue-400 text-sm font-semibold px-0

        block
            css w-full bg-gray-900
            img_block "https://picsum.photos/600/600?random=10"
                css w-full aspect-square object-cover block

        row
            css px-4 pt-3 pb-1 gap-4 items-center
            btn "♡"
                var ghost
                css text-2xl p-0
                act tog ig/liked/p_bob_1
            btn "💬"
                var ghost
                css text-2xl p-0
            btn "✈"
                var ghost
                css text-2xl p-0
            block
                css flex-1
            btn "🔖"
                var ghost
                css text-2xl p-0
                act tog ig/saved/p_bob_1

        col
            css px-4 pb-3 gap-0.5
            txt "2,401 likes"
                css text-sm font-semibold
            row
                css gap-1.5 flex-wrap items-baseline
                txt "Bob Smith"
                    css text-sm font-semibold
                txt "Golden hour at the harbor 🌅"
                    css text-sm text-gray-200
            txt "2h ago"
                css text-xs text-gray-500 uppercase tracking-wide mt-0.5

        block
            css border-t border-gray-800 mx-4 mt-2
        txt "Comments"
            css px-4 pt-3 pb-2 text-sm font-semibold text-gray-400

        row
            css px-4 py-2 gap-3 items-start
            img_block "https://i.pravatar.cc/150?u=diana"
                css w-8 h-8 rounded-full object-cover flex-shrink-0 mt-0.5
            col
                css flex-1 gap-0.5
                row
                    css gap-1.5 flex-wrap items-baseline
                    txt "Diana Prince"
                        css text-sm font-semibold
                    txt "Absolutely stunning shot! 😍"
                        css text-sm text-gray-200
                row
                    css gap-4 mt-0.5 items-center
                    txt "1h"
                        css text-xs text-gray-500
                    txt "12 likes"
                        css text-xs text-gray-500
                    btn "Reply"
                        var ghost
                        sz sm
                        css text-xs text-gray-500 px-0

        row
            css px-4 py-2 gap-3 items-start
            img_block "https://i.pravatar.cc/150?u=charlie"
                css w-8 h-8 rounded-full object-cover flex-shrink-0 mt-0.5
            col
                css flex-1 gap-0.5
                row
                    css gap-1.5 flex-wrap items-baseline
                    txt "Charlie Day"
                        css text-sm font-semibold
                    txt "The lighting is perfect here 📸"
                        css text-sm text-gray-200
                row
                    css gap-4 mt-0.5 items-center
                    txt "45m"
                        css text-xs text-gray-500
                    txt "7 likes"
                        css text-xs text-gray-500
                    btn "Reply"
                        var ghost
                        sz sm
                        css text-xs text-gray-500 px-0

        col
            css px-4 pt-3 pb-2 gap-1
            txt "Add new comments:"
                css text-xs text-gray-500
            text_read ig/comments/p_bob_1
                css font-mono text-xs bg-gray-900 p-2 rounded break-all max-h-24 overflow-auto text-gray-300

        row
            css sticky bottom-16 bg-black border-t border-gray-800 px-4 py-2.5 gap-3 items-center z-40
            img_block "https://i.pravatar.cc/150?u=me"
                css w-8 h-8 rounded-full object-cover flex-shrink-0
            txtinp "Add a comment...", ig/comment_input
                css flex-1 bg-gray-900 rounded-full px-4 py-2 border border-gray-700 text-sm text-gray-100 focus:outline-none focus:border-gray-500
            btn "Post"
                var ghost
                sz sm
                css text-blue-400 font-semibold text-sm
                act store_push ig/comments/p_bob_1, ig/comment_input

        row
            css fixed bottom-0 left-0 right-0 bg-black/95 backdrop-blur border-t border-gray-800 py-2 px-4 justify-around items-center z-50 max-w-lg mx-auto
            btn "🏠"
                var ghost
                css text-2xl
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
