// src/tokens/builders/ig_shared.rs
//
// Shared Instagram UI utilities — eliminates duplicated patterns across
// the 12+ Instagram page files.

use crate::tokens::action::TokenAction;
use crate::tokens::core::id::next_id;
use crate::tokens::node::{IntoToken, Layout, Str, TokenNode};
use super::types::Container;

type Block = Container;
type Row = Container;
type Col = Container;

// ── Bottom Navigation Bar ─────────────────────────────────────────────────
//
// The 6-icon mobile bottom tab bar used by every Instagram page.
// `active` selects which tab is visually highlighted (no opacity dimming):
//   "home" | "explore" | "create" | "reels" | "notifications" | "profile"

pub fn ig_bottom_nav(active: impl Into<Str>) -> Block {
    let active = active.into().to_string();
    let dim = "opacity-60";

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.layout = Layout::Row;
    root.class = "fixed bottom-0 left-0 right-0 bg-gray-900 backdrop-blur border-t border-gray-800 py-2 px-4 justify-around items-center z-50 max-w-lg mx-auto lg:max-w-6xl lg:hidden".into();

    let items: &[(&str, &str, &str)] = &[
        ("\u{1F3E0}",  "instagram_home",          "home"),
        ("\u{1F50D}",  "instagram_explore",        "explore"),
        ("\u{2795}",   "instagram_create",         "create"),
        ("\u{1F3AC}",  "instagram_profile",        "reels"),
        ("\u{2661}",   "instagram_notifications",  "notifications"),
        ("\u{1F464}",  "instagram",                "profile"),
    ];

    for &(icon, dest, page) in items {
        let mut btn = TokenNode::new(next_id());
        btn.tag = "button".into();
        btn.content = Some(icon.into());
        let dim_class = if active == page { String::new() } else { format!(" {}", dim) };
        btn.class = format!(
            "inline-flex items-center justify-center gap-1 rounded font-medium transition-colors whitespace-nowrap text-2xl{}",
            dim_class
        ).into();
        btn.style.extra = "cursor:pointer;user-select:none;background:transparent;border:none;color:inherit;padding:0;".into();
        btn.actions.push(TokenAction::Navigate(dest.into()));
        root.children.push(btn);
    }

    Container { stack: vec![root] }
}

// ── Post Grid Thumbnail ───────────────────────────────────────────────────
//
// Clickable square thumbnail used in explore grids, saved posts, tagged
// posts, and profile post grids.  Tapping sets the viewing post_id and
// navigates to the post detail page.

pub fn ig_post_thumb(post_id: impl Into<Str>, image_url: impl Into<Str>) -> Block {
    let post_id: Str = post_id.into();
    let image_url: Str = image_url.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.class = "aspect-square bg-gray-900 overflow-hidden relative cursor-pointer".into();
    root.actions.push(TokenAction::StoreSet {
        key: "ig/viewing/post_id".into(),
        value: post_id,
    });
    root.actions.push(TokenAction::Navigate("instagram_post".into()));

    let mut img = TokenNode::new(next_id());
    img.tag = "img".into();
    img.content = Some(image_url);
    img.class = "w-full h-full object-cover block".into();
    root.children.push(img);

    Container { stack: vec![root] }
}

// ── Reel Card ─────────────────────────────────────────────────────────────
//
// Tall portrait card with a gradient overlay, title, and view count.
// Used in profile reel grids.

pub fn ig_reel_card(image_url: impl Into<Str>, title: impl Into<Str>, views: impl Into<Str>) -> Block {
    let image_url: Str = image_url.into();
    let title: Str = title.into();
    let views: Str = views.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.class = "h-[320px] lg:h-[520px] bg-black relative overflow-hidden cursor-pointer".into();

    // Background image
    let mut img = TokenNode::new(next_id());
    img.tag = "img".into();
    img.content = Some(image_url);
    img.class = "absolute inset-0 w-full h-full object-cover".into();
    root.children.push(img);

    // Gradient scrim
    let mut scrim = TokenNode::new(next_id());
    scrim.tag = "div".into();
    scrim.class = "absolute bottom-0 left-0 right-0 h-20 bg-gradient-to-t from-black to-transparent pointer-events-none".into();
    root.children.push(scrim);

    // Bottom info row
    let mut info_row = TokenNode::new(next_id());
    info_row.tag = "div".into();
    info_row.layout = Layout::Row;
    info_row.class = "absolute bottom-2 left-2 right-2 items-end justify-between".into();

    let mut info_col = TokenNode::new(next_id());
    info_col.tag = "div".into();
    info_col.layout = Layout::Col;
    info_col.class = "gap-0.5".into();

    let mut title_node = TokenNode::new(next_id());
    title_node.content = Some(title);
    title_node.class = "text-xs lg:text-sm font-semibold text-white".into();
    info_col.children.push(title_node);

    let mut views_node = TokenNode::new(next_id());
    views_node.content = Some(views);
    views_node.class = "text-xs text-white".into();
    info_col.children.push(views_node);

    info_row.children.push(info_col);

    let mut play_btn = TokenNode::new(next_id());
    play_btn.tag = "button".into();
    play_btn.content = Some("\u{25B6}".into());
    play_btn.class = "inline-flex items-center justify-center gap-1 rounded font-medium transition-colors whitespace-nowrap text-lg lg:text-xl text-white".into();
    play_btn.style.extra = "cursor:pointer;user-select:none;background:transparent;border:none;color:inherit;padding:0;".into();
    info_row.children.push(play_btn);

    root.children.push(info_row);

    Container { stack: vec![root] }
}

