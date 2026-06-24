// src/tokens/schema.rs
//
// Typed data schema primitive for the storage DSL.
// Schemas are registered globally at startup and drive:
//   - Storage key namespacing
//   - Automatic preloading when a read/write touches a cold key
//   - Serialization format
//   - Cache invalidation strategy

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::OnceLock;

// ── Schema ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub name: &'static str,
    pub namespace: &'static str,
    pub fields: Vec<FieldDef>,
    pub preload: PreloadStrategy,
    pub cache_ttl_secs: Option<u64>,
    pub format: StorageFormat,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PreloadStrategy {
    Eager,
    OnFirstRead,
    OnFirstWrite,
    Manual,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StorageFormat {
    Json,
    Bincode,
    MsgPack,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: &'static str,
    pub kind: FieldKind,
    pub default: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FieldKind {
    Text,
    Number,
    Bool,
    ImageUrl,
    VideoUrl,
    Json,
    List(Box<FieldKind>),
    Map(Box<FieldKind>, Box<FieldKind>),
    Timestamp,
    UserId,
}

// ── Global SchemaRegistry ───────────────────────────────────────────────────

static SCHEMA_REGISTRY: OnceLock<Arc<DashMap<&'static str, Schema>>> = OnceLock::new();

pub fn schema_registry() -> &'static DashMap<&'static str, Schema> {
    SCHEMA_REGISTRY.get_or_init(|| Arc::new(DashMap::new()))
}

pub fn register_schema(schema: Schema) {
    schema_registry().insert(schema.name, schema);
}

pub fn get_schema(name: &str) -> Option<Schema> {
    schema_registry().get(name).map(|s| s.clone())
}

/// Find which schema owns a given key prefix.
/// Key format: "namespace:id:field" (e.g., "u:alice:avatar_url")
pub fn schema_for_key(key: &str) -> Option<Schema> {
    let prefix = key.split(':').next()?;
    schema_registry()
        .iter()
        .find(|s| s.namespace == prefix)
        .map(|s| s.clone())
}

/// Parse a key like "u:alice:avatar_url" → (schema_name, id, field)
pub fn parse_key(key: &str) -> Option<(&str, &str, &str)> {
    let parts: Vec<&str> = key.splitn(3, ':').collect();
    if parts.len() == 3 {
        Some((parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_schema(name: &'static str, ns: &'static str) -> Schema {
        Schema {
            name,
            namespace: ns,
            fields: vec![FieldDef {
                name: "display_name",
                kind: FieldKind::Text,
                default: None,
            }],
            preload: PreloadStrategy::OnFirstRead,
            cache_ttl_secs: Some(300),
            format: StorageFormat::Json,
        }
    }

    #[test]
    fn register_and_get_schema() {
        let schema = test_schema("test_user", "tu");
        register_schema(schema);
        let retrieved = get_schema("test_user");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().namespace, "tu");
    }

    #[test]
    fn get_missing_schema_returns_none() {
        assert!(get_schema("nonexistent_xyz").is_none());
    }

    #[test]
    fn schema_for_key_finds_by_namespace() {
        let schema = test_schema("profile_schema", "prof");
        register_schema(schema);
        let found = schema_for_key("prof:alice:avatar");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "profile_schema");
    }

    #[test]
    fn schema_for_key_returns_none_for_unknown_prefix() {
        assert!(schema_for_key("zzz:id:field").is_none());
    }

    #[test]
    fn parse_key_valid() {
        let result = parse_key("u:alice:avatar_url");
        assert_eq!(result, Some(("u", "alice", "avatar_url")));
    }

    #[test]
    fn parse_key_with_colons_in_field() {
        let result = parse_key("ns:id:field:with:colons");
        assert_eq!(result, Some(("ns", "id", "field:with:colons")));
    }

    #[test]
    fn parse_key_too_few_parts() {
        assert!(parse_key("only_one").is_none());
        assert!(parse_key("two:parts").is_none());
    }

    #[test]
    fn field_kind_list_nesting() {
        let kind = FieldKind::List(Box::new(FieldKind::Text));
        assert_eq!(kind, FieldKind::List(Box::new(FieldKind::Text)));
    }

    #[test]
    fn field_kind_map_nesting() {
        let kind = FieldKind::Map(Box::new(FieldKind::Text), Box::new(FieldKind::Number));
        if let FieldKind::Map(k, v) = kind {
            assert_eq!(*k, FieldKind::Text);
            assert_eq!(*v, FieldKind::Number);
        } else {
            panic!("expected Map variant");
        }
    }
}
