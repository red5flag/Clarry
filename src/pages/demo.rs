
pub fn page_token() -> impl IntoToken {
    // Pre-seed storage so both SSR and CSR see the same initial data
    Store::write("demo.messages", r#"[{"id":"msg_1","text":"Welcome to the messaging demo!","sender":"alice","timestamp":"now"},{"id":"msg_2","text":"How are you today?","sender":"alice","timestamp":"now"}]"#);
    Store::write("demo.legacy_chat", r#"[{"id":"l1","text":"Hello from Alice","sender":"alice","timestamp":"now"},{"id":"l2","text":"Hi Alice!","sender":"user","timestamp":"now"}]"#);
    Store::write("demo.user.name", "Guest User");

    col()
        .id("demo_page")
        .css("min-h-screen bg-gray-50 p-6 space-y-8")
        .child(text("Token DSL Comprehensive Demo").css("text-3xl font-bold text-gray-900"))
        .child(text("Every primitive, action, and layout — interactive & working").css("text-sm text-gray-500"))
        // ── 1. Layout ────────────────────────────────────────────────────
        .child(section_title("1. Layout Primitives"))
        .child(ui!(grid2().css("gap-4") => {
            ui!(card("Row") => {
                ui!(row().css("gap-2") => {
                    block().css("w-8 h-8 bg-red-400 rounded"),
                    block().css("w-8 h-8 bg-green-400 rounded"),
                    block().css("w-8 h-8 bg-blue-400 rounded")
                })
            }),
            ui!(card("Column") => {
                ui!(col().css("gap-2") => {
                    block().css("w-full h-4 bg-purple-400 rounded"),
                    block().css("w-full h-4 bg-pink-400 rounded")
                })
            }),
            ui!(card("Grid (3 cols)") => {
                ui!(grid(3).css("gap-2") => {
                    block().css("h-8 bg-orange-400 rounded"),
                    block().css("h-8 bg-teal-400 rounded"),
                    block().css("h-8 bg-indigo-400 rounded")
                })
            }),
            ui!(card("Stack") => {
                ui!(stack().css("gap-2 p-2 bg-gray-100 rounded") => {
                    text("Stacked A").css("text-xs"),
                    text("Stacked B").css("text-xs")
                })
            })
        }))
        .child(ui!(row().css("gap-4 mt-4") => {
            ui!(card("Split (0.4)") => {
                ui!(split(0.4) => {
                    block().css("h-16 bg-cyan-400 rounded"),
                    block().css("h-16 bg-lime-400 rounded")
                })
            }),
            ui!(card("Aspect 16:9") => {
                ui!(aspect(16, 9).css("bg-yellow-300 rounded flex items-center justify-center") => {
                    text("16:9").css("text-xs font-bold")
                })
            })
        }))
        // ── 2. Typography ────────────────────────────────────────────────
        .child(section_title("2. Typography & Content"))
        .child(ui!(card("Text variants") => {
            text("Regular text"),
            text("Bold text").bold(),
            text("Muted text").muted(),
            text("Uppercase").uppercase(),
            text("Centered").center(),
            text("H1 heading").h1(),
            text("H2 heading").h2(),
            text("H3 heading").h3(),
            text("Caption").caption(),
            text("Label").label(),
            text("Monospace").mono(),
            text("Italic").italic(),
            text("Strikethrough").strike(),
            text("Underline").underline(),
            text("Custom color").color("#7c3aed")
        }))
        .child(ui!(card("Dynamic / Reactive text") => {
            counter_text("demo_counter", "Count:")
        }))
        .child(ui!(card("Image block") => {
            img_block("https://via.placeholder.com/150").css("w-full h-32 rounded-lg")
        }))
        // ── 3. Inputs ────────────────────────────────────────────────────
        .child(section_title("3. Input Primitives"))
        .child(ui!(grid2().css("gap-4") => {
            ui!(card("Text Input") => {
                text_input("Enter name...", "demo_name").css("w-full p-2 border rounded")
            }),
            ui!(card("Number Input") => {
                input_number("demo_num").css("w-full p-2 border rounded")
            }),
            ui!(card("Password Input") => {
                input_password("demo_pwd").css("w-full p-2 border rounded")
            }),
            ui!(card("Checkbox") => {
                checkbox("demo_chk", "I agree to terms").css("flex items-center gap-2")
            }),
            ui!(card("Textarea") => {
                textarea("demo_textarea", 4).css("w-full p-2 border rounded")
            }),
            ui!(card("Select") => {
                select("demo_select", vec!["Option A", "Option B", "Option C"]).css("w-full p-2 border rounded")
            }),
            ui!(card("Named Input (in:)") => {
                ui!(block().css("flex items-center gap-2") => {
                    text("Name:").css("text-sm"),
                    block().css("border p-1 rounded flex-1").act(in_("full_name"))
                })
            })
        }))
        // ── 4. Buttons ───────────────────────────────────────────────────
        .child(section_title("4. Buttons — Variants, Sizes, States"))
        .child(ui!(card("Variant buttons") => {
            ui!(row().css("gap-2 flex-wrap") => {
                btn("Primary").variant("primary").size_str("md"),
                btn("Secondary").variant("secondary").size_str("md"),
                btn("Danger").variant("danger").size_str("md"),
                btn("Ghost").variant("ghost").size_str("md"),
                btn("Success").variant("success").size_str("md")
            })
        }))
        .child(ui!(card("Size buttons") => {
            ui!(row().css("gap-2 items-end") => {
                btn("Small").variant("primary").size_str("sm"),
                btn("Medium").variant("primary").size_str("md"),
                btn("Large").variant("primary").size_str("lg")
            })
        }))
        .child(ui!(card("State buttons") => {
            ui!(row().css("gap-2 flex-wrap") => {
                btn("Loading").variant("primary").loading(true),
                btn("Disabled").variant("primary").disabled(true),
                btn("Loading+Disabled").variant("secondary").loading(true).disabled(true)
            })
        }))
        .child(ui!(card("Action buttons") => {
            ui!(row().css("gap-2 flex-wrap") => {
                btn("Increment").variant("primary").size_str("sm").act(increment("demo_counter")),
                btn("Decrement").variant("secondary").size_str("sm").act(decrement("demo_counter")),
                btn("Toggle").variant("ghost").size_str("sm").act(toggle("demo_toggle_target")),
                btn("Copy").variant("primary").size_str("sm").act(copy_to_clipboard("Hello!")),
                btn("Open URL").variant("secondary").size_str("sm").act(open_url("https://example.com")),
                btn("Navigate").variant("ghost").size_str("sm").act(navigate("demo"))
            })
        }))
        .child(ui!(block().id("demo_toggle_target").css("hidden mt-2 p-3 bg-blue-50 rounded-lg") => {
            text("Toggled content is visible!").css("text-sm text-blue-700")
        }))
        // ── 5. Modal ─────────────────────────────────────────────────────
        .child(section_title("5. Modal Primitive"))
        .child(card("Built-in modal()")
            .child(btn("Open Modal").variant("primary").act(show("my_modal")))
            .child(
                modal("my_modal", "Modal Title",
                    col().css("gap-3")
                        .child(text("This modal was built with the modal() factory.").css("text-gray-600"))
                        .child(text("It auto-generates the backdrop, card, title, and close button.").css("text-sm text-gray-500"))
                        .child(row().css("gap-2 mt-2")
                            .child(btn("OK").variant("primary").act(hide("my_modal")))
                            .child(btn("Cancel").variant("ghost").act(hide("my_modal")))
                        )
                )
            )
        )
        // ── 6. Tabs & Accordion ────────────────────────────────────────
        .child(section_title("6. Tabs & Accordion"))
        .child(card("Tabs")
            .child(tabs("demo_tabs", vec![
                ("Overview", text("Overview content here.").css("text-gray-600")),
                ("Details", text("Detailed information goes here.").css("text-gray-600")),
                ("Settings", text("Settings panel content.").css("text-gray-600")),
            ]))
        )
        .child(card("Accordion")
            .child(accordion(vec![
                ("Section 1", text("Content for section 1.").css("text-gray-600")),
                ("Section 2", text("Content for section 2.").css("text-gray-600")),
                ("Section 3", text("Content for section 3.").css("text-gray-600")),
            ]))
        )
        // ── 7. Storage Test Suite ───────────────────────────────────────
        .child(section_title("7. Storage & Reactivity (Comprehensive)"))
        .child(card("LocalStore CRUD")
            .child(text_input("Type a note...", "store_input").css("w-full p-2 border rounded mb-2"))
            .child(row().css("gap-2 flex-wrap")
                .child(btn("Set").variant("primary").size_str("sm").act(store_set_input("user_note", "store_input")))
                .child(btn("Get").variant("secondary").size_str("sm").act(store_get("user_note", "user_note_display")))
                .child(btn("Delete").variant("danger").size_str("sm").act(store_delete("user_note")))
                .child(btn("Set TTL 60s").variant("ghost").size_str("sm").act(store_set_ttl("ttl_key", "expires", 60)))
                .child(btn("Watch").variant("ghost").size_str("sm").act(store_watch("user_note")))
            )
            .child(row().css("mt-2 gap-2")
                .child(text("Stored:").css("text-sm text-gray-600"))
                .child(text_bind("user_note").css("text-sm font-mono text-blue-600"))
            )
            .child(row().css("mt-1 gap-2")
                .child(text("Fetched:").css("text-sm text-gray-600"))
                .child(text_bind("user_note_display").css("text-sm font-mono text-green-600"))
            )
        )
        .child(card("Counter + Cycle")
            .child(row().css("gap-2")
                .child(btn("+1").variant("primary").size_str("sm").act(increment("demo_counter")))
                .child(btn("-1").variant("secondary").size_str("sm").act(decrement("demo_counter")))
                .child(btn("Cycle").variant("ghost").size_str("sm").act(cycle_state("demo_state", vec!["off", "on"])))
                .child(btn("Toggle").variant("ghost").size_str("sm").act(toggle_state("demo_state")))
            )
            .child(counter_text("demo_counter", "Count:"))
            .child(text("State:").css("text-xs text-gray-500 mt-1"))
            .child(text_bind("demo_state").css("text-sm font-mono text-blue-600"))
            .child(block().id("demo_state").css("mt-2 p-2 bg-green-100 rounded text-sm text-green-800")
                .child(text("Toggle target is visible!"))
            )
        )
        .child(card("Preload / Fetch")
            .child(row().css("gap-2")
                .child(btn("Preload API").variant("primary").size_str("sm").act(preload("api_data", "/api/data")))
                .child(btn("Fetch GET").variant("secondary").size_str("sm").act(fetch_get("/api/data")))
            )
        )
        // ── 8. Animation ───────────────────────────────────────────────────
        .child(section_title("8. Animation"))
        .child(grid2().css("gap-4")
            .child(card("Fade In")
                .child(block().css("w-16 h-16 bg-pink-500 rounded").append_css("animation:tok-fade-in 1.5s ease-in-out infinite alternate both;"))
            )
            .child(card("Slide Up")
                .child(block().css("w-16 h-16 bg-cyan-500 rounded").append_css("animation:tok-slide-up 1.5s ease-in-out infinite alternate both;"))
            )
            .child(card("Scale In")
                .child(block().css("w-16 h-16 bg-amber-500 rounded").append_css("animation:tok-scale-in 1.5s ease-in-out infinite alternate both;"))
            )
            .child(card("Pulse")
                .child(block().css("w-16 h-16 bg-rose-500 rounded").anim_pulse()))
        )
        .child(card("Scroll Enter")
            .child(block().css("w-16 h-16 bg-emerald-500 rounded").on_scroll_enter()))
        // ── 9. Terminal / Embedded ─────────────────────────────────────────
        .child(section_title("9. Terminal / Embedded"))
        .child(grid2().css("gap-4")
            .child(card("Terminal")
                .child(terminal("demo_terminal"))
            )
            .child(card("Log View")
                .child(log_view("demo_logs"))
            )
            .child(card("Status Bar")
                .child(status_bar(vec!["CPU: 12%", "Mem: 4.2GB", "Net: OK"]))
            )
            .child(card("Command Palette")
                .child(command_palette(vec![
                    toggle_state("cmd_dark"),
                    store_set("cmd_key", "cmd_val"),
                ]))
            )
        )
        // ── 10. Overlays ─────────────────────────────────────────────────
        .child(section_title("10. Overlays"))
        .child(grid2().css("gap-4")
            .child(card("Overlay")
                .child(block().css("relative h-24 bg-gradient-to-r from-blue-400 to-purple-500 rounded")
                    .child(overlay().css("absolute bottom-0 left-0 right-0 p-2 bg-black/50 text-white text-xs rounded-b")
                        .child(text("Overlay content"))
                    )
                )
            )
            .child(card("Tooltip (hover me)")
                .child(row().css("relative group")
                    .child(text("Hover this row").css("text-sm"))
                    .child(tooltip("tt1", "I am a tooltip!").css("absolute -top-8 left-0 bg-gray-800 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none"))
                )
            )
            .child(card("Drawer trigger")
                .child(btn("Open Drawer").variant("primary").size_str("sm").act(show("demo_drawer")))
                .child(drawer("demo_drawer", "right", "Drawer content here"))
            )
            .child(card("Portal target")
                .child(portal("demo_portal")
                    .child(text("Portal content renders here").css("text-sm text-gray-600"))
                )
            )
        )
        // ── 11. Media ────────────────────────────────────────────────────
        .child(section_title("11. Media Primitives"))
        .child(card("Video")
            .child(video("https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4")
                .css("w-full rounded-lg"))
        )
        .child(card("Audio")
            .child(audio_player("https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3")
                .css("w-full"))
        )
        .child(card("Model Viewer (3D)")
            .child(model_viewer("https://modelviewer.dev/shared-assets/models/Astronaut.glb")
                .css("w-full h-64 rounded-lg"))
        )
        .child(card("iframe")
            .child(iframe("https://example.com")
                .css("w-full h-40 rounded-lg border"))
        )
        // ── 12. Chat & Messaging (Primitive Storage) ─────────────────────
        .child(section_title("12. Chat & Messaging (Primitive Storage)"))
        .child(grid2().css("gap-4")
            .child(card("Data Source")
                .child(text("Raw JSON from demo.messages:").css("text-xs text-gray-500"))
                .child(text_read("demo.messages").css("font-mono text-xs bg-gray-100 p-2 rounded break-all"))
                .child(text("Nested read demo.user.name:").css("text-xs text-gray-500 mt-2"))
                .child(text_read("demo.user.name").css("font-mono text-xs bg-gray-100 p-2 rounded"))
            )
            .child(card("Alice's Messages")
                .child(chat_messages("demo.messages"))
            )
        )
        .child(card("Send a reply")
            .child(text_input("Type your reply...", "chat_input").css("w-full"))
            .child(row().css("gap-2 mt-2")
                .child(btn("Send").variant("primary").size_str("sm")
                    .act(chat_send("chat_input", "demo.messages", "user")))
                .child(btn("Clear").variant("danger").size_str("sm")
                    .act(store_set("demo.messages", "[]")))
            )
        )
        .child(card("Legacy Chat Bubble")
            .child(chat_bubble_messages("demo.legacy_chat"))
            .child(text_input("Type a message...", "legacy_chat_input").css("w-full mt-2"))
            .child(row().css("gap-2 mt-2")
                .child(btn("Send").variant("primary").size_str("sm")
                    .act(chat_send("legacy_chat_input", "demo.legacy_chat", "user")))
                .child(btn("Clear").variant("danger").size_str("sm")
                    .act(store_set("demo.legacy_chat", "[]")))
            )
        )
        // ── 13. Data Display ─────────────────────────────────────────────
        .child(section_title("13. Data Display"))
        .child(grid2().css("gap-4")
            .child(card("QR Code")
                .child(qr_code("https://example.com", "demo_qr")
                    .css("w-24 h-24"))
            )
            .child(card("Progress Bar")
                .child(progress_bar("demo_progress", 65))
            )
            .child(card("Rating")
                .child(rating("demo_rating", 5))
            )
            .child(card("Badge")
                .child(row().css("gap-2")
                    .child(badge("New", "#3b82f6"))
                    .child(badge("Hot", "#ef4444"))
                    .child(badge("Pro", "#22c55e"))
                )
            )
            .child(card("Chip")
                .child(row().css("gap-2 flex-wrap")
                    .child(chip("Rust", None))
                    .child(chip("React", None))
                    .child(chip("Vue", None))
                )
            )
            .child(card("Divider")
                .child(text("Above divider"))
                .child(divider())
                .child(text("Below divider"))
            )
            .child(card("Spacer")
                .child(text("Before spacer"))
                .child(spacer(32.0))
                .child(text("After 32px spacer"))
            )
            .child(card("Skeleton")
                .child(skeleton(200.0, 24.0))
                .child(skeleton_text(3))
            )
            .child(card("Copy Block")
                .child(copy_block("npm install clarry"))
            )
            .child(card("Toast Container")
                .child(toast_container("demo_toast"))
            )
        )
        // ── 14. Theme ────────────────────────────────────────────────────
        .child(section_title("14. Theme System"))
        .child(card("Theme Provider + Variables")
            .child(theme_provider(vec![
                ("primary", "#3b82f6"),
                ("danger", "#ef4444"),
                ("radius", "8px"),
            ], block().child(text("Themed content"))))
            .child(row().css("gap-2")
                .child(btn("Set Primary Red").variant("primary").size_str("sm")
                    .act(set_theme_var("primary", "#ef4444")))
                .child(btn("Set Primary Blue").variant("secondary").size_str("sm")
                    .act(set_theme_var("primary", "#3b82f6")))
            )
            .child(text("Current primary: var(--primary)").css("text-sm text-gray-600")
                .use_var("color", "primary"))
        )
        // ── 15. Accessibility ────────────────────────────────────────────
        .child(section_title("15. Accessibility"))
        .child(card("Screen-reader only")
            .child(sr_only("This text is only visible to screen readers"))
            .child(text("Visual text here"))
        )
        .child(card("Live region")
            .child(live_region("demo_live", "Status updates will be announced here"))
        )
        .child(card("Skip link")
            .child(skip_link("demo_page"))
        )
        // ── 16. Responsive ───────────────────────────────────────────────
        .child(section_title("16. Responsive Helpers"))
        .child(card("Breakpoint demo")
            .child(block().css("p-4 bg-gray-200 rounded sm:bg-blue-200 md:bg-green-200 lg:bg-purple-200 xl:bg-orange-200")
                .child(text("Resize window to see color change").css("text-sm"))
                .child(text("sm:640+ md:768+ lg:1024+ xl:1280+").css("text-xs text-gray-500 mt-1"))
            )
        )
        // ── Footer ───────────────────────────────────────────────────────
        .child(
            text("End of Demo — every prelude primitive has been exercised")
                .css("text-center text-xs text-gray-400 pt-8 pb-4")
        )
}

fn section_title(t: &str) -> Text {
    text(t).css("text-xl font-bold text-gray-800 mt-6")
}

fn card(title: &str) -> Block {
    block()
        .css("p-4 bg-white rounded-lg shadow-sm space-y-2")
        .child(text(title).css("text-sm font-semibold text-gray-700 mb-1"))
}