// ── Story Circle ──────────────────────────────────────────────────────────
//
// Avatar inside a gradient ring with a label underneath.  Used in the
// stories tray on the home and profile pages.

pub fn ig_story_circle(avatar_url: impl Into<Str>, label: impl Into<Str>) -> Col {
    let avatar_url: Str = avatar_url.into();
    let label: Str = label.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.layout = Layout::Col;
    root.class = "items-center gap-1 min-w-[64px]".into();

    let mut ring = TokenNode::new(next_id());
    ring.tag = "div".into();
    ring.class = "w-14 h-14 lg:w-16 lg:h-16 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600".into();

    let mut img = TokenNode::new(next_id());
    img.tag = "img".into();
    img.content = Some(avatar_url);
    img.class = "w-full h-full rounded-full border-2 border-black object-cover".into();
    ring.children.push(img);

    root.children.push(ring);

    let mut label_node = TokenNode::new(next_id());
    label_node.content = Some(label);
    label_node.class = "text-xs text-gray-300".into();
    root.children.push(label_node);

    Container { stack: vec![root] }
}

// ── Clickable Story Circle ────────────────────────────────────────────────
//
// Like ig_story_circle but with navigation actions: sets the story user
// id, resets the slide index, and navigates to the story viewer.

pub fn ig_story_circle_nav(avatar_url: impl Into<Str>, label: impl Into<Str>, user_id: impl Into<Str>) -> Col {
    let avatar_url: Str = avatar_url.into();
    let label: Str = label.into();
    let user_id: Str = user_id.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.layout = Layout::Col;
    root.class = "items-center gap-1 min-w-[64px] cursor-pointer".into();
    root.actions.push(TokenAction::StoreSet {
        key: "ig/viewing/story_id".into(),
        value: user_id,
    });
    root.actions.push(TokenAction::StoreSet {
        key: "ig/story/slide_index".into(),
        value: "0".into(),
    });
    root.actions.push(TokenAction::Navigate("instagram_story".into()));

    let mut ring = TokenNode::new(next_id());
    ring.tag = "div".into();
    ring.class = "w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0".into();

    let mut img = TokenNode::new(next_id());
    img.tag = "img".into();
    img.content = Some(avatar_url);
    img.class = "w-full h-full rounded-full border-2 border-black object-cover".into();
    ring.children.push(img);

    root.children.push(ring);

    let mut label_node = TokenNode::new(next_id());
    label_node.content = Some(label);
    label_node.class = "text-xs text-gray-300 truncate text-center".into();
    root.children.push(label_node);

    Container { stack: vec![root] }
}

// ── Suggested User Card ───────────────────────────────────────────────────
//
// Compact card with avatar, name, handle, and Follow button.  Used in the
// explore page suggestions row.

