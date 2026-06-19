// src/tokens/structural.rs
//
// Core structural primitives for the token DSL.
// Architecture-first: data-driven, reusable templates, composition over specialization.
//
// Primitives:
//   - foreach()     : iterate over collections
//   - bind()          : bind values from scope
//   - component()     : reusable templates
//   - slot()          : composition points within components
//   - if_true()       : conditional rendering (truthy check)
//   - if_false()      : conditional rendering (falsy check)
//   - if_eq()         : conditional rendering (equality)
//   - if_exists()     : conditional rendering (presence check)
//   - count()         : collection count
//   - filter()        : filtered collection view
//   - sort()          : sorted collection view
//   - find()          : single item lookup
//   - limit()         : limited collection view
//   - relation()      : relational data traversal
//   - query()         : derived datasets
//   - global()        : global scope access
//   - local()         : local scope access

use std::sync::Arc;
use crate::tokens::node::{IntoToken, Str, TokenNode};
use crate::tokens::core::id::next_id;
use crate::tokens::storage::primitive::Store;

// =============================================================================
// TYPE DEFINITIONS
// =============================================================================

/// A template component that can be instantiated with data.
#[derive(Clone)]
pub struct Template {
    pub name: Str,
    pub params: Vec<(Str, Str)>, // (param_name, default_value)
    pub root: TokenNode,
    pub slots: Vec<SlotDef>,
}

#[derive(Clone)]
pub struct SlotDef {
    pub name: Str,
    pub default_content: Option<TokenNode>,
}

/// A component instance with bound data.
#[derive(Clone)]
pub struct ComponentInstance {
    pub template_name: Str,
    pub bindings: Vec<(Str, BindingSource)>, // (param_name, source)
    pub slot_overrides: Vec<(Str, TokenNode)>, // (slot_name, content)
}

#[derive(Clone)]
pub enum BindingSource {
    Global(Str),      // global("user.name")
    Local(Str),       // local("post.title")
    Literal(Str),     // "literal value"
    Computed(Str),    // computed expression
}

/// Collection iterator with transformation.
#[derive(Clone)]
pub struct Foreach {
    pub source: Str,           // path to collection: "posts", "user.comments"
    pub item_template: Arc<dyn Fn(Str) -> TokenNode + Send + Sync>, // item -> node
    pub empty_state: Option<TokenNode>,
    pub key_extractor: Option<Arc<dyn Fn(&str) -> Str + Send + Sync>>, // item -> unique key
}

/// Conditional rendering nodes.
#[derive(Clone)]
pub struct Conditional {
    pub condition: Condition,
    pub then_branch: TokenNode,
    pub else_branch: Option<TokenNode>,
}

#[derive(Clone)]
pub enum Condition {
    True(Str),          // if_true("user.logged_in")
    False(Str),         // if_false("user.banned")
    Eq(Str, Str),       // if_eq("user.role", "admin")
    Exists(Str),        // if_exists("user.avatar")
    Gt(Str, f64),       // if_gt("cart.total", 100.0)
    Lt(Str, f64),       // if_lt("inventory.count", 10)
}

/// Collection operation that produces a derived view.
#[derive(Clone)]
pub struct CollectionView {
    pub source: Str,
    pub operation: CollectionOp,
    pub item_template: Arc<dyn Fn(Str) -> TokenNode + Send + Sync>,
}

#[derive(Clone)]
pub enum CollectionOp {
    Count,
    Filter { predicate: FilterPredicate },
    Sort { key: Str, descending: bool },
    Find { predicate: FilterPredicate },
    Limit { n: usize },
    Skip { n: usize },
    Query { query_def: QueryDef },
}

#[derive(Clone)]
pub enum FilterPredicate {
    Eq { field: Str, value: Str },
    Contains { field: Str, value: Str },
    Gt { field: Str, value: f64 },
    Exists { field: Str },
    And(Vec<FilterPredicate>),
    Or(Vec<FilterPredicate>),
}

