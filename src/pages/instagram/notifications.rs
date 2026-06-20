use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_notifications_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            btn "← Back"
                var ghost
                sz sm
                act nav instagram_home
            txt "Activity"
                css text-base font-semibold
            block
                css w-16

        row
            css mx-4 my-3 px-4 py-3 bg-gray-900 rounded-xl gap-3 items-center
            txt "👥"
                css text-2xl
            col
                css flex-1 gap-0
                txt "Follow Requests"
                    css text-sm font-semibold
                txt "5 new"
                    css text-xs text-gray-400
            txt "›"
                css text-gray-400

        txt "New"
            css px-4 py-2 text-sm font-semibold

        row
            css px-4 py-3 gap-3 items-center bg-blue-950/20
            block
                css relative flex-shrink-0 w-11 h-11
                img_block "https://i.pravatar.cc/150?u=diana"
                    css w-11 h-11 rounded-full object-cover
                block
                    css absolute -bottom-0.5 -right-0.5 w-5 h-5 rounded-full bg-black flex items-center justify-center text-xs
                    txt "❤️"
            col
                css flex-1 gap-0 cursor-pointer min-w-0
                act store_set ig/viewing/post_id, p_alice_1
                act nav instagram_post
                row
                    css flex-wrap gap-1 items-baseline
                    txt "Diana Prince"
                        css text-sm font-semibold
                    txt "liked your photo"
                        css text-sm text-gray-200
                txt "2m"
                    css text-xs text-gray-500 mt-0.5
            block
                css w-11 h-11 flex-shrink-0 overflow-hidden rounded-sm cursor-pointer
                act store_set ig/viewing/post_id, p_alice_1
                act nav instagram_post
                img_block "https://picsum.photos/100/100?random=1"
                    css w-full h-full object-cover block

        row
            css px-4 py-3 gap-3 items-center bg-blue-950/20
            block
                css relative flex-shrink-0 w-11 h-11
                img_block "https://i.pravatar.cc/150?u=bob"
                    css w-11 h-11 rounded-full object-cover
                block
                    css absolute -bottom-0.5 -right-0.5 w-5 h-5 rounded-full bg-black flex items-center justify-center text-xs
                    txt "💬"
            col
                css flex-1 gap-0 cursor-pointer min-w-0
                act store_set ig/viewing/post_id, p_alice_1
                act nav instagram_post
                row
                    css flex-wrap gap-1 items-baseline
                    txt "Bob Smith"
                        css text-sm font-semibold
                    txt "commented: Amazing shot! 🔥"
                        css text-sm text-gray-200
                txt "15m"
                    css text-xs text-gray-500 mt-0.5
            block
                css w-11 h-11 flex-shrink-0 overflow-hidden rounded-sm cursor-pointer
                act store_set ig/viewing/post_id, p_alice_1
                act nav instagram_post
                img_block "https://picsum.photos/100/100?random=2"
                    css w-full h-full object-cover block

        row
            css px-4 py-3 gap-3 items-center bg-blue-950/20
            block
                css relative flex-shrink-0 w-11 h-11
                img_block "https://i.pravatar.cc/150?u=eve"
                    css w-11 h-11 rounded-full object-cover
                block
                    css absolute -bottom-0.5 -right-0.5 w-5 h-5 rounded-full bg-black flex items-center justify-center text-xs
                    txt "👤"
            col
                css flex-1 gap-0 min-w-0
                row
                    css flex-wrap gap-1 items-baseline
                    txt "Eve Adams"
                        css text-sm font-semibold
                    txt "started following you"
                        css text-sm text-gray-200
                txt "1h"
                    css text-xs text-gray-500 mt-0.5
            btn "Follow Back"
                var primary
                sz sm
                css text-xs font-semibold px-3 py-1.5
                act tog ig/following/eve

        txt "This week"
            css px-4 py-2 text-sm font-semibold

        row
            css px-4 py-3 gap-3 items-center
            block
                css relative flex-shrink-0 w-11 h-11
                img_block "https://i.pravatar.cc/150?u=charlie"
                    css w-11 h-11 rounded-full object-cover
                block
                    css absolute -bottom-0.5 -right-0.5 w-5 h-5 rounded-full bg-black flex items-center justify-center text-xs
                    txt "❤️"
            col
                css flex-1 gap-0 cursor-pointer min-w-0
                act store_set ig/viewing/post_id, p_alice_1
                act nav instagram_post
                row
                    css flex-wrap gap-1 items-baseline
                    txt "Charlie Day"
                        css text-sm font-semibold
                    txt "liked your photo"
                        css text-sm text-gray-200
                txt "3d"
                    css text-xs text-gray-500 mt-0.5
            block
                css w-11 h-11 flex-shrink-0 overflow-hidden rounded-sm cursor-pointer
                act store_set ig/viewing/post_id, p_alice_1
                act nav instagram_post
                img_block "https://picsum.photos/100/100?random=3"
                    css w-full h-full object-cover block

        row
            css px-4 py-3 gap-3 items-center
            block
                css relative flex-shrink-0 w-11 h-11
                img_block "https://i.pravatar.cc/150?u=frank"
                    css w-11 h-11 rounded-full object-cover
                block
                    css absolute -bottom-0.5 -right-0.5 w-5 h-5 rounded-full bg-black flex items-center justify-center text-xs
                    txt "👤"
            col
                css flex-1 gap-0 min-w-0
                row
                    css flex-wrap gap-1 items-baseline
                    txt "Frank Miller"
                        css text-sm font-semibold
                    txt "started following you"
                        css text-sm text-gray-200
                txt "5d"
                    css text-xs text-gray-500 mt-0.5
            btn "Following"
                var ghost
                sz sm
                css border border-gray-600 text-xs font-medium px-3 py-1.5
                act tog ig/following/frank

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
                css text-2xl
                act nav instagram_notifications
            btn "👤"
                var ghost
                css text-2xl opacity-60
                act nav instagram
