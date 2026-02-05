// projects/products/varina/backend/src/cargo/cargo_helpers.rs
use std::path::Path;

use command_runner::run_cmd_allow_failure;

use crate::autopilot::AutopilotError;

type Result<T> = std::result::Result<T, AutopilotError>;

pub fn cargo_fmt_check(repo_path: &Path, logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_allow_failure(repo_path, "cargo", &["fmt", "--", "--check"], logs)?;
    if !out.status.success() {
        return Err("Pre-check failed: code not formatted. Run `cargo fmt`."
            .to_string()
            .into());
    }
    Ok(())
}

pub fn cargo_test(repo_path: &Path, logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_allow_failure(repo_path, "cargo", &["test"], logs)?;
    if !out.status.success() {
        return Err("Pre-check failed: some tests failed (`cargo test`)."
            .to_string()
            .into());
    }
    Ok(())
}
