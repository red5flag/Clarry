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
        .css("min-h-screen bg-black text-white")

        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-black border-b border-gray-800 px-4 py-3")
            .child(txt("Instagram").bold())
            .child(if_true("user.notifications")
                .then(badge().count(bind(global("user.notification_count"))))
            )
        )

        // Stories (horizontal scroll of users)
        .child(row()
            .css("px-4 py-3 gap-4 overflow-x-auto")
            .child(foreach("stories")
                .item(|story_id| story_bubble()
                    .bind("story", local(story_id))
                )
            )
        )

        // Posts feed (main content from data)
        .child(col()
            .css("divide-y divide-gray-800")
            .child(foreach("posts")
                .item(|post_id| post_card()
                    .bind("post", local(post_id))
                )
                .empty(txt("No posts yet. Follow someone!"))
            )
        )

        // Bottom nav
        .child(bottom_nav())
}

/// Reusable PostCard template - works with any post data
fn post_card() -> ComponentInstanceBuilder {
    use_component("PostCard")
        // Slots allow composition - different actions per use case
        .fill("actions", row()
            .css("flex gap-4")
            .child(btn("❤️").act(like_post(bind(local("post.id")))))
            .child(btn("💬").act(show_comments(bind(local("post.id")))))
            .child(btn("↗️").act(share_post(bind(local("post.id")))))
        )
}

fn story_bubble() -> ComponentInstanceBuilder {
    use_component("StoryBubble")
}

fn bottom_nav() -> impl IntoToken {
    row()
        .css("fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 px-4 py-3 justify-around")
        .child(btn("🏠").on_click_nav("home"))
        .child(btn("🔍").on_click_nav("search"))
        .child(btn("➕").on_click_nav("create"))
        .child(btn("❤️").on_click_nav("activity"))
        .child(btn("👤").on_click_nav("profile"))
}

// =============================================================================
// EXAMPLE 2: Reddit-like Forum (Nested comments, voting)
// =============================================================================

