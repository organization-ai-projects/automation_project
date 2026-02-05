// projects/libraries/versioning/src/lib.rs
pub mod document_builder;
pub mod release_id;
pub mod release_tracker;
pub mod revision_log;
mod tests;

pub use document_builder::{DocumentBuilder, OutputFormat};
pub use release_id::{ReleaseId, ReleaseIdError};
pub use release_tracker::ReleaseTracker;
pub use revision_log::{ModificationCategory, ModificationEntry, RevisionEntry, RevisionLog};
