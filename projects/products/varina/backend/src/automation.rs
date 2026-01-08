use std::path::Path;

use crate::autopilot::{
    AutopilotError, AutopilotMode, AutopilotPlan, AutopilotPolicy, AutopilotReport,
};
use crate::cargo::{cargo_fmt_check, cargo_test};
use git_lib::commands::{
    build_commit_message, build_commit_subject, current_branch, ensure_git_repo, git_add_paths,
    git_commit, git_commit_dry_run, git_push_current_branch, git_status_porcelain_z,
    normalize_repo_path,
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
            "Refus: HEAD détaché (branch='{}'). Checkout une branche avant d'utiliser l'autopilot.",
            branch
        )
        .into());
    }

    if policy.protected_branches.iter().any(|b| b == &branch) {
        return Err(format!(
            "Refus: la branche '{}' est protégée ({:?}).",
            branch, policy.protected_branches
        )
        .into());
    }

    let changes = git_status_porcelain_z(&repo_path, &mut logs)?;
    let classified = classify_changes(&changes, policy);

    let mut notes = Vec::new();

    // Nothing to do: return a clean report with empty plan.
    if changes.is_empty() {
        notes.push("Aucun changement détecté.".into());

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

    println!("[debug] Validation des changements pertinents et bloqués");
    println!("[debug] Changements détectés: {:?}", changes);
    println!("[debug] Changements classifiés: {:?}", classified);

    // Conflicts: check XY bytes directly (robust).
    if has_merge_conflicts(&changes) {
        println!("[debug] Conflits de fusion détectés");
        return Err(
            "Conflits de fusion détectés. Résolvez les conflits avant de continuer."
                .to_string()
                .into(),
        );
    }

    // Blocked -> hard stop.
    if !classified.blocked.is_empty() {
        println!(
            "[debug] Changements bloqués détectés: {:?}",
            classified.blocked
        );
        return Err(format!(
            "Refus: des changements bloqués ont été détectés: {:?}",
            classified.blocked
        )
        .to_string()
        .into());
    }

    // Unrelated -> hard stop if policy says so.
    if policy.fail_on_unrelated_changes && !classified.unrelated.is_empty() {
        println!(
            "[debug] Changements non liés détectés: {:?}",
            classified.unrelated
        );
        return Err(format!(
            "Refus: des changements non liés ont été détectés: {:?}",
            classified.unrelated
        )
        .to_string()
        .into());
    }

    // Stage only relevant.
    let will_stage = classified
        .relevant
        .iter()
        .map(|c| c.path.clone())
        .collect::<Vec<_>>();
    let will_commit = !will_stage.is_empty();
    let will_push = policy.allow_push && will_commit;

    // Build commit message: subject + body (safe sizes).
    let (commit_subject, commit_body) = build_commit_message(&branch, &classified.relevant);
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

    // Vérification et suppression du fichier .git/index.lock avant toute opération Git
    let lock_file = repo_path.join(".git/index.lock");
    logs.push(format!(
        "[debug] Vérification de l'existence de .git/index.lock: {:?}",
        lock_file
    ));
    if lock_file.exists() {
        logs.push("[debug] .git/index.lock détecté, tentative de suppression...".to_string());
        std::fs::remove_file(&lock_file).map_err(|e| {
            let error_message = format!("[error] Échec de suppression de .git/index.lock: {}", e);
            logs.push(error_message.clone());
            AutopilotError::from(error_message)
        })?;
        logs.push("[debug] Fichier verrouillé supprimé avec succès: .git/index.lock".to_string());
    } else {
        logs.push("[debug] Aucun fichier .git/index.lock détecté.".to_string());
    }

    // Vérification des processus Git actifs
    let git_processes = std::process::Command::new("pgrep")
        .arg("git")
        .output()
        .expect("Impossible de vérifier les processus Git");

    if !git_processes.stdout.is_empty() {
        println!(
            "[warning] Des processus Git actifs ont été détectés. Cela pourrait causer des conflits."
        );
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

    if mode == AutopilotMode::ApplySafe {
        if plan.will_commit {
            // Stage relevant (batch)
            logs.push(format!("[debug] Staging files: {:?}", plan.will_stage));
            git_add_paths(&repo_path, &plan.will_stage, &mut logs)?;

            println!(
                "[debug] Ajout des chemins pertinents à l'index Git: {:?}",
                classified.relevant
            );
            git_add_paths(
                &repo_path,
                &classified
                    .relevant
                    .iter()
                    .map(|c| c.path.clone())
                    .collect::<Vec<_>>(),
                &mut logs,
            )?;

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

        // Effectuer un commit en mode "dry-run" pour valider les changements
        git_commit_dry_run(&repo_path, &plan.commit_message, "", &mut logs)?;
    }

    // Add notes after apply/dryrun
    if !classified.unrelated.is_empty() && !policy.fail_on_unrelated_changes {
        plan.notes.push(format!(
            "Attention: {} changements non liés ignorés (fail_on_unrelated_changes=false).",
            classified.unrelated.len()
        ));
    }

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
/// SECTION: Optional tiny util
/// ==============================
#[allow(dead_code)]
fn exists(path: &str) -> bool {
    Path::new(path).exists()
}
