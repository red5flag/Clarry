// src/tokens/schema.rs
//
// Typed data schema primitive for the storage DSL.
// Schemas are registered globally at startup and drive:
//   - Storage key namespacing
//   - Automatic preloading when a read/write touches a cold key
//   - Serialization format
//   - Cache invalidation strategy

use std::sync::Arc;
use std::sync::OnceLock;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

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
    schema_registry().iter().find(|s| s.namespace == prefix).map(|s| s.clone())
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
