crate::data::app_data::ensure_ig_seeded();

col
        id instagram_tagged
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("profile")

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center gap-3
            btn "←"
                var ghost
                css text-xl text-white
                act nav instagram
            txt "Photos of You"
                css text-lg font-semibold flex-1
            btn "⋮"
                var ghost
                css text-xl

        row
            css px-4 py-3 gap-3 items-start bg-gray-900/50 border-b border-gray-800
            txt "👤"
                css text-2xl flex-shrink-0 mt-0.5
            col
                css gap-0.5 flex-1
                txt "Posts you're tagged in appear here"
                    css text-sm text-gray-300
                txt "You can remove tags or hide posts from your profile"
                    css text-xs text-gray-500

        grid 3
            css grid-cols-3 gap-0.5
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_bob_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=10"
                    css w-full h-full object-cover block
                row
                    css absolute bottom-1.5 left-1.5 items-center gap-1 bg-black/60 rounded-full pr-2
                    block
                        css w-5 h-5 rounded-full overflow-hidden border border-white/50
                        img_block "https://i.pravatar.cc/150?u=bob"
                            css w-full h-full object-cover
                    txt "Bob"
                        css text-white text-[10px] font-medium truncate max-w-[60px]
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_diana_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=20"
                    css w-full h-full object-cover block
                row
                    css absolute bottom-1.5 left-1.5 items-center gap-1 bg-black/60 rounded-full pr-2
                    block
                        css w-5 h-5 rounded-full overflow-hidden border border-white/50
                        img_block "https://i.pravatar.cc/150?u=diana"
                            css w-full h-full object-cover
                    txt "Diana"
                        css text-white text-[10px] font-medium truncate max-w-[60px]
            block
                css aspect-square bg-gray-900 overflow-hidden relative cursor-pointer
                act store_set ig/viewing/post_id, p_charlie_1
                act nav instagram_post
                img_block "https://picsum.photos/600/600?random=30"
                    css w-full h-full object-cover block
                row
                    css absolute bottom-1.5 left-1.5 items-center gap-1 bg-black/60 rounded-full pr-2
                    block
                        css w-5 h-5 rounded-full overflow-hidden border border-white/50
                        img_block "https://i.pravatar.cc/150?u=charlie"
                            css w-full h-full object-cover
                    txt "Charlie"
                        css text-white text-[10px] font-medium truncate max-w-[60px]

        ig_bottom_nav("profile")
