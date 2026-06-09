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
