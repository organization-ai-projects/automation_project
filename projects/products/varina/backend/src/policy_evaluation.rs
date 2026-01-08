// projects/products/varina/backend/src/policy_evaluation.rs

use git_lib::git_change::GitChange;

use crate::{
    AutopilotPolicy, ClassifiedChangesRef,
    autopilot::{
        CompiledAutopilotPolicy,
        compiled_autopilot_policy::{normalize_path, path_has_compiled_prefix},
    },
};

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
    let compiled = CompiledAutopilotPolicy::from(policy);
    is_relevant_norm(&normalize_path(path), &compiled)
}

pub fn is_relevant_norm(path_norm: &str, policy: &CompiledAutopilotPolicy) -> bool {
    if policy.relevant_files_norm.contains(path_norm) {
        return true;
    }

    policy
        .relevant_prefixes_norm
        .iter()
        .any(|pref| path_has_compiled_prefix(path_norm, pref))
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