#[derive(Clone)]
pub struct QueryDef {
    pub select: Vec<Str>,
    pub from: Str,
    pub joins: Vec<JoinDef>,
    pub where_clause: Option<FilterPredicate>,
    pub order_by: Vec<(Str, bool)>, // (field, descending)
    pub limit: Option<usize>,
}

#[derive(Clone)]
pub struct JoinDef {
    pub relation_name: Str,
    pub source_key: Str,
    pub target_collection: Str,
    pub target_key: Str,
}

/// Relational data traversal.
#[derive(Clone)]
pub struct Relation {
    pub from: Str,           // source record path
    pub through: Str,        // relation field name
    pub to: Str,             // target collection
    pub template: Arc<dyn Fn(Str) -> TokenNode + Send + Sync>,
}

/// Scope binding for local context.
#[derive(Clone)]
pub struct ScopeBinding {
    pub scope_name: Str,
    pub parent_scope: Option<Str>,
    pub bindings: Vec<(Str, BindingSource)>,
}

// =============================================================================
// FACTORY FUNCTIONS
// =============================================================================

/// Iterate over a collection, rendering a template for each item.
///
/// # Examples
/// ```rust
/// foreach("posts")
///     .item(|id| PostCard().bind("post", local(id)))
///     .empty(|| txt("No posts yet"))
/// ```
pub fn foreach(source: impl Into<Str>) -> ForeachBuilder {
    ForeachBuilder {
        source: source.into(),
        item_template: None,
        empty_state: None,
        key_extractor: None,
    }
}

pub struct ForeachBuilder {
    source: Str,
    item_template: Option<Arc<dyn Fn(Str) -> TokenNode + Send + Sync>>,
    empty_state: Option<TokenNode>,
    key_extractor: Option<Arc<dyn Fn(&str) -> Str + Send + Sync>>,
}

impl ForeachBuilder {
    /// Define the template for each item. The closure receives the item identifier.
    pub fn item<F>(mut self, template: F) -> Self
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        self.item_template = Some(Arc::new(template));
        self
    }

    /// Define content to show when the collection is empty.
    pub fn empty(mut self, content: impl IntoToken) -> Self {
        self.empty_state = Some(content.into_node());
        self
    }

    /// Extract a unique key from each item for stable rendering.
    pub fn key_by<F>(mut self, extractor: F) -> Self
    where
        F: Fn(&str) -> Str + Send + Sync + 'static,
    {
        self.key_extractor = Some(Arc::new(extractor));
        self
    }

    /// Build the Foreach node.
    pub fn build(self) -> Foreach {
        Foreach {
            source: self.source,
            item_template: self.item_template
                .unwrap_or_else(|| Arc::new(|_| TokenNode::new(next_id()))),
            empty_state: self.empty_state,
            key_extractor: self.key_extractor,
        }
    }
}

impl IntoToken for ForeachBuilder {
    fn into_node(self) -> TokenNode {
        let foreach = self.build();
        // Store the foreach definition in a special node that the renderer will process
        let mut node = TokenNode::new(format!("foreach_{}", foreach.source));
        node.tag = "foreach".into();
        node.attributes.insert("data-source".into(), foreach.source.clone());
        // The renderer will use this to dynamically generate children
        node
    }
}

// =============================================================================
// CONDITIONAL PRIMITIVES
// =============================================================================

/// Render content only if the condition is truthy.
pub fn if_true(condition: impl Into<Str>) -> ConditionalBuilder {
    ConditionalBuilder {
        condition: Condition::True(condition.into()),
        then_branch: None,
        else_branch: None,
    }
}

/// Render content only if the condition is falsy.
pub fn if_false(condition: impl Into<Str>) -> ConditionalBuilder {
    ConditionalBuilder {
        condition: Condition::False(condition.into()),
        then_branch: None,
        else_branch: None,
    }
}

