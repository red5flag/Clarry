// src/data/app_data.rs
//
// Application-level typed seed data using the storage schema system.
// This module defines concrete schemas for the page variants and
// provides factory functions to load seed data into the TokenStore.

use crate::tokens::schema::*;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// ── Seed data structs ────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub display_name: String,
    pub handle: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers: u32,
    pub following: u32,
    pub posts_count: u32,
    pub verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaPost {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub author_avatar: String,
    pub content: String,
    pub image_url: Option<String>,
    pub likes: u32,
    pub comments: u32,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_avatar: String,
    pub text: String,
    pub timestamp: String,
    pub is_me: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductItem {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub currency: String,
    pub image_url: String,
    pub rating: f32,
    pub reviews: u32,
    pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardMetric {
    pub label: String,
    pub value: String,
    pub change: f32,
    pub is_positive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub source: String,
    pub summary: String,
    pub image_url: Option<String>,
    pub timestamp: String,
    pub read_time: u32,
}

// ── Seed data ────────────────────────────────────────────────────────────────

static SEED_USERS: OnceLock<Vec<UserProfile>> = OnceLock::new();
static SEED_POSTS: OnceLock<Vec<MediaPost>> = OnceLock::new();
static SEED_MESSAGES: OnceLock<Vec<ChatMessage>> = OnceLock::new();
static SEED_PRODUCTS: OnceLock<Vec<ProductItem>> = OnceLock::new();
static SEED_METRICS: OnceLock<Vec<DashboardMetric>> = OnceLock::new();
static SEED_FEED: OnceLock<Vec<FeedItem>> = OnceLock::new();

pub fn seed_users() -> &'static [UserProfile] {
    SEED_USERS.get_or_init(|| vec![
        UserProfile {
            id: "alice".into(),
            display_name: "Alice Chen".into(),
            handle: "@alicechen".into(),
            avatar_url: "https://i.pravatar.cc/150?u=alice".into(),
            bio: "Design systems engineer. Building accessible UI.".into(),
            followers: 12400,
            following: 340,
            posts_count: 89,
            verified: true,
        },
        UserProfile {
            id: "bob".into(),
            display_name: "Bob Ross".into(),
            handle: "@bobross".into(),
            avatar_url: "https://i.pravatar.cc/150?u=bob".into(),
            bio: "Happy little trees.".into(),
            followers: 56000,
            following: 12,
            posts_count: 420,
            verified: true,
        },
    ])
}

pub fn seed_posts() -> &'static [MediaPost] {
    SEED_POSTS.get_or_init(|| vec![
        MediaPost {
            id: "p1".into(),
            author_id: "alice".into(),
            author_name: "Alice Chen".into(),
            author_avatar: "https://i.pravatar.cc/150?u=alice".into(),
            content: "Just shipped a new design system update. The token DSL is getting really smooth.".into(),
            image_url: Some("https://picsum.photos/600/400?random=1".into()),
            likes: 234,
            comments: 18,
            timestamp: "2h ago".into(),
        },
        MediaPost {
            id: "p2".into(),
            author_id: "bob".into(),
            author_name: "Bob Ross".into(),
            author_avatar: "https://i.pravatar.cc/150?u=bob".into(),
            content: "Every day is a good day when you paint.".into(),
            image_url: Some("https://picsum.photos/600/400?random=2".into()),
            likes: 3400,
            comments: 156,
            timestamp: "5h ago".into(),
        },
        MediaPost {
            id: "p3".into(),
            author_id: "alice".into(),
            author_name: "Alice Chen".into(),
            author_avatar: "https://i.pravatar.cc/150?u=alice".into(),
            content: "Working on some new animation primitives for the DSL. Scroll-triggered keyframes are surprisingly elegant.".into(),
            image_url: None,
            likes: 89,
            comments: 7,
            timestamp: "1d ago".into(),
        },
    ])
}

pub fn seed_messages() -> &'static [ChatMessage] {
    SEED_MESSAGES.get_or_init(|| vec![
        ChatMessage {
            id: "m1".into(),
            sender_id: "alice".into(),
            sender_name: "Alice".into(),
            sender_avatar: "https://i.pravatar.cc/150?u=alice".into(),
            text: "Hey! Have you seen the new token DSL preprocessor?".into(),
            timestamp: "10:23 AM".into(),
            is_me: false,
        },
        ChatMessage {
            id: "m2".into(),
            sender_id: "me".into(),
            sender_name: "You".into(),
            sender_avatar: "https://i.pravatar.cc/150?u=me".into(),
            text: "Yeah, the indentation handling is much better now.".into(),
            timestamp: "10:25 AM".into(),
            is_me: true,
        },
        ChatMessage {
            id: "m3".into(),
            sender_id: "alice".into(),
            sender_name: "Alice".into(),
            sender_avatar: "https://i.pravatar.cc/150?u=alice".into(),
            text: "And the schema system means we can finally type the storage layer.".into(),
            timestamp: "10:26 AM".into(),
            is_me: false,
        },
    ])
}

pub fn seed_products() -> &'static [ProductItem] {
    SEED_PRODUCTS.get_or_init(|| vec![
        ProductItem {
            id: "prod1".into(),
            name: "Minimal Desk Lamp".into(),
            price: 89.99,
            currency: "USD".into(),
            image_url: "https://picsum.photos/400/400?random=3".into(),
            rating: 4.7,
            reviews: 128,
            category: "Home".into(),
        },
        ProductItem {
            id: "prod2".into(),
            name: "Mechanical Keyboard".into(),
            price: 149.00,
            currency: "USD".into(),
            image_url: "https://picsum.photos/400/400?random=4".into(),
            rating: 4.9,
            reviews: 342,
            category: "Tech".into(),
        },
        ProductItem {
            id: "prod3".into(),
            name: "Ceramic Vase Set".into(),
            price: 45.00,
            currency: "USD".into(),
            image_url: "https://picsum.photos/400/400?random=5".into(),
            rating: 4.5,
            reviews: 67,
            category: "Home".into(),
        },
    ])
}

pub fn seed_metrics() -> &'static [DashboardMetric] {
    SEED_METRICS.get_or_init(|| vec![
        DashboardMetric {
            label: "Total Revenue".into(),
            value: "$48,290".into(),
            change: 12.5,
            is_positive: true,
        },
        DashboardMetric {
            label: "Active Users".into(),
            value: "2,845".into(),
            change: 8.2,
            is_positive: true,
        },
        DashboardMetric {
            label: "Bounce Rate".into(),
            value: "24.3%".into(),
            change: -2.1,
            is_positive: true,
        },
        DashboardMetric {
            label: "Avg Session".into(),
            value: "4m 12s".into(),
            change: -0.5,
            is_positive: false,
        },
    ])
}

pub fn seed_feed() -> &'static [FeedItem] {
    SEED_FEED.get_or_init(|| vec![
        FeedItem {
            id: "f1".into(),
            title: "The Future of Rust UI Frameworks".into(),
            source: "Rust Blog".into(),
            summary: "Exploring how token-based DSLs and fine-grained reactivity are shaping the next generation of web UI.".into(),
            image_url: Some("https://picsum.photos/300/200?random=6".into()),
            timestamp: "2h ago".into(),
            read_time: 6,
        },
        FeedItem {
            id: "f2".into(),
            title: "Designing Accessible Component Libraries".into(),
            source: "A11y Weekly".into(),
            summary: "Practical patterns for building UI primitives that work for everyone.".into(),
            image_url: Some("https://picsum.photos/300/200?random=7".into()),
            timestamp: "5h ago".into(),
            read_time: 4,
        },
        FeedItem {
            id: "f3".into(),
            title: "WASM Performance Deep Dive".into(),
            source: "WebPerf".into(),
            summary: "Understanding memory layouts and garbage collection in modern WebAssembly runtimes.".into(),
            image_url: None,
            timestamp: "1d ago".into(),
            read_time: 12,
        },
    ])
}

// ── Instagram storage seed ───────────────────────────────────────────────────

