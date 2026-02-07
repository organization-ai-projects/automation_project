//! projects/products/varina/backend/src/main.rs
mod app;
mod automation;
mod autopilot;
mod cargo;
mod classified_changes;
mod classified_changes_ref;
mod git_github;
mod handler_error;
mod policy_evaluation;
mod pre_checks;
mod repo_path_validator;
mod router;
mod validation_error;

#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    app::run_backend()
}