/// Render content only if values are equal.
pub fn if_eq(left: impl Into<Str>, right: impl Into<Str>) -> ConditionalBuilder {
    ConditionalBuilder {
        condition: Condition::Eq(left.into(), right.into()),
        then_branch: None,
        else_branch: None,
    }
}

/// Render content only if the value exists (not null/undefined/empty).
pub fn if_exists(path: impl Into<Str>) -> ConditionalBuilder {
    ConditionalBuilder {
        condition: Condition::Exists(path.into()),
        then_branch: None,
        else_branch: None,
    }
}

/// Render content only if value is greater than threshold.
pub fn if_gt(path: impl Into<Str>, threshold: f64) -> ConditionalBuilder {
    ConditionalBuilder {
        condition: Condition::Gt(path.into(), threshold),
        then_branch: None,
        else_branch: None,
    }
}

/// Render content only if value is less than threshold.
pub fn if_lt(path: impl Into<Str>, threshold: f64) -> ConditionalBuilder {
    ConditionalBuilder {
        condition: Condition::Lt(path.into(), threshold),
        then_branch: None,
        else_branch: None,
    }
}

pub struct ConditionalBuilder {
    condition: Condition,
    then_branch: Option<TokenNode>,
    else_branch: Option<TokenNode>,
}

impl ConditionalBuilder {
    /// Content to render when condition is true.
    pub fn then(mut self, content: impl IntoToken) -> Self {
        self.then_branch = Some(content.into_node());
        self
    }

    /// Content to render when condition is false (optional).
    pub fn else_(mut self, content: impl IntoToken) -> Self {
        self.else_branch = Some(content.into_node());
        self
    }

    pub fn build(self) -> Conditional {
        Conditional {
            condition: self.condition,
            then_branch: self.then_branch.unwrap_or_else(|| TokenNode::new(next_id())),
            else_branch: self.else_branch,
        }
    }
}

impl IntoToken for ConditionalBuilder {
    fn into_node(self) -> TokenNode {
        let cond = self.build();
        let mut node = TokenNode::new(format!("cond_{}", next_id()));
        node.tag = "conditional".into();
        node.children.push(cond.then_branch);
        if let Some(else_branch) = cond.else_branch {
            node.children.push(else_branch);
        }
        // Store condition in attributes for renderer
        let cond_str = match cond.condition {
            Condition::True(p) => format!("true:{}", p),
            Condition::False(p) => format!("false:{}", p),
            Condition::Eq(l, r) => format!("eq:{}:{}", l, r),
            Condition::Exists(p) => format!("exists:{}", p),
            Condition::Gt(p, v) => format!("gt:{}:{}", p, v),
            Condition::Lt(p, v) => format!("lt:{}:{}", p, v),
        };
        node.attributes.insert("data-condition".into(), cond_str.into());
        node
    }
}

// =============================================================================
// COLLECTION OPERATIONS
// =============================================================================

/// Count items in a collection.
pub fn count(source: impl Into<Str>) -> impl IntoToken {
    let source = source.into();
    let mut node = TokenNode::new(format!("count_{}", source));
    node.tag = "count".into();
    node.attributes.insert("data-source".into(), source);
    node
}

/// Filter a collection by predicate.
pub fn filter(source: impl Into<Str>) -> FilterBuilder {
    FilterBuilder {
        source: source.into(),
        predicates: Vec::new(),
    }
}

pub struct FilterBuilder {
    source: Str,
    predicates: Vec<FilterPredicate>,
}

impl FilterBuilder {
    /// Filter where field equals value.
    pub fn where_eq(mut self, field: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.predicates.push(FilterPredicate::Eq {
            field: field.into(),
            value: value.into(),
        });
        self
    }

    /// Filter where field contains value.
    pub fn where_contains(mut self, field: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.predicates.push(FilterPredicate::Contains {
            field: field.into(),
            value: value.into(),
        });
        self
    }

