crate::data::app_data::ensure_ig_seeded();

col
        id instagram_profile_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("profile")

        block
            id ig_reels_precache
            css fixed -top-1 -left-1 w-px h-px overflow-hidden opacity-0 pointer-events-none z-0
            img_block "https://picsum.photos/600/900?random=11"
            img_block "https://picsum.photos/600/900?random=22"
            img_block "https://picsum.photos/600/900?random=33"
            img_block "https://picsum.photos/600/900?random=44"

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 lg:px-6 py-3 items-center justify-between
            btn "←"
                var ghost
                css text-xl lg:text-2xl
                act nav instagram_home
            txtbnd ig/viewing/user_id
                css text-base lg:text-lg font-bold
            btn "⋯"
                var ghost
                css text-xl lg:text-2xl text-gray-500

        col
            css px-4 lg:px-6 pt-5 pb-2 gap-4
            row
                css gap-5 lg:gap-8 items-center
                block
                    css w-20 h-20 lg:w-28 lg:h-28 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0
                    img_block "https://i.pravatar.cc/150?u=bob"
                        css w-full h-full rounded-full border-2 border-black object-cover
                row
                    css flex-1 justify-around text-center
                    col
                        css items-center gap-0.5
                        txt "12"
                            css text-base lg:text-lg font-bold
                        txt "posts"
                            css text-xs text-gray-400
                    col
                        css items-center gap-0.5
                        txt "8.4K"
                            css text-base lg:text-lg font-bold
                        txt "followers"
                            css text-xs text-gray-400
                    col
                        css items-center gap-0.5
                        txt "512"
                            css text-base lg:text-lg font-bold
                        txt "following"
                            css text-xs text-gray-400

            col
                css gap-0.5
                txt "Bob Smith"
                    css text-sm lg:text-base font-semibold
                txt "@bob"
                    css text-xs text-gray-400
                txt "Photographer • Harbor enthusiast 🌅"
                    css text-sm text-gray-300
                txt "harborphotos.com"
                    css text-sm text-blue-400

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
                    css border border-gray-600 rounded-lg px-2 lg:px-3

        row
            css px-4 lg:px-6 py-3 gap-4 lg:gap-6 overflow-x-auto scrollbar-none border-b border-gray-800
            ig_story_circle("https://i.pravatar.cc/150?u=bob", "Story")

        row
            css border-b border-gray-800
            block
                css flex-1 flex flex-col items-center py-3 cursor-pointer border-t-2 border-transparent
                id ig_posts_tab
                act show_hiding ig_posts_panel, vec![ig_reels_panel, ig_tagged_panel]
                txt "▦"
                    css text-xl lg:text-2xl
            block
                css flex-1 flex flex-col items-center py-3 cursor-pointer border-t-2 border-transparent
                id ig_reels_tab
                act show_hiding ig_reels_panel, vec![ig_posts_panel, ig_tagged_panel]
                txt "▶"
                    css text-xl lg:text-2xl
            block
                css flex-1 flex flex-col items-center py-3 cursor-pointer border-t-2 border-transparent
                id ig_tagged_tab
                act show_hiding ig_tagged_panel, vec![ig_posts_panel, ig_reels_panel]
                txt "👤"
                    css text-xl lg:text-2xl

        block
            id ig_posts_panel
            grid 3
                css grid-cols-3 gap-1 lg:gap-2
                ig_post_thumb("p_bob_1", "https://picsum.photos/600/600?random=10")
                ig_post_thumb("p_bob_2", "https://picsum.photos/600/600?random=15")
                ig_post_thumb("p_bob_3", "https://picsum.photos/600/600?random=16")

        block
            id ig_reels_panel
            css hidden
            grid 3
                css grid-cols-3 gap-1 lg:gap-2
                ig_reel_card("https://picsum.photos/600/900?random=11", "Harbor sunset 🌅", "12.5K views")
                ig_reel_card("https://picsum.photos/600/900?random=22", "Morning coffee ☕", "8.2K views")
                ig_reel_card("https://picsum.photos/600/900?random=33", "City lights 🌃", "6.7K views")
                ig_reel_card("https://picsum.photos/600/900?random=44", "Weekend hike 🥾", "4.1K views")

        block
            id ig_tagged_panel
            css hidden
            grid 3
                css grid-cols-3 gap-1 lg:gap-2
                ig_post_thumb("p_diana_1", "https://picsum.photos/600/600?random=20")
                ig_post_thumb("p_charlie_1", "https://picsum.photos/600/600?random=30")
                ig_post_thumb("p_eve_1", "https://picsum.photos/600/600?random=40")

        ig_bottom_nav("profile")
