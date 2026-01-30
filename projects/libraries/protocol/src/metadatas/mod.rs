// projects/libraries/protocol/src/metadatas/mod.rs
pub mod metadata;
pub mod metadata_ai_hints;
pub mod metadata_domain;
pub mod metadata_entry_point;
pub mod metadata_entry_points;
pub mod project_metadata;

pub use metadata::Metadata;
pub use metadata_ai_hints::MetadataAIHints;
pub use metadata_domain::MetadataDomain;
pub use metadata_entry_point::MetadataEntrypoint;
pub use metadata_entry_points::MetadataEntrypoints;
pub use project_metadata::ProjectMetadata;