    /// Filter where field exists.
    pub fn where_exists(mut self, field: impl Into<Str>) -> Self {
        self.predicates.push(FilterPredicate::Exists {
            field: field.into(),
        });
        self
    }

    /// Render items matching the filter.
    pub fn render<F>(self, template: F) -> CollectionView
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        let predicate = if self.predicates.len() == 1 {
            self.predicates.into_iter().next().unwrap()
        } else {
            FilterPredicate::And(self.predicates)
        };
        CollectionView {
            source: self.source,
            operation: CollectionOp::Filter { predicate },
            item_template: Arc::new(template),
        }
    }
}

/// Sort a collection by field.
pub fn sort(source: impl Into<Str>, by: impl Into<Str>) -> SortBuilder {
    SortBuilder {
        source: source.into(),
        key: by.into(),
        descending: false,
    }
}

pub struct SortBuilder {
    source: Str,
    key: Str,
    descending: bool,
}

impl SortBuilder {
    /// Sort in descending order.
    pub fn descending(mut self) -> Self {
        self.descending = true;
        self
    }

    /// Render sorted items.
    pub fn render<F>(self, template: F) -> CollectionView
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        CollectionView {
            source: self.source,
            operation: CollectionOp::Sort {
                key: self.key,
                descending: self.descending,
            },
            item_template: Arc::new(template),
        }
    }
}

/// Find a single item in a collection.
pub fn find(source: impl Into<Str>) -> FindBuilder {
    FindBuilder {
        source: source.into(),
        predicates: Vec::new(),
    }
}

pub struct FindBuilder {
    source: Str,
    predicates: Vec<FilterPredicate>,
}

impl FindBuilder {
    /// Find where field equals value.
    pub fn where_eq(mut self, field: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.predicates.push(FilterPredicate::Eq {
            field: field.into(),
            value: value.into(),
        });
        self
    }

    /// Render the found item (or empty if not found).
    pub fn render<F>(self, template: F) -> CollectionView
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        let predicate = if self.predicates.len() == 1 {
            self.predicates.into_iter().next().unwrap()
        } else {
            FilterPredicate::And(self.predicates)
        };
        CollectionView {
            source: self.source,
            operation: CollectionOp::Find { predicate },
            item_template: Arc::new(template),
        }
    }
}

/// Limit collection to N items.
pub fn limit(source: impl Into<Str>, n: usize) -> LimitBuilder {
    LimitBuilder {
        source: source.into(),
        n,
        skip: 0,
    }
}

pub struct LimitBuilder {
    source: Str,
    n: usize,
    skip: usize,
}

impl LimitBuilder {
    /// Skip N items before limiting.
    pub fn skip(mut self, n: usize) -> Self {
        self.skip = n;
        self
    }

    /// Render limited items.
    pub fn render<F>(self, template: F) -> CollectionView
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        let mut view = CollectionView {
            source: self.source.clone(),
            operation: CollectionOp::Limit { n: self.n },
            item_template: Arc::new(template),
        };
        if self.skip > 0 {
            // Chain operations: skip then limit
            view.operation = CollectionOp::Query {
                query_def: QueryDef {
                    select: vec!["*".into()],
                    from: self.source,
                    joins: Vec::new(),
                    where_clause: None,
                    order_by: Vec::new(),
                    limit: Some(self.n),
                },
            };
        }
        view
    }
}

impl IntoToken for CollectionView {
    fn into_node(self) -> TokenNode {
        let mut node = TokenNode::new(format!("view_{}", self.source));
        node.tag = "collection-view".into();
        node.attributes.insert("data-source".into(), self.source);
        // The renderer will process the operation and generate children
        node
    }
}

// =============================================================================
// COMPONENT SYSTEM
// =============================================================================

