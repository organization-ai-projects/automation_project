// projects/libraries/versioning/src/lib.rs
pub mod release_id;
pub mod revision_log;
pub mod release_tracker;
pub mod document_builder;
mod tests;

pub use release_id::{ReleaseId, ReleaseIdError};
pub use revision_log::{ModificationCategory, ModificationEntry, RevisionEntry, RevisionLog};
pub use release_tracker::ReleaseTracker;
pub use document_builder::{DocumentBuilder, OutputFormat};
