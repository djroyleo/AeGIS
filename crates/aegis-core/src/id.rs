//! Stable identifiers for entities that outlive a single frame.

use std::sync::atomic::{AtomicU64, Ordering};

/// A process-unique identifier for a layer in a project.
///
/// GUI code should reference layers by `LayerId` rather than by index or
/// name, so that reordering or renaming layers never invalidates a
/// reference held elsewhere (selection state, open attribute tables, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LayerId(u64);

impl LayerId {
    /// Allocate the next process-unique id.
    #[must_use]
    pub fn next() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
