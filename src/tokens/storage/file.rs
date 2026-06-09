// src/tokens/storage/file.rs
//
// Filesystem-backed storage with dot-notation key paths.
// Maps keys like "user.profile.name" to nested directory files.

use std::path::{Path, PathBuf};
#[cfg(target_arch = "wasm32")]
use dashmap::DashMap;
use once_cell::sync::Lazy;

// ── Dot-notation helpers ────────────────────────────────────────────────────

/// Convert a dot-notation key to a filesystem path.
/// "user.profile.name" → "data/user/profile/name.json"
pub fn dot_to_path(key: &str) -> PathBuf {
    let relative = key.replace('.', "/");
    Path::new("data").join(&relative).with_extension("json")
}

/// Convert a filesystem path back to a dot-notation key.
/// "data/user/profile/name.json" → "user.profile.name"
pub fn path_to_dot(path: &Path) -> Option<String> {
    let stripped = path.strip_prefix("data").ok()?;
    let without_ext = stripped.with_extension("");
    let components: Vec<_> = without_ext
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    if components.is_empty() {
        return None;
    }
    Some(components.join("."))
}

// ── FileStore (native: tokio::fs, wasm: in-memory fallback) ────────────────

#[cfg(not(target_arch = "wasm32"))]
static FILE_STORE: Lazy<FileStore> = Lazy::new(|| FileStore::new("data"));

#[cfg(target_arch = "wasm32")]
static FILE_STORE: Lazy<FileStore> = Lazy::new(FileStore::new_wasm);

pub struct FileStore {
    base_dir: PathBuf,
    /// WASM fallback: in-memory cache when real FS is unavailable.
    #[cfg(target_arch = "wasm32")]
    cache: DashMap<String, String>,
}

impl FileStore {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self { base_dir: base_dir.into() }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_wasm() -> Self {
        Self {
            base_dir: PathBuf::from("data"),
            cache: DashMap::new(),
        }
    }

    pub fn global() -> &'static Self {
        &FILE_STORE
    }

    fn resolve(&self, key: &str) -> PathBuf {
        self.base_dir.join(key.replace('.', "/")).with_extension("json")
    }

    /// Write a value to the filesystem (or WASM cache).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn set(&self, key: &str, value: &str) -> std::io::Result<()> {
        let path = self.resolve(key);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, value).await
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn set(&self, key: &str, value: &str) -> std::io::Result<()> {
        self.cache.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Read a value from the filesystem (or WASM cache).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get(&self, key: &str) -> std::io::Result<Option<String>> {
        let path = self.resolve(key);
        match tokio::fs::read_to_string(&path).await {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get(&self, key: &str) -> std::io::Result<Option<String>> {
        Ok(self.cache.get(key).map(|v| v.clone()))
    }

    /// Delete a key.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn delete(&self, key: &str) -> std::io::Result<()> {
        let path = self.resolve(key);
        match tokio::fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn delete(&self, key: &str) -> std::io::Result<()> {
        self.cache.remove(key);
        Ok(())
    }

    /// List all keys under a prefix (e.g. "user.profile").
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn list(&self, prefix: &str) -> std::io::Result<Vec<String>> {
        let mut keys = Vec::new();
        let dir = if prefix.is_empty() {
            self.base_dir.clone()
        } else {
            self.base_dir.join(prefix.replace('.', "/"))
        };
        let mut entries = match tokio::fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(keys),
            Err(e) => return Err(e),
        };
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(dot_key) = path_to_dot(&path) {
                keys.push(dot_key);
            }
        }
        Ok(keys)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn list(&self, prefix: &str) -> std::io::Result<Vec<String>> {
        let mut keys = Vec::new();
        for entry in self.cache.iter() {
            let key = entry.key();
            if prefix.is_empty() || key.starts_with(prefix) {
                keys.push(key.clone());
            }
        }
        Ok(keys)
    }
}
