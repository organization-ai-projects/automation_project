use crate::automation::commands::CheckDependenciesOptions;

use super::execute::{command_available, ensure_git_repo, run_command_status};

pub(crate) fn run_check_dependencies(opts: CheckDependenciesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    if opts.check_outdated {
        if command_available("cargo-outdated") {
            run_command_status(
                "cargo",
                &["outdated", "--workspace", "--root-deps-only"],
                true,
            )?;
        } else {
            println!("cargo-outdated not found, skipping outdated dependencies check.");
        }
    }

    run_command_status("cargo", &["check", "--workspace", "--all-targets"], false)?;

    if opts.check_unused {
        if command_available("cargo-udeps") {
            run_command_status("cargo", &["+nightly", "udeps", "--workspace"], true)?;
        } else {
            println!("cargo-udeps not found, skipping unused dependencies check.");
        }
    }
    Ok(())
}
