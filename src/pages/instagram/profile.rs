use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_profile_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            btn "← Back"
                var ghost
                sz sm
                act nav instagram_home
            txtbnd ig/viewing/user_id
                css text-base font-bold tracking-tight
            txt "•••"
                css text-gray-500

        col
            css px-4 pt-5 pb-2 gap-4
            row
                css gap-5 items-start
                block
                    css w-20 h-20 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0
                    img_block "https://i.pravatar.cc/150?u=bob"
                        css w-full h-full rounded-full border-2 border-black object-cover
                col
                    css flex-1 gap-2
                    row
                        css justify-around text-center
                        col
                            css items-center gap-0.5
                            txt "12"
                                css text-base font-bold
                            txt "posts"
                                css text-xs text-gray-400
                        col
                            css items-center gap-0.5
                            txt "8.4K"
                                css text-base font-bold
                            txt "followers"
                                css text-xs text-gray-400
                        col
                            css items-center gap-0.5
                            txt "512"
                                css text-base font-bold
                            txt "following"
                                css text-xs text-gray-400
                    row
                        css gap-2
                        btn "Follow"
                            var primary
                            sz sm
                            css flex-1 rounded-lg text-sm font-semibold
                            act tog ig/following/bob
                        btn "Message"
                            var ghost
                            sz sm
                            css flex-1 border border-gray-600 rounded-lg text-sm font-medium
                            act store_set ig/viewing/dm_id, dm_bob
                            act store_set ig/viewing/user_id, bob
                            act nav instagram_messages_detail
                        btn "▾"
                            var ghost
                            sz sm
                            css border border-gray-600 rounded-lg px-2
            col
                css gap-0.5
                txt "Bob Smith"
                    css text-sm font-semibold
                txt "@bob"
                    css text-xs text-gray-400
                txt "Photographer • Harbor enthusiast 🌅"
                    css text-sm text-gray-300 mt-1

        row
            css px-4 py-3 gap-4 overflow-x-auto scrollbar-none border-b border-gray-800
            col
                css items-center gap-1 min-w-[64px]
                block
                    css w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0
                    img_block "https://i.pravatar.cc/150?u=bob"
                        css w-full h-full rounded-full border-2 border-black object-cover
                txt "Story"
                    css text-xs text-gray-300

        row
            css border-b border-gray-800 px-4
            block
                css border-b-2 border-white py-2 px-4
                txt "▦"
                    css text-lg
            block
                css py-2 px-4
                txt "☰"
                    css text-lg text-gray-600
            block
                css py-2 px-4
                txt "🏷"
                    css text-lg text-gray-600

        grid 3
            css grid-cols-3 gap-0.5
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_bob_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=10"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_bob_2
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=15"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_bob_3
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=16"
                    css w-full h-full object-cover block

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
                css text-2xl
                act nav instagram
