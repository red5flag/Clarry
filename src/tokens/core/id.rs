// src/tokens/core/id.rs
//
// Auto-generated token ID counter.
// Shared module-level thread_local so reset_id_counter affects next_id.
// This is critical for SSR/hydration ID matching.

use std::cell::Cell;

use super::types::Str;

thread_local! {
    static ID_COUNTER: Cell<usize> = const { Cell::new(0) };
}

pub fn next_id() -> Str {
    ID_COUNTER.with(|n| {
        let val = n.get();
        n.set(val + 1);
        format!("t{val}").into()
    })
}

/// Call this at the very start of every page component function **before** the
/// token tree is constructed.  That guarantees SSR and the WASM hydration pass
/// assign identical auto-generated IDs so Leptos can match the DOM.
pub fn reset_id_counter() {
    ID_COUNTER.with(|n| n.set(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_id_starts_at_zero() {
        reset_id_counter();
        assert_eq!(&*next_id(), "t0");
    }

    #[test]
    fn next_id_increments() {
        reset_id_counter();
        let _ = next_id();
        assert_eq!(&*next_id(), "t1");
        assert_eq!(&*next_id(), "t2");
    }

    #[test]
    fn reset_restarts_counter() {
        reset_id_counter();
        let _ = next_id();
        let _ = next_id();
        reset_id_counter();
        assert_eq!(&*next_id(), "t0");
    }

    #[test]
    fn id_format_matches_prefix() {
        reset_id_counter();
        let id = next_id();
        assert!(id.starts_with('t'));
    }
}
