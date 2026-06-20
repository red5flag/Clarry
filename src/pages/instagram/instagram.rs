use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            txtbnd ig/me/name
                css text-xl font-bold tracking-tight
            row
                css gap-4 items-center
                btn "✉"
                    var ghost
                    css text-xl
                    act nav instagram_messages
                btn "☰"
                    var ghost
                    css text-xl

        col
            css px-4 pt-5 pb-2 gap-4
            row
                css gap-5 items-start
                block
                    css w-20 h-20 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0
                    img_block "https://i.pravatar.cc/150?u=me"
                        id ig_me_avatar
                        css w-full h-full rounded-full border-2 border-black object-cover
                col
                    css flex-1 gap-2
                    row
                        css justify-around text-center
                        col
                            css items-center gap-0.5
                            txtbnd ig/me/posts_count
                                css text-base font-bold
                            txt "posts"
                                css text-xs text-gray-400
                        col
                            css items-center gap-0.5
                            txtbnd ig/me/followers
                                css text-base font-bold
                            txt "followers"
                                css text-xs text-gray-400
                        col
                            css items-center gap-0.5
                            txtbnd ig/me/following
                                css text-base font-bold
                            txt "following"
                                css text-xs text-gray-400
                    row
                        css gap-2
                        btn "Edit Profile"
                            var ghost
                            sz sm
                            css flex-1 border border-gray-600 rounded-lg text-sm font-medium
                            act nav instagram_edit
                        btn "+ New Post"
                            var ghost
                            sz sm
                            css flex-1 border border-gray-600 rounded-lg text-sm font-medium
                            act nav instagram_create
            col
                css gap-0.5
                txtbnd ig/me/name
                    css text-sm font-semibold
                txtbnd ig/me/handle
                    css text-xs text-gray-400
                txtbnd ig/me/bio
                    css text-sm text-gray-300 mt-1

        row
            css border-b border-gray-800
            block
                css flex-1 flex flex-col items-center py-2 border-b-2 border-white cursor-pointer
                txt "▦"
                    css text-lg
            block
                css flex-1 flex flex-col items-center py-2 border-b-2 border-transparent cursor-pointer
                act nav instagram_reels
                txt "🎬"
                    css text-lg text-gray-600 opacity-60
            block
                css flex-1 flex flex-col items-center py-2 border-b-2 border-transparent cursor-pointer
                act nav instagram_tagged
                txt "👤"
                    css text-lg text-gray-600 opacity-60

        col
            css py-16 items-center gap-3
            txt "📷"
                css text-5xl text-gray-700
            txt "Your posts appear here"
                css text-sm text-gray-500
            btn "Create your first post"
                var ghost
                sz sm
                css text-blue-400 text-sm
                act nav instagram_create

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
