use std::path::Path;

use crate::autopilot::AutopilotError;
use command_runner::run_cmd_allow_failure;

type Result<T> = std::result::Result<T, AutopilotError>;

pub fn cargo_fmt_check(repo_path: &Path, logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_allow_failure(repo_path, "cargo", &["fmt", "--", "--check"], logs)?;
    if !out.status.success() {
        return Err("Pré-check échoué: code non formaté. Exécute `cargo fmt`."
            .to_string()
            .into());
    }
    Ok(())
}

pub fn cargo_test(repo_path: &Path, logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_allow_failure(repo_path, "cargo", &["test"], logs)?;
    if !out.status.success() {
        return Err(
            "Pré-check échoué: certains tests ont échoué (`cargo test`)."
                .to_string()
                .into(),
        );
    }
    Ok(())
}