/// Seed all Instagram storage keys once per session.
/// Profiles: alice (me), bob, charlie, diana, eve, frank, grace, henry
/// Includes: posts, reels, stories, DMs (realistic chat logs), comments,
///           notifications, likes/saves/follows, highlights, tagged posts,
///           explore grid, search index — all VLM-ready training data.
pub fn seed_instagram_storage() {
    use crate::tokens::storage::primitive::Store;

    // ── Alice Chen (current user / "me") ─────────────────────────────────────
    Store::write("ig.me.name",        "Alice Chen");
    Store::write("ig.me.handle",      "@alicechen");
    Store::write("ig.me.bio",         "Design systems engineer ✦ Building accessible UI 🛠️\nOpen source · alicechen.dev");
    Store::write("ig.me.avatar",      "https://i.pravatar.cc/150?u=alice");
    Store::write("ig.me.followers",   "12.4K");
    Store::write("ig.me.following",   "340");
    Store::write("ig.me.posts_count", "6");
    Store::write("ig.me.website",     "alicechen.dev");
    Store::write("ig.me.verified",    "false");

    let alice_posts = serde_json::json!([
        {
            "id": "p_alice_1", "author_id": "alice", "author_name": "Alice Chen",
            "author_handle": "@alicechen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
            "caption": "Just shipped token DSL v2 🚀 Indentation-based syntax, zero boilerplate. #design #rust",
            "image_url": "https://picsum.photos/600/600?random=10",
            "likes": 2340, "comments": 48, "saves": 312, "timestamp": "2h ago",
            "location": "San Francisco, CA", "tags": ["grace", "eve"], "type": "photo"
        },
        {
            "id": "p_alice_2", "author_id": "alice", "author_name": "Alice Chen",
            "author_handle": "@alicechen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
            "caption": "Scroll-triggered animations are live ✨ Zero JS, pure DSL. Try the demo!",
            "image_url": "https://picsum.photos/600/600?random=11",
            "likes": 1890, "comments": 34, "saves": 245, "timestamp": "1d ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_alice_3", "author_id": "alice", "author_name": "Alice Chen",
            "author_handle": "@alicechen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
            "caption": "Morning coffee + code ☕ Working on accessibility primitives today. Skip links, live regions, all of it.",
            "image_url": "https://picsum.photos/600/600?random=12",
            "likes": 876, "comments": 21, "saves": 88, "timestamp": "3d ago",
            "location": "Hayes Valley, SF", "tags": [], "type": "photo"
        },
        {
            "id": "p_alice_4", "author_id": "alice", "author_name": "Alice Chen",
            "author_handle": "@alicechen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
            "caption": "New primitive: card() — auto-handles shadow, radius, bg. One line of DSL 🃏",
            "image_url": "https://picsum.photos/600/600?random=13",
            "likes": 1120, "comments": 29, "saves": 156, "timestamp": "5d ago",
            "location": "", "tags": ["eve"], "type": "photo"
        },
        {
            "id": "p_alice_5", "author_id": "alice", "author_name": "Alice Chen",
            "author_handle": "@alicechen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
            "caption": "Back from @gracekim.design's sprint 🎨 So much inspiration. The future of UI tooling is wild",
            "image_url": "https://picsum.photos/600/600?random=14",
            "likes": 654, "comments": 17, "saves": 42, "timestamp": "1w ago",
            "location": "Brooklyn, NY", "tags": ["grace"], "type": "photo"
        },
        {
            "id": "p_alice_reel_1", "author_id": "alice", "author_name": "Alice Chen",
            "author_handle": "@alicechen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
            "caption": "Live coding a full Instagram UI in pure DSL 😤 No React, no templates #coding #rust",
            "image_url": "https://picsum.photos/400/700?random=15",
            "video_url": "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4",
            "likes": 8900, "comments": 234, "saves": 1200, "plays": "42K",
            "timestamp": "2w ago", "location": "", "tags": [], "type": "reel"
        }
    ]);
    Store::write("ig.me.posts", &alice_posts.to_string());

    // ── Bob Ross — verified artist ────────────────────────────────────────────
    Store::write("ig.users.bob.name",        "Bob Ross");
    Store::write("ig.users.bob.handle",      "@bobross");
    Store::write("ig.users.bob.bio",         "Painter 🎨 Happy little trees 🌲\nJoy of Painting instructor · joyofpainting.com");
    Store::write("ig.users.bob.avatar",      "https://i.pravatar.cc/150?u=bob");
    Store::write("ig.users.bob.followers",   "560K");
    Store::write("ig.users.bob.following",   "12");
    Store::write("ig.users.bob.posts_count", "5");
    Store::write("ig.users.bob.website",     "joyofpainting.com");
    Store::write("ig.users.bob.verified",    "true");

    let bob_posts = serde_json::json!([
        {
            "id": "p_bob_1", "author_id": "bob", "author_name": "Bob Ross",
            "author_handle": "@bobross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
            "caption": "Every day is a good day when you paint 🎨 New canvas — going for autumn colours. What should I name this one?",
            "image_url": "https://picsum.photos/600/600?random=20",
            "likes": 34000, "comments": 1560, "saves": 4200, "timestamp": "5h ago",
            "location": "Studio, Oregon", "tags": [], "type": "photo"
        },
        {
            "id": "p_bob_2", "author_id": "bob", "author_name": "Bob Ross",
            "author_handle": "@bobross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
            "caption": "There are no mistakes, only happy accidents 💛 This one started as grey sky and became something unexpected 🌈",
            "image_url": "https://picsum.photos/600/600?random=21",
            "likes": 21000, "comments": 880, "saves": 3100, "timestamp": "3d ago",
            "location": "Studio, Oregon", "tags": [], "type": "photo"
        },
        {
            "id": "p_bob_3", "author_id": "bob", "author_name": "Bob Ross",
            "author_handle": "@bobross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
            "caption": "Mountains are so peaceful 🏔️ I could paint them every day. #landscape #oilpainting",
            "image_url": "https://picsum.photos/600/600?random=22",
            "likes": 18400, "comments": 445, "saves": 2800, "timestamp": "1w ago",
            "location": "Rocky Mountains", "tags": ["charlie"], "type": "photo"
        },
        {
            "id": "p_bob_4", "author_id": "bob", "author_name": "Bob Ross",
            "author_handle": "@bobross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
            "caption": "My palette setup 🎨 Titanium white is the most important colour you own. Period.",
            "image_url": "https://picsum.photos/600/600?random=23",
            "likes": 29000, "comments": 2100, "saves": 5600, "timestamp": "2w ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_bob_reel_1", "author_id": "bob", "author_name": "Bob Ross",
            "author_handle": "@bobross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
            "caption": "Speed paint — complete landscape in 60 seconds 🎬 Full tutorial on YouTube. Happy little accidents only 🌲✨",
            "image_url": "https://picsum.photos/400/700?random=24",
            "video_url": "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            "likes": 92000, "comments": 4400, "saves": 12000, "plays": "2.1M",
            "timestamp": "3w ago", "location": "", "tags": [], "type": "reel"
        }
    ]);
    Store::write("ig.users.bob.posts", &bob_posts.to_string());

    // ── Charlie Day — street photographer ────────────────────────────────────
    Store::write("ig.users.charlie.name",        "Charlie Day");
    Store::write("ig.users.charlie.handle",      "@charlieday");
    Store::write("ig.users.charlie.bio",         "Street photography 📸 Finding light in the mundane\nNYC · Commissions open · charlieday.photo");
    Store::write("ig.users.charlie.avatar",      "https://i.pravatar.cc/150?u=charlie");
    Store::write("ig.users.charlie.followers",   "8.9K");
    Store::write("ig.users.charlie.following",   "420");
    Store::write("ig.users.charlie.posts_count", "4");
    Store::write("ig.users.charlie.website",     "charlieday.photo");
    Store::write("ig.users.charlie.verified",    "false");

    let charlie_posts = serde_json::json!([
        {
            "id": "p_charlie_1", "author_id": "charlie", "author_name": "Charlie Day",
            "author_handle": "@charlieday", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
            "caption": "Golden hour in the city 🌆 That 20-minute window where everything turns gold. Worth waking up at 5am every time.",
            "image_url": "https://picsum.photos/600/600?random=30",
            "likes": 8910, "comments": 320, "saves": 890, "timestamp": "12h ago",
            "location": "Brooklyn Bridge, NYC", "tags": ["bob"], "type": "photo"
        },
        {
            "id": "p_charlie_2", "author_id": "charlie", "author_name": "Charlie Day",
            "author_handle": "@charlieday", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
            "caption": "Subway life 🚇 Everyone in their own world. I love capturing these quiet moments between the noise.",
            "image_url": "https://picsum.photos/600/600?random=31",
            "likes": 5400, "comments": 124, "saves": 540, "timestamp": "2d ago",
            "location": "L Train, NYC", "tags": [], "type": "photo"
        },
        {
            "id": "p_charlie_3", "author_id": "charlie", "author_name": "Charlie Day",
            "author_handle": "@charlieday", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
            "caption": "Rain makes the best photos. The reflections, the mood, everyone rushing. Pure cinema 🌧️ #streetphotography #nyc",
            "image_url": "https://picsum.photos/600/600?random=32",
            "likes": 6700, "comments": 198, "saves": 720, "timestamp": "4d ago",
            "location": "Midtown Manhattan", "tags": [], "type": "photo"
        },
        {
            "id": "p_charlie_4", "author_id": "charlie", "author_name": "Charlie Day",
            "author_handle": "@charlieday", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
            "caption": "Gear breakdown 📷 Leica Q3, 28mm — that's it. Less gear = more presence.",
            "image_url": "https://picsum.photos/600/600?random=33",
            "likes": 3200, "comments": 410, "saves": 1100, "timestamp": "1w ago",
            "location": "", "tags": [], "type": "photo"
        }
    ]);
    Store::write("ig.users.charlie.posts", &charlie_posts.to_string());

    // ── Diana Prince — travel & food blogger ─────────────────────────────────
    Store::write("ig.users.diana.name",        "Diana Prince");
    Store::write("ig.users.diana.handle",      "@dianaprince");
    Store::write("ig.users.diana.bio",         "Travel ✈️ Food 🍜 Currently: Tokyo 🗼\n30 countries · always hungry · eatwithdianaprince.com");
    Store::write("ig.users.diana.avatar",      "https://i.pravatar.cc/150?u=diana");
    Store::write("ig.users.diana.followers",   "241K");
    Store::write("ig.users.diana.following",   "880");
    Store::write("ig.users.diana.posts_count", "5");
    Store::write("ig.users.diana.website",     "eatwithdianaprince.com");
    Store::write("ig.users.diana.verified",    "true");

    let diana_posts = serde_json::json!([
        {
            "id": "p_diana_1", "author_id": "diana", "author_name": "Diana Prince",
            "author_handle": "@dianaprince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
            "caption": "Ramen at midnight hits different 🍜 Tokyo never sleeps. Ichiran Shibuya — highly recommend.",
            "image_url": "https://picsum.photos/600/600?random=40",
            "likes": 18200, "comments": 740, "saves": 2900, "timestamp": "3h ago",
            "location": "Ichiran, Shibuya, Tokyo", "tags": [], "type": "photo"
        },
        {
            "id": "p_diana_2", "author_id": "diana", "author_name": "Diana Prince",
            "author_handle": "@dianaprince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
            "caption": "Shibuya crossing at 6am 🌅 Before the crowds. The city belongs to you in those early hours.",
            "image_url": "https://picsum.photos/600/600?random=41",
            "likes": 32000, "comments": 1200, "saves": 4500, "timestamp": "1d ago",
            "location": "Shibuya Crossing, Tokyo", "tags": ["charlie"], "type": "photo"
        },
        {
            "id": "p_diana_3", "author_id": "diana", "author_name": "Diana Prince",
            "author_handle": "@dianaprince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
            "caption": "Tsukiji fish market morning auction 🐟 Nothing compares. The energy, the speed, the freshness!",
            "image_url": "https://picsum.photos/600/600?random=42",
            "likes": 14500, "comments": 330, "saves": 1800, "timestamp": "3d ago",
            "location": "Tsukiji Market, Tokyo", "tags": [], "type": "photo"
        },
        {
            "id": "p_diana_4", "author_id": "diana", "author_name": "Diana Prince",
            "author_handle": "@dianaprince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
            "caption": "Matcha everything ☕ Rated 8 matcha lattes in Kyoto. Blog post coming — link in bio!",
            "image_url": "https://picsum.photos/600/600?random=43",
            "likes": 9800, "comments": 520, "saves": 1200, "timestamp": "5d ago",
            "location": "Kyoto, Japan", "tags": ["henry"], "type": "photo"
        },
        {
            "id": "p_diana_reel_1", "author_id": "diana", "author_name": "Diana Prince",
            "author_handle": "@dianaprince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
            "caption": "48 hours in Tokyo 🗼 Everything I ate, everywhere I went. This city is insane in the best way 🙏�",
            "image_url": "https://picsum.photos/400/700?random=44",
            "video_url": "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
            "likes": 145000, "comments": 6800, "saves": 22000, "plays": "8.4M",
            "timestamp": "1w ago", "location": "Tokyo, Japan", "tags": [], "type": "reel"
        }
    ]);
    Store::write("ig.users.diana.posts", &diana_posts.to_string());

    // ── Eve Nakamura — UX researcher ──────────────────────────────────────────
    Store::write("ig.users.eve.name",        "Eve Nakamura");
    Store::write("ig.users.eve.handle",      "@eve.nakamura");
    Store::write("ig.users.eve.bio",         "UX Researcher @ Linear 💙 Making things simpler\nMinimalism · Systems thinking · Coffee");
    Store::write("ig.users.eve.avatar",      "https://i.pravatar.cc/150?u=eve");
    Store::write("ig.users.eve.followers",   "6.7K");
    Store::write("ig.users.eve.following",   "310");
    Store::write("ig.users.eve.posts_count", "4");
    Store::write("ig.users.eve.website",     "");
    Store::write("ig.users.eve.verified",    "false");

    let eve_posts = serde_json::json!([
        {
            "id": "p_eve_1", "author_id": "eve", "author_name": "Eve Nakamura",
            "author_handle": "@eve.nakamura", "author_avatar": "https://i.pravatar.cc/150?u=eve",
            "caption": "Home office v3 🖥️ Finally nailed the minimalist desk. Less on the desk, more in the mind.",
            "image_url": "https://picsum.photos/600/600?random=50",
            "likes": 4450, "comments": 290, "saves": 780, "timestamp": "6h ago",
            "location": "Tokyo, Japan", "tags": ["alice"], "type": "photo"
        },
        {
            "id": "p_eve_2", "author_id": "eve", "author_name": "Eve Nakamura",
            "author_handle": "@eve.nakamura", "author_avatar": "https://i.pravatar.cc/150?u=eve",
            "caption": "User testing day 🎯 17 participants, 6 hours. Every session teaches you something you thought you knew.",
            "image_url": "https://picsum.photos/600/600?random=51",
            "likes": 3120, "comments": 150, "saves": 290, "timestamp": "2d ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_eve_3", "author_id": "eve", "author_name": "Eve Nakamura",
            "author_handle": "@eve.nakamura", "author_avatar": "https://i.pravatar.cc/150?u=eve",
            "caption": "Reading stack 📚 'The Design of Everyday Things'. Bad UX is never the user's fault.",
            "image_url": "https://picsum.photos/600/600?random=52",
            "likes": 2800, "comments": 112, "saves": 490, "timestamp": "5d ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_eve_4", "author_id": "eve", "author_name": "Eve Nakamura",
            "author_handle": "@eve.nakamura", "author_avatar": "https://i.pravatar.cc/150?u=eve",
            "caption": "Walked through @alicechen's new DSL demo. The a11y primitives are genuinely impressive 💙",
            "image_url": "https://picsum.photos/600/600?random=53",
            "likes": 1980, "comments": 88, "saves": 230, "timestamp": "1w ago",
            "location": "", "tags": ["alice"], "type": "photo"
        }
    ]);
    Store::write("ig.users.eve.posts", &eve_posts.to_string());

    // ── Frank Garcia — fitness & lifestyle ───────────────────────────────────
    Store::write("ig.users.frank.name",        "Frank Garcia");
    Store::write("ig.users.frank.handle",      "@frankgarcia");
    Store::write("ig.users.frank.bio",         "Fitness coach 💪 Plant-based athlete 🌱\nOnline coaching · 5am club · frankgarcia.fit");
    Store::write("ig.users.frank.avatar",      "https://i.pravatar.cc/150?u=frank");
    Store::write("ig.users.frank.followers",   "89.2K");
    Store::write("ig.users.frank.following",   "650");
    Store::write("ig.users.frank.posts_count", "4");
    Store::write("ig.users.frank.website",     "frankgarcia.fit");
    Store::write("ig.users.frank.verified",    "false");

    let frank_posts = serde_json::json!([
        {
            "id": "p_frank_1", "author_id": "frank", "author_name": "Frank Garcia",
            "author_handle": "@frankgarcia", "author_avatar": "https://i.pravatar.cc/150?u=frank",
            "caption": "5am crew ☀️ Nobody ever regretted the early morning workout. Hardest part is getting out of bed.",
            "image_url": "https://picsum.photos/600/600?random=70",
            "likes": 12400, "comments": 380, "saves": 1400, "timestamp": "4h ago",
            "location": "Gold's Gym, LA", "tags": [], "type": "photo"
        },
        {
            "id": "p_frank_2", "author_id": "frank", "author_name": "Frank Garcia",
            "author_handle": "@frankgarcia", "author_avatar": "https://i.pravatar.cc/150?u=frank",
            "caption": "Plant-based meal prep Sunday 🥗 5 days of food in 2 hours. Recipe in stories!",
            "image_url": "https://picsum.photos/600/600?random=71",
            "likes": 8900, "comments": 620, "saves": 3200, "timestamp": "2d ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_frank_3", "author_id": "frank", "author_name": "Frank Garcia",
            "author_handle": "@frankgarcia", "author_avatar": "https://i.pravatar.cc/150?u=frank",
            "caption": "Before vs After — 1 year of consistency � No shortcuts. No secrets. Just showing up every day.",
            "image_url": "https://picsum.photos/600/600?random=72",
            "likes": 45000, "comments": 2800, "saves": 8900, "timestamp": "1w ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_frank_reel_1", "author_id": "frank", "author_name": "Frank Garcia",
            "author_handle": "@frankgarcia", "author_avatar": "https://i.pravatar.cc/150?u=frank",
            "caption": "Full body workout anywhere 🔥 No equipment. 20 minutes, all out. Save this for tomorrow!",
            "image_url": "https://picsum.photos/400/700?random=73",
            "video_url": "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/SubaruOutbackOnStreetAndDirt.mp4",
            "likes": 234000, "comments": 8900, "saves": 45000, "plays": "12.8M",
            "timestamp": "2w ago", "location": "", "tags": [], "type": "reel"
        }
    ]);
    Store::write("ig.users.frank.posts", &frank_posts.to_string());

    // ── Grace Kim — graphic designer ─────────────────────────────────────────
    Store::write("ig.users.grace.name",        "Grace Kim");
    Store::write("ig.users.grace.handle",      "@gracekim.design");
    Store::write("ig.users.grace.bio",         "Creative Director @ Figma 🎨 Typography nerd · Brand identity\ngracekim.design");
    Store::write("ig.users.grace.avatar",      "https://i.pravatar.cc/150?u=grace");
    Store::write("ig.users.grace.followers",   "34.1K");
    Store::write("ig.users.grace.following",   "1200");
    Store::write("ig.users.grace.posts_count", "4");
    Store::write("ig.users.grace.website",     "gracekim.design");
    Store::write("ig.users.grace.verified",    "false");

    let grace_posts = serde_json::json!([
        {
            "id": "p_grace_1", "author_id": "grace", "author_name": "Grace Kim",
            "author_handle": "@gracekim.design", "author_avatar": "https://i.pravatar.cc/150?u=grace",
            "caption": "New brand identity reveal 🎨 6 months of work. The logo has a hidden 'g' — how long to see it?",
            "image_url": "https://picsum.photos/600/600?random=80",
            "likes": 18600, "comments": 890, "saves": 4200, "timestamp": "8h ago",
            "location": "Brooklyn, NY", "tags": ["alice"], "type": "photo"
        },
        {
            "id": "p_grace_2", "author_id": "grace", "author_name": "Grace Kim",
            "author_handle": "@gracekim.design", "author_avatar": "https://i.pravatar.cc/150?u=grace",
            "caption": "Typography is not decoration. It's architecture 🔤 Obsessing over kerning again. Send help.",
            "image_url": "https://picsum.photos/600/600?random=81",
            "likes": 12400, "comments": 340, "saves": 2800, "timestamp": "2d ago",
            "location": "", "tags": [], "type": "photo"
        },
        {
            "id": "p_grace_3", "author_id": "grace", "author_name": "Grace Kim",
            "author_handle": "@gracekim.design", "author_avatar": "https://i.pravatar.cc/150?u=grace",
            "caption": "Design sprint with @alicechen ⚡ Two days, one product, hundreds of sticky notes. Electric energy.",
            "image_url": "https://picsum.photos/600/600?random=82",
            "likes": 7800, "comments": 210, "saves": 560, "timestamp": "1w ago",
            "location": "Figma HQ, SF", "tags": ["alice", "eve"], "type": "photo"
        },
        {
            "id": "p_grace_reel_1", "author_id": "grace", "author_name": "Grace Kim",
            "author_handle": "@gracekim.design", "author_avatar": "https://i.pravatar.cc/150?u=grace",
            "caption": "Logo design process — sketch to final in 60 seconds ✏️ Bold but approachable brief. 12 concepts 🎨",
            "image_url": "https://picsum.photos/400/700?random=83",
            "video_url": "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4",
            "likes": 56000, "comments": 2400, "saves": 12000, "plays": "3.2M",
            "timestamp": "3w ago", "location": "", "tags": [], "type": "reel"
        }
    ]);
    Store::write("ig.users.grace.posts", &grace_posts.to_string());

    // ── Henry Park — staff engineer ───────────────────────────────────────────
    Store::write("ig.users.henry.name",        "Henry Park");
    Store::write("ig.users.henry.handle",      "@henrypark.dev");
    Store::write("ig.users.henry.bio",         "Staff Eng @ Vercel 🚀 WebPerf · Rust 🦀 · DX obsessed\nhenrypark.dev");
    Store::write("ig.users.henry.avatar",      "https://i.pravatar.cc/150?u=henry");
    Store::write("ig.users.henry.followers",   "28.4K");
    Store::write("ig.users.henry.following",   "890");
    Store::write("ig.users.henry.posts_count", "4");
    Store::write("ig.users.henry.website",     "henrypark.dev");
    Store::write("ig.users.henry.verified",    "false");

    let henry_posts = serde_json::json!([
        {
            "id": "p_henry_1", "author_id": "henry", "author_name": "Henry Park",
            "author_handle": "@henrypark.dev", "author_avatar": "https://i.pravatar.cc/150?u=henry",
            "caption": "100ms TTI in production 🚀 WebAssembly + streaming SSR + zero-cost abstractions. Full breakdown on blog.",
            "image_url": "https://picsum.photos/600/600?random=90",
            "likes": 9800, "comments": 450, "saves": 1800, "timestamp": "1h ago",
            "location": "", "tags": ["alice"], "type": "photo"
        },
        {
            "id": "p_henry_2", "author_id": "henry", "author_name": "Henry Park",
            "author_handle": "@henrypark.dev", "author_avatar": "https://i.pravatar.cc/150?u=henry",
            "caption": "2026 dev setup 💻 MacBook Pro M4, 4K, Moonlander keyboard. Split keyboard took 2 weeks. Can't go back.",
            "image_url": "https://picsum.photos/600/600?random=91",
            "likes": 14200, "comments": 780, "saves": 2400, "timestamp": "3d ago",
            "location": "San Francisco, CA", "tags": [], "type": "photo"
        },
        {
            "id": "p_henry_3", "author_id": "henry", "author_name": "Henry Park",
            "author_handle": "@henrypark.dev", "author_avatar": "https://i.pravatar.cc/150?u=henry",
            "caption": "Rust is eating the web 🦀 @alicechen's token DSL is the abstraction the ecosystem needed. Thread 🧵",
            "image_url": "https://picsum.photos/600/600?random=92",
            "likes": 7600, "comments": 320, "saves": 940, "timestamp": "5d ago",
            "location": "", "tags": ["alice"], "type": "photo"
        },
        {
            "id": "p_henry_4", "author_id": "henry", "author_name": "Henry Park",
            "author_handle": "@henrypark.dev", "author_avatar": "https://i.pravatar.cc/150?u=henry",
            "caption": "Tokyo for Vercel ship week ✈️ First time in Japan. @dianaprince gave the best food recs. Tsukemen tonight 🍜",
            "image_url": "https://picsum.photos/600/600?random=93",
            "likes": 5400, "comments": 180, "saves": 340, "timestamp": "1w ago",
            "location": "Tokyo, Japan", "tags": ["diana"], "type": "photo"
        }
    ]);
    Store::write("ig.users.henry.posts", &henry_posts.to_string());

    // ── Home feed — newest-first interleaved ─────────────────────────────────
    let feed = serde_json::json!([
        henry_posts[0].clone(), grace_posts[0].clone(), alice_posts[0].clone(),
        diana_posts[0].clone(), frank_posts[0].clone(), bob_posts[0].clone(),
        charlie_posts[0].clone(), eve_posts[0].clone(),
        alice_posts[1].clone(), diana_posts[1].clone(), bob_posts[1].clone(),
        charlie_posts[1].clone(), eve_posts[1].clone(), henry_posts[1].clone(),
        grace_posts[1].clone(), alice_posts[2].clone(), diana_posts[2].clone(),
        frank_posts[1].clone(), bob_posts[2].clone(), charlie_posts[2].clone()
    ]);
    Store::write("ig.home.feed", &feed.to_string());

    // ── Reels feed ────────────────────────────────────────────────────────────
    let reels_feed = serde_json::json!([
        frank_posts[3].clone(), diana_posts[4].clone(), bob_posts[4].clone(),
        grace_posts[3].clone(), alice_posts[5].clone()
    ]);
    Store::write("ig.reels.feed", &reels_feed.to_string());

    // ── Stories ───────────────────────────────────────────────────────────────
    let stories = serde_json::json!([
        { "id": "me", "name": "Your Story", "avatar": "https://i.pravatar.cc/150?u=alice", "has_story": false, "slides": [] },
        { "id": "bob", "name": "Bob", "avatar": "https://i.pravatar.cc/150?u=bob", "has_story": true, "slides": [
            { "image": "https://picsum.photos/600/900?random=60", "caption": "New canvas today 🎨", "duration": 5000 },
            { "image": "https://picsum.photos/600/900?random=61", "caption": "Happy little trees ✨", "duration": 5000 },
            { "image": "https://picsum.photos/600/900?random=66", "caption": "Drying overnight 🌲", "duration": 5000 }
        ]},
        { "id": "charlie", "name": "Charlie", "avatar": "https://i.pravatar.cc/150?u=charlie", "has_story": true, "slides": [
            { "image": "https://picsum.photos/600/900?random=62", "caption": "Golden hour begins 📸", "duration": 5000 },
            { "image": "https://picsum.photos/600/900?random=67", "caption": "Found this shot 🌆", "duration": 5000 }
        ]},
        { "id": "diana", "name": "Diana", "avatar": "https://i.pravatar.cc/150?u=diana", "has_story": true, "slides": [
            { "image": "https://picsum.photos/600/900?random=63", "caption": "Tokyo morning ☀️", "duration": 5000 },
            { "image": "https://picsum.photos/600/900?random=64", "caption": "Late night ramen 🍜 Ichiran", "duration": 5000 },
            { "image": "https://picsum.photos/600/900?random=68", "caption": "Shibuya from above 🗼", "duration": 5000 }
        ]},
        { "id": "frank", "name": "Frank", "avatar": "https://i.pravatar.cc/150?u=frank", "has_story": true, "slides": [
            { "image": "https://picsum.photos/600/900?random=69", "caption": "5am workout done ✅", "duration": 5000 },
            { "image": "https://picsum.photos/600/900?random=74", "caption": "Meal prep time 🥗", "duration": 5000 }
        ]},
        { "id": "grace", "name": "Grace", "avatar": "https://i.pravatar.cc/150?u=grace", "has_story": true, "slides": [
            { "image": "https://picsum.photos/600/900?random=84", "caption": "New project reveal 🎨", "duration": 5000 }
        ]},
        { "id": "henry", "name": "Henry", "avatar": "https://i.pravatar.cc/150?u=henry", "has_story": true, "slides": [
            { "image": "https://picsum.photos/600/900?random=94", "caption": "Tokyo! First time in Japan 🇯🇵", "duration": 5000 }
        ]}
    ]);
    Store::write("ig.stories", &stories.to_string());

    // ── Rich comment threads ──────────────────────────────────────────────────
    Store::write("ig.comments.p_alice_1", &serde_json::json!([
        { "id": "c1", "author_id": "bob", "author_name": "Bob Ross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
          "text": "Love the new direction! The DSL feels very natural 🎨", "likes": 142, "timestamp": "1h ago" },
        { "id": "c2", "author_id": "charlie", "author_name": "Charlie Day", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
          "text": "Been waiting for this 🙌 When is the npm package dropping?", "likes": 48, "timestamp": "2h ago" },
        { "id": "c3", "author_id": "diana", "author_name": "Diana Prince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
          "text": "Can it handle complex animation sequences? The scroll-trigger stuff looks incredible", "likes": 24, "timestamp": "2h ago" },
        { "id": "c4", "author_id": "eve", "author_name": "Eve Nakamura", "author_avatar": "https://i.pravatar.cc/150?u=eve",
          "text": "The accessibility story looks solid 👏 Tested with VoiceOver — works perfectly", "likes": 89, "timestamp": "90m ago" },
        { "id": "c5", "author_id": "grace", "author_name": "Grace Kim", "author_avatar": "https://i.pravatar.cc/150?u=grace",
          "text": "The visual output is so clean! What renderer are you using under the hood?", "likes": 34, "timestamp": "45m ago" },
        { "id": "c6", "author_id": "henry", "author_name": "Henry Park", "author_avatar": "https://i.pravatar.cc/150?u=henry",
          "text": "This is exactly what Rust-based UI needed. Writing a post about it this week 🦀", "likes": 67, "timestamp": "30m ago" }
    ]).to_string());

    Store::write("ig.comments.p_bob_1", &serde_json::json!([
        { "id": "c10", "author_id": "alice", "author_name": "Alice Chen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
          "text": "Every stroke is intentional ❤️ The depth in that treeline is incredible", "likes": 2340, "timestamp": "4h ago" },
        { "id": "c11", "author_id": "diana", "author_name": "Diana Prince", "author_avatar": "https://i.pravatar.cc/150?u=diana",
          "text": "This is literally my phone wallpaper now 🌲 The colours are perfection", "likes": 880, "timestamp": "5h ago" },
        { "id": "c12", "author_id": "charlie", "author_name": "Charlie Day", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
          "text": "As a photographer — this composition is flawless. Rule of thirds, the lighting, everything", "likes": 445, "timestamp": "4h ago" },
        { "id": "c13", "author_id": "grace", "author_name": "Grace Kim", "author_avatar": "https://i.pravatar.cc/150?u=grace",
          "text": "The colour palette! Burnt sienna + titanium white is unbeatable 🎨", "likes": 320, "timestamp": "3h ago" },
        { "id": "c14", "author_id": "frank", "author_name": "Frank Garcia", "author_avatar": "https://i.pravatar.cc/150?u=frank",
          "text": "You make me want to pick up painting again. Incredible work as always 🙏", "likes": 156, "timestamp": "2h ago" }
    ]).to_string());

    Store::write("ig.comments.p_diana_1", &serde_json::json!([
        { "id": "c20", "author_id": "eve", "author_name": "Eve Nakamura", "author_avatar": "https://i.pravatar.cc/150?u=eve",
          "text": "Going to Tokyo next week! 🙏 Any ramen recs beyond Ichiran?", "likes": 54, "timestamp": "2h ago" },
        { "id": "c21", "author_id": "bob", "author_name": "Bob Ross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
          "text": "The light in this photo is absolute perfection. The photographer in you is showing 📸", "likes": 312, "timestamp": "3h ago" },
        { "id": "c22", "author_id": "henry", "author_name": "Henry Park", "author_avatar": "https://i.pravatar.cc/150?u=henry",
          "text": "Just arrived in Tokyo! DM'd you for that Ichiran rec 🍜", "likes": 28, "timestamp": "1h ago" },
        { "id": "c23", "author_id": "charlie", "author_name": "Charlie Day", "author_avatar": "https://i.pravatar.cc/150?u=charlie",
          "text": "This shot is everything. The bokeh, the noodles in focus. Frame-worthy 📷", "likes": 190, "timestamp": "2h ago" }
    ]).to_string());

    Store::write("ig.comments.p_charlie_1", &serde_json::json!([
        { "id": "c30", "author_id": "alice", "author_name": "Alice Chen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
          "text": "This is stunning 😍 The way the light hits the buildings. Brooklyn Bridge?", "likes": 234, "timestamp": "10h ago" },
        { "id": "c31", "author_id": "bob", "author_name": "Bob Ross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
          "text": "That golden hour — it's like nature painting itself 🌅 Beautiful capture", "likes": 567, "timestamp": "11h ago" },
        { "id": "c32", "author_id": "grace", "author_name": "Grace Kim", "author_avatar": "https://i.pravatar.cc/150?u=grace",
          "text": "The composition is so strong. Would love to print this large format for our office 🖼️", "likes": 145, "timestamp": "9h ago" },
        { "id": "c33", "author_id": "frank", "author_name": "Frank Garcia", "author_avatar": "https://i.pravatar.cc/150?u=frank",
          "text": "Was this shot on film? The grain and colours look incredible", "likes": 89, "timestamp": "8h ago" }
    ]).to_string());

    Store::write("ig.comments.p_eve_1", &serde_json::json!([
        { "id": "c40", "author_id": "alice", "author_name": "Alice Chen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
          "text": "That monitor arm setup 😍 Which arm is that? I need this immediately", "likes": 178, "timestamp": "5h ago" },
        { "id": "c41", "author_id": "henry", "author_name": "Henry Park", "author_avatar": "https://i.pravatar.cc/150?u=henry",
          "text": "The cable management alone deserves a follow 🙏 Super clean", "likes": 234, "timestamp": "4h ago" },
        { "id": "c42", "author_id": "grace", "author_name": "Grace Kim", "author_avatar": "https://i.pravatar.cc/150?u=grace",
          "text": "Minimalism goals. Saved this for my next desk iteration 💾", "likes": 112, "timestamp": "3h ago" }
    ]).to_string());

    Store::write("ig.comments.p_henry_1", &serde_json::json!([
        { "id": "c50", "author_id": "alice", "author_name": "Alice Chen", "author_avatar": "https://i.pravatar.cc/150?u=alice",
          "text": "100ms TTI is wild 🚀 What's the WASM bundle size?", "likes": 89, "timestamp": "30m ago" },
        { "id": "c51", "author_id": "grace", "author_name": "Grace Kim", "author_avatar": "https://i.pravatar.cc/150?u=grace",
          "text": "The performance charts look amazing. Bookmarked the blog post!", "likes": 34, "timestamp": "45m ago" },
        { "id": "c52", "author_id": "bob", "author_name": "Bob Ross", "author_avatar": "https://i.pravatar.cc/150?u=bob",
          "text": "I don't understand any of this but the numbers look good 😄 Congrats!", "likes": 456, "timestamp": "20m ago" }
    ]).to_string());

    // Empty comment lists for remaining posts
    for pid in &["p_alice_2","p_alice_3","p_alice_4","p_alice_5","p_alice_reel_1",
                 "p_bob_2","p_bob_3","p_bob_4","p_bob_reel_1",
                 "p_charlie_2","p_charlie_3","p_charlie_4",
                 "p_diana_2","p_diana_3","p_diana_4","p_diana_reel_1",
                 "p_eve_2","p_eve_3","p_eve_4",
                 "p_frank_1","p_frank_2","p_frank_3","p_frank_reel_1",
                 "p_grace_1","p_grace_2","p_grace_3","p_grace_reel_1",
                 "p_henry_2","p_henry_3","p_henry_4"] {
        let key = format!("ig.comments.{}", pid);
        if Store::read(&key).is_none() {
            Store::write(&key, "[]");
        }
    }

    // ── Notifications — rich cross-profile activity ───────────────────────────
    Store::write("ig.notifications", &serde_json::json!([
        { "id": "n1", "type": "like", "actor_id": "bob", "actor_name": "Bob Ross",
          "actor_avatar": "https://i.pravatar.cc/150?u=bob", "text": "liked your photo",
          "post_image": "https://picsum.photos/60/60?random=10", "post_id": "p_alice_1", "timestamp": "2m ago", "read": false },
        { "id": "n2", "type": "comment", "actor_id": "henry", "actor_name": "Henry Park",
          "actor_avatar": "https://i.pravatar.cc/150?u=henry", "text": "commented: \"This is exactly what Rust UI needed 🦀\"",
          "post_image": "https://picsum.photos/60/60?random=10", "post_id": "p_alice_1", "timestamp": "8m ago", "read": false },
        { "id": "n3", "type": "follow", "actor_id": "frank", "actor_name": "Frank Garcia",
          "actor_avatar": "https://i.pravatar.cc/150?u=frank", "text": "started following you",
          "post_image": null, "post_id": null, "timestamp": "32m ago", "read": false },
        { "id": "n4", "type": "like", "actor_id": "grace", "actor_name": "Grace Kim",
          "actor_avatar": "https://i.pravatar.cc/150?u=grace", "text": "liked your photo",
          "post_image": "https://picsum.photos/60/60?random=11", "post_id": "p_alice_2", "timestamp": "1h ago", "read": false },
        { "id": "n5", "type": "mention", "actor_id": "henry", "actor_name": "Henry Park",
          "actor_avatar": "https://i.pravatar.cc/150?u=henry", "text": "mentioned you in a post",
          "post_image": "https://picsum.photos/60/60?random=92", "post_id": "p_henry_3", "timestamp": "3h ago", "read": false },
        { "id": "n6", "type": "comment", "actor_id": "charlie", "actor_name": "Charlie Day",
          "actor_avatar": "https://i.pravatar.cc/150?u=charlie", "text": "commented: \"Been waiting for this 🙌\"",
          "post_image": "https://picsum.photos/60/60?random=10", "post_id": "p_alice_1", "timestamp": "2h ago", "read": true },
        { "id": "n7", "type": "follow", "actor_id": "diana", "actor_name": "Diana Prince",
          "actor_avatar": "https://i.pravatar.cc/150?u=diana", "text": "started following you",
          "post_image": null, "post_id": null, "timestamp": "5h ago", "read": true },
        { "id": "n8", "type": "save", "actor_id": "eve", "actor_name": "Eve Nakamura",
          "actor_avatar": "https://i.pravatar.cc/150?u=eve", "text": "saved your post",
          "post_image": "https://picsum.photos/60/60?random=12", "post_id": "p_alice_3", "timestamp": "8h ago", "read": true },
        { "id": "n9", "type": "like", "actor_id": "frank", "actor_name": "Frank Garcia",
          "actor_avatar": "https://i.pravatar.cc/150?u=frank", "text": "liked your reel",
          "post_image": "https://picsum.photos/60/60?random=15", "post_id": "p_alice_reel_1", "timestamp": "1d ago", "read": true }
    ]).to_string());
    Store::write("ig.notifications.unread", "5");

    // ── DM threads ────────────────────────────────────────────────────────────
    Store::write("ig.dms", &serde_json::json!([
        { "id": "dm_bob", "user_id": "bob", "user_name": "Bob Ross",
          "user_avatar": "https://i.pravatar.cc/150?u=bob", "verified": true,
          "last_message": "Love it! Can't wait to see the finished piece 🎨", "last_ts": "5m ago", "unread": 2 },
        { "id": "dm_grace", "user_id": "grace", "user_name": "Grace Kim",
          "user_avatar": "https://i.pravatar.cc/150?u=grace", "verified": false,
          "last_message": "The sprint next week is confirmed! You're in right?", "last_ts": "22m ago", "unread": 1 },
        { "id": "dm_henry", "user_id": "henry", "user_name": "Henry Park",
          "user_avatar": "https://i.pravatar.cc/150?u=henry", "verified": false,
          "last_message": "Would love to collab on a blog post about the DSL!", "last_ts": "1h ago", "unread": 1 },
        { "id": "dm_charlie", "user_id": "charlie", "user_name": "Charlie Day",
          "user_avatar": "https://i.pravatar.cc/150?u=charlie", "verified": false,
          "last_message": "Are you going to the meetup next Friday?", "last_ts": "2h ago", "unread": 0 },
        { "id": "dm_diana", "user_id": "diana", "user_name": "Diana Prince",
          "user_avatar": "https://i.pravatar.cc/150?u=diana", "verified": true,
          "last_message": "Tokyo is incredible — you have to visit!", "last_ts": "1d ago", "unread": 0 },
        { "id": "dm_eve", "user_id": "eve", "user_name": "Eve Nakamura",
          "user_avatar": "https://i.pravatar.cc/150?u=eve", "verified": false,
          "last_message": "Thanks for the feedback on my setup post 🙏", "last_ts": "3d ago", "unread": 0 }
    ]).to_string());

    // ── DM message threads — realistic conversations ──────────────────────────
    Store::write("ig.dms.dm_bob.messages", &serde_json::json!([
        { "id": "msg_b1", "sender_id": "bob", "sender_name": "Bob Ross",
          "text": "Hey Alice! Just saw your latest post about the DSL v2 🔥 This is incredible work", "timestamp": "Yesterday 2:14 PM", "is_me": false },
        { "id": "msg_b2", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Thank you so much! Been heads down on it for weeks. The indentation parser was tricky 😅", "timestamp": "Yesterday 2:18 PM", "is_me": true },
        { "id": "msg_b3", "sender_id": "bob", "sender_name": "Bob Ross",
          "text": "I can imagine! I tried building something similar for my art tutorials and gave up lol", "timestamp": "Yesterday 2:22 PM", "is_me": false },
        { "id": "msg_b4", "sender_id": "bob", "sender_name": "Bob Ross",
          "text": "The animation primitives are exactly what I needed for my joy of painting app btw 🎨", "timestamp": "Yesterday 2:23 PM", "is_me": false },
        { "id": "msg_b5", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Oh wow, you're building something with it? I'd love to see!", "timestamp": "Yesterday 3:01 PM", "is_me": true },
        { "id": "msg_b6", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Let me know if you hit any issues — happy to jump on a call if needed", "timestamp": "Yesterday 3:02 PM", "is_me": true },
        { "id": "msg_b7", "sender_id": "bob", "sender_name": "Bob Ross",
          "text": "Will do! Currently building a colour mixing tutorial app. The scroll-triggered animations for the brush strokes are 🤌", "timestamp": "2h ago", "is_me": false },
        { "id": "msg_b8", "sender_id": "bob", "sender_name": "Bob Ross",
          "text": "Love it! Can't wait to see the finished piece 🎨", "timestamp": "5m ago", "is_me": false }
    ]).to_string());

    Store::write("ig.dms.dm_grace.messages", &serde_json::json!([
        { "id": "msg_g1", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Hey Grace! Loved the new brand identity post 🎨 The hidden 'g' is so clever", "timestamp": "Yesterday 10:30 AM", "is_me": true },
        { "id": "msg_g2", "sender_id": "grace", "sender_name": "Grace Kim",
          "text": "Thank you!! The client almost rejected it at first haha, they thought it was too subtle", "timestamp": "Yesterday 10:45 AM", "is_me": false },
        { "id": "msg_g3", "sender_id": "grace", "sender_name": "Grace Kim",
          "text": "How are you finding the new DSL workflow? Saw your post — looked super smooth", "timestamp": "Yesterday 10:46 AM", "is_me": false },
        { "id": "msg_g4", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "It's finally getting to the point where it feels effortless. Still some rough edges but much better", "timestamp": "Yesterday 11:15 AM", "is_me": true },
        { "id": "msg_g5", "sender_id": "grace", "sender_name": "Grace Kim",
          "text": "We should do a collab! Figma to DSL pipeline would be insane 🤯", "timestamp": "Yesterday 11:20 AM", "is_me": false },
        { "id": "msg_g6", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "YES. Let's explore that. Maybe after the sprint?", "timestamp": "Yesterday 11:35 AM", "is_me": true },
        { "id": "msg_g7", "sender_id": "grace", "sender_name": "Grace Kim",
          "text": "The sprint next week is confirmed! You're in right? 🙌", "timestamp": "22m ago", "is_me": false }
    ]).to_string());

    Store::write("ig.dms.dm_henry.messages", &serde_json::json!([
        { "id": "msg_h1", "sender_id": "henry", "sender_name": "Henry Park",
          "text": "Alice! Your DSL work is making rounds on the Rust discord 👀", "timestamp": "3d ago", "is_me": false },
        { "id": "msg_h2", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Haha is that good or bad 😅", "timestamp": "3d ago", "is_me": true },
        { "id": "msg_h3", "sender_id": "henry", "sender_name": "Henry Park",
          "text": "Definitely good! The performance benchmarks people are seeing are wild", "timestamp": "3d ago", "is_me": false },
        { "id": "msg_h4", "sender_id": "henry", "sender_name": "Henry Park",
          "text": "I wrote a thread about it on my Instagram. Hope that's okay!", "timestamp": "2d ago", "is_me": false },
        { "id": "msg_h5", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Of course! Thank you for the signal boost 🙏 Loved your thread btw", "timestamp": "2d ago", "is_me": true },
        { "id": "msg_h6", "sender_id": "henry", "sender_name": "Henry Park",
          "text": "Would love to collab on a blog post about the DSL! Full deep dive.", "timestamp": "1h ago", "is_me": false }
    ]).to_string());

    Store::write("ig.dms.dm_charlie.messages", &serde_json::json!([
        { "id": "msg_c1", "sender_id": "charlie", "sender_name": "Charlie Day",
          "text": "Your shot at golden hour was stunning 📸 Did you shoot on film?", "timestamp": "4d ago", "is_me": false },
        { "id": "msg_c2", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "No just iPhone Pro! The computational photography has gotten crazy good", "timestamp": "4d ago", "is_me": true },
        { "id": "msg_c3", "sender_id": "charlie", "sender_name": "Charlie Day",
          "text": "No way! I need to reconsider my entire gear setup lol 😂", "timestamp": "4d ago", "is_me": false },
        { "id": "msg_c4", "sender_id": "charlie", "sender_name": "Charlie Day",
          "text": "Are you going to the meetup next Friday? It's at that new space in SoHo", "timestamp": "2h ago", "is_me": false }
    ]).to_string());

    Store::write("ig.dms.dm_diana.messages", &serde_json::json!([
        { "id": "msg_d1", "sender_id": "diana", "sender_name": "Diana Prince",
          "text": "Landed in Tokyo! 🗼 Finally here after rescheduling 3 times", "timestamp": "3d ago", "is_me": false },
        { "id": "msg_d2", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "FINALLY! So jealous 😭 How's the food situation?", "timestamp": "3d ago", "is_me": true },
        { "id": "msg_d3", "sender_id": "diana", "sender_name": "Diana Prince",
          "text": "I've eaten ramen for every single meal and I have zero regrets", "timestamp": "3d ago", "is_me": false },
        { "id": "msg_d4", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "That is the correct approach. Are you going to Ichiran?", "timestamp": "2d ago", "is_me": true },
        { "id": "msg_d5", "sender_id": "diana", "sender_name": "Diana Prince",
          "text": "Already went twice 😂 Also just stumbled into the best kaiseki restaurant by accident", "timestamp": "2d ago", "is_me": false },
        { "id": "msg_d6", "sender_id": "diana", "sender_name": "Diana Prince",
          "text": "Tokyo is incredible — you have to visit! I'll send you the full list 📝", "timestamp": "1d ago", "is_me": false }
    ]).to_string());

    Store::write("ig.dms.dm_eve.messages", &serde_json::json!([
        { "id": "msg_e1", "sender_id": "eve", "sender_name": "Eve Nakamura",
          "text": "Hi Alice! Walked through your DSL demo today — the a11y stuff is genuinely impressive", "timestamp": "1w ago", "is_me": false },
        { "id": "msg_e2", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "Thank you!! That part took the longest to get right. The live regions especially", "timestamp": "1w ago", "is_me": true },
        { "id": "msg_e3", "sender_id": "eve", "sender_name": "Eve Nakamura",
          "text": "It shows. Most devs skip this stuff entirely. Really refreshing to see it as a first-class primitive 💙", "timestamp": "1w ago", "is_me": false },
        { "id": "msg_e4", "sender_id": "me", "sender_name": "Alice Chen",
          "text": "That means a lot coming from you! Would love your feedback on the screen reader flow if you have time", "timestamp": "6d ago", "is_me": true },
        { "id": "msg_e5", "sender_id": "eve", "sender_name": "Eve Nakamura",
          "text": "Thanks for the feedback on my setup post 🙏 The monitor arm you asked about is Ergotron LX", "timestamp": "3d ago", "is_me": false }
    ]).to_string());

    // ── Interaction state: likes / saves / follows ────────────────────────────
    Store::write("ig.liked.p_bob_1",       "true");
    Store::write("ig.liked.p_diana_1",     "true");
    Store::write("ig.liked.p_charlie_1",   "true");
    Store::write("ig.liked.p_grace_1",     "true");
    Store::write("ig.liked.p_henry_1",     "true");
    Store::write("ig.saved.p_alice_2",     "true");
    Store::write("ig.saved.p_bob_1",       "true");
    Store::write("ig.saved.p_diana_reel_1","true");
    Store::write("ig.saved.p_frank_reel_1","true");
    Store::write("ig.following.bob",       "true");
    Store::write("ig.following.charlie",   "true");
    Store::write("ig.following.diana",     "true");
    Store::write("ig.following.grace",     "true");
    Store::write("ig.following.henry",     "true");
    Store::write("ig.following.eve",       "false");
    Store::write("ig.following.frank",     "false");

    // ── Tagged posts (posts where alice is tagged) ────────────────────────────
    Store::write("ig.me.tagged_posts", &serde_json::json!([
        grace_posts[0].clone(), grace_posts[2].clone(),
        eve_posts[0].clone(), eve_posts[3].clone(),
        henry_posts[0].clone(), henry_posts[2].clone()
    ]).to_string());

    // ── Saved posts collection ────────────────────────────────────────────────
    Store::write("ig.me.saved_posts", &serde_json::json!([
        alice_posts[1].clone(), bob_posts[0].clone(),
        diana_posts[4].clone(), frank_posts[3].clone()
    ]).to_string());

    // ── Explore / search user index ───────────────────────────────────────────
    Store::write("ig.users.index", &serde_json::json!([
        { "id": "bob",     "name": "Bob Ross",     "handle": "@bobross",         "avatar": "https://i.pravatar.cc/150?u=bob",     "verified": true,  "followers": "560K" },
        { "id": "diana",   "name": "Diana Prince", "handle": "@dianaprince",     "avatar": "https://i.pravatar.cc/150?u=diana",   "verified": true,  "followers": "241K" },
        { "id": "frank",   "name": "Frank Garcia", "handle": "@frankgarcia",     "avatar": "https://i.pravatar.cc/150?u=frank",   "verified": false, "followers": "89.2K" },
        { "id": "grace",   "name": "Grace Kim",    "handle": "@gracekim.design", "avatar": "https://i.pravatar.cc/150?u=grace",   "verified": false, "followers": "34.1K" },
        { "id": "henry",   "name": "Henry Park",   "handle": "@henrypark.dev",   "avatar": "https://i.pravatar.cc/150?u=henry",   "verified": false, "followers": "28.4K" },
        { "id": "charlie", "name": "Charlie Day",  "handle": "@charlieday",      "avatar": "https://i.pravatar.cc/150?u=charlie", "verified": false, "followers": "8.9K" },
        { "id": "eve",     "name": "Eve Nakamura", "handle": "@eve.nakamura",    "avatar": "https://i.pravatar.cc/150?u=eve",     "verified": false, "followers": "6.7K" }
    ]).to_string());

    // ── Alice's highlights ────────────────────────────────────────────────────
    Store::write("ig.me.highlights", &serde_json::json!([
        { "id": "hl_dsl",    "title": "DSL",    "cover": "https://picsum.photos/60/60?random=10" },
        { "id": "hl_work",   "title": "Work",   "cover": "https://picsum.photos/60/60?random=13" },
        { "id": "hl_travel", "title": "Travel", "cover": "https://picsum.photos/60/60?random=14" },
        { "id": "hl_setup",  "title": "Setup",  "cover": "https://picsum.photos/60/60?random=12" }
    ]).to_string());
}

// ── Schema definitions ───────────────────────────────────────────────────────

pub fn register_app_schemas() {
    register_schema(Schema {
        name: "user_profile",
        namespace: "u",
        fields: vec![
            FieldDef { name: "display_name", kind: FieldKind::Text, default: None },
            FieldDef { name: "handle", kind: FieldKind::Text, default: None },
            FieldDef { name: "avatar_url", kind: FieldKind::ImageUrl, default: None },
            FieldDef { name: "bio", kind: FieldKind::Text, default: None },
            FieldDef { name: "followers", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
            FieldDef { name: "following", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
            FieldDef { name: "posts_count", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
            FieldDef { name: "verified", kind: FieldKind::Bool, default: Some(serde_json::json!(false)) },
        ],
        preload: PreloadStrategy::OnFirstRead,
        cache_ttl_secs: Some(300),
        format: StorageFormat::Json,
    });

    register_schema(Schema {
        name: "media_post",
        namespace: "p",
        fields: vec![
            FieldDef { name: "author_id", kind: FieldKind::UserId, default: None },
            FieldDef { name: "author_name", kind: FieldKind::Text, default: None },
            FieldDef { name: "author_avatar", kind: FieldKind::ImageUrl, default: None },
            FieldDef { name: "content", kind: FieldKind::Text, default: None },
            FieldDef { name: "image_url", kind: FieldKind::ImageUrl, default: None },
            FieldDef { name: "likes", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
            FieldDef { name: "comments", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
            FieldDef { name: "timestamp", kind: FieldKind::Text, default: None },
        ],
        preload: PreloadStrategy::OnFirstRead,
        cache_ttl_secs: Some(60),
        format: StorageFormat::Json,
    });

    register_schema(Schema {
        name: "chat_message",
        namespace: "m",
        fields: vec![
            FieldDef { name: "sender_id", kind: FieldKind::UserId, default: None },
            FieldDef { name: "sender_name", kind: FieldKind::Text, default: None },
            FieldDef { name: "sender_avatar", kind: FieldKind::ImageUrl, default: None },
            FieldDef { name: "text", kind: FieldKind::Text, default: None },
            FieldDef { name: "timestamp", kind: FieldKind::Text, default: None },
            FieldDef { name: "is_me", kind: FieldKind::Bool, default: Some(serde_json::json!(false)) },
        ],
        preload: PreloadStrategy::Eager,
        cache_ttl_secs: None,
        format: StorageFormat::Json,
    });

    register_schema(Schema {
        name: "product_item",
        namespace: "prod",
        fields: vec![
            FieldDef { name: "name", kind: FieldKind::Text, default: None },
            FieldDef { name: "price", kind: FieldKind::Number, default: Some(serde_json::json!(0.0)) },
            FieldDef { name: "currency", kind: FieldKind::Text, default: Some(serde_json::json!("USD")) },
            FieldDef { name: "image_url", kind: FieldKind::ImageUrl, default: None },
            FieldDef { name: "rating", kind: FieldKind::Number, default: Some(serde_json::json!(0.0)) },
            FieldDef { name: "reviews", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
            FieldDef { name: "category", kind: FieldKind::Text, default: None },
        ],
        preload: PreloadStrategy::OnFirstRead,
        cache_ttl_secs: Some(600),
        format: StorageFormat::Json,
    });

    register_schema(Schema {
        name: "dashboard_metric",
        namespace: "dash",
        fields: vec![
            FieldDef { name: "label", kind: FieldKind::Text, default: None },
            FieldDef { name: "value", kind: FieldKind::Text, default: None },
            FieldDef { name: "change", kind: FieldKind::Number, default: Some(serde_json::json!(0.0)) },
            FieldDef { name: "is_positive", kind: FieldKind::Bool, default: Some(serde_json::json!(true)) },
        ],
        preload: PreloadStrategy::Eager,
        cache_ttl_secs: None,
        format: StorageFormat::Json,
    });

    register_schema(Schema {
        name: "feed_item",
        namespace: "feed",
        fields: vec![
            FieldDef { name: "title", kind: FieldKind::Text, default: None },
            FieldDef { name: "source", kind: FieldKind::Text, default: None },
            FieldDef { name: "summary", kind: FieldKind::Text, default: None },
            FieldDef { name: "image_url", kind: FieldKind::ImageUrl, default: None },
            FieldDef { name: "timestamp", kind: FieldKind::Text, default: None },
            FieldDef { name: "read_time", kind: FieldKind::Number, default: Some(serde_json::json!(0)) },
        ],
        preload: PreloadStrategy::OnFirstRead,
        cache_ttl_secs: Some(120),
        format: StorageFormat::Json,
    });
}

/// Ensure Instagram seed data is loaded into the token store.
/// Idempotent — only seeds on the first call per session.
pub fn ensure_ig_seeded() {
    use crate::tokens::storage::primitive::Store;
    if Store::read("ig.me.name").is_none() {
        seed_instagram_storage();
    }
}
