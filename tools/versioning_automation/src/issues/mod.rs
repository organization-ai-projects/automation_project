//! tools/versioning_automation/src/issues/mod.rs
mod commands;
mod dispatch;
mod execute;
mod parse;
mod render;
mod required_fields;

#[cfg(test)]
mod tests;

pub fn run(args: &[String]) -> i32 {
    dispatch::run(args)
}
