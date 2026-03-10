//! projects/products/stable/platform_ide/backend/src/diff/mod.rs
pub mod diff_line;
pub mod local_diff;

pub use diff_line::DiffLine;
pub use local_diff::LocalDiff;

#[cfg(test)]
mod tests;
