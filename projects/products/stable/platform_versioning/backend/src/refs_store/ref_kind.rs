// projects/products/stable/platform_versioning/backend/src/refs_store/ref_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of a ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefKind {
    /// A mutable branch pointer (fast-forward-only by default).
    Branch,
    /// An immutable tag pointer.
    Tag,
}
