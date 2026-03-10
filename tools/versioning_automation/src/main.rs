//! tools/versioning_automation/src/main.rs
mod app;
mod cli_action;
mod issues;
mod pr;
mod repo_name;

#[cfg(test)]
mod tests;

fn main() {
    std::process::exit(app::run(std::env::args().collect()));
}