pub fn ig_suggest_card(
    user_id: impl Into<Str>,
    name: impl Into<Str>,
    handle: impl Into<Str>,
    avatar_url: impl Into<Str>,
) -> Col {
    let user_id: Str = user_id.into();
    let name: Str = name.into();
    let handle: Str = handle.into();
    let avatar_url: Str = avatar_url.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.layout = Layout::Col;
    root.class = "items-center gap-1 min-w-[100px] bg-gray-900 rounded-xl p-3 border border-gray-800".into();

    // Avatar ring (clickable → profile)
    let mut ring = TokenNode::new(next_id());
    ring.tag = "div".into();
    ring.class = "w-14 h-14 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 cursor-pointer".into();
    ring.actions.push(TokenAction::StoreSet {
        key: "ig/viewing/user_id".into(),
        value: user_id.clone(),
    });
    ring.actions.push(TokenAction::Navigate("instagram_profile".into()));

    let mut img = TokenNode::new(next_id());
    img.tag = "img".into();
    img.content = Some(avatar_url);
    img.class = "w-full h-full rounded-full border-2 border-black object-cover".into();
    ring.children.push(img);
    root.children.push(ring);

    // Name + handle
    let mut name_node = TokenNode::new(next_id());
    name_node.content = Some(name);
    name_node.class = "text-xs font-semibold truncate".into();
    root.children.push(name_node);

    let mut handle_node = TokenNode::new(next_id());
    handle_node.content = Some(handle);
    handle_node.class = "text-xs text-gray-500 truncate".into();
    root.children.push(handle_node);

    // Follow button
    let mut follow_btn = TokenNode::new(next_id());
    follow_btn.tag = "button".into();
    follow_btn.content = Some("Follow".into());
    follow_btn.class = "inline-flex items-center justify-center gap-1 rounded font-medium transition-colors whitespace-nowrap w-full bg-blue-500 text-white rounded-lg text-xs font-medium mt-2".into();
    follow_btn.style.extra = "cursor:pointer;user-select:none;".into();
    follow_btn.actions.push(TokenAction::ToggleState {
        key: format!("ig/following/{}", user_id).into(),
        on_state: "true".into(),
        off_state: "false".into(),
    });
    root.children.push(follow_btn);

    Container { stack: vec![root] }
}

// ── Post Action Row ───────────────────────────────────────────────────────
//
// Like / Comment / Share / [spacer] / Save — the interaction strip under
// every post image.

pub fn ig_action_row(post_id: impl Into<Str>) -> Row {
    let post_id: Str = post_id.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.layout = Layout::Row;
    root.class = "px-4 pt-3 pb-1 gap-4 items-center".into();

    let ghost_btn_style = "cursor:pointer;user-select:none;background:transparent;border:none;color:inherit;padding:0;";
    let ghost_btn_class = "inline-flex items-center justify-center gap-1 rounded font-medium transition-colors whitespace-nowrap text-2xl p-0";

    // Like
    let mut like_btn = TokenNode::new(next_id());
    like_btn.tag = "button".into();
    like_btn.content = Some("\u{2661}".into());
    like_btn.class = ghost_btn_class.into();
    like_btn.style.extra = ghost_btn_style.into();
    like_btn.actions.push(TokenAction::ToggleState {
        key: format!("ig/liked/{}", post_id).into(),
        on_state: "true".into(),
        off_state: "false".into(),
    });
    root.children.push(like_btn);

    // Comment
    let mut comment_btn = TokenNode::new(next_id());
    comment_btn.tag = "button".into();
    comment_btn.content = Some("\u{1F4AC}".into());
    comment_btn.class = ghost_btn_class.into();
    comment_btn.style.extra = ghost_btn_style.into();
    comment_btn.actions.push(TokenAction::StoreSet {
        key: "ig/viewing/post_id".into(),
        value: post_id.clone(),
    });
    comment_btn.actions.push(TokenAction::Navigate("instagram_post".into()));
    root.children.push(comment_btn);

    // Share
    let mut share_btn = TokenNode::new(next_id());
    share_btn.tag = "button".into();
    share_btn.content = Some("\u{2708}".into());
    share_btn.class = ghost_btn_class.into();
    share_btn.style.extra = ghost_btn_style.into();
    root.children.push(share_btn);

    // Spacer
    let mut spacer = TokenNode::new(next_id());
    spacer.tag = "div".into();
    spacer.class = "flex-1".into();
    root.children.push(spacer);

    // Save
    let mut save_btn = TokenNode::new(next_id());
    save_btn.tag = "button".into();
    save_btn.content = Some("\u{1F516}".into());
    save_btn.class = ghost_btn_class.into();
    save_btn.style.extra = ghost_btn_style.into();
    save_btn.actions.push(TokenAction::ToggleState {
        key: format!("ig/saved/{}", post_id).into(),
        on_state: "true".into(),
        off_state: "false".into(),
    });
    root.children.push(save_btn);

    Container { stack: vec![root] }
}

// ── Feed Post Card ────────────────────────────────────────────────────────
//
// Complete Instagram home feed post: header (avatar + name + handle),
// image, action row, likes, caption, and timestamp.

