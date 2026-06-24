crate::data::app_data::ensure_ig_seeded();

col
        id instagram_home_page
        css min-h-screen bg-black text-white pb-20 max-w-lg mx-auto lg:max-w-6xl lg:pb-0 lg:ml-64

        instagram_sidebar("home")

        row
            css sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3 items-center justify-between
            txt "Instagram"
                css text-xl font-bold tracking-tight italic
            row
                css gap-4 items-center
                btn "♡"
                    var ghost
                    css text-2xl
                    act nav instagram_notifications
                btn "✉"
                    var ghost
                    css text-2xl
                    act nav instagram_messages

        row
            css px-4 py-3 gap-4 overflow-x-auto scrollbar-none border-b border-gray-800
            ig_story_circle("https://i.pravatar.cc/150?u=me", "Your Story")
            ig_story_circle_nav("https://i.pravatar.cc/150?u=bob", "Bob", "bob")
            ig_story_circle_nav("https://i.pravatar.cc/150?u=charlie", "Charlie", "charlie")
            ig_story_circle_nav("https://i.pravatar.cc/150?u=diana", "Diana", "diana")

        ig_feed_post("bob", "Bob Smith", "@bob", "https://i.pravatar.cc/150?u=bob", "p_bob_1", "https://picsum.photos/600/600?random=10", "2.4K likes", "Golden hour at the harbor \u{1F305}", "2h ago")

        ig_feed_post("diana", "Diana Prince", "@diana", "https://i.pravatar.cc/150?u=diana", "p_diana_1", "https://picsum.photos/600/600?random=20", "5.1K likes", "Studio vibes today \u{1F3A8}\u{2728}", "5h ago")

        ig_feed_post("charlie", "Charlie Day", "@charlie", "https://i.pravatar.cc/150?u=charlie", "p_charlie_1", "https://picsum.photos/600/600?random=30", "891 likes", "Mountain air hits different \u{1F3D4}\u{FE0F}", "8h ago")

        ig_bottom_nav("home")
