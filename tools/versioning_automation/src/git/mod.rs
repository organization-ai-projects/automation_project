//! tools/versioning_automation/src/git/mod.rs
mod commands;
mod render;

#[cfg(test)]
mod tests;

pub(crate) use render::print_usage;
