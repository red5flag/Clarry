// examples/structural_primitives.rs
//
// Demonstrates the core structural primitives for data-driven UI generation.
// This example shows how to build Instagram, Reddit, LinkedIn, ecommerce,
// dashboards, and other application types using the same small set of primitives.

use l8_loader::tokens::prelude::*;

// =============================================================================
// EXAMPLE 1: Instagram-like Feed (Posts from data)
// =============================================================================

pub fn instagram_feed_page() -> impl IntoToken {
    // Data-driven: UI generated from structured data
    col()
        .id("instagram_feed")
        .set_class("min-h-screen bg-black text-white")

        // Header
        .add(row()
            .set_class("sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3")
            .add(txt("Instagram").bold())
            .add(if_true("user.notifications")
                .then(badge().count(bind(global("user.notification_count"))))
            )
        )

        // Stories (horizontal scroll of users)
        .add(row()
            .set_class("px-4 py-3 gap-4 overflow-x-auto")
            .add(foreach("stories")
                .item(|story_id| story_bubble()
                    .bind("story", local(story_id))
                )
            )
        )

        // Posts feed (main content from data)
        .add(col()
            .set_class("divide-y divide-gray-800")
            .add(foreach("posts")
                .item(|post_id| post_card()
                    .bind("post", local(post_id))
                )
                .empty(txt("No posts yet. Follow someone!"))
            )
        )

        // Bottom nav
        .add(bottom_nav())
}

/// Reusable PostCard template - works with any post data
fn post_card() -> ComponentInstanceBuilder {
    use_component("PostCard")
        // Slots allow composition - different actions per use case
        .fill("actions", row()
            .set_class("flex gap-4")
            .add(btn("❤️").push_action(like_post(bind(local("post.id")))))
            .add(btn("💬").push_action(show_comments(bind(local("post.id")))))
            .add(btn("↗️").push_action(share_post(bind(local("post.id")))))
        )
}

fn story_bubble() -> ComponentInstanceBuilder {
    use_component("StoryBubble")
}

fn bottom_nav() -> impl IntoToken {
    row()
        .set_class("fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 px-4 py-3 justify-around")
        .add(btn("🏠").on_click_nav("home"))
        .add(btn("🔍").on_click_nav("search"))
        .add(btn("➕").on_click_nav("create"))
        .add(btn("❤️").on_click_nav("activity"))
        .add(btn("👤").on_click_nav("profile"))
}

// =============================================================================
// EXAMPLE 2: Reddit-like Forum (Nested comments, voting)
// =============================================================================

pub fn forum_page() -> impl IntoToken {
    col()
        .id("forum_page")
        .set_class("min-h-screen bg-gray-100")

        // Sort/filter controls
        .add(row()
            .set_class("px-4 py-3 gap-2 bg-white shadow-sm")
            .add(btn("Hot").push_action(set_sort("hot")))
            .add(btn("New").push_action(set_sort("new")))
            .add(btn("Top").push_action(set_sort("top")))
        )

        // Posts with filtering
        .add(col()
            .set_class("gap-2 p-2")
            .add(filter("posts")
                .where_eq("subreddit", bind(global("current_subreddit")))
                .render(|post_id| forum_post()
                    .bind("post", local(post_id))
                )
            )
        )
}

fn forum_post() -> ComponentInstanceBuilder {
    use_component("ForumPost")
        .fill("vote_section", vote_section())
        .fill("comment_section", col()
            .add(if_gt("post.comment_count", 0)
                .then(relation("post", "comments", "comments")
                    .render(|comment_id| comment_tree()
                        .bind("comment", local(comment_id))
                        .bind("depth", literal("0"))
                    )
                )
            )
        )
}

fn vote_section() -> impl IntoToken {
    col()
        .set_class("items-center gap-1")
        .add(btn("▲").push_action(upvote(bind(local("post.id")))))
        .add(txt(bind(local("post.score"))))
        .add(btn("▼").push_action(downvote(bind(local("post.id")))))
}

/// Recursive comment tree using relations
fn comment_tree() -> ComponentInstanceBuilder {
    use_component("Comment")
        .fill("replies", if_gt("comment.reply_count", 0)
            .then(relation("comment", "replies", "comments")
                .render(|reply_id| comment_tree()
                    .bind("comment", local(reply_id))
                    .bind("depth", computed("depth + 1"))
                )
            )
            .else_(txt(""))
        )
}

// =============================================================================
// EXAMPLE 3: LinkedIn-like Professional Network
// =============================================================================

