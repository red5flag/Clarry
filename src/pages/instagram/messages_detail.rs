use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col
        id instagram_messages_detail_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("messages")

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-2.5 items-center gap-3
            btn "←"
                var ghost
                css text-xl
                act nav instagram_messages
            block
                css relative
                block
                    css w-9 h-9 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                    img_block "https://i.pravatar.cc/150?u=bob"
                        css w-full h-full rounded-full border-2 border-black object-cover
                block
                    css absolute bottom-0 right-0 w-2.5 h-2.5 rounded-full bg-green-500 border-2 border-black
            col
                css flex-1 gap-0
                txt "Bob Smith"
                    css text-sm font-semibold
                txt "Active now"
                    css text-xs text-gray-400
            row
                css gap-3 items-center
                btn "📞"
                    var ghost
                    css text-xl
                btn "📹"
                    var ghost
                    css text-xl
                btn "ℹ"
                    var ghost
                    css text-xl

        col
            css items-center py-6 gap-2 border-b border-gray-800
            block
                css w-20 h-20 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600
                img_block "https://i.pravatar.cc/150?u=bob"
                    css w-full h-full rounded-full border-2 border-black object-cover
            col
                css items-center gap-0
                txt "Bob Smith"
                    css text-base font-semibold
                txt "@bob"
                    css text-sm text-gray-400
            row
                css gap-2 mt-1
                btn "View Profile"
                    var ghost
                    sz sm
                    css border border-gray-600 rounded-lg text-sm px-4
                    act store_set ig/viewing/user_id, bob
                    act nav instagram_profile

        col
            css flex-1 px-4 py-4 gap-3 overflow-y-auto
            row
                css justify-start gap-2 items-end
                img_block "https://i.pravatar.cc/150?u=bob"
                    css w-6 h-6 rounded-full object-cover flex-shrink-0
                col
                    css items-start gap-0.5
                    block
                        css max-w-[240px] bg-gray-800 rounded-2xl rounded-bl-sm px-4 py-2.5
                        txt "Hey! That harbor shot was incredible 🌅"
                            css text-sm leading-relaxed text-gray-100
                    txt "2:30 PM"
                        css text-xs text-gray-500 pl-1
            row
                css justify-end gap-2 items-end
                col
                    css items-end gap-0.5
                    block
                        css max-w-[240px] bg-blue-500 text-white rounded-2xl rounded-br-sm px-4 py-2.5
                        txt "Thanks! The light was perfect that evening"
                            css text-sm leading-relaxed
                    txt "2:31 PM"
                        css text-xs text-gray-500 pr-1
                img_block "https://i.pravatar.cc/150?u=me"
                    css w-6 h-6 rounded-full object-cover flex-shrink-0
            row
                css justify-start gap-2 items-end
                img_block "https://i.pravatar.cc/150?u=bob"
                    css w-6 h-6 rounded-full object-cover flex-shrink-0
                col
                    css items-start gap-0.5
                    block
                        css max-w-[240px] bg-gray-800 rounded-2xl rounded-bl-sm px-4 py-2.5
                        txt "We should go together next time!"
                            css text-sm leading-relaxed text-gray-100
                    txt "2:33 PM"
                        css text-xs text-gray-500 pl-1
            row
                css justify-end gap-2 items-end
                col
                    css items-end gap-0.5
                    block
                        css max-w-[240px] bg-blue-500 text-white rounded-2xl rounded-br-sm px-4 py-2.5
                        txt "Definitely! 📸"
                            css text-sm leading-relaxed
                    txt "2:34 PM"
                        css text-xs text-gray-500 pr-1
                img_block "https://i.pravatar.cc/150?u=me"
                    css w-6 h-6 rounded-full object-cover flex-shrink-0

        col
            css px-4 pt-3 pb-2 gap-1
            txt "New messages:"
                css text-xs text-gray-500
            text_read ig/dms/dm_bob/messages
                css font-mono text-xs bg-gray-900 p-2 rounded break-all max-h-20 overflow-auto text-gray-300

        row
            css sticky bottom-0 bg-black border-t border-gray-800 px-4 py-2.5 gap-3 items-center z-40
            row
                css gap-3 items-center
                btn "📷"
                    var ghost
                    css text-2xl
                btn "🎤"
                    var ghost
                    css text-2xl
            txtinp "Message...", ig/dm_input/dm_bob
                css flex-1 bg-gray-900 rounded-full px-4 py-2.5 border border-gray-700 text-sm text-gray-100 focus:outline-none focus:border-gray-600
            btn "Send"
                var primary
                sz sm
                css rounded-full px-4 text-sm font-semibold flex-shrink-0
                act store_push ig/dms/dm_bob/messages, ig/dm_input/dm_bob

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
                css text-2xl
                act nav instagram_notifications
            btn "👤"
                var ghost
                css text-2xl opacity-60
                act nav instagram
