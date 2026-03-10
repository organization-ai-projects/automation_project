//! projects/products/stable/platform_ide/backend/src/slices/mod.rs
mod allowed_path;
mod slice_manifest;

#[cfg(test)]
mod tests;

pub(crate) use allowed_path::AllowedPath;
pub use slice_manifest::SliceManifest;
