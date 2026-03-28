use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::lazy_regex::SEMVER_REGEX;

use crate::automation::commands::ReleasePrepareOptions;
use crate::git_cli;

use super::execute::{
    command_available, ensure_git_repo, repo_root, run_command_status, run_git, run_git_output,
    run_git_output_preserve,
};

pub(crate) fn run_release_prepare(opts: ReleasePrepareOptions) -> Result<(), String> {
    ensure_git_repo()?;
    require_clean_tree()?;
    validate_semver(&opts.version)?;

    let current_branch = run_git_output(&["branch", "--show-current"])?;
    if current_branch.trim() != "main" {
        println!(
            "Warning: current branch is '{}', not 'main'.",
            current_branch.trim()
        );
    }

    run_command_status("cargo", &["test", "--workspace"], false)?;

    if command_available("cargo-audit") {
        run_command_status("cargo", &["audit"], false)?;
    }

    let root = repo_root()?;
    let root_cargo = root.join("Cargo.toml");
    if root_cargo.is_file() {
        update_version_in_cargo_file(&root_cargo, &opts.version)?;
    }

    let mut project_cargos = Vec::new();
    collect_files_named(&root.join("projects"), "Cargo.toml", &mut project_cargos)?;
    for cargo_toml in project_cargos {
        update_version_in_cargo_file(&cargo_toml, &opts.version)?;
    }

    let changelog_path = root.join("CHANGELOG.md");
    if opts.auto_changelog {
        update_changelog(&changelog_path, &opts.version)?;
    } else {
        println!("Skipping automatic changelog generation.");
    }

    run_git(&["add", "-u"])?;
    let commit_message = format!(
        "chore: prepare release v{}\n\nRelease preparation for version {}.\n",
        opts.version, opts.version
    );
    run_git(&["commit", "-m", &commit_message])?;
    let tag_name = format!("v{}", opts.version);
    run_git(&[
        "tag",
        "-a",
        &tag_name,
        "-m",
        &format!("Release {}", tag_name),
    ])?;
    println!("Release {} prepared.", tag_name);
    Ok(())
}

fn require_clean_tree() -> Result<(), String> {
    let unstaged_clean = git_cli::status_success(&["diff", "--quiet"]);
    let staged_clean = git_cli::status_success(&["diff", "--cached", "--quiet"]);
    if unstaged_clean && staged_clean {
        Ok(())
    } else {
        Err("Working tree is dirty. Commit/stash your changes first.".to_string())
    }
}

fn validate_semver(version: &str) -> Result<(), String> {
    let re = match SEMVER_REGEX.as_ref() {
        Ok(re) => re,
        Err(e) => return Err(format!("Failed to compile semver regex: {e}")),
    };
    if re.is_match(version) {
        Ok(())
    } else {
        Err(format!(
            "Invalid version format: {version}. Expected semver format."
        ))
    }
}

fn update_version_in_cargo_file(path: &Path, version: &str) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
    let mut changed = false;
    let updated = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("version = \"") {
                changed = true;
                format!("version = \"{version}\"")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    if changed {
        fs::write(path, format!("{updated}\n"))
            .map_err(|e| format!("Failed to write '{}': {e}", path.display()))?;
    }
    Ok(())
}

fn collect_files_named(root: &Path, file_name: &str, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", root.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            collect_files_named(&path, file_name, out)?;
            continue;
        }
        if file_type.is_file() && path.file_name().and_then(|v| v.to_str()) == Some(file_name) {
            out.push(path);
        }
    }
    Ok(())
}

fn update_changelog(path: &Path, version: &str) -> Result<(), String> {
    let today = run_command_capture("date", &["+%Y-%m-%d"])?;
    let last_tag = run_git_output(&["describe", "--tags", "--abbrev=0"]).unwrap_or_default();
    let commits = if last_tag.trim().is_empty() {
        run_git_output_preserve(&["log", "--oneline", "--no-merges"])?
    } else {
        run_git_output_preserve(&[
            "log",
            &format!("{}..HEAD", last_tag.trim()),
            "--oneline",
            "--no-merges",
        ])?
    };
    let mut lines = vec![
        "# Changelog".to_string(),
        "".to_string(),
        format!("## [v{version}] - {}", today.trim()),
        "".to_string(),
        "### Changes".to_string(),
        "".to_string(),
    ];
    lines.extend(
        commits
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| format!("- {line}")),
    );
    lines.push("".to_string());

    if path.is_file() {
        let existing = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
        let mut existing_lines = existing.lines();
        let _ = existing_lines.next();
        lines.extend(existing_lines.map(ToString::to_string));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))
        .map_err(|e| format!("Failed to write '{}': {e}", path.display()))?;
    Ok(())
}

fn run_command_capture(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run {program} {}: {e}", args.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "{program} {} failed with exit {:?}",
            args.join(" "),
            output.status.code()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
