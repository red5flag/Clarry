crate::data::app_data::ensure_ig_seeded();

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
            ig_post_thumb("p_bob_1", "https://picsum.photos/600/600?random=10")
            ig_post_thumb("p_diana_1", "https://picsum.photos/600/600?random=20")
            ig_post_thumb("p_charlie_1", "https://picsum.photos/600/600?random=30")
            ig_post_thumb("p_eve_1", "https://picsum.photos/600/600?random=40")
            ig_post_thumb("p_henry_1", "https://picsum.photos/600/600?random=50")
            ig_post_thumb("p_bob_1", "https://picsum.photos/600/600?random=60")

        ig_bottom_nav("profile")
