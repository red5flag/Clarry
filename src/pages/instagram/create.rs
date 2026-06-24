crate::data::app_data::ensure_ig_seeded();

col
        id instagram_create_page
        css min-h-screen bg-black text-white pb-24 max-w-lg mx-auto lg:max-w-6xl

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            btn "← Back"
                var ghost
                sz sm
                act nav instagram
            txt "New Post"
                css text-lg font-bold
            block
                css w-16

        col
            css px-4 pt-4 pb-6 gap-5
            row
                css gap-3 items-center
                img_block "https://i.pravatar.cc/150?u=me"
                    css w-10 h-10 rounded-full object-cover border border-gray-700 flex-shrink-0
                col
                    css gap-0
                    text_read ig/me/name
                        css text-sm font-semibold
                    text_read ig/me/handle
                        css text-xs text-gray-400

            col
                css gap-2
                txt "Photo URL"
                    css text-xs font-medium text-gray-400 uppercase tracking-wide
                txtinp "https://picsum.photos/600/600", ig/create/image_url
                    css w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500

            col
                css gap-2
                txt "Caption"
                    css text-xs font-medium text-gray-400 uppercase tracking-wide
                txtinp "Write a caption...", ig/create/caption
                    css w-full bg-gray-900 border border-gray-700 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500

            row
                css items-center justify-between py-2 border-t border-gray-800
                txt "Tag people"
                    css text-sm text-gray-300
                txt "›"
                    css text-gray-500
            row
                css items-center justify-between py-2 border-t border-gray-800
                txt "Add location"
                    css text-sm text-gray-300
                txt "›"
                    css text-gray-500
            row
                css items-center justify-between py-2 border-t border-gray-800
                txt "Share to Story too"
                    css text-sm text-gray-300
                block
                    css w-10 h-6 bg-blue-500 rounded-full relative cursor-pointer
                    block
                        css absolute right-1 top-1 w-4 h-4 bg-white rounded-full shadow

            btn "Share Post"
                var primary
                sz md
                css w-full rounded-xl py-3 text-base font-semibold mt-2
                act store_set_input ig/me/posts_caption, ig/create/caption
                act store_set_input ig/me/posts_image_url, ig/create/image_url
                act store_set ig/create/image_url, ""
                act store_set ig/create/caption, ""
                act nav instagram

        ig_bottom_nav("create")
