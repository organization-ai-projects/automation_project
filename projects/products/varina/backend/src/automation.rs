//! projects/products/varina/backend/src/automation.rs
use std::path::{Path, PathBuf};
use std::{fs, process};

use crate::autopilot::{
    AutopilotError, AutopilotMode, AutopilotPlan, AutopilotPolicy, AutopilotReport,
};
use crate::cargo::{cargo_fmt_check, cargo_test};
use crate::git_github::commands::status::git_status_porcelain_z;
use crate::git_github::commands::{
    current_branch, ensure_git_repo, git_add_paths, git_commit, git_commit_dry_run,
    git_push_current_branch, git_reset,
};
use crate::{PreChecks, classify_changes, has_merge_conflicts};

type Result<T> = std::result::Result<T, AutopilotError>;

/// ==============================
/// SECTION: Public entry points
/// ==============================
/// Backward-compatible entry point (uses current working directory).
/// Prefer `run_git_autopilot_in_repo` for production.
pub fn run_git_autopilot(mode: AutopilotMode, policy: &AutopilotPolicy) -> Result<AutopilotReport> {
    run_git_autopilot_in_repo(Path::new("."), mode, policy)
}

/// Production entry point: execute autopilot inside a specific repo directory.
/// This removes any dependency on process CWD (important when engine spawns backends).
pub fn run_git_autopilot_in_repo(
    repo_path: &Path,
    mode: AutopilotMode,
    policy: &AutopilotPolicy,
) -> Result<AutopilotReport> {
    let repo_path = normalize_repo_path(repo_path)?;
    let mut logs = Vec::<String>::new();

    logs.push(format!("[ctx] repo_path={}", repo_path.display()));
    ensure_git_repo(&repo_path, &mut logs)?;
    let (branch, detached_head) = current_branch(&repo_path, &mut logs)?;
    logs.push(format!(
        "[git] branch={branch} detached_head={detached_head}"
    ));

    println!(
        "[debug] run_git_autopilot_in_repo: Starting with repo_path: {:?}, mode: {:?}, policy: {:?}",
        repo_path, mode, policy
    );
    println!("[debug] Logs initialized: {:?}", logs);

    if detached_head {
        return Err(format!(
            "Refusal: Detached HEAD (branch='{}'). Checkout a branch before using autopilot.",
            branch
        )
        .into());
    }

    if policy.protected_branches.iter().any(|b| b == &branch) {
        return Err(format!(
            "Refusal: The branch '{}' is protected ({:?}).",
            branch, policy.protected_branches
        )
        .into());
    }

    let changes = git_status_porcelain_z(&repo_path, &mut logs)?;
    let classified = classify_changes(&changes, policy);

    let mut notes = Vec::new();

    // Nothing to do: return a clean report with empty plan.
    if changes.is_empty() {
        notes.push("No changes detected.".into());

        let plan = AutopilotPlan {
            branch: branch.clone(),
            will_stage: vec![],
            will_commit: false,
            commit_message: build_commit_subject(&branch),
            will_push: false,
            notes,
        };

        logs.push("[plan] empty (no changes)".into());

        return Ok(AutopilotReport {
            mode,
            branch,
            detached_head,
            changes,
            classified,
            plan,
            applied: false,
            logs,
        });
    }

    println!("[debug] Validating relevant and blocked changes");
    println!("[debug] Detected changes: {:?}", changes);
    println!("[debug] Classified changes: {:?}", classified);

    // Conflicts: check XY bytes directly (robust).
    if has_merge_conflicts(&changes) {
        println!("[debug] Merge conflicts detected");
        return Err(
            "Merge conflicts detected. Resolve conflicts before continuing."
                .to_string()
                .into(),
        );
    }

    // Blocked -> hard stop.
    if !classified.blocked.is_empty() {
        println!("[debug] Blocked changes detected: {:?}", classified.blocked);
        return Err(format!(
            "Refusal: Blocked changes detected: {:?}",
            classified.blocked
        )
        .to_string()
        .into());
    }

    // Unrelated -> hard stop if policy says so.
    if policy.fail_on_unrelated_changes && !classified.unrelated.is_empty() {
        println!(
            "[debug] Unrelated changes detected: {:?}",
            classified.unrelated
        );
        return Err(format!(
            "Refusal: Unrelated changes detected: {:?}",
            classified.unrelated
        )
        .to_string()
        .into());
    }

    // Stage only relevant.
    let will_stage = classified.relevant.to_vec();
    let will_commit = !will_stage.is_empty();
    let will_push = policy.allow_push && will_commit;

    // Build commit message: subject + body (safe sizes).
    let commit = build_commit_message(&branch, &classified.relevant);
    let commit_subject = commit.0.to_string();
    let commit_body = commit.1.to_string();
    let commit_message_for_report = if commit_body.is_empty() {
        commit_subject.clone()
    } else {
        format!("{commit_subject}\n\n{commit_body}")
    };

    let mut plan = AutopilotPlan {
        branch: branch.clone(),
        will_stage: will_stage.clone(),
        will_commit,
        commit_message: commit_message_for_report,
        will_push,
        notes,
    };

    // Plan diagnostics (useful for UI)
    logs.push(format!(
        "[classify] relevant={} blocked={} unrelated={}",
        classified.relevant.len(),
        classified.blocked.len(),
        classified.unrelated.len()
    ));
    logs.push(format!(
        "[plan] stage_count={} will_commit={} will_push={}",
        plan.will_stage.len(),
        plan.will_commit,
        plan.will_push
    ));

    if !policy.relevant_files.is_empty() {
        logs.push(format!(
            "[policy] relevant_files={:?}",
            policy.relevant_files
        ));
    }
    if !policy.blocked_prefixes.is_empty() {
        logs.push(format!(
            "[policy] blocked_prefixes={:?}",
            policy.blocked_prefixes
        ));
    }

    // Check and remove .git/index.lock file before any Git operation
    let lock_file = repo_path.join(".git/index.lock");
    logs.push(format!(
        "[debug] Checking for existence of .git/index.lock: {:?}",
        lock_file
    ));
    if lock_file.exists() {
        logs.push("[debug] .git/index.lock detected, attempting removal...".to_string());
        fs::remove_file(&lock_file).map_err(|e| {
            let error_message = format!("[error] Failed to remove .git/index.lock: {}", e);
            logs.push(error_message.clone());
            AutopilotError::from(error_message)
        })?;
        logs.push("[debug] Successfully removed lock file: .git/index.lock".to_string());
    } else {
        logs.push("[debug] No .git/index.lock file detected.".to_string());
    }

    // Check for active Git processes
    let git_processes = process::Command::new("pgrep")
        .arg("git")
        .output()
        .expect("Unable to check Git processes");

    if !git_processes.stdout.is_empty() {
        println!("[warning] Active Git processes detected. This might cause conflicts.");
    }

    // Pre-checks before apply
    if mode == AutopilotMode::ApplySafe && plan.will_commit {
        match policy.pre_checks {
            PreChecks::None => logs.push("[prechecks] none".into()),
            PreChecks::FmtCheck => {
                logs.push("[prechecks] cargo fmt --check".into());
                cargo_fmt_check(&repo_path, &mut logs)?;
            }
            PreChecks::FmtCheckAndTests => {
                logs.push("[prechecks] cargo fmt --check".into());
                cargo_fmt_check(&repo_path, &mut logs)?;
                logs.push("[prechecks] cargo test".into());
                cargo_test(&repo_path, &mut logs)?;
            }
        }
    }

    let mut applied = false;

    logs.push(format!(
        "[debug] automation.rs: mode={:?}, will_commit={}, will_stage_empty={}",
        mode,
        plan.will_commit,
        plan.will_stage.is_empty()
    ));

    if mode == AutopilotMode::ApplySafe {
        logs.push("[debug] automation.rs: Entering ApplySafe block".to_string());
        if plan.will_commit {
            // Stage relevant (batch)
            logs.push(format!("[debug] Staging files: {:?}", plan.will_stage));
            git_add_paths(&repo_path, &plan.will_stage, &mut logs)?;

            logs.push(format!(
                "[debug] automation.rs: paths transmitted to git_add_paths (ApplySafe)={:?}",
                plan.will_stage
            ));

            logs.push(format!(
                "[debug] automation.rs: relevant paths transmitted to git_add_paths={:?}",
                classified.relevant.to_vec()
            ));

            println!(
                "[debug] Adding relevant paths to Git index: {:?}",
                classified.relevant
            );
            git_add_paths(&repo_path, &classified.relevant.to_vec(), &mut logs)?;

            // Commit (subject + body)
            println!("[debug] Attempting commit with changes: {:?}", changes);

            println!(
                "[debug] Commit message: {:?}",
                build_commit_message(&branch, &classified.relevant)
            );
            git_commit(&repo_path, &commit_subject, &commit_body, &mut logs)?;
            applied = true;

            // Push (optional)
            if policy.allow_push {
                git_push_current_branch(&repo_path, &branch, true, &mut logs)?;
            }
        } else {
            logs.push("[apply] no relevant files to commit; no action".into());
        }
    } else {
        logs.push("[dryrun] no changes applied".into());
        logs.push("[debug] automation.rs: Entering DryRun block".to_string());

        // Temporarily stage files for dry-run
        if mode == AutopilotMode::DryRun && !plan.will_stage.is_empty() {
            logs.push(format!(
                "[debug] Temporarily staging files for dry-run: {:?}",
                plan.will_stage
            ));
            git_add_paths(&repo_path, &plan.will_stage, &mut logs)?;

            // Perform dry-run commit
            git_commit_dry_run(&repo_path, &plan.commit_message, "", &mut logs)?;

            // Restore initial index state
            logs.push("[debug] Restoring index state after dry-run".into());
            git_reset(&repo_path, &plan.will_stage, &mut logs)?;
        }
    }

    // Add notes after apply/dryrun
    if !classified.unrelated.is_empty() && !policy.fail_on_unrelated_changes {
        plan.notes.push(format!(
            "Warning: {} unrelated changes ignored (fail_on_unrelated_changes=false).",
            classified.unrelated.len()
        ));
    }

    println!("[debug] Final logs: {:?}", logs);

    Ok(AutopilotReport {
        mode,
        branch,
        detached_head,
        changes,
        classified,
        plan,
        applied,
        logs,
    })
}

/// ==============================
/// SECTION: Path Normalization
/// ==============================
/// Normalizes the repository path to ensure it is absolute and valid.
pub fn normalize_repo_path(repo_path: &Path) -> Result<PathBuf> {
    let path = Path::new(repo_path);
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir()
            .map_err(|e| AutopilotError::from(format!("Failed to get current directory: {}", e)))?
            .join(path)
            .canonicalize()
            .map_err(|e| AutopilotError::from(format!("Failed to normalize path: {}", e)))
    }
}

/// Builds the commit subject based on the current branch name.
pub fn build_commit_subject(branch: &str) -> String {
    format!("Commit on branch: {}", branch)
}

/// Builds the commit message based on the branch name and relevant changes.
pub fn build_commit_message(branch: &str, relevant_changes: &[String]) -> (String, String) {
    let changes_summary = relevant_changes.join(", ");
    let subject = format!("Commit on branch: {}", branch);
    let body = format!("Changes:\n{}", changes_summary);

    (subject, body)
}