pub fn forum_page() -> impl IntoToken {
    col()
        .id("forum_page")
        .css("min-h-screen bg-gray-100")

        // Sort/filter controls
        .child(row()
            .css("px-4 py-3 gap-2 bg-white shadow-sm")
            .child(btn("Hot").act(set_sort("hot")))
            .child(btn("New").act(set_sort("new")))
            .child(btn("Top").act(set_sort("top")))
        )

        // Posts with filtering
        .child(col()
            .css("gap-2 p-2")
            .child(filter("posts")
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
            .child(if_gt("post.comment_count", 0)
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
        .css("items-center gap-1")
        .child(btn("▲").act(upvote(bind(local("post.id")))))
        .child(txt(bind(local("post.score"))))
        .child(btn("▼").act(downvote(bind(local("post.id")))))
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
        .css("min-h-screen bg-gray-50")

        // Create post composer (conditional)
        .child(if_true("user.logged_in")
            .then(post_composer())
        )

        // Feed with mixed content types
        .child(col()
            .css("gap-4 p-4 max-w-2xl mx-auto")
            .child(query()
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
        .css("bg-white rounded-lg shadow p-4 mb-4")
        .child(text_input()
            .placeholder("What's on your mind?")
            .bind(value(global("composer.text")))
        )
        .child(if_exists("composer.text")
            .then(row()
                .child(btn("Post").act(submit_post()))
            )
        )
}

// =============================================================================
// EXAMPLE 4: E-commerce Store (Products, cart, filters)
// =============================================================================

pub fn ecommerce_store() -> impl IntoToken {
    col()
        .id("store")
        .css("min-h-screen bg-white")

        // Header with cart count
        .child(row()
            .css("px-4 py-3 border-b justify-between items-center")
            .child(txt("Shop").bold())
            .child(row()
                .child(if_gt("cart.item_count", 0)
                    .then(badge()
                        .count(bind(global("cart.item_count")))
                        .css("bg-red-500 text-white")
                    )
                )
                .child(btn("🛒").on_click_nav("cart"))
            )
        )

        // Filters sidebar + product grid
        .child(row()
            .css("flex-1 gap-4 p-4")
            .child(col()
                .css("w-64 hidden md:flex gap-2")
                .child(txt("Filters").bold())
                .child(filter_controls())
            )
            .child(grid(3)
                .css("flex-1 gap-4")
                .child(filter("products")
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
        .css("gap-2")
        .child(txt("Category"))
        .child(foreach("categories")
            .item(|cat_id| row()
                .child(checkbox()
                    .checked(eq(bind(local(cat_id)), bind(global("filters.category"))))
                )
                .child(txt(bind(local("category.name"))))
            )
        )
        .child(txt("Price Range"))
        .child(range_slider()
            .min(0)
            .max(1000)
            .bind(value(global("filters.max_price")))
        )
}

fn product_card() -> ComponentInstanceBuilder {
    use_component("ProductCard")
        .fill("quick_actions", row()
            .child(btn("Add to Cart").act(add_to_cart(bind(local("product.id")))))
            .child(if_true("product.in_wishlist")
                .then(btn("❤️").act(remove_from_wishlist(bind(local("product.id")))))
                .else_(btn("🤍").act(add_to_wishlist(bind(local("product.id")))))
            )
        )
}

// =============================================================================
// EXAMPLE 5: Dashboard with Analytics
// =============================================================================

pub fn analytics_dashboard() -> impl IntoToken {
    col()
        .id("dashboard")
        .css("min-h-screen bg-gray-100 p-4")

        // KPI cards row
        .child(grid(4)
            .css("gap-4 mb-4")
            .child(kpi_card("Revenue", "metrics.revenue", "$"))
            .child(kpi_card("Users", "metrics.active_users", ""))
            .child(kpi_card("Conversion", "metrics.conversion", "%"))
            .child(kpi_card("Churn", "metrics.churn", "%"))
        )

        // Charts section
        .child(row()
            .css("gap-4 mb-4")
            .child(col()
                .css("flex-1 bg-white rounded-lg p-4")
                .child(txt("Revenue Over Time").bold())
                .child(chart(bind(global("data.revenue_chart"))))
            )
            .child(col()
                .css("w-80 bg-white rounded-lg p-4")
                .child(txt("Traffic Sources").bold())
                .child(pie_chart(bind(global("data.traffic_sources"))))
            )
        )

        // Data table with pagination
        .child(col()
            .css("bg-white rounded-lg")
            .child(row()
                .css("px-4 py-3 border-b justify-between")
                .child(txt("Recent Transactions").bold())
                .child(txt(format!("Showing {} of {}",
                    bind(global("pagination.per_page")),
                    count("transactions")
                )))
            )
            .child(table()
                .headers(vec!["Date", "Customer", "Amount", "Status"])
                .rows(limit("transactions")
                    .skip(computed("pagination.page * pagination.per_page"))
                    .limit(10)
                    .render(|tx_id| table_row()
                        .bind("tx", local(tx_id))
                    )
                )
            )
            .child(pagination_controls("transactions"))
        )
}

fn kpi_card(title: &str, metric_path: &str, prefix: &str) -> impl IntoToken {
    col()
        .css("bg-white rounded-lg p-4")
        .child(txt(title).muted())
        .child(row()
            .child(txt(prefix))
            .child(txt(bind(global(metric_path)))
                .css("text-2xl font-bold")
            )
        )
}

fn pagination_controls(collection: &str) -> impl IntoToken {
    row()
        .css("px-4 py-3 justify-center gap-2")
        .child(if_gt("pagination.page", 0)
            .then(btn("Previous").act(prev_page())))
        )
        .child(txt(format!("Page {} of {}",
            bind(global("pagination.page")),
            computed(format!("ceil(count({}) / pagination.per_page)", collection))
        )))
        .child(if_lt("pagination.page", computed("total_pages - 1"))
            .then(btn("Next").act(next_page())))
        )
}

// =============================================================================
// EXAMPLE 6: CRM - Customer Detail with Related Data
// =============================================================================

pub fn customer_detail_page() -> impl IntoToken {
    col()
        .id("customer_detail")
        .css("min-h-screen bg-gray-50")

        // Customer header
        .child(row()
            .css("bg-white p-6 shadow-sm")
            .child(img_block(bind(global("customer.avatar")))
                .css("w-20 h-20 rounded-full")
            )
            .child(col()
                .css("ml-4")
                .child(txt(bind(global("customer.name")))
                    .css("text-xl font-bold")
                )
                .child(txt(bind(global("customer.company"))).muted())
                .child(row()
                    .css("gap-2 mt-2")
                    .child(badge()
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
        .child(tabs("active_tab", vec![
            ("Overview", customer_overview()),
            ("Orders", customer_orders()),
            ("Activity", customer_activity()),
            ("Notes", customer_notes()),
        ]))
}

fn customer_orders() -> impl IntoToken {
    col()
        .css("p-4 gap-2")
        .child(row()
            .css("justify-between mb-4")
            .child(txt("Order History").bold())
            .child(txt(format!("Total: {}", count("customer.orders"))))
        )
        .child(sort("customer.orders", "date")
            .descending()
            .render(|order_id| order_row()
                .bind("order", local(order_id))
            )
        )
}

fn customer_activity() -> impl IntoToken {
    col()
        .css("p-4")
        .child(query()
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
        .css("min-h-screen bg-white max-w-3xl mx-auto px-4 py-8")

        // Post header with author relation
        .child(col()
            .css("mb-8")
            .child(txt(bind(global("post.title")))
                .css("text-3xl font-bold mb-4")
            )
            .child(row()
                .css("items-center gap-3")
                // Author loaded via relation
                .child(relation("post", "author", "authors")
                    .render(|author_id| row()
                        .css("items-center gap-2")
                        .child(img_block(bind(local("author.avatar")))
                            .css("w-10 h-10 rounded-full")
                        )
                        .child(txt(bind(local("author.name"))))
                    )
                )
                .child(txt("·").muted())
                .child(txt(bind(global("post.published_at"))).muted())
            )
        )

        // Post content
        .child(txt(bind(global("post.content"))))

        // Related posts
        .child(col()
            .css("mt-12 pt-8 border-t")
            .child(txt("Related Posts").bold())
            .child(query()
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
        .css("min-h-screen bg-gray-100 p-4")

        // Search and filters
        .child(row()
            .css("bg-white rounded-lg p-4 mb-4 gap-4")
            .child(text_input()
                .placeholder("Search users...")
                .bind(value(global("admin.user_search")))
                .css("flex-1")
            )
            .child(select()
                .options(vec!["All", "Active", "Inactive", "Suspended"])
                .bind(value(global("admin.user_filter_status")))
            )
        )

        // User table with filtering
        .child(col()
            .css("bg-white rounded-lg")
            .child(table_header(vec![
                "User", "Email", "Role", "Status", "Last Active", "Actions"
            ]))
            .child(filter("users")
                .where_contains("name", bind(global("admin.user_search")))
                .where_eq("status", bind(global("admin.user_filter_status")))
                .render(|user_id| user_admin_row()
                    .bind("user", local(user_id))
                )
            )
            .child(if_false("users")
                .then(col()
                    .css("p-8 text-center")
                    .child(txt("No users found").muted())
                )
            )
        )

        // Bulk actions (conditional)
        .child(if_gt("admin.selected_users.count", 0)
            .then(row()
                .css("fixed bottom-4 left-4 right-4 bg-white shadow-lg rounded-lg p-4 justify-between items-center")
                .child(txt(format!("{} users selected",
                    bind(global("admin.selected_users.count"))
                )))
                .child(row()
                    .css("gap-2")
                    .child(btn("Activate").act(bulk_activate())))
                    .child(btn("Suspend").act(bulk_suspend())))
                    .child(btn("Delete").var("danger").act(bulk_delete())))
                )
            )
        )
}

fn user_admin_row() -> impl IntoToken {
    row()
        .css("px-4 py-3 border-b items-center hover:bg-gray-50")
        .child(checkbox()
            .checked(in_(local("user.id"), global("admin.selected_users")))
            .act(toggle_selection(bind(local("user.id"))))
        )
        .child(img_block(bind(local("user.avatar")))
            .css("w-8 h-8 rounded-full ml-2")
        )
        .child(txt(bind(local("user.name"))))
        .child(txt(bind(local("user.email"))).muted())
        .child(badge().label(bind(local("user.role"))))
        .child(badge()
            .label(bind(local("user.status")))
            .variant(if_eq("user.status", "active")
                .then(literal("success"))
                .else_(literal("warning"))
            )
        )
        .child(txt(bind(local("user.last_active"))).muted())
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
