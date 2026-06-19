# Structural Primitives for the Token DSL

## Core Philosophy

> Architecture over widgets. Solve structural problems, not application-specific ones.

The DSL is now capable of expressing **Instagram, Reddit, LinkedIn, ecommerce stores, dashboards, CRMs, blogs, SaaS products, forums, and future application types** using the same small set of primitives.

## Primitives Overview

### 1. Collection Iteration: `foreach()`

Iterate over any collection. Scales indefinitely without changing page structure.

```rust
// Good: Scales to any number of posts
foreach("posts")
    .item(|post_id| PostCard().bind("post", local(post_id)))
    .empty(txt("No posts yet"))

// Bad: Fixed number of hardcoded cards
PostCard1()
PostCard2()
PostCard3()
```

### 2. Bindings: `bind()`, `global()`, `local()`

Data separate from presentation. UI generated from structured data.

```rust
// Data-driven: UI reflects data structure
col()
    .child(txt(bind(global("user.name"))))
    .child(txt(bind(local("post.title"))))

// Template written once, reused indefinitely
component("PostCard")
    .param("title", "")
    .param("author", "")
    .body(|| col()
        .child(txt(bind("title")).bold())
        .child(txt(bind("author")).muted())
    )
```

### 3. Conditionals: `if_true()`, `if_false()`, `if_eq()`, `if_exists()`

Readable, predictable conditional rendering.

```rust
// Conditional UI based on data state
if_true("user.logged_in")
    .then(show_user_menu())
    .else_(show_login_button())

if_eq("user.role", "admin")
    .then(show_admin_panel())

if_exists("post.image")
    .then(show_image_preview())

if_gt("cart.total", 100.0)
    .then(show_free_shipping_banner())
```

### 4. Collection Operations: `count()`, `filter()`, `sort()`, `find()`, `limit()`

Transform collections declaratively.

```rust
// Count items
count("notifications")

// Filter with predicates
filter("products")
    .where_eq("category", "electronics")
    .where_exists("price")
    .render(|id| ProductCard())

// Sort results
sort("posts", "date")
    .descending()
    .render(|id| PostCard())

// Find single item
find("users")
    .where_eq("email", "user@example.com")
    .render(|id| UserProfile())

// Paginate results
limit("comments", 10)
    .skip(20)
    .render(|id| CommentCard())
```

### 5. Components: `component()`, `use_component()`, `slot()`

Reusable templates with composition via slots.

```rust
// Define a component template
component("PostCard")
    .param("title", "")
    .param("author", "")
    .slot("actions")
    .body(|| col()
        .child(txt(bind("title")))
        .child(txt(bind("author")))
        .child(slot("actions")) // Composition point
    )

// Use with different slot content
use_component("PostCard")
    .bind("title", global("post.title"))
    .bind("author", global("post.author"))
    .fill("actions", row()
        .child(btn("Like"))
        .child(btn("Share"))
    )
```

### 6. Relations: `relation()`

First-class relationships. Traversal between related records.

```rust
// User's posts
relation("user", "posts", "posts")
    .render(|post_id| PostCard())

// Post's comments
relation("post", "comments", "comments")
    .render(|comment_id| CommentCard())

// Product's reviews
relation("product", "reviews", "reviews")
    .render(|review_id| ReviewCard())
```

### 7. Queries: `query()`

Derived datasets from existing collections.

```rust
// Complex query with joins and filters
query()
    .select(vec!["title", "author", "date"])
    .from("posts")
    .join("author", "users", "author_id", "id")
    .where_eq("published", "true")
    .order_by("date", true)
    .limit(10)
    .render(|post_id| PostCard())
```

### 8. Scopes: `local_scope()`

Local context with inheritance from parent scopes.

```rust
// Create local scope for template
local_scope("post")
    .set("title", global("posts.0.title"))
    .set("author", global("posts.0.author"))
    .render(|| col()
        .child(txt(bind(local("title"))))
        .child(txt(bind(local("author"))))
    )
```

## Application Examples

### Instagram-like Feed

```rust
col()
    // Stories (horizontal scroll)
    .child(foreach("stories").item(|id| StoryBubble()))

    // Posts feed
    .child(foreach("posts")
        .item(|id| PostCard()
            .bind("post", local(id))
            .fill("actions", row()
                .child(btn("❤️").act(like_post(local(id))))
                .child(btn("💬").act(show_comments(local(id))))
            )
        )
    )
```

### Reddit-like Forum

```rust
// Nested comments via relations
fn comment_tree() -> impl IntoToken {
    use_component("Comment")
        .fill("replies",
            if_gt("comment.reply_count", 0)
                .then(relation("comment", "replies", "comments")
                    .render(|id| comment_tree()) // Recursive
                )
        )
}
```

### E-commerce Store

```rust
col()
    // Filtered product grid
    .child(filter("products")
        .where_eq("category", global("filters.category"))
        .where_exists("price")
        .render(|id| ProductCard()))

    // Cart count badge (conditional)
    .child(if_gt("cart.item_count", 0)
        .then(badge().count(count("cart.items"))))
```

### Dashboard Analytics

```rust
col()
    // KPI cards with live data
    .child(grid(4).child(
        foreach("kpi_metrics").item(|id| KpiCard())
    ))

    // Charts from queries
    .child(query()
        .select(vec!["revenue", "date"])
        .from("sales")
        .where_eq("period", "30d")
        .order_by("date", true)
        .render(|id| ChartPoint()))
```

### CRM - Customer Detail

```rust
col()
    // Customer header
    .child(txt(bind(global("customer.name"))))

    // Tabs with related data
    .child(tabs("active_tab", vec![
        ("Orders", relation("customer", "orders", "orders")
            .render(|id| OrderRow())),
        ("Activity", query()
            .from("activities")
            .where_eq("customer_id", global("customer.id"))
            .render(|id| ActivityItem())),
    ]))
```

## Design Principles Applied

| Principle | Implementation |
|-----------|----------------|
| **Reduce generated code** | Templates written once, data drives instances |
| **Improve readability** | Obvious primitives: `foreach`, `filter`, `if_true` |
| **Improve scalability** | Collections scale indefinitely; no structural changes |
| **Improve reusability** | `component()` with `slot()` for composition |
| **Data-driven design** | UI generated from `global()` and `local()` bindings |
| **Templates over duplication** | Define once in `component()`, use everywhere |
| **Composition over specialization** | `slot()` fills generic components with specific content |
| **General over specific** | `query()`, `relation()` work across all app types |
| **Data over duplication** | Store data in `Store`, bind in UI |
| **Shorter over configurable** | `if_exists("avatar")` vs `if_not_null_and_not_empty(...)` |

## Rejection Criteria

Primitives are rejected if they:
- Duplicate existing capabilities
- Solve only one product category
- Increase conceptual complexity
- Introduce framework-style abstractions
- Require excessive configuration

## Acceptance Criteria

Each primitive:
- Reduces generated code
- Improves readability
- Improves scalability
- Improves reuse
- Works across many application types
- Remains understandable without documentation

## Result

The ideal DSL allows entire applications to be generated from:
- **Relational data** via `relation()`
- **Reusable templates** via `component()` and `slot()`
- **Global state** via `global()`
- **Local state** via `local()` and `local_scope()`
- **Collection iteration** via `foreach()`
- **Composable components** via `use_component()`

While remaining readable from top to bottom without external documentation.