pub fn ig_feed_post(
    user_id: impl Into<Str>,
    name: impl Into<Str>,
    handle: impl Into<Str>,
    avatar_url: impl Into<Str>,
    post_id: impl Into<Str>,
    image_url: impl Into<Str>,
    likes: impl Into<Str>,
    caption: impl Into<Str>,
    timestamp: impl Into<Str>,
) -> Col {
    let user_id: Str = user_id.into();
    let name: Str = name.into();
    let handle: Str = handle.into();
    let avatar_url: Str = avatar_url.into();
    let post_id: Str = post_id.into();
    let image_url: Str = image_url.into();
    let likes: Str = likes.into();
    let caption: Str = caption.into();
    let timestamp: Str = timestamp.into();

    let mut root = TokenNode::new(next_id());
    root.tag = "div".into();
    root.layout = Layout::Col;
    root.class = "border-b border-gray-800".into();

    // ── Header row ────────────────────────────────────────────────────
    let mut header = TokenNode::new(next_id());
    header.tag = "div".into();
    header.layout = Layout::Row;
    header.class = "px-4 py-2.5 gap-3 items-center".into();

    // Avatar ring (clickable)
    let mut ring = TokenNode::new(next_id());
    ring.tag = "div".into();
    ring.class = "w-9 h-9 rounded-full p-0.5 bg-gradient-to-tr from-yellow-400 via-pink-500 to-purple-600 flex-shrink-0 cursor-pointer".into();
    ring.actions.push(TokenAction::StoreSet {
        key: "ig/viewing/user_id".into(),
        value: user_id,
    });
    ring.actions.push(TokenAction::Navigate("instagram_profile".into()));

    let mut avatar = TokenNode::new(next_id());
    avatar.tag = "img".into();
    avatar.content = Some(avatar_url);
    avatar.class = "w-full h-full rounded-full border-2 border-black object-cover".into();
    ring.children.push(avatar);
    header.children.push(ring);

    // Name + handle col
    let mut name_col = TokenNode::new(next_id());
    name_col.tag = "div".into();
    name_col.layout = Layout::Col;
    name_col.class = "flex-1 gap-0".into();

    let mut name_node = TokenNode::new(next_id());
    name_node.content = Some(name.clone());
    name_node.class = "text-sm font-semibold".into();
    name_col.children.push(name_node);

    let mut handle_node = TokenNode::new(next_id());
    handle_node.content = Some(handle);
    handle_node.class = "text-xs text-gray-500".into();
    name_col.children.push(handle_node);

    header.children.push(name_col);

    // More button
    let mut more_btn = TokenNode::new(next_id());
    more_btn.tag = "button".into();
    more_btn.content = Some("\u{2022}\u{2022}\u{2022}".into());
    more_btn.class = "inline-flex items-center justify-center gap-1 rounded font-medium transition-colors whitespace-nowrap text-gray-500 text-lg p-0".into();
    more_btn.style.extra = "cursor:pointer;user-select:none;background:transparent;border:none;color:inherit;".into();
    header.children.push(more_btn);

    root.children.push(header);

    // ── Image (clickable → post detail) ───────────────────────────────
    let mut img_wrap = TokenNode::new(next_id());
    img_wrap.tag = "div".into();
    img_wrap.class = "w-full bg-gray-900 overflow-hidden cursor-pointer".into();
    img_wrap.actions.push(TokenAction::StoreSet {
        key: "ig/viewing/post_id".into(),
        value: post_id.clone(),
    });
    img_wrap.actions.push(TokenAction::Navigate("instagram_post".into()));

    let mut img = TokenNode::new(next_id());
    img.tag = "img".into();
    img.content = Some(image_url);
    img.class = "w-full aspect-square object-cover block".into();
    img_wrap.children.push(img);

    root.children.push(img_wrap);

    // ── Action row ────────────────────────────────────────────────────
    let action_row_node = ig_action_row(post_id).into_node();
    root.children.push(action_row_node);

    // ── Caption section ───────────────────────────────────────────────
    let mut cap_col = TokenNode::new(next_id());
    cap_col.tag = "div".into();
    cap_col.layout = Layout::Col;
    cap_col.class = "px-4 pb-3 gap-0.5".into();

    let mut likes_node = TokenNode::new(next_id());
    likes_node.content = Some(likes);
    likes_node.class = "text-sm font-semibold".into();
    cap_col.children.push(likes_node);

    // Author name + caption in a row
    let mut cap_row = TokenNode::new(next_id());
    cap_row.tag = "div".into();
    cap_row.layout = Layout::Row;
    cap_row.class = "gap-1.5 flex-wrap items-baseline".into();

    let mut author = TokenNode::new(next_id());
    author.content = Some(name);
    author.class = "text-sm font-semibold".into();
    cap_row.children.push(author);

    let mut cap_text = TokenNode::new(next_id());
    cap_text.content = Some(caption);
    cap_text.class = "text-sm text-gray-200 line-clamp-2".into();
    cap_row.children.push(cap_text);

    cap_col.children.push(cap_row);

    let mut time_node = TokenNode::new(next_id());
    time_node.content = Some(timestamp);
    time_node.class = "text-xs text-gray-600 uppercase tracking-wide mt-0.5".into();
    cap_col.children.push(time_node);

    root.children.push(cap_col);

    Container { stack: vec![root] }
}
