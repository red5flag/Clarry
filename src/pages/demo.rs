
// Inline storage demo: data is seeded via the same storage controls used elsewhere,
// not a global Store::write() at the top of the page.
col
    id demo_page
    css min-h-screen bg-gray-50 p-6 space-y-8 overflow-x-hidden
    txt "Token DSL Comprehensive Demo"
        css text-3xl font-bold text-gray-900
    txt "Every primitive, action, and layout — interactive & working"
        css text-sm text-gray-500
    section_title "0. Demo Data — Inline Storage"
    txt "Seed shared keys with write+read in one card. Data created here is live everywhere."
        css text-sm text-gray-500
    grid2
        css gap-4
        card_store demo/user/name
        card_store demo/note
        card_store demo/title
        card_store demo/profile
        card_store demo/config
    grid2
        css gap-4
        card "Chat messages (list)"
            add_to demo/messages
        card "Comments (list)"
            add_to demo/comments
        card "Items (list)"
            list_panel demo/items
        card "Tags (list)"
            list_panel demo/tags
    section_title "1. Layout Primitives"
    grid2
        css gap-4
        card "Row"
            row
                css gap-2
                block
                    css w-8 h-8 bg-red-400 rounded
                block
                    css w-8 h-8 bg-green-400 rounded
                block
                    css w-8 h-8 bg-blue-400 rounded
        card "Column"
            col
                css gap-2
                block
                    css w-full h-4 bg-purple-400 rounded
                block
                    css w-full h-4 bg-pink-400 rounded
        card "Grid (3 cols)"
            grid 3
                css gap-2
                block
                    css h-8 bg-orange-400 rounded
                block
                    css h-8 bg-teal-400 rounded
                block
                    css h-8 bg-indigo-400 rounded
        card "Stack"
            stack
                css gap-2 p-2 bg-gray-100 rounded
                txt "Stacked A"
                    css text-xs
                txt "Stacked B"
                    css text-xs
    row
        css gap-4 mt-4
        card "Split (0.4)"
            split 0.4
                block
                    css h-16 bg-cyan-400 rounded
                block
                    css h-16 bg-lime-400 rounded
        card "Aspect 16:9"
            aspect 16, 9
                css bg-yellow-300 rounded flex items-center justify-center
                txt "16:9"
                    css text-xs font-bold
    section_title "2. Typography & Content"
    card "Text variants"
        txt "Regular text"
        bold "Bold text"
        muted "Muted text"
        uppercase "Uppercase"
        center "Centered"
        h1 "H1 heading"
        h2 "H2 heading"
        h3 "H3 heading"
        caption "Caption"
        label "Label"
        mono "Monospace"
        italic "Italic"
        strike "Strikethrough"
        underline "Underline"
        color "Custom color", #7c3aed
    card "Dynamic / Reactive text"
        counter_text demo_counter, "Count:"
    card "Image block"
        img_block "https://via.placeholder.com/150"
            css w-full h-32 rounded-lg
    section_title "3. Input Primitives"
    grid2
        css gap-4
        card "Text Input"
            txtinp "Enter name...", demo_name
                css w-full p-2 border rounded
        card "Number Input"
            innum demo_num
                css w-full p-2 border rounded
        card "Password Input"
            inpsw demo_pwd
                css w-full p-2 border rounded
        card "Checkbox"
            checkbox demo_chk, "I agree to terms"
                css flex items-center gap-2
        card "Textarea"
            txtarea demo_textarea, 4
                css w-full p-2 border rounded
        card "Select"
            select demo_select, ["Option A", "Option B", "Option C"]
                css w-full p-2 border rounded
        card "Named Input (in:)"
            block
                css flex items-center gap-2
                txt "Name:"
                    css text-sm
                block
                    css border p-1 rounded flex-1
                    act in_ full_name
    section_title "4. Buttons — Variants, Sizes, States"
    card "Variant buttons"
        row
            css gap-2 flex-wrap
            btn "Primary"
                var primary
                sz md
            btn "Secondary"
                var secondary
                sz md
            btn "Danger"
                var danger
                sz md
            btn "Ghost"
                var ghost
                sz md
            btn "Success"
                var success
                sz md
    card "Size buttons"
        row
            css gap-2 items-end
            btn "Small"
                var primary
                sz sm
            btn "Medium"
                var primary
                sz md
            btn "Large"
                var primary
                sz lg
    card "State buttons"
        row
            css gap-2 flex-wrap
            loading "Loading btn"
            disabled "Disabled btn"
    card "Action buttons"
        row
            css gap-2 flex-wrap
            btn "Increment"
                var primary
                sz sm
                inc demo_counter
            btn "Decrement"
                var secondary
                sz sm
                dec demo_counter
            btn "Toggle"
                var ghost
                sz sm
                act toggle demo_toggle_target
            btn "Copy"
                var primary
                sz sm
                act copy_to_clipboard "Hello!"
            btn "Open URL"
                var secondary
                sz sm
                act url "https://example.com"
            btn "Navigate"
                var ghost
                sz sm
                act nav demo
    block
        id demo_toggle_target
        css hidden mt-2 p-3 bg-blue-50 rounded-lg
        txt "Toggled content is visible!"
            css text-sm text-blue-700
    section_title "5. Modal Primitive"
    card "Built-in modal()"
        btn "Open Modal"
            var primary
            act show my_modal
        modal my_modal, Modal Title
            col
                css gap-3
                txt "This modal was built with the modal() factory."
                    css text-gray-600
                txt "It auto-generates the backdrop, card, title, and close button."
                    css text-sm text-gray-500
                row
                    css gap-2 mt-2
                    btn "OK"
                        var primary
                        act hide my_modal
                    btn "Cancel"
                        var ghost
                        act hide my_modal
    section_title "6. Tabs & Accordion"
    card "Tabs"
        tabs demo_tabs
            tab Overview, txt "Overview content here."
            tab Details, txt "Detailed information goes here."
            tab Settings, txt "Settings panel content."
    card "Accordion"
        accordion
            section Section 1, txt "Content for section 1."
            section Section 2, txt "Content for section 2."
            section Section 3, txt "Content for section 3."
    section_title "7. Storage & Reactivity (Comprehensive)"
    card "LocalStore CRUD"
        txtinp "Type a note...", store_input
            css w-full p-2 border rounded mb-2
        row
            css gap-2 flex-wrap
            btn "Set"
                var primary
                sz sm
                act store_set_input user_note, store_input
            btn "Get"
                var secondary
                sz sm
                act store_get user_note, user_note_display
            btn "Delete"
                var danger
                sz sm
                act store_delete user_note
            btn "Set TTL 60s"
                var ghost
                sz sm
                act store_set_ttl ttl_key, expires, 60
            btn "Watch"
                var ghost
                sz sm
                act store_watch user_note
        row
            css mt-2 gap-2
            txt "Stored:"
                css text-sm text-gray-600
            txtbnd user_note
                css text-sm font-mono text-blue-600
        row
            css mt-1 gap-2
            txt "Fetched:"
                css text-sm text-gray-600
            txtbnd user_note_display
                css text-sm font-mono text-green-600
    card "Counter + Cycle"
        row
            css gap-2
            btn "+1"
                var primary
                sz sm
                inc demo_counter
            btn "-1"
                var secondary
                sz sm
                dec demo_counter
            btn "Cycle"
                var ghost
                sz sm
                cyc demo_state, off, on
            btn "Toggle"
                var ghost
                sz sm
                tog demo_state
        counter_text demo_counter, "Count:"
        txt "State:"
            css text-xs text-gray-500 mt-1
        txtbnd demo_state
            css text-sm font-mono text-blue-600
        block
            id demo_state
            css mt-2 p-2 bg-green-100 rounded text-sm text-green-800
            txt "Toggle target is visible!"
    card "Preload / Fetch"
        row
            css gap-2
            btn "Preload API"
                var primary
                sz sm
                act preload api_data, /api/data
            btn "Fetch GET"
                var secondary
                sz sm
                act fetch_get /api/data
    section_title "8. Storage Controls (Unified)"
    txt "card_store combines write + read in one card. List panels add/remove items."
        css text-sm text-gray-500
    grid2
        css gap-4
        card_store demo/note
        card_store demo/title
        card_store demo/profile
        card_store demo/config
    grid2
        css gap-4
        card "Items — add/remove"
            list_panel demo/items
        card "Tags — add/remove"
            list_panel demo/tags
        card "Messages — add"
            add_to demo/messages
        card "Comments — add"
            add_to demo/comments
    grid2
        css gap-4
        card "Load from endpoint"
            load_from demo/fetched, /api/ping
        card "Dynamic-path file storage"
            file_storage_panel demo/file
        card "Clear keys"
            clear_key demo/note
            clear_key demo/items
            clear_key demo/tags
    card "Cross-key read inspector"
        txt "demo/note value:"
            css text-xs text-gray-500
        text_read demo/note
            css font-mono text-sm bg-gray-100 p-2 rounded mb-2
        txt "demo/title value:"
            css text-xs text-gray-500
        text_read demo/title
            css font-mono text-sm bg-gray-100 p-2 rounded mb-2
        txt "demo/items JSON:"
            css text-xs text-gray-500
        text_read demo/items
            css font-mono text-xs bg-gray-100 p-2 rounded break-all max-h-24 overflow-auto mb-2
        txt "demo/tags JSON:"
            css text-xs text-gray-500
        text_read demo/tags
            css font-mono text-xs bg-gray-100 p-2 rounded break-all max-h-24 overflow-auto
    section_title "9. Animation"
    grid2
        css gap-4
        card "Fade In"
            block
                css w-16 h-16 bg-pink-500 rounded
                append_css animation:tok-fade-in 1.5s ease-in-out infinite alternate both;
        card "Slide Up"
            block
                css w-16 h-16 bg-cyan-500 rounded
                append_css animation:tok-slide-up 1.5s ease-in-out infinite alternate both;
        card "Scale In"
            block
                css w-16 h-16 bg-amber-500 rounded
                append_css animation:tok-scale-in 1.5s ease-in-out infinite alternate both;
        card "Pulse"
            block
                css w-16 h-16 bg-rose-500 rounded
                anim_pulse
    card "Scroll Enter"
        block
            css w-16 h-16 bg-emerald-500 rounded
            on_scroll_enter
    section_title "10. Terminal / Embedded"
    grid2
        css gap-4
        card "Terminal"
            terminal demo_terminal
        card "Log View"
            log_view demo_logs
        card "Status Bar"
            status_bar
                txt "CPU: 12%"
                txt "Mem: 4.2GB"
                txt "Net: OK"
        card "Command Palette"
            command_palette
                tog cmd_dark
                store_set cmd_key, cmd_val
    section_title "11. Overlays"
    grid2
        css gap-4
        card "Overlay"
            block
                css relative h-24 bg-gradient-to-r from-blue-400 to-purple-500 rounded
                overlay
                    css absolute bottom-0 left-0 right-0 p-2 bg-black/50 text-white text-xs rounded-b
                    txt "Overlay content"
        card "Tooltip (hover me)"
            row
                css relative group
                txt "Hover this row"
                    css text-sm
                tooltip tt1, "I am a tooltip!"
                    css absolute -top-8 left-0 bg-gray-800 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none
        card "Drawer trigger"
            btn "Open Drawer"
                var primary
                sz sm
                act show demo_drawer
            drawer demo_drawer, right, "Drawer content here"
        card "Portal target"
            portal demo_portal
                txt "Portal content renders here"
                    css text-sm text-gray-600
    section_title "12. Media Primitives"
    card "Video"
        video "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
            css w-full rounded-lg
    card "Audio"
        audio "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3"
            css w-full
    card "Model Viewer (3D)"
        model "https://modelviewer.dev/shared-assets/models/Astronaut.glb"
            css w-full h-64 rounded-lg
    card "iframe"
        iframe "https://example.com"
            css w-full h-40 rounded-lg border
    section_title "13. Chat & Messaging (Inline Storage)"
    grid2
        css gap-4
        card "Live message feed"
            chat_messages demo/messages
        card "Send a reply"
            txtinp "Type your reply...", chat_input
                css w-full
            row
                css gap-2 mt-2
                btn "Send"
                    var primary
                    sz sm
                    act chat_send chat_input, demo/messages, user
                btn "Clear"
                    var danger
                    sz sm
                    act store_set demo/messages, "[]"
        card "Write directly to key"
            write_to demo/messages/note
        card "Read live value"
            read_from demo/messages/note
    grid2
        css gap-4
        card "Add to message list"
            add_to demo/messages
        card "Remove from message list"
            remove_from demo/messages
    card "Full list panel — demo/messages"
        list_panel demo/messages
    card "Raw storage inspector"
        txt "demo/messages JSON:"
            css text-xs text-gray-500
        text_read demo/messages
            css font-mono text-xs bg-gray-100 p-2 rounded break-all max-h-32 overflow-auto
        txt "demo/user/name:"
            css text-xs text-gray-500 mt-2
        text_read demo/user/name
            css font-mono text-xs bg-gray-100 p-2 rounded
    section_title "14. Data Display"
    grid2
        css gap-4
        card "QR Code"
            qr_code "https://example.com", demo_qr
                css w-24 h-24
        card "Progress Bar"
            progress_bar demo_progress, 65
        card "Rating"
            rating demo_rating, 5
        card "Badge"
            row
                css gap-2
                badge "New", #3b82f6
                badge "Hot", #ef4444
                badge "Pro", #22c55e
        card "Chip"
            row
                css gap-2 flex-wrap
                chip "Rust", None
                chip "React", None
                chip "Vue", None
        card "Divider"
            txt "Above divider"
            divider
            txt "Below divider"
        card "Spacer"
            txt "Before spacer"
            spacer 32.0
            txt "After 32px spacer"
        card "Skeleton"
            skeleton 200.0, 24.0
            skeleton_text 3
        card "Copy Block"
            copy_block "npm install clarry"
        card "Toast Container"
            toast_container demo_toast
    section_title "15. Theme System"
    card "Theme Provider + Variables"
        theme primary #3b82f6, danger #ef4444, radius 8px
            block
                txt "Themed content"
        row
            css gap-2
            btn "Set Primary Red"
                var primary
                sz sm
                act set_theme_var primary, #ef4444
            btn "Set Primary Blue"
                var secondary
                sz sm
                act set_theme_var primary, #3b82f6
        txt "Current primary: var(--primary)"
            css text-sm text-gray-600
            use_var color, primary
    section_title "16. Accessibility"
    card "Screen-reader only"
        sr "This text is only visible to screen readers"
        txt "Visual text here"
    card "Live region"
        live demo_live, "Status updates will be announced here"
    card "Skip link"
        skip_link demo_page
    section_title "17. Responsive Helpers"
    card "Breakpoint demo"
        block
            css p-4 bg-gray-200 rounded sm:bg-blue-200 md:bg-green-200 lg:bg-purple-200 xl:bg-orange-200
            txt "Resize window to see color change"
                css text-sm
            txt "sm:640+ md:768+ lg:1024+ xl:1280+"
                css text-xs text-gray-500 mt-1
    section_title "18. Structural Primitives (Data-Driven UI)"
    txt "Architecture-first: UI generated from data, not hardcoded"
        css text-sm text-gray-500 mb-4
    card "Named Containers"
        col
            css gap-2
            txt "Named parent defined below, then referenced again:"
                css text-xs text-gray-500
            col_named Parent
                css p-3 bg-blue-50 rounded border border-blue-200
                txt "Hello from Parent"
                    css text-sm text-blue-800
            txt "Reference to Parent:"
                css text-xs text-gray-500 mt-2
            col_ref Parent
    card "foreach() — Iterate Collections"
        col
            css gap-2
            txt "Messages from Store:"
                css text-xs text-gray-500
            txt "(collection rendering placeholder)"
                css text-sm text-gray-400
            txt "Empty state shown when collection is empty:"
                css text-xs text-gray-400 mt-2
            txt "No items in collection"
                css text-sm text-gray-400 italic
    card "Tick & Remove List"
        col
            css gap-2
            txt "Click ✓ to highlight green, ✗ to remove:"
                css text-xs text-gray-500
            json_list demo/items, text
    card "Conditional Rendering"
        col
            css gap-2
            txt "✓ User is logged in"
                css text-green-600
            btn "Login"
                var primary
            txt "Welcome, Guest"
                css text-sm
            badge "Active", green
    card "Collection Operations — filter(), sort(), limit()"
        col
            css gap-2
            txt "All comments rendered by json_list:"
                css text-xs text-gray-500
            json_list demo/comments, text
            txt "Filter + Sort demo (active on messages):"
                css text-xs text-gray-500 mt-2
            row
                css gap-2
                btn "Show alice"
                    var secondary
                    sz sm
                    act store_set demo/filter_author, alice
                btn "Show user"
                    var secondary
                    sz sm
                    act store_set demo/filter_author, user
                btn "Clear filter"
                    var ghost
                    sz sm
                    act store_set demo/filter_author, ""
            txt "Count of messages:"
                css text-xs text-gray-500 mt-2
            json_count demo/messages
                css text-sm font-bold
    card "Like & Comment (Interactive)"
        col
            css gap-2
            txt "Post: First Post"
                css font-bold
            txt "by alice"
                css text-xs text-gray-500
            row
                css gap-2 mt-2 items-center
                btn "Like"
                    var ghost
                    sz sm
                    act store_inc demo/posts/0/likes
                txt "Likes:"
                    css text-sm
                json_field demo/posts/0, likes
                    css text-sm font-bold
            row
                css gap-2 mt-2 items-center
                txtinp "Add a comment...", comment_input
                    css flex-1 p-2 border rounded
                btn "Comment"
                    var ghost
                    sz sm
                    act chat_send comment_input, demo/comments, guest
            txt "Comments on this post:"
                css text-xs text-gray-500 mt-2
            json_list demo/comments, text
    card "Query — Derived Datasets"
        col
            css gap-2
            txt "Derived: post author + like count"
                css text-xs text-gray-500
            row
                css gap-2
                txt "Post:"
                    css text-sm
                json_field demo/posts/0, title
                    css text-sm font-bold
            row
                css gap-2
                txt "Author:"
                    css text-sm
                json_field demo/posts/0, author
                    css text-sm font-bold
            row
                css gap-2
                txt "Likes:"
                    css text-sm
                json_field demo/posts/0, likes
                    css text-sm font-bold
    card "Relation — Traversal"
        col
            css gap-2
            txt "Comments related to post p1"
                css text-xs text-gray-500
            json_list demo/comments, text
            txt "All messages from alice"
                css text-xs text-gray-500 mt-2
            chat_messages demo/messages
            row
                css gap-2 mt-2
                btn "Send as alice"
                    var secondary
                    sz sm
                    act chat_send chat_input, demo/messages, alice
                btn "Send as user"
                    var ghost
                    sz sm
                    act chat_send chat_input, demo/messages, user
    txt "End of Demo — every prelude primitive has been exercised"
        css text-center text-xs text-gray-400 pt-8 pb-4