pub fn linkedin_feed() -> impl IntoToken {
    col()
        .id("linkedin_feed")
        .set_class("min-h-screen bg-gray-50")

        // Create post composer (conditional)
        .add(if_true("user.logged_in")
            .then(post_composer())
        )

        // Feed with mixed content types
        .add(col()
            .set_class("gap-4 p-4 max-w-2xl mx-auto")
            .add(query()
                .select(vec!["*"])
                .from("feed_items")
                .join("author", "users", "author_id", "id")
                .order_by("created_at", true)
                .limit(20)
                .render(|item_id| feed_item()
                    .bind("item", local(item_id))
                )
            )
        )
}

fn feed_item() -> impl IntoToken {
    // Conditional rendering based on item type
    if_eq("item.type", "post")
        .then(use_component("LinkedInPost"))
        .else_(if_eq("item.type", "job")
            .then(use_component("JobListing"))
            .else_(if_eq("item.type", "article")
                .then(use_component("ArticleCard"))
            )
        )
}

fn post_composer() -> impl IntoToken {
    col()
        .set_class("bg-white rounded-lg shadow p-4 mb-4")
        .add(text_input()
            .placeholder("What's on your mind?")
            .bind(value(global("composer.text")))
        )
        .add(if_exists("composer.text")
            .then(row()
                .add(btn("Post").push_action(submit_post()))
            )
        )
}

// =============================================================================
// EXAMPLE 4: E-commerce Store (Products, cart, filters)
// =============================================================================

pub fn ecommerce_store() -> impl IntoToken {
    col()
        .id("store")
        .set_class("min-h-screen bg-white")

        // Header with cart count
        .add(row()
            .set_class("px-4 py-3 border-b justify-between items-center")
            .add(txt("Shop").bold())
            .add(row()
                .add(if_gt("cart.item_count", 0)
                    .then(badge()
                        .count(bind(global("cart.item_count")))
                        .set_class("bg-red-500 text-white")
                    )
                )
                .add(btn("🛒").on_click_nav("cart"))
            )
        )

        // Filters sidebar + product grid
        .add(row()
            .set_class("flex-1 gap-4 p-4")
            .add(col()
                .set_class("w-64 hidden md:flex gap-2")
                .add(txt("Filters").bold())
                .add(filter_controls())
            )
            .add(grid(3)
                .set_class("flex-1 gap-4")
                .add(filter("products")
                    .where_eq("category", bind(global("filters.category")))
                    .where_exists("price")
                    .render(|product_id| product_card()
                        .bind("product", local(product_id))
                    )
                )
            )
        )
}

fn filter_controls() -> impl IntoToken {
    col()
        .set_class("gap-2")
        .add(txt("Category"))
        .add(foreach("categories")
            .item(|cat_id| row()
                .add(checkbox()
                    .checked(eq(bind(local(cat_id)), bind(global("filters.category"))))
                )
                .add(txt(bind(local("category.name"))))
            )
        )
        .add(txt("Price Range"))
        .add(range_slider()
            .min(0)
            .max(1000)
            .bind(value(global("filters.max_price")))
        )
}

fn product_card() -> ComponentInstanceBuilder {
    use_component("ProductCard")
        .fill("quick_actions", row()
            .add(btn("Add to Cart").push_action(add_to_cart(bind(local("product.id")))))
            .add(if_true("product.in_wishlist")
                .then(btn("❤️").push_action(remove_from_wishlist(bind(local("product.id")))))
                .else_(btn("🤍").push_action(add_to_wishlist(bind(local("product.id")))))
            )
        )
}

// =============================================================================
// EXAMPLE 5: Dashboard with Analytics
// =============================================================================

pub fn analytics_dashboard() -> impl IntoToken {
    col()
        .id("dashboard")
        .set_class("min-h-screen bg-gray-100 p-4")

        // KPI cards row
        .add(grid(4)
            .set_class("gap-4 mb-4")
            .add(kpi_card("Revenue", "metrics.revenue", "$"))
            .add(kpi_card("Users", "metrics.active_users", ""))
            .add(kpi_card("Conversion", "metrics.conversion", "%"))
            .add(kpi_card("Churn", "metrics.churn", "%"))
        )

        // Charts section
        .add(row()
            .set_class("gap-4 mb-4")
            .add(col()
                .set_class("flex-1 bg-white rounded-lg p-4")
                .add(txt("Revenue Over Time").bold())
                .add(chart(bind(global("data.revenue_chart"))))
            )
            .add(col()
                .set_class("w-80 bg-white rounded-lg p-4")
                .add(txt("Traffic Sources").bold())
                .add(pie_chart(bind(global("data.traffic_sources"))))
            )
        )

        // Data table with pagination
        .add(col()
            .set_class("bg-white rounded-lg")
            .add(row()
                .set_class("px-4 py-3 border-b justify-between")
                .add(txt("Recent Transactions").bold())
                .add(txt(format!("Showing {} of {}",
                    bind(global("pagination.per_page")),
                    count("transactions")
                )))
            )
            .add(table()
                .headers(vec!["Date", "Customer", "Amount", "Status"])
                .rows(limit("transactions")
                    .skip(computed("pagination.page * pagination.per_page"))
                    .limit(10)
                    .render(|tx_id| table_row()
                        .bind("tx", local(tx_id))
                    )
                )
            )
            .add(pagination_controls("transactions"))
        )
}

