// projects/products/unstable/protocol_builder/tooling/src/validate/mod.rs
pub mod emitted_validator;
pub mod golden_transcript_validator;

pub use emitted_validator::{EmittedManifest, EmittedValidator};
pub use golden_transcript_validator::{GoldenTranscript, GoldenTranscriptValidator};
