// src/tokens/async_resource.rs
//
// Unified async resource pattern for the token DSL.
//
// On WASM:  futures are spawned via `wasm_bindgen_futures::spawn_local`.
// On SSR:    the caller must drive the future inside a Leptos `Suspense`
//            boundary; `spawn_local` is a no-op and logs a warning.
//
// Invariant: `rayon` is NEVER used in this file (WASM-safe).

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceState<T, E = String> {
    Idle,
    Loading,
    Success(T),
    Error(E),
}

/// Lightweight async resource wrapper around a `RwSignal`.
///
/// Example (WASM):
/// ```ignore
/// let res = AsyncResource::<String, String>::new();
/// res.load(async { reqwest::get("/api").await?.text().await });
/// ```
pub struct AsyncResource<T: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static = String> {
    pub signal: RwSignal<ResourceState<T, E>>,
}

impl<T: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> AsyncResource<T, E> {
    pub fn new() -> Self {
        Self {
            signal: RwSignal::new(ResourceState::Idle),
        }
    }

    /// Start loading. On WASM the future is spawned; on SSR the caller must
    /// drive the future inside a Leptos `Suspense` boundary.
    pub fn load<F>(&self, future: F)
    where
        F: std::future::Future<Output = Result<T, E>> + 'static,
    {
        self.signal.set(ResourceState::Loading);

        #[cfg(target_arch = "wasm32")]
        {
            let signal = self.signal;
            wasm_bindgen_futures::spawn_local(async move {
                match future.await {
                    Ok(val) => signal.set(ResourceState::Success(val)),
                    Err(err) => signal.set(ResourceState::Error(err)),
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // SSR: futures must be driven by the caller inside a Suspense
            // boundary. We cannot block the synchronous render path.
            drop(future);
            leptos::logging::warn!(
                "[AsyncResource] SSR load() called without Suspense boundary. \
                 Wrap in leptos::prelude::Suspense or drive the future manually."
            );
        }
    }

    pub fn get(&self) -> ResourceState<T, E> {
        self.signal.get()
    }

    pub fn set(&self, state: ResourceState<T, E>) {
        self.signal.set(state);
    }
}

impl<T: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> Default for AsyncResource<T, E> {
    fn default() -> Self { Self::new() }
}
