use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_saved
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("profile")

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center gap-3
            btn "←"
                var ghost
                css text-xl text-white
                act nav instagram
            txt "Saved"
                css text-lg font-semibold flex-1
            btn "⋯"
                var ghost
                css text-xl

        row
            css px-4 py-3 items-center justify-between border-b border-gray-800
            row
                css items-center gap-2
                block
                    css w-10 h-10 rounded-lg bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center
                    txt "🔖"
                        css text-lg
                col
                    css gap-0.5
                    txt "All Posts"
                        css text-sm font-semibold
                    txt "6 posts"
                        css text-xs text-gray-400
            btn "⋮"
                var ghost
                css text-xl text-gray-400

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

        row
            css fixed bottom-0 left-0 right-0 bg-gray-900 backdrop-blur border-t border-gray-800 py-2 px-4 justify-around items-center z-50 max-w-lg mx-auto lg:max-w-6xl lg:hidden
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
                act nav instagram_profile
            btn "♡"
                var ghost
                css text-2xl opacity-60
                act nav instagram_notifications
            btn "👤"
                var ghost
                css text-2xl
                act nav instagram
