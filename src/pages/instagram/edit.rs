use crate::data::app_data::seed_instagram_storage;
use crate::tokens::storage::primitive::Store;
if Store::read("ig.me.name").is_none() { seed_instagram_storage(); }

col()
        id("instagram_edit_page")
        css("min-h-screen bg-black text-white pb-24 max-w-lg mx-auto")

        row()
            css("sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between")
            btn("← Back")
                var("ghost")
                sz("sm")
                nav("instagram")
            txt("Edit Profile")
                css("text-lg font-bold")
            btn("Save")
                var("ghost")
                sz("sm")
                css("text-blue-400 font-semibold")
                acts(vec![
                    store_set_input("ig.me.name",   "ig.edit.name"),
                    store_set_input("ig.me.handle", "ig.edit.handle"),
                    store_set_input("ig.me.bio",    "ig.edit.bio"),
                    store_set_input("ig.me.avatar", "ig.edit.avatar"),
                    navigate("instagram"),
                ])

        col()
            css("px-4 pt-6 pb-8 gap-6")
            col()
                css("items-center gap-3")
                block()
                    css("w-24 h-24 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600")
                    img_block("https://i.pravatar.cc/150?u=me")
                        css("w-full h-full rounded-full border-2 border-black object-cover")
                txt("Change profile photo")
                    css("text-sm font-medium text-blue-400 cursor-pointer")

            col()
                css("gap-1")
                txt("Name")
                    css("text-xs font-medium text-gray-400 uppercase tracking-wide")
                txtinp("Your name", "ig.edit.name")
                    css("w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500")

            col()
                css("gap-1")
                txt("Username")
                    css("text-xs font-medium text-gray-400 uppercase tracking-wide")
                txtinp("@handle", "ig.edit.handle")
                    css("w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500")

            col()
                css("gap-1")
                txt("Bio")
                    css("text-xs font-medium text-gray-400 uppercase tracking-wide")
                txtinp("Write a bio...", "ig.edit.bio")
                    css("w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500 min-h-[80px]")

            col()
                css("gap-1")
                txt("Avatar URL")
                    css("text-xs font-medium text-gray-400 uppercase tracking-wide")
                txtinp("https://i.pravatar.cc/150?u=me", "ig.edit.avatar")
                    css("w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-400 focus:outline-none focus:border-blue-500 break-all")

            block()
                css("border-t border-gray-800 pt-2")
                row()
                    css("py-3 items-center justify-between")
                    txt("Switch to Professional account")
                        css("text-sm text-gray-300")
                    txt("›")
                        css("text-gray-500")
                row()
                    css("py-3 items-center justify-between border-t border-gray-800")
                    txt("Personal information settings")
                        css("text-sm text-gray-300")
                    txt("›")
                        css("text-gray-500")

            btn("Save changes")
                var("primary")
                sz("md")
                css("w-full rounded-xl py-3 text-base font-semibold")
                acts(vec![
                    store_set_input("ig.me.name",   "ig.edit.name"),
                    store_set_input("ig.me.handle", "ig.edit.handle"),
                    store_set_input("ig.me.bio",    "ig.edit.bio"),
                    store_set_input("ig.me.avatar", "ig.edit.avatar"),
                    navigate("instagram"),
                ])

        row()
            css("fixed bottom-0 left-0 right-0 bg-black/95 backdrop-blur border-t border-gray-800 py-2 px-4 justify-around items-center z-50 max-w-lg mx-auto")
            btn("🏠")
                var("ghost")
                css("text-2xl opacity-60")
                nav("instagram_home")
            btn("🔍")
                var("ghost")
                css("text-2xl opacity-60")
                nav("instagram_explore")
            btn("➕")
                var("ghost")
                css("text-2xl opacity-60")
                nav("instagram_create")
            btn("🎬")
                var("ghost")
                css("text-2xl opacity-60")
                nav("instagram_reels")
            btn("♡")
                var("ghost")
                css("text-2xl opacity-60")
                nav("instagram_notifications")
            btn("👤")
                var("ghost")
                css("text-2xl")
                nav("instagram")
