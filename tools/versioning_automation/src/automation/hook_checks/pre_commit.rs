//! tools/versioning_automation/src/automation/hook_checks/pre_commit.rs
use std::env;

use crate::{
    automation::{current_branch_name, execute},
    repo_name,
};

pub(crate) fn validate_pre_commit_branch_guard() -> Result<(), String> {
    let current_branch = current_branch_name()?;
    if env::var("ALLOW_PROTECTED_BRANCH_COMMIT").unwrap_or_default() != "1"
        && (current_branch.trim() == "dev" || current_branch.trim() == "main")
    {
        return Err(format!(
            "❌ Direct commits on protected branch '{}' are blocked.\n   Create a feature/fix/docs branch, then open a PR.\n   Temporary bypass (exception only): ALLOW_PROTECTED_BRANCH_COMMIT=1 git commit ...",
            current_branch.trim()
        ));
    }
    Ok(())
}

pub(crate) fn validate_pre_commit_assignment_policy(push_commits: &str) -> Result<(), String> {
    if push_commits.trim().is_empty() {
        return Ok(());
    }
    execute::validate_part_of_only_policy(
        push_commits,
        repo_name::resolve_repo_name(None).ok().as_deref(),
    )
    .map_err(|err| format!("{err}\n\n❌ Assignment policy check failed (early pre-commit guard)."))
}

pub(crate) fn print_affected_crates(crates: &[String]) {
    if crates.is_empty() {
        println!("🎯 No Rust crates detected, checking all files");
        return;
    }
    println!("🎯 Affected crates:");
    for crate_name in crates {
        println!("   - {crate_name}");
    }
    println!();
}

pub(crate) fn run_pre_commit_markdown(markdown_files: &[String]) -> Result<(), String> {
    if markdown_files.is_empty() {
        println!("📝 Skipping markdown lint (no staged markdown files)");
        return Ok(());
    }
    println!("📝 Auto-fixing markdown files...");
    execute::run_markdownlint_files(markdown_files)
        .map_err(|err| format!("{err}\n\n❌ Markdown lint failed on staged markdown files."))
}

pub(crate) fn run_pre_commit_shell_syntax(staged_files: &[String]) -> Result<(), String> {
    println!("🔎 Checking shell syntax...");
    for file in staged_files {
        if execute::is_shell_file_path(file)
            && let Err(err) = execute::run_command_status("bash", &["-n", file], false)
        {
            return Err(format!(
                "   ❌ Shell syntax error: {file}\n{err}\n\n❌ Shell syntax checks failed!"
            ));
        }
    }
    Ok(())
}

pub(crate) fn run_pre_commit_rustfmt(staged_files: &[String]) -> Result<(), String> {
    if staged_files.iter().any(|file| file.ends_with(".rs")) {
        println!("✨ Formatting code...");
        return execute::run_command_status("cargo", &["fmt", "--all"], false);
    }
    println!("✨ Skipping Rust formatting (no staged Rust files)");
    Ok(())
}

pub(crate) fn restage_staged_files() -> Result<(), String> {
    let restage_files = execute::list_staged_changed_files();
    if restage_files.is_empty() {
        return Ok(());
    }
    let mut args = vec!["add".to_string(), "--".to_string()];
    args.extend(restage_files);
    execute::run_command_status_owned("git", &args, false)
}
