use std::collections::BTreeMap;

use std::path::Path;
use std::process::{Command, Output};

use crate::PreChecks;
use crate::autopilot::{
    AutopilotError, AutopilotMode, AutopilotPlan, AutopilotPolicy, AutopilotReport,
};
use crate::classified_changes::ClassifiedChanges;
use crate::git_github::GitChange;

type Result<T> = std::result::Result<T, AutopilotError>;

/// Point d'entrée: à appeler depuis ton backend (Varina) en réaction à une commande UI.
pub fn run_git_autopilot(mode: AutopilotMode, policy: &AutopilotPolicy) -> Result<AutopilotReport> {
    let mut logs = Vec::<String>::new();

    ensure_git_repo(&mut logs)?;

    let (branch, detached_head) = current_branch(&mut logs)?;
    logs.push(format!(
        "Branch courante: {branch} (detached_head={detached_head})"
    ));

    if detached_head {
        return Err(format!(
            "Refus: HEAD détaché (branch='{}'). Checkout une branche avant d'utiliser l'autopilot.",
            branch
        )
        .into());
    }

    if policy
        .protected_branches
        .iter()
        .any(|b| b.as_str() == branch.as_str())
    {
        return Err(format!(
            "Refus: la branche '{}' est protégée ({:?}).",
            branch, policy.protected_branches
        )
        .into());
    }

    let changes = git_status_porcelain_z(&mut logs)?;
    let classified = classify_changes(&changes, policy);

    // Si rien à commit, on s'arrête proprement avec un plan vide.
    let mut notes = Vec::new();
    if changes.is_empty() {
        notes.push("Aucun changement détecté.".into());
    }

    if !classified.blocked.is_empty() {
        let blocked_list = classified
            .blocked
            .iter()
            .map(|c| format!("{} {}", c.status_str(), display_change_path(c)))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(format!("Refus: changements bloqués par policy:\n{}", blocked_list).into());
    }

    if policy.fail_on_unrelated_changes && !classified.unrelated.is_empty() {
        let unrelated_list = classified
            .unrelated
            .iter()
            .map(|c| format!("{} {}", c.status_str(), display_change_path(c)))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(format!(
            "Refus: changements non liés détectés (policy fail_on_unrelated_changes=true).\n\
             Déplace/commit ces fichiers séparément, ou ajuste la policy.\n{}",
            unrelated_list
        )
        .into());
    }

    // Ce que l’on stage: uniquement relevant.
    let will_stage = classified
        .relevant
        .iter()
        .map(|c| c.path.clone())
        .collect::<Vec<_>>();

    let will_commit = !will_stage.is_empty();

    let commit_message = build_commit_message(&branch, &classified.relevant);

    let will_push = policy.allow_push && will_commit;

    if changes.iter().any(|c| c.status_str().contains('U')) {
        return Err(
            "Refus: conflits de merge détectés (status U). Résous-les d'abord."
                .to_string()
                .into(),
        );
    }

    let mut plan = AutopilotPlan {
        branch: branch.clone(),
        will_stage,
        will_commit,
        commit_message,
        will_push,
        notes,
    };

    // Log des changements classifiés
    logs.push(format!(
        "Classified Changes: Relevant: {}, Blocked: {}, Unrelated: {}",
        classified.relevant.len(),
        classified.blocked.len(),
        classified.unrelated.len()
    ));

    // Log du plan d'action
    logs.push(format!(
        "Plan: Stage: {:?}, Commit: {}, Push: {}, Notes: {:?}",
        plan.will_stage, plan.will_commit, plan.will_push, plan.notes
    ));

    // Vérification des fichiers pertinents spécifiques
    if !policy.relevant_files.is_empty() {
        logs.push(format!(
            "Fichiers pertinents spécifiques autorisés: {:?}",
            policy.relevant_files
        ));
    }

    // Vérification des préfixes bloqués
    if !policy.blocked_prefixes.is_empty() {
        logs.push(format!("Préfixes bloqués: {:?}", policy.blocked_prefixes));
    }

    // Pre-checks avant apply
    if mode == AutopilotMode::ApplySafe && plan.will_commit {
        match policy.pre_checks {
            PreChecks::None => logs.push("PreChecks: None".into()),
            PreChecks::FmtCheck => {
                logs.push("PreChecks: cargo fmt --check".into());
                cargo_fmt_check(&mut logs)?;
            }
            PreChecks::FmtCheckAndTests => {
                logs.push("PreChecks: cargo fmt --check + cargo test".into());
                cargo_fmt_check(&mut logs)?;
                cargo_test(&mut logs)?;
            }
        }
    }

    let mut applied = false;

    if mode == AutopilotMode::ApplySafe {
        if plan.will_commit {
            // Stage relevant
            for path in &plan.will_stage {
                git_add_path(path, &mut logs)?;
            }

            // Commit
            git_commit(&plan.commit_message, &mut logs)?;
            applied = true;

            // Push (optionnel)
            if policy.allow_push {
                git_push_current_branch(policy, &branch, &mut logs)?;
            }
        } else {
            logs.push(
                "ApplySafe: aucun fichier pertinent à commit, aucune action effectuée.".into(),
            );
        }
    } else {
        logs.push("DryRun: aucune action appliquée.".into());
    }

    // Ajoute des notes utiles au plan
    if !classified.unrelated.is_empty() && !policy.fail_on_unrelated_changes {
        plan.notes.push(format!(
            "Attention: {} changements non liés ignorés par la policy (fail_on_unrelated_changes=false).",
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

/* ----------------------------- Helpers: policy ----------------------------- */

fn classify_changes(changes: &[GitChange], policy: &AutopilotPolicy) -> ClassifiedChanges {
    let mut relevant = Vec::new();
    let mut unrelated = Vec::new();
    let mut blocked = Vec::new();

    for ch in changes {
        let path = ch.path.as_str();

        if is_blocked(path, policy) {
            blocked.push(ch.clone());
            continue;
        }

        if is_relevant(path, policy) {
            relevant.push(ch.clone());
        } else {
            unrelated.push(ch.clone());
        }
    }

    ClassifiedChanges {
        relevant,
        unrelated,
        blocked,
    }
}

fn is_blocked(path: &str, policy: &AutopilotPolicy) -> bool {
    // blocked prefixes
    if policy
        .blocked_prefixes
        .iter()
        .any(|p| path_starts_with(path, p))
    {
        return true;
    }
    false
}

fn is_relevant(path: &str, policy: &AutopilotPolicy) -> bool {
    if policy.relevant_files.iter().any(|f| f == path) {
        return true;
    }
    policy
        .relevant_prefixes
        .iter()
        .any(|p| path_starts_with(path, p))
}

/// Un starts_with "safe" (normalise séparateurs).
fn path_starts_with(path: &str, prefix: &str) -> bool {
    let p = path.replace('\\', "/");
    let pref = prefix.replace('\\', "/");
    p.starts_with(&pref)
}

fn display_change_path(ch: &GitChange) -> String {
    match &ch.orig_path {
        Some(orig) => format!("{orig} -> {}", ch.path),
        None => ch.path.clone(),
    }
}

fn build_commit_message(branch: &str, relevant: &[GitChange]) -> String {
    // Message simple, stable, pas trop long.
    // Tu pourras ensuite brancher une IA pour mieux résumer, mais toujours via policy.
    let mut by_status: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for c in relevant {
        by_status
            .entry(c.status_str())
            .or_default()
            .push(display_change_path(c));
    }

    let mut lines = Vec::new();
    lines.push(format!("varina: update ({})", branch));

    for (status, files) in by_status {
        lines.push(format!("- {}:", status));
        for f in files.into_iter().take(20) {
            lines.push(format!("  - {}", f));
        }
    }

    // git commit -m: éviter d'envoyer un message monstrueux.
    // On met le résumé en sujet + corps compact.
    // Ici on fait un seul -m (git accepte, mais c’est un seul message multi-ligne).
    lines.join("\n")
}

/* ----------------------------- Helpers: git/cargo ----------------------------- */

fn ensure_git_repo(logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd("git", &["rev-parse", "--is-inside-work-tree"], logs)?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s != "true" {
        return Err("Refus: pas dans un repository git.".to_string().into());
    }
    Ok(())
}

fn current_branch(logs: &mut Vec<String>) -> Result<(String, bool)> {
    // rev-parse --abbrev-ref HEAD retourne "HEAD" si detached.
    let out = run_cmd("git", &["rev-parse", "--abbrev-ref", "HEAD"], logs)?;
    let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let detached = branch == "HEAD" || branch.is_empty();
    Ok((branch, detached))
}

fn git_status_porcelain_z(logs: &mut Vec<String>) -> Result<Vec<GitChange>> {
    let out = run_cmd("git", &["status", "--porcelain=v1", "-z"], logs)?;
    parse_porcelain_v1_z(&out.stdout)
}

fn git_add_path(path: &str, logs: &mut Vec<String>) -> Result<()> {
    // -- ajoute, -- path séparateur pour éviter confusion avec options.
    run_cmd("git", &["add", "--", path], logs)?;
    Ok(())
}

fn git_commit(message: &str, logs: &mut Vec<String>) -> Result<()> {
    // Si rien à commit, git commit échoue. On préfère détecter avant, mais on garde un guard.
    let out = run_cmd_allow_failure("git", &["commit", "-m", message], logs)?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        // Cas classique: "nothing to commit"
        if stderr.to_lowercase().contains("nothing to commit") {
            logs.push("git commit: nothing to commit (ignore)".into());
            return Ok(());
        }
        return Err(format!("git commit failed: {}", stderr.trim()).into());
    }
    Ok(())
}

fn git_push_current_branch(
    policy: &AutopilotPolicy,
    branch: &str,
    logs: &mut Vec<String>,
) -> Result<()> {
    if !policy.allow_push {
        logs.push("Push désactivé par policy.".into());
        return Ok(());
    }

    // Vérifie upstream
    let upstream = run_cmd_allow_failure(
        "git",
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
        logs,
    );

    let has_upstream = match upstream {
        Ok(o) => o.status.success(),
        Err(_) => false,
    };

    if has_upstream {
        run_cmd("git", &["push"], logs)?;
        return Ok(());
    }

    if policy.push_set_upstream_if_missing {
        run_cmd(
            "git",
            &["push", "-u", policy.push_remote.as_str(), branch],
            logs,
        )?;
        return Ok(());
    }

    logs.push("Upstream manquant, push ignoré (policy.push_set_upstream_if_missing=false)".into());
    Ok(())
}

fn cargo_fmt_check(logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_allow_failure("cargo", &["fmt", "--", "--check"], logs)?;
    if !out.status.success() {
        return Err("Pré-check échoué: code non formaté. Exécute `cargo fmt`."
            .to_string()
            .into());
    }
    Ok(())
}

fn cargo_test(logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_allow_failure("cargo", &["test"], logs)?;
    if !out.status.success() {
        return Err(
            "Pré-check échoué: certains tests ont échoué (`cargo test`)."
                .to_string()
                .into(),
        );
    }
    Ok(())
}

/* ----------------------------- Command runner ----------------------------- */

fn run_cmd(bin: &str, args: &[&str], logs: &mut Vec<String>) -> Result<Output> {
    let out = run_cmd_allow_failure(bin, args, logs)?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(format!("Commande échouée: {} {:?}\n{}", bin, args, stderr).into());
    }
    Ok(out)
}

fn run_cmd_allow_failure(bin: &str, args: &[&str], logs: &mut Vec<String>) -> Result<Output> {
    logs.push(format!("$ {} {}", bin, args.join(" ")));

    let out = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| AutopilotError {
            message: format!("Erreur lancement commande {} {:?}: {}", bin, args, e),
        })?;

    // log léger (évite de spammer l'UI avec 10k lignes)
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    if !stdout.trim().is_empty() {
        logs.push(truncate_log(format!("stdout: {}", stdout.trim())));
    }
    if !stderr.trim().is_empty() {
        logs.push(truncate_log(format!("stderr: {}", stderr.trim())));
    }

    Ok(out)
}

fn truncate_log(s: String) -> String {
    const MAX: usize = 2000;
    if s.len() > MAX {
        format!("{} ... (truncated)", &s[..MAX])
    } else {
        s
    }
}

/* ----------------------------- Porcelain parser ----------------------------- */

fn parse_porcelain_v1_z(bytes: &[u8]) -> Result<Vec<GitChange>> {
    // Format v1 -z: séries d'entrées séparées par '\0'
    // Chaque entrée: XY<space>PATH\0
    // Pour rename/copy: XY<space>ORIG\0NEW\0
    //
    // Exemples XY:
    // " M", "M ", "A ", "R ", "C ", "??", "UU" etc.
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < bytes.len() {
        // Lire XY + espace minimum: 3 bytes (X, Y, ' ')
        if i + 3 > bytes.len() {
            break;
        }
        let x = bytes[i];
        let y = bytes[i + 1];
        // v1 a généralement un espace en [i+2]
        // mais pour "??" aussi c'est ' ' en [i+2]
        let _space = bytes[i + 2];
        i += 3;

        // Lire premier champ path jusqu'à \0
        let (field1, next_i) = read_cstr(bytes, i)?;
        i = next_i;

        let xy = [x, y];

        // Rename/Copy: X == 'R' ou 'C' (parfois Y peut aussi indiquer)
        let is_rename_like = x == b'R' || x == b'C';

        if is_rename_like {
            // field1 = orig, next field = new
            let orig = field1;
            let (field2, next_i2) = read_cstr(bytes, i)?;
            i = next_i2;

            out.push(GitChange {
                xy,
                path: field2,
                orig_path: Some(orig),
            });
        } else {
            out.push(GitChange {
                xy,
                path: field1,
                orig_path: None,
            });
        }
    }

    Ok(out)
}

fn read_cstr(bytes: &[u8], start: usize) -> Result<(String, usize)> {
    let mut end = start;
    while end < bytes.len() && bytes[end] != 0 {
        end += 1;
    }
    if end >= bytes.len() {
        return Err(
            "Parse error: expected NUL terminator in porcelain -z output"
                .to_string()
                .into(),
        );
    }
    let s = String::from_utf8_lossy(&bytes[start..end]).to_string();
    Ok((s, end + 1))
}

/* ----------------------------- Optional: tiny util ----------------------------- */

#[allow(dead_code)]
fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_relevant() {
        let policy = AutopilotPolicy {
            relevant_prefixes: vec!["src/".into(), "tests/".into()],
            relevant_files: vec!["Cargo.toml".into(), "README.md".into()],
            ..Default::default()
        };

        assert!(is_relevant("src/main.rs", &policy));
        assert!(is_relevant("tests/test.rs", &policy));
        assert!(is_relevant("Cargo.toml", &policy));
        assert!(!is_relevant("target/debug/app", &policy));
        assert!(!is_relevant("random/file.txt", &policy));
    }

    #[test]
    fn test_is_blocked() {
        let policy = AutopilotPolicy {
            blocked_prefixes: vec!["target/".into(), ".env".into()],
            ..Default::default()
        };

        assert!(is_blocked("target/debug/app", &policy));
        assert!(is_blocked(".env", &policy));
        assert!(!is_blocked("src/main.rs", &policy));
        assert!(!is_blocked("README.md", &policy));
    }

    #[test]
    fn test_classify_changes() {
        let policy = AutopilotPolicy {
            relevant_prefixes: vec!["src/".into(), "tests/".into()],
            relevant_files: vec!["Cargo.toml".into()],
            blocked_prefixes: vec!["target/".into()],
            ..Default::default()
        };

        let changes = vec![
            GitChange {
                xy: [b'M', b' '],
                path: "src/main.rs".into(),
                orig_path: None,
            },
            GitChange {
                xy: [b'M', b' '],
                path: "tests/test.rs".into(),
                orig_path: None,
            },
            GitChange {
                xy: [b'M', b' '],
                path: "Cargo.toml".into(),
                orig_path: None,
            },
            GitChange {
                xy: [b'M', b' '],
                path: "target/debug/app".into(),
                orig_path: None,
            },
            GitChange {
                xy: [b'M', b' '],
                path: "random/file.txt".into(),
                orig_path: None,
            },
        ];

        let classified = classify_changes(&changes, &policy);

        assert_eq!(classified.relevant.len(), 3);
        assert_eq!(classified.unrelated.len(), 1);
        assert_eq!(classified.blocked.len(), 1);
    }
}
