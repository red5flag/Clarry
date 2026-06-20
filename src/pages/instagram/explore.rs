use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_explore_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 gap-3 items-center
            row
                css flex-1 bg-gray-900 border border-gray-700 rounded-xl px-3 py-2 gap-2 items-center
                txt "🔍"
                    css text-gray-500 text-base flex-shrink-0
                txtinp "Search", ig/search/query
                    css flex-1 bg-transparent border-none outline-none text-sm text-gray-100 placeholder-gray-500
            btn "Cancel"
                var ghost
                sz sm
                css text-blue-400 text-sm flex-shrink-0
                act store_set ig/search/query, ""

        col
            css px-4 pt-4 pb-2 gap-3
            txt "Suggested for you"
                css text-sm font-semibold
            row
                css gap-3 overflow-x-auto scrollbar-none pb-1
                col
                    css items-center gap-1 min-w-[100px] bg-gray-900 rounded-xl p-3 border border-gray-800
                    block
                        css w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 cursor-pointer
                        act store_set ig/viewing/user_id, bob
                        act nav instagram_profile
                        img_block "https://i.pravatar.cc/150?u=bob"
                            css w-full h-full rounded-full border-2 border-black object-cover
                    txt "Bob Smith"
                        css text-xs font-semibold truncate
                    txt "@bob"
                        css text-xs text-gray-500 truncate
                    btn "Follow"
                        var ghost
                        sz sm
                        css w-full bg-blue-500 text-white rounded-lg text-xs font-medium mt-2
                        act tog ig/following/bob
                col
                    css items-center gap-1 min-w-[100px] bg-gray-900 rounded-xl p-3 border border-gray-800
                    block
                        css w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 cursor-pointer
                        act store_set ig/viewing/user_id, diana
                        act nav instagram_profile
                        img_block "https://i.pravatar.cc/150?u=diana"
                            css w-full h-full rounded-full border-2 border-black object-cover
                    txt "Diana Prince"
                        css text-xs font-semibold truncate
                    txt "@diana"
                        css text-xs text-gray-500 truncate
                    btn "Follow"
                        var ghost
                        sz sm
                        css w-full bg-blue-500 text-white rounded-lg text-xs font-medium mt-2
                        act tog ig/following/diana
                col
                    css items-center gap-1 min-w-[100px] bg-gray-900 rounded-xl p-3 border border-gray-800
                    block
                        css w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 cursor-pointer
                        act store_set ig/viewing/user_id, charlie
                        act nav instagram_profile
                        img_block "https://i.pravatar.cc/150?u=charlie"
                            css w-full h-full rounded-full border-2 border-black object-cover
                    txt "Charlie Day"
                        css text-xs font-semibold truncate
                    txt "@charlie"
                        css text-xs text-gray-500 truncate
                    btn "Follow"
                        var ghost
                        sz sm
                        css w-full bg-blue-500 text-white rounded-lg text-xs font-medium mt-2
                        act tog ig/following/charlie

        block
            css border-t border-gray-800 mx-4 my-1
        txt "Discover"
            css px-4 pt-3 pb-2 text-sm font-semibold

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
                act store_set ig/viewing/post_id, p_diana_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=20"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_charlie_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=30"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_eve_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=40"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_henry_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=50"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_bob_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=60"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_diana_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=70"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_charlie_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=80"
                    css w-full h-full object-cover block
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_eve_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=90"
                    css w-full h-full object-cover block

        row
            css fixed bottom-0 left-0 right-0 bg-black/95 backdrop-blur border-t border-gray-800 py-2 px-4 justify-around items-center z-50 max-w-lg mx-auto
            btn "🏠"
                var ghost
                css text-2xl opacity-60
                act nav instagram_home
            btn "🔍"
                var ghost
                css text-2xl
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