/// Define a reusable template component.
///
/// # Examples
/// ```rust
/// component("PostCard")
///     .param("title", "")
///     .param("author", "")
///     .param("image", "")
///     .slot("actions")
///     .body(|| col()
///         .child(img_block(bind("image")))
///         .child(txt(bind("title")).bold())
///         .child(txt(bind("author")).muted())
///         .child(slot("actions"))
///     )
/// ```
pub fn component(name: impl Into<Str>) -> ComponentDefBuilder {
    ComponentDefBuilder {
        name: name.into(),
        params: Vec::new(),
        slots: Vec::new(),
        body: None,
    }
}

pub struct ComponentDefBuilder {
    name: Str,
    params: Vec<(Str, Str)>,
    slots: Vec<SlotDef>,
    body: Option<TokenNode>,
}

impl ComponentDefBuilder {
    /// Define a parameter with optional default value.
    pub fn param(mut self, name: impl Into<Str>, default: impl Into<Str>) -> Self {
        self.params.push((name.into(), default.into()));
        self
    }

    /// Define a slot that can be filled when instantiating.
    pub fn slot(mut self, name: impl Into<Str>) -> Self {
        self.slots.push(SlotDef {
            name: name.into(),
            default_content: None,
        });
        self
    }

    /// Define the component body.
    pub fn body(mut self, content: impl IntoToken) -> Self {
        self.body = Some(content.into_node());
        self
    }

    /// Build and register the template.
    pub fn build(self) -> Template {
        Template {
            name: self.name,
            params: self.params,
            root: self.body.unwrap_or_else(|| TokenNode::new(next_id())),
            slots: self.slots,
        }
    }
}

/// Instantiate a component with bindings.
pub fn use_component(name: impl Into<Str>) -> ComponentInstanceBuilder {
    ComponentInstanceBuilder {
        template_name: name.into(),
        bindings: Vec::new(),
        slot_overrides: Vec::new(),
    }
}

pub struct ComponentInstanceBuilder {
    template_name: Str,
    bindings: Vec<(Str, BindingSource)>,
    slot_overrides: Vec<(Str, TokenNode)>,
}

impl ComponentInstanceBuilder {
    /// Bind a parameter to a value.
    pub fn bind(mut self, param: impl Into<Str>, source: BindingSource) -> Self {
        self.bindings.push((param.into(), source));
        self
    }

    /// Fill a slot with content.
    pub fn fill(mut self, slot: impl Into<Str>, content: impl IntoToken) -> Self {
        self.slot_overrides.push((slot.into(), content.into_node()));
        self
    }

    pub fn build(self) -> ComponentInstance {
        ComponentInstance {
            template_name: self.template_name,
            bindings: self.bindings,
            slot_overrides: self.slot_overrides,
        }
    }
}

impl IntoToken for ComponentInstanceBuilder {
    fn into_node(self) -> TokenNode {
        let instance = self.build();
        let mut node = TokenNode::new(format!("comp_{}", instance.template_name));
        node.tag = "component-instance".into();
        node.attributes.insert("data-template".into(), instance.template_name);
        // Store bindings as serialized attribute for renderer
        let bindings_str = instance.bindings.iter()
            .map(|(k, v)| {
                let src_str = match v {
                    BindingSource::Global(p) => format!("global:{}", p),
                    BindingSource::Local(p) => format!("local:{}", p),
                    BindingSource::Literal(v) => format!("literal:{}", v),
                    BindingSource::Computed(e) => format!("computed:{}", e),
                };
                format!("{}={}", k, src_str)
            })
            .collect::<Vec<_>>()
            .join(",");
        node.attributes.insert("data-bindings".into(), bindings_str.into());
        node
    }
}

/// Create a slot placeholder within a component template.
pub fn slot(name: impl Into<Str>) -> impl IntoToken {
    let name = name.into();
    let mut node = TokenNode::new(format!("slot_{}", name));
    node.tag = "slot".into();
    node.attributes.insert("data-slot".into(), name);
    node
}

// =============================================================================
// BINDING PRIMITIVES
// =============================================================================

