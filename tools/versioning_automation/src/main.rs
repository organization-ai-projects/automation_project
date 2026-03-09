//! tools/versioning_automation/src/main.rs
mod app;
mod cli_action;
mod issues;

#[cfg(test)]
mod tests;

fn main() {
    std::process::exit(app::run(std::env::args().collect()));
}