fn kpi_card(title: &str, metric_path: &str, prefix: &str) -> impl IntoToken {
    col()
        .set_class("bg-white rounded-lg p-4")
        .add(txt(title).muted())
        .add(row()
            .add(txt(prefix))
            .add(txt(bind(global(metric_path)))
                .set_class("text-2xl font-bold")
            )
        )
}

fn pagination_controls(collection: &str) -> impl IntoToken {
    row()
        .set_class("px-4 py-3 justify-center gap-2")
        .add(if_gt("pagination.page", 0)
            .then(btn("Previous").push_action(prev_page()))
        )
        .add(txt(format!("Page {} of {}",
            bind(global("pagination.page")),
            computed(format!("ceil(count({}) / pagination.per_page)", collection))
        )))
        .add(if_lt("pagination.page", computed("total_pages - 1"))
            .then(btn("Next").push_action(next_page()))
        )
}

// =============================================================================
// EXAMPLE 6: CRM - Customer Detail with Related Data
// =============================================================================

pub fn customer_detail_page() -> impl IntoToken {
    col()
        .id("customer_detail")
        .set_class("min-h-screen bg-gray-50")

        // Customer header
        .add(row()
            .set_class("bg-white p-6 shadow-sm")
            .add(img_block(bind(global("customer.avatar")))
                .set_class("w-20 h-20 rounded-full")
            )
            .add(col()
                .set_class("ml-4")
                .add(txt(bind(global("customer.name")))
                    .set_class("text-xl font-bold")
                )
                .add(txt(bind(global("customer.company"))).muted())
                .add(row()
                    .set_class("gap-2 mt-2")
                    .add(badge()
                        .label(bind(global("customer.status")))
                        .variant(if_eq("customer.status", "active")
                            .then(literal("success"))
                            .else_(literal("warning"))
                        )
                    )
                )
            )
        )

        // Tabs for related data
        .add(tabs("active_tab", vec![
            ("Overview", customer_overview()),
            ("Orders", customer_orders()),
            ("Activity", customer_activity()),
            ("Notes", customer_notes()),
        ]))
}

fn customer_orders() -> impl IntoToken {
    col()
        .set_class("p-4 gap-2")
        .add(row()
            .set_class("justify-between mb-4")
            .add(txt("Order History").bold())
            .add(txt(format!("Total: {}", count("customer.orders"))))
        )
        .add(sort("customer.orders", "date")
            .descending()
            .render(|order_id| order_row()
                .bind("order", local(order_id))
            )
        )
}

fn customer_activity() -> impl IntoToken {
    col()
        .set_class("p-4")
        .add(query()
            .select(vec!["action", "timestamp", "details"])
            .from("activities")
            .where_eq("customer_id", bind(global("customer.id")))
            .order_by("timestamp", true)
            .limit(50)
            .render(|activity_id| activity_item()
                .bind("activity", local(activity_id))
            )
        )
}

// =============================================================================
// EXAMPLE 7: Blog with Author Info via Relations
// =============================================================================

pub fn blog_post_page() -> impl IntoToken {
    col()
        .id("blog_post")
        .set_class("min-h-screen bg-white max-w-3xl mx-auto px-4 py-8")

        // Post header with author relation
        .add(col()
            .set_class("mb-8")
            .add(txt(bind(global("post.title")))
                .set_class("text-3xl font-bold mb-4")
            )
            .add(row()
                .set_class("items-center gap-3")
                // Author loaded via relation
                .add(relation("post", "author", "authors")
                    .render(|author_id| row()
                        .set_class("items-center gap-2")
                        .add(img_block(bind(local("author.avatar")))
                            .set_class("w-10 h-10 rounded-full")
                        )
                        .add(txt(bind(local("author.name"))))
                    )
                )
                .add(txt("·").muted())
                .add(txt(bind(global("post.published_at"))).muted())
            )
        )

        // Post content
        .add(txt(bind(global("post.content"))))

        // Related posts
        .add(col()
            .set_class("mt-12 pt-8 border-t")
            .add(txt("Related Posts").bold())
            .add(query()
                .select(vec!["title", "excerpt"])
                .from("posts")
                .where_eq("category", bind(global("post.category")))
                .where_exists("published_at")
                .order_by("published_at", true)
                .limit(3)
                .render(|post_id| related_post()
                    .bind("post", local(post_id))
                )
            )
        )
}

