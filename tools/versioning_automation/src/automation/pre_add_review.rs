//! tools/versioning_automation/src/automation/pre_add_review.rs
use std::collections::BTreeSet;
use std::process::Command;

use crate::automation::execute;

pub(crate) fn run_pre_add_review() -> Result<(), String> {
    execute::ensure_git_repo()?;
    let mut issues = 0u32;

    println!("Running pre-add review...");

    println!("Checking code formatting...");
    if command_status_success("cargo", &["fmt", "--all", "--check"])? {
        println!("OK: code is properly formatted.");
    } else {
        eprintln!("Formatting issues detected. Run: cargo fmt");
        issues += 1;
    }

    println!("Running clippy...");
    if command_status_success(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )? {
        println!("OK: no clippy warnings.");
    } else {
        eprintln!("Clippy warnings or errors detected.");
        issues += 1;
    }

    println!("Running tests...");
    if command_status_success("cargo", &["test", "--workspace"])? {
        println!("OK: all tests passed.");
    } else {
        eprintln!("Some tests failed.");
        issues += 1;
    }

    println!("Checking staged changes for risky patterns...");
    let staged_patch = execute::run_git_output_preserve(&["diff", "--cached", "--unified=0"])?;
    let risky_patterns = ["unwrap(", "expect(", "todo!", "unimplemented!", "panic!"];
    let mut found_patterns = 0u32;
    for pattern in risky_patterns {
        if staged_patch
            .lines()
            .any(|line| line.starts_with('+') && !line.starts_with("+++") && line.contains(pattern))
        {
            eprintln!("Found '{}' in staged changes.", pattern);
            found_patterns += 1;
        }
    }
    if found_patterns > 0 {
        issues += 1;
    }

    println!("Summarizing touched crates...");
    let staged_files = execute::run_git_output_preserve(&[
        "diff",
        "--cached",
        "--name-only",
        "--diff-filter=ACMRU",
    ])?;
    let root = execute::repo_root()?;
    let mut crates = BTreeSet::new();
    for file in staged_files
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        if let Some(path) = execute::find_crate_dir_for_file(&root, file) {
            crates.insert(path);
        }
    }
    if crates.is_empty() {
        println!("No crates touched.");
    } else {
        println!("Touched crates:");
        for path in crates {
            println!("  - {path}");
        }
    }

    if issues == 0 {
        println!("Pre-add review passed.");
        Ok(())
    } else {
        Err(format!(
            "Pre-add review found {issues} issue(s). Please review before staging."
        ))
    }
}

fn command_status_success(program: &str, args: &[&str]) -> Result<bool, String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to run {program} {}: {e}", args.join(" ")))?;
    Ok(status.success())
}