/// Reference a value from global scope.
pub fn global(path: impl Into<Str>) -> BindingSource {
    BindingSource::Global(path.into())
}

/// Reference a value from local (iteration) scope.
pub fn local(path: impl Into<Str>) -> BindingSource {
    BindingSource::Local(path.into())
}

/// Create a literal binding value.
pub fn literal(value: impl Into<Str>) -> BindingSource {
    BindingSource::Literal(value.into())
}

/// Create a computed binding expression.
pub fn computed(expr: impl Into<Str>) -> BindingSource {
    BindingSource::Computed(expr.into())
}

/// Bind a value directly into a node (shorthand for text content).
pub fn bind(source: BindingSource) -> impl IntoToken {
    let mut node = TokenNode::new(format!("bind_{}", next_id()));
    node.tag = "bind".into();
    let src_str = match source {
        BindingSource::Global(p) => format!("global:{}", p),
        BindingSource::Local(p) => format!("local:{}", p),
        BindingSource::Literal(v) => format!("literal:{}", v),
        BindingSource::Computed(e) => format!("computed:{}", e),
    };
    node.attributes.insert("data-source".into(), src_str.into());
    node
}

// =============================================================================
// RELATION PRIMITIVES
// =============================================================================

/// Traverse a relationship to render related data.
///
/// # Examples
/// ```rust
/// // User's posts
/// relation("user", "posts", "posts")
///     .render(|post_id| PostCard().bind("post", local(post_id)))
///
/// // Post's comments
/// relation("post", "comments", "comments")
///     .render(|comment_id| CommentCard().bind("comment", local(comment_id)))
/// ```
pub fn relation(from: impl Into<Str>, through: impl Into<Str>, to: impl Into<Str>) -> RelationBuilder {
    RelationBuilder {
        from: from.into(),
        through: through.into(),
        to: to.into(),
        template: None,
    }
}

pub struct RelationBuilder {
    from: Str,
    through: Str,
    to: Str,
    template: Option<Arc<dyn Fn(Str) -> TokenNode + Send + Sync>>,
}

impl RelationBuilder {
    /// Define the template for rendering each related item.
    pub fn render<F>(mut self, template: F) -> Relation
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        Relation {
            from: self.from,
            through: self.through,
            to: self.to,
            template: Arc::new(template),
        }
    }
}

impl IntoToken for RelationBuilder {
    fn into_node(self) -> TokenNode {
        let rel = self.render(|_| TokenNode::new(next_id()));
        let mut node = TokenNode::new(format!("rel_{}_{}", rel.from, rel.through));
        node.tag = "relation".into();
        node.attributes.insert("data-from".into(), rel.from);
        node.attributes.insert("data-through".into(), rel.through);
        node.attributes.insert("data-to".into(), rel.to);
        node
    }
}

// =============================================================================
// QUERY PRIMITIVES
// =============================================================================

/// Define a derived dataset from existing collections.
///
/// # Examples
/// ```rust
/// query()
///     .select(vec!["title", "author", "date"])
///     .from("posts")
///     .join("author", "users", "author_id", "id")
///     .where_eq("published", "true")
///     .order_by("date", true)
///     .limit(10)
///     .render(|post| PostCard())
/// ```
pub fn query() -> QueryBuilder {
    QueryBuilder {
        select: Vec::new(),
        from: "".into(),
        joins: Vec::new(),
        where_clause: None,
        order_by: Vec::new(),
        limit: None,
    }
}

pub struct QueryBuilder {
    select: Vec<Str>,
    from: Str,
    joins: Vec<JoinDef>,
    where_clause: Option<FilterPredicate>,
    order_by: Vec<(Str, bool)>,
    limit: Option<usize>,
}