// =============================================================================
// EXAMPLE 8: SaaS Admin Panel with User Management
// =============================================================================

pub fn admin_user_management() -> impl IntoToken {
    col()
        .id("admin_users")
        .set_class("min-h-screen bg-gray-100 p-4")

        // Search and filters
        .add(row()
            .set_class("bg-white rounded-lg p-4 mb-4 gap-4")
            .add(text_input()
                .placeholder("Search users...")
                .bind(value(global("admin.user_search")))
                .set_class("flex-1")
            )
            .add(select()
                .options(vec!["All", "Active", "Inactive", "Suspended"])
                .bind(value(global("admin.user_filter_status")))
            )
        )

        // User table with filtering
        .add(col()
            .set_class("bg-white rounded-lg")
            .add(table_header(vec![
                "User", "Email", "Role", "Status", "Last Active", "Actions"
            ]))
            .add(filter("users")
                .where_contains("name", bind(global("admin.user_search")))
                .where_eq("status", bind(global("admin.user_filter_status")))
                .render(|user_id| user_admin_row()
                    .bind("user", local(user_id))
                )
            )
            .add(if_false("users")
                .then(col()
                    .set_class("p-8 text-center")
                    .add(txt("No users found").muted())
                )
            )
        )

        // Bulk actions (conditional)
        .add(if_gt("admin.selected_users.count", 0)
            .then(row()
                .set_class("fixed bottom-4 left-4 right-4 bg-white shadow-lg rounded-lg p-4 justify-between items-center")
                .add(txt(format!("{} users selected",
                    bind(global("admin.selected_users.count"))
                )))
                .add(row()
                    .set_class("gap-2")
                    .add(btn("Activate").push_action(bulk_activate()))
                    .add(btn("Suspend").push_action(bulk_suspend()))
                    .add(btn("Delete").var("danger").push_action(bulk_delete()))
                )
            )
        )
}

fn user_admin_row() -> impl IntoToken {
    row()
        .set_class("px-4 py-3 border-b items-center hover:bg-gray-50")
        .add(checkbox()
            .checked(in_(local("user.id"), global("admin.selected_users")))
            .push_action(toggle_selection(bind(local("user.id"))))
        )
        .add(img_block(bind(local("user.avatar")))
            .set_class("w-8 h-8 rounded-full ml-2")
        )
        .add(txt(bind(local("user.name"))))
        .add(txt(bind(local("user.email"))).muted())
        .add(badge().label(bind(local("user.role"))))
        .add(badge()
            .label(bind(local("user.status")))
            .variant(if_eq("user.status", "active")
                .then(literal("success"))
                .else_(literal("warning"))
            )
        )
        .add(txt(bind(local("user.last_active"))).muted())
}

// =============================================================================
// Helper functions (these would be defined elsewhere in the real app)
// =============================================================================

fn like_post(id: impl IntoToken) -> impl IntoToken { id }
fn show_comments(id: impl IntoToken) -> impl IntoToken { id }
fn share_post(id: impl IntoToken) -> impl IntoToken { id }
fn upvote(id: impl IntoToken) -> impl IntoToken { id }
fn downvote(id: impl IntoToken) -> impl IntoToken { id }
fn set_sort(sort: &str) -> impl IntoToken { literal(sort) }
fn submit_post() -> impl IntoToken { literal("") }
fn add_to_cart(id: impl IntoToken) -> impl IntoToken { id }
fn add_to_wishlist(id: impl IntoToken) -> impl IntoToken { id }
fn remove_from_wishlist(id: impl IntoToken) -> impl IntoToken { id }
fn prev_page() -> impl IntoToken { literal("") }
fn next_page() -> impl IntoToken { literal("") }
fn chart(data: impl IntoToken) -> impl IntoToken { data }
fn pie_chart(data: impl IntoToken) -> impl IntoToken { data }
fn table() -> impl IntoToken { row() }
fn table_header(headers: Vec<&str>) -> impl IntoToken { row() }
fn table_row() -> impl IntoToken { row() }
fn badge() -> impl IntoToken { txt("") }
fn range_slider() -> impl IntoToken { txt("") }
fn customer_overview() -> impl IntoToken { txt("") }
fn customer_notes() -> impl IntoToken { txt("") }
fn order_row() -> impl IntoToken { row() }
fn activity_item() -> impl IntoToken { row() }
fn related_post() -> impl IntoToken { col() }
fn bulk_activate() -> impl IntoToken { literal("") }
fn bulk_suspend() -> impl IntoToken { literal("") }
fn bulk_delete() -> impl IntoToken { literal("") }
fn toggle_selection(id: impl IntoToken) -> impl IntoToken { id }
fn in_(item: impl IntoToken, collection: impl IntoToken) -> impl IntoToken { item }
