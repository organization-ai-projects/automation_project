//! projects/products/stable/platform_ide/backend/src/changes/mod.rs
pub mod change_set;
pub mod patch_entry;

pub use change_set::ChangeSet;
pub use patch_entry::PatchEntry;

#[cfg(test)]
mod tests;
