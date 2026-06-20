use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_reels
        css min-h-screen bg-black text-white max-w-lg mx-auto relative overflow-hidden

        row
            css sticky top-0 z-50 px-4 py-3 items-center justify-between bg-transparent
            txt "Reels"
                css text-xl font-bold text-white
            row
                css gap-3 items-center
                btn "📷"
                    var ghost
                    css text-xl text-white
                btn "✉"
                    var ghost
                    css text-xl text-white
                    act nav instagram_messages

        block
            css relative w-full aspect-[9/16] bg-black overflow-hidden
            img_block "https://picsum.photos/600/900?random=11"
                css absolute inset-0 w-full h-full object-cover
            block
                css absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-black/20 pointer-events-none
            row
                css absolute top-12 right-3 items-center gap-1 bg-black/40 rounded-full px-2 py-0.5
                txt "▶"
                    css text-xs text-white
                txt "1.2M"
                    css text-xs text-white font-medium
            col
                css absolute bottom-20 left-3 right-16 gap-2
                row
                    css items-center gap-2
                    block
                        css w-8 h-8 rounded-full overflow-hidden border border-white/60 flex-shrink-0
                        img_block "https://i.pravatar.cc/150?u=bob"
                            css w-full h-full object-cover
                    txt "Bob Smith"
                        css text-sm font-semibold text-white
                    btn "Follow"
                        var ghost
                        sz sm
                        css border border-white text-white text-xs px-3 py-0.5 rounded-full
                        act tog ig/following/bob
                txt "Sunset at the harbor — worth every second 🌅"
                    css text-sm text-white leading-tight
                txt "2h ago"
                    css text-xs text-white/60
            col
                css absolute bottom-20 right-3 items-center gap-5
                col
                    css items-center gap-1
                    btn "🤍"
                        var ghost
                        css text-3xl
                        act tog ig/liked/r_bob_1
                    txt "2.4K"
                        css text-xs text-white
                col
                    css items-center gap-1
                    btn "💬"
                        var ghost
                        css text-3xl
                        act store_set ig/viewing/post_id, r_bob_1
                        act nav instagram_post
                    txt "184"
                        css text-xs text-white
                col
                    css items-center gap-1
                    btn "🔖"
                        var ghost
                        css text-3xl
                        act tog ig/saved/r_bob_1
                    txt "312"
                        css text-xs text-white
                btn "➦"
                    var ghost
                    css text-3xl text-white
                block
                    css w-9 h-9 rounded-full overflow-hidden border-2 border-white cursor-pointer
                    act store_set ig/viewing/user_id, bob
                    act nav instagram_profile
                    img_block "https://i.pravatar.cc/150?u=bob"
                        css w-full h-full object-cover

        block
            css relative w-full aspect-[9/16] bg-black overflow-hidden
            img_block "https://picsum.photos/600/900?random=22"
                css absolute inset-0 w-full h-full object-cover
            block
                css absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-black/20 pointer-events-none
            row
                css absolute top-12 right-3 items-center gap-1 bg-black/40 rounded-full px-2 py-0.5
                txt "▶"
                    css text-xs text-white
                txt "890K"
                    css text-xs text-white font-medium
            col
                css absolute bottom-20 left-3 right-16 gap-2
                row
                    css items-center gap-2
                    block
                        css w-8 h-8 rounded-full overflow-hidden border border-white/60 flex-shrink-0
                        img_block "https://i.pravatar.cc/150?u=diana"
                            css w-full h-full object-cover
                    txt "Diana Prince"
                        css text-sm font-semibold text-white
                    btn "Follow"
                        var ghost
                        sz sm
                        css border border-white text-white text-xs px-3 py-0.5 rounded-full
                        act tog ig/following/diana
                txt "Studio flow — creating magic ✨🎨"
                    css text-sm text-white leading-tight
                txt "5h ago"
                    css text-xs text-white/60
            col
                css absolute bottom-20 right-3 items-center gap-5
                col
                    css items-center gap-1
                    btn "🤍"
                        var ghost
                        css text-3xl
                        act tog ig/liked/r_diana_1
                    txt "5.1K"
                        css text-xs text-white
                col
                    css items-center gap-1
                    btn "💬"
                        var ghost
                        css text-3xl
                        act store_set ig/viewing/post_id, r_diana_1
                        act nav instagram_post
                    txt "423"
                        css text-xs text-white
                col
                    css items-center gap-1
                    btn "🔖"
                        var ghost
                        css text-3xl
                        act tog ig/saved/r_diana_1
                    txt "671"
                        css text-xs text-white
                btn "➦"
                    var ghost
                    css text-3xl text-white
                block
                    css w-9 h-9 rounded-full overflow-hidden border-2 border-white cursor-pointer
                    act store_set ig/viewing/user_id, diana
                    act nav instagram_profile
                    img_block "https://i.pravatar.cc/150?u=diana"
                        css w-full h-full object-cover

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
                css text-2xl
                act nav instagram_reels
            btn "♡"
                var ghost
                css text-2xl opacity-60
                act nav instagram_notifications
            btn "👤"
                var ghost
                css text-2xl opacity-60
                act nav instagram
