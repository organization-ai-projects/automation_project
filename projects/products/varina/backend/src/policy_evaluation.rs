// projects/products/varina/backend/src/policy_evaluation.rs
use git_lib::git_change::GitChange;
use ignore::gitignore::GitignoreBuilder;

use crate::{
    AutopilotPolicy, ClassifiedChangesRef,
    autopilot::{
        CompiledAutopilotPolicy,
        compiled_autopilot_policy::{normalize_path, path_has_compiled_prefix},
    },
};

use std::path::Path;

/// ==============================
/// SECTION: Policy evaluation
/// ==============================
/// Best: policy compilée + résultat borrowed (zero clone).
pub fn classify_changes_ref<'a>(
    changes: &'a [GitChange],
    policy: &AutopilotPolicy,
) -> ClassifiedChangesRef<'a> {
    let compiled = CompiledAutopilotPolicy::from(policy);
    classify_changes_ref_with_policy(changes, &compiled)
}

/// Best: caller fournit la policy compilée (si tu fais plusieurs opérations).
pub fn classify_changes_ref_with_policy<'a>(
    changes: &'a [GitChange],
    policy: &CompiledAutopilotPolicy,
) -> ClassifiedChangesRef<'a> {
    let mut out = ClassifiedChangesRef::new();

    for ch in changes.iter() {
        let path_norm = normalize_path(&ch.path);

        if is_blocked_norm(&path_norm, policy) {
            out.blocked.push(ch);
            continue;
        }

        if is_relevant_norm(&path_norm, policy) {
            out.relevant.push(ch);
        } else {
            out.unrelated.push(ch);
        }
    }

    out
}

/// Compat: version owning (clones explicites au dernier moment).
pub fn classify_changes(
    changes: &[GitChange],
    policy: &AutopilotPolicy,
) -> crate::ClassifiedChanges {
    classify_changes_ref(changes, policy).to_owned()
}

pub fn is_blocked(path: &str, policy: &AutopilotPolicy) -> bool {
    let compiled = CompiledAutopilotPolicy::from(policy);
    is_blocked_norm(&normalize_path(path), &compiled)
}

pub fn is_blocked_norm(path_norm: &str, policy: &CompiledAutopilotPolicy) -> bool {
    policy
        .blocked_prefixes_norm
        .iter()
        .any(|pref| path_has_compiled_prefix(path_norm, pref))
}

pub fn is_relevant(path: &str, policy: &AutopilotPolicy) -> bool {
    let compiled = CompiledAutopilotPolicy {
        relevant_prefixes_norm: vec![
            "src/".to_string(),
            "tests/".to_string(),
            "projects/libraries/ai/".to_string(),
            "projects/libraries/command_runner/".to_string(),
            "projects/libraries/common/".to_string(),
            "projects/libraries/common_calendar/".to_string(),
            "projects/libraries/common_time/".to_string(),
            "projects/libraries/git_lib/".to_string(),
            "projects/libraries/neural/".to_string(),
            "projects/libraries/protocol/".to_string(),
            "projects/libraries/security/".to_string(),
            "projects/libraries/symbolic/".to_string(),
            "projects/libraries/ui/".to_string(),
            "projects/products/core/".to_string(),
            "projects/products/varina/".to_string(),
        ],
        ..CompiledAutopilotPolicy::from(policy)
    };

    println!("[debug] is_relevant: Vérification du chemin {}", path);
    // Vérification de l'existence du fichier
    if !std::path::Path::new(path).exists() {
        println!("[debug] is_relevant: Le fichier {} n'existe pas", path);
        return false;
    }

    // Vérification avec .gitignore
    let gitignore_path = std::path::Path::new(".gitignore");
    if !gitignore_path.exists() {
        println!("[debug] is_relevant: .gitignore introuvable, aucun fichier ne sera ignoré");
        return is_relevant_norm(&normalize_path(path), &compiled);
    }

    let mut gitignore_builder = GitignoreBuilder::new(".");
    gitignore_builder.add(gitignore_path);
    let gitignore = gitignore_builder
        .build()
        .expect("Échec de l'analyse du fichier .gitignore");

    if gitignore.matched(path, false).is_ignore() {
        println!(
            "[debug] is_relevant: Le chemin {} est ignoré par .gitignore",
            path
        );
        return false;
    }

    is_relevant_norm(&normalize_path(path), &compiled)
}

pub fn is_relevant_norm(path_norm: &str, policy: &CompiledAutopilotPolicy) -> bool {
    let repo_root = Path::new(".");
    let mut builder = GitignoreBuilder::new(repo_root);
    builder.add(repo_root.join(".gitignore"));
    let gitignore = builder
        .build()
        .expect("Erreur lors de la construction de .gitignore");

    let is_ignored = gitignore
        .matched_path_or_any_parents(path_norm, false)
        .is_ignore();

    if !is_ignored {
        // Utilisation de `policy` pour vérifier les préfixes pertinents
        return policy
            .relevant_prefixes_norm
            .iter()
            .any(|prefix| path_has_compiled_prefix(path_norm, prefix));
    }

    false
}

pub fn display_change_path(ch: &GitChange) -> String {
    match &ch.orig_path {
        Some(orig) => format!("{orig} -> {}", ch.path),
        None => ch.path.clone(),
    }
}

/// Unmerged states (porcelain XY):
/// - 'U' is canonical conflict marker
/// - 'AA' and 'DD' are also unmerged
pub fn has_merge_conflicts(changes: &[GitChange]) -> bool {
    changes.iter().any(|c| {
        let x = c.xy[0];
        let y = c.xy[1];
        x == b'U' || y == b'U' || (x == b'A' && y == b'A') || (x == b'D' && y == b'D')
    })
}

// Implémentation supprimée car redondante avec celle dans compiled_autopilot_policy.rs.
