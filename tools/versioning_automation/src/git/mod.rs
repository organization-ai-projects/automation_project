//! tools/versioning_automation/src/git/mod.rs
mod commands;
mod execute;
mod parse;
mod render;

#[cfg(test)]
mod tests;

pub(crate) use execute::run;
