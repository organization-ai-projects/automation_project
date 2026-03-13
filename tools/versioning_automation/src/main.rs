//! tools/versioning_automation/src/main.rs
mod app;
mod automation;
mod cli_action;
mod gh_cli;
mod git;
mod git_cli;
mod issues;
mod pr;
mod repo_name;

#[cfg(test)]
mod tests;

fn main() {
    std::process::exit(app::run(std::env::args().collect()));
}
