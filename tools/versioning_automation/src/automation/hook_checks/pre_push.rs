//! tools/versioning_automation/src/automation/hook_checks/pre_push.rs
use crate::automation::execute;

pub(crate) fn validate_pre_push_commit_policies(
    commits: &str,
    repo: Option<&str>,
) -> Result<(), String> {
    if commits.trim().is_empty() {
        return Ok(());
    }
    execute::validate_no_root_parent_refs(commits, repo)?;
    execute::validate_part_of_only_policy(commits, repo)
}

pub(crate) fn run_pre_push_docs_scripts_mode(
    changed_files: &[String],
    markdown_files: &[String],
) -> Result<(), String> {
    if !markdown_files.is_empty() {
        execute::run_markdownlint_files(markdown_files)?;
    }
    execute::run_shell_syntax_checks(changed_files)?;
    println!("Pre-push checks passed (docs/scripts-only mode)");
    Ok(())
}

pub(crate) fn run_pre_push_rust_mode(
    changed_files: &[String],
    markdown_files: &[String],
) -> Result<(), String> {
    if !markdown_files.is_empty() {
        execute::run_markdownlint_files(markdown_files)?;
    }
    execute::run_command_status("cargo", &["fmt", "--all", "--", "--check"], false)?;

    let mut crates = collect_changed_crates(changed_files)?;
    let has_lockfile = execute::repo_root()?.join("Cargo.lock").is_file();
    let (clippy_args, test_args) = build_quality_check_args(&mut crates, has_lockfile);

    let mut clippy_run = vec!["clippy".to_string()];
    clippy_run.extend(clippy_args);
    clippy_run.push("--".to_string());
    clippy_run.push("-D".to_string());
    clippy_run.push("warnings".to_string());
    execute::run_command_status_owned("cargo", &clippy_run, false)?;

    let mut test_run = vec!["test".to_string()];
    test_run.extend(test_args);
    execute::run_command_status_owned("cargo", &test_run, false)?;

    println!("All pre-push checks passed");
    Ok(())
}

pub(crate) fn collect_changed_crates(changed_files: &[String]) -> Result<Vec<String>, String> {
    let changed_file_text = changed_files.join("\n");
    let mut crates = execute::collect_crates_from_changed_files(&changed_file_text)?;
    crates.sort();
    crates.dedup();
    Ok(crates)
}

pub(crate) fn build_quality_check_args(
    crates: &mut [String],
    has_lockfile: bool,
) -> (Vec<String>, Vec<String>) {
    let mut clippy_args = Vec::<String>::new();
    let mut test_args = Vec::<String>::new();
    if has_lockfile {
        clippy_args.push("--locked".to_string());
        test_args.push("--locked".to_string());
    }
    if crates.is_empty() {
        clippy_args.extend(["--workspace", "--all-targets", "--all-features"].map(String::from));
        test_args.extend(["--workspace", "--all-targets", "--all-features"].map(String::from));
    } else {
        clippy_args.extend(["--all-targets", "--all-features"].map(String::from));
        test_args.extend(["--all-targets", "--all-features"].map(String::from));
        for crate_name in crates.iter() {
            clippy_args.push("-p".to_string());
            clippy_args.push(crate_name.clone());
            test_args.push("-p".to_string());
            test_args.push(crate_name.clone());
        }
    }
    (clippy_args, test_args)
}
