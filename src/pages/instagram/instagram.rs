use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("profile")

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 lg:px-6 py-3 items-center justify-between
            text_read ig/me/name
                css text-base lg:text-lg font-bold
            row
                css gap-4 lg:gap-6 items-center
                btn "✉"
                    var ghost
                    css text-xl lg:text-2xl
                    act nav instagram_messages
                btn "☰"
                    var ghost
                    css text-xl lg:text-2xl

        col
            css px-4 lg:px-6 pt-5 pb-2 gap-4
            row
                css gap-5 lg:gap-8 items-center
                block
                    css w-20 h-20 lg:w-28 lg:h-28 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0
                    img_bind ig/me/avatar, "https://i.pravatar.cc/150?u=alice"
                    css w-full h-full rounded-full border-2 border-black object-cover
                row
                    css flex-1 justify-around text-center
                    col
                        css items-center gap-0.5
                        text_read ig/me/posts_count
                            css text-base lg:text-lg font-bold
                        txt "posts"
                            css text-xs text-gray-400
                    col
                        css items-center gap-0.5
                        text_read ig/me/followers
                            css text-base lg:text-lg font-bold
                        txt "followers"
                            css text-xs text-gray-400
                    col
                        css items-center gap-0.5
                        text_read ig/me/following
                            css text-base lg:text-lg font-bold
                        txt "following"
                            css text-xs text-gray-400

            col
                css gap-0.5
                text_read ig/me/name
                    css text-sm lg:text-base font-semibold
                text_read ig/me/handle
                    css text-xs text-gray-400
                text_read ig/me/bio
                    css text-sm text-gray-300 mt-1
                text_read ig/me/website
                    css text-xs text-blue-400 font-medium

            row
                css gap-2
                btn "Edit profile"
                    var ghost
                    sz sm
                    css flex-1 border border-gray-600 rounded-lg text-sm font-medium
                    act show ig_edit_modal
                btn "Share profile"
                    var ghost
                    sz sm
                    css flex-1 border border-gray-600 rounded-lg text-sm font-medium
                btn "Add tools"
                    var ghost
                    sz sm
                    css flex-1 border border-gray-600 rounded-lg text-sm font-medium

            btn "Professional dashboard"
                var ghost
                sz sm
                css w-full border border-gray-600 rounded-lg text-sm font-medium text-left flex items-center justify-between

        row
            css px-4 lg:px-6 py-3 gap-4 lg:gap-6 overflow-x-auto scrollbar-none border-b border-gray-800
            col
                css items-center gap-1 min-w-[64px]
                block
                    css w-14 h-14 lg:w-16 lg:h-16 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://picsum.photos/60/60?random=10"
                        css w-full h-full rounded-full border-2 border-black object-cover
                txt "DSL"
                    css text-xs text-gray-300
            col
                css items-center gap-1 min-w-[64px]
                block
                    css w-14 h-14 lg:w-16 lg:h-16 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://picsum.photos/60/60?random=13"
                        css w-full h-full rounded-full border-2 border-black object-cover
                txt "Work"
                    css text-xs text-gray-300
            col
                css items-center gap-1 min-w-[64px]
                block
                    css w-14 h-14 lg:w-16 lg:h-16 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://picsum.photos/60/60?random=14"
                        css w-full h-full rounded-full border-2 border-black object-cover
                txt "Travel"
                    css text-xs text-gray-300
            col
                css items-center gap-1 min-w-[64px]
                block
                    css w-14 h-14 lg:w-16 lg:h-16 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://picsum.photos/60/60?random=12"
                        css w-full h-full rounded-full border-2 border-black object-cover
                txt "Setup"
                    css text-xs text-gray-300

        row
            css border-b border-gray-800
            block
                css flex-1 flex flex-col items-center py-3 cursor-pointer border-t-2 border-transparent
                id ig_own_posts_tab
                act show_hiding ig_own_posts_panel, vec![ig_own_reels_panel, ig_own_tagged_panel]
                txt "▦"
                    css text-xl lg:text-2xl
            block
                css flex-1 flex flex-col items-center py-3 cursor-pointer border-t-2 border-transparent
                id ig_own_reels_tab
                act show_hiding ig_own_reels_panel, vec![ig_own_posts_panel, ig_own_tagged_panel]
                txt "▶"
                    css text-xl lg:text-2xl
            block
                css flex-1 flex flex-col items-center py-3 cursor-pointer border-t-2 border-transparent
                id ig_own_tagged_tab
                act show_hiding ig_own_tagged_panel, vec![ig_own_posts_panel, ig_own_reels_panel]
                txt "👤"
                    css text-xl lg:text-2xl

        block
            id ig_own_posts_panel
            grid 3
                css grid-cols-3 gap-1 lg:gap-2
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_1
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=10"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_2
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=11"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_3
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=12"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_4
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=13"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_5
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=14"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_reel_1
                    act nav instagram_post
                    img_block "https://picsum.photos/400/700?random=15"
                        css w-full h-full object-cover block
                    txt "▶"
                        css absolute top-2 right-2 text-xs text-white drop-shadow

        block
            id ig_own_reels_panel
            css hidden
            grid 3
                css grid-cols-3 gap-1 lg:gap-2
                block
                    css h-[320px] lg:h-[520px] bg-black relative overflow-hidden cursor-pointer
                    act store_set ig/viewing/post_id, p_alice_reel_1
                    act nav instagram_post
                    img_block "https://picsum.photos/400/700?random=15"
                        css absolute inset-0 w-full h-full object-cover
                    block
                        css absolute bottom-0 left-0 right-0 h-20 bg-gradient-to-t from-black to-transparent pointer-events-none
                    row
                        css absolute bottom-2 left-2 right-2 items-end justify-between
                        col
                            css gap-0.5
                            txt "Live coding a full Instagram UI"
                                css text-xs lg:text-sm font-semibold text-white
                            txt "42K views"
                                css text-xs text-white
                        btn "▶"
                            var ghost
                            css text-lg lg:text-xl text-white

        block
            id ig_own_tagged_panel
            css hidden
            grid 3
                css grid-cols-3 gap-1 lg:gap-2
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_grace_1
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=20"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_grace_3
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=21"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_eve_1
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=22"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_eve_4
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=23"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_henry_1
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=24"
                        css w-full h-full object-cover block
                block
                    css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                    act store_set ig/viewing/post_id, p_henry_4
                    act nav instagram_post
                    img_block "https://picsum.photos/600/600?random=25"
                        css w-full h-full object-cover block


        block
            id ig_edit_modal
            css hidden fixed inset-0 z-50 bg-black/70 flex items-center justify-center px-4
            col
                css w-full max-w-md bg-gray-900 border border-gray-800 rounded-2xl p-5 gap-5
                row
                    css items-center justify-between pb-2 border-b border-gray-800
                    txt "Edit Profile"
                        css text-lg font-bold
                    btn "✕"
                        var ghost
                        css text-xl text-gray-400
                        act hide ig_edit_modal

                col
                    css items-center gap-3
                    block
                        css w-20 h-20 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                        img_bind ig/me/avatar, "https://i.pravatar.cc/150?u=alice"
                            css w-full h-full rounded-full border-2 border-black object-cover
                    txt "Change profile photo"
                        css text-sm font-medium text-blue-400 cursor-pointer

                col
                    css gap-1
                    txt "Name"
                        css text-xs font-medium text-gray-400 uppercase tracking-wide
                    txtinp "Your name", ig_edit_name
                        css w-full bg-gray-950 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500

                col
                    css gap-1
                    txt "Username"
                        css text-xs font-medium text-gray-400 uppercase tracking-wide
                    txtinp "@handle", ig_edit_handle
                        css w-full bg-gray-950 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500

                col
                    css gap-1
                    txt "Bio"
                        css text-xs font-medium text-gray-400 uppercase tracking-wide
                    txtinp "Write a bio...", ig_edit_bio
                        css w-full bg-gray-950 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500 min-h-[80px]

                col
                    css gap-1
                    txt "Avatar URL"
                        css text-xs font-medium text-gray-400 uppercase tracking-wide
                    txtinp "https://i.pravatar.cc/150?u=alice", ig_edit_avatar
                        css w-full bg-gray-950 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-400 focus:outline-none focus:border-blue-500 break-all

                row
                    css gap-2
                    btn "Save"
                        var primary
                        sz md
                        css flex-1 rounded-xl py-2.5 text-sm font-semibold
                        act store_set_input ig/me/name, ig_edit_name
                        act store_set_input ig/me/handle, ig_edit_handle
                        act store_set_input ig/me/bio, ig_edit_bio
                        act store_set_input ig/me/avatar, ig_edit_avatar
                        act hide ig_edit_modal
                    btn "Cancel"
                        var ghost
                        sz md
                        css flex-1 rounded-xl py-2.5 text-sm font-semibold border border-gray-700
                        act hide ig_edit_modal

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
