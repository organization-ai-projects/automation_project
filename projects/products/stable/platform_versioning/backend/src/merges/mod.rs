// projects/products/stable/platform_versioning/backend/src/merge/mod.rs
pub mod conflict;
pub mod conflict_kind;
pub mod merge;
pub mod merge_result;

pub use conflict::Conflict;
pub use conflict_kind::ConflictKind;
pub use merge::Merge;
pub use merge_result::MergeResult;