impl QueryBuilder {
    /// Select fields (use "*" for all).
    pub fn select(mut self, fields: Vec<impl Into<Str>>) -> Self {
        self.select = fields.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Source collection.
    pub fn from(mut self, collection: impl Into<Str>) -> Self {
        self.from = collection.into();
        self
    }

    /// Join with another collection via relation.
    pub fn join(
        mut self,
        relation: impl Into<Str>,
        target: impl Into<Str>,
        source_key: impl Into<Str>,
        target_key: impl Into<Str>,
    ) -> Self {
        self.joins.push(JoinDef {
            relation_name: relation.into(),
            source_key: source_key.into(),
            target_collection: target.into(),
            target_key: target_key.into(),
        });
        self
    }

    /// Filter: field equals value.
    pub fn where_eq(mut self, field: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.where_clause = Some(FilterPredicate::Eq {
            field: field.into(),
            value: value.into(),
        });
        self
    }

    /// Sort results.
    pub fn order_by(mut self, field: impl Into<Str>, descending: bool) -> Self {
        self.order_by.push((field.into(), descending));
        self
    }

    /// Limit results.
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Render the query results.
    pub fn render<F>(self, template: F) -> CollectionView
    where
        F: Fn(Str) -> TokenNode + Send + Sync + 'static,
    {
        CollectionView {
            source: self.from.clone(),
            operation: CollectionOp::Query {
                query_def: QueryDef {
                    select: self.select,
                    from: self.from,
                    joins: self.joins,
                    where_clause: self.where_clause,
                    order_by: self.order_by,
                    limit: self.limit,
                },
            },
            item_template: Arc::new(template),
        }
    }
}

// =============================================================================
// SCOPE PRIMITIVES
// =============================================================================

/// Create a local scope with bindings.
///
/// # Examples
/// ```rust
/// local_scope("post")
///     .set("title", global("posts.0.title"))
///     .set("author", global("posts.0.author"))
///     .render(|| col()
///         .child(txt(bind(local("title"))))
///         .child(txt(bind(local("author"))))
///     )
/// ```
pub fn local_scope(name: impl Into<Str>) -> ScopeBuilder {
    ScopeBuilder {
        name: name.into(),
        parent: None,
        bindings: Vec::new(),
        content: None,
    }
}

pub struct ScopeBuilder {
    name: Str,
    parent: Option<Str>,
    bindings: Vec<(Str, BindingSource)>,
    content: Option<TokenNode>,
}

impl ScopeBuilder {
    /// Set a binding in this scope.
    pub fn set(mut self, key: impl Into<Str>, source: BindingSource) -> Self {
        self.bindings.push((key.into(), source));
        self
    }

    /// Specify parent scope for inheritance.
    pub fn parent(mut self, parent: impl Into<Str>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Content to render within this scope.
    pub fn render(mut self, content: impl IntoToken) -> Self {
        self.content = Some(content.into_node());
        self
    }

    pub fn build(self) -> ScopeBinding {
        ScopeBinding {
            scope_name: self.name,
            parent_scope: self.parent,
            bindings: self.bindings,
        }
    }
}

impl IntoToken for ScopeBuilder {
    fn into_node(self) -> TokenNode {
        let content = self.content.clone();
        let scope = self.build();
        let mut node = TokenNode::new(format!("scope_{}", scope.scope_name));
        node.tag = "scope".into();
        if let Some(parent) = scope.parent_scope {
            node.attributes.insert("data-parent".into(), parent);
        }
        // Store bindings
        let bindings_str = scope.bindings.iter()
            .map(|(k, v)| {
                let src_str = match v {
                    BindingSource::Global(p) => format!("global:{}", p),
                    BindingSource::Local(p) => format!("local:{}", p),
                    BindingSource::Literal(v) => format!("literal:{}", v),
                    BindingSource::Computed(e) => format!("computed:{}", e),
                };
                format!("{}={}", k, src_str)
            })
            .collect::<Vec<_>>()
            .join(",");
        node.attributes.insert("data-bindings".into(), bindings_str.into());
        if let Some(content) = content {
            node.children.push(content);
        }
        node
    }
}
