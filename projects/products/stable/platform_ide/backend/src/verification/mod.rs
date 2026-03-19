//! projects/products/stable/platform_ide/backend/src/verification/mod.rs
pub mod finding_severity;
pub mod raw_finding;
pub mod result_view;
pub mod verification_finding;

pub use finding_severity::FindingSeverity;
pub use raw_finding::RawFinding;
pub use result_view::VerificationResultView;
pub use verification_finding::VerificationFinding;

#[cfg(test)]
mod tests;
