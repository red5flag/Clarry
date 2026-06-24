crate::data::app_data::ensure_ig_seeded();

col
        id instagram_explore_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("explore")

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
                ig_suggest_card("bob", "Bob Smith", "@bob", "https://i.pravatar.cc/150?u=bob")
                ig_suggest_card("diana", "Diana Prince", "@diana", "https://i.pravatar.cc/150?u=diana")
                ig_suggest_card("charlie", "Charlie Day", "@charlie", "https://i.pravatar.cc/150?u=charlie")

        block
            css border-t border-gray-800 mx-4 my-1
        txt "Discover"
            css px-4 pt-3 pb-2 text-sm font-semibold

        grid 3
            css grid-cols-3 gap-0.5
            ig_post_thumb("p_bob_1", "https://picsum.photos/600/600?random=10")
            ig_post_thumb("p_diana_1", "https://picsum.photos/600/600?random=20")
            ig_post_thumb("p_charlie_1", "https://picsum.photos/600/600?random=30")
            ig_post_thumb("p_eve_1", "https://picsum.photos/600/600?random=40")
            ig_post_thumb("p_henry_1", "https://picsum.photos/600/600?random=50")
            ig_post_thumb("p_bob_1", "https://picsum.photos/600/600?random=60")
            ig_post_thumb("p_diana_1", "https://picsum.photos/600/600?random=70")
            ig_post_thumb("p_charlie_1", "https://picsum.photos/600/600?random=80")
            ig_post_thumb("p_eve_1", "https://picsum.photos/600/600?random=90")

        ig_bottom_nav("explore")
