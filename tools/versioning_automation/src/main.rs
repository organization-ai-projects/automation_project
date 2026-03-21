//! tools/versioning_automation/src/main.rs
mod app;
mod automation;
mod category_resolver;
mod cli_action;
mod compare_snapshot;
mod gh_cli;
mod git;
mod git_cli;
mod issues;
mod open_pr_issue_refs;
mod pr;
mod pr_remote_snapshot;
mod pr_run_snapshot;
mod repo_name;

#[cfg(test)]
mod tests;

use std::{env, process};

fn main() {
    process::exit(app::run(env::args().collect()));
}
