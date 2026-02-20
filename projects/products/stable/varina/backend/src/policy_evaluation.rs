//! projects/products/varina/backend/src/policy_evaluation.rs
use ignore::gitignore::GitignoreBuilder;

use crate::autopilot::{
    AutopilotPolicy, CompiledAutopilotPolicy,
    compiled_autopilot_policy::{normalize_path, path_has_compiled_prefix},
};
use crate::classified_changes::ClassifiedChanges;
use crate::classified_changes_ref::ClassifiedChangesRef;

use std::path::{self, Path};

/// ==============================
/// SECTION: Policy evaluation
/// ==============================
/// Best: compiled policy + borrowed result (zero clone).
pub fn classify_changes_ref(
    changes: &[impl AsRef<str>],
    policy: &AutopilotPolicy,
) -> ClassifiedChangesRef {
    let compiled = CompiledAutopilotPolicy::from(policy);
    classify_changes_ref_with_policy(changes, &compiled)
}

/// Best: caller provides the compiled policy (useful for multiple operations).
pub fn classify_changes_ref_with_policy(
    changes: &[impl AsRef<str>],
    policy: &CompiledAutopilotPolicy,
) -> ClassifiedChangesRef {
    let mut out = ClassifiedChangesRef::new();

    for ch in changes.iter() {
        let path_norm = normalize_path(ch.as_ref());

        if is_blocked_norm(&path_norm, policy) {
            out.blocked.push(ch.as_ref().to_string());
            continue;
        }

        if is_relevant_norm(&path_norm, policy) {
            out.relevant.push(ch.as_ref().to_string())
        } else {
            out.unrelated.push(ch.as_ref().to_string());
        }
    }

    out
}

/// Compatibility: owning version (explicit clones at the last moment).
pub fn classify_changes(
    changes: &[impl AsRef<str>],
    policy: &AutopilotPolicy,
) -> ClassifiedChanges {
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
            "projects/libraries/core/foundation/command_runner/".to_string(),
            "projects/libraries/common/".to_string(),
            "projects/libraries/core/foundation/common_calendar/".to_string(),
            "projects/libraries/core/foundation/common_time/".to_string(),
            "projects/libraries/git_lib/".to_string(),
            "projects/libraries/neural/".to_string(),
            "projects/libraries/core/contracts/protocol/".to_string(),
            "projects/libraries/security/".to_string(),
            "projects/libraries/symbolic/".to_string(),
            "projects/libraries/ui/".to_string(),
            "projects/products/stable/core/".to_string(),
            "projects/products/stable/varina/".to_string(),
        ],
        ..CompiledAutopilotPolicy::from(policy)
    };

    println!("[debug] is_relevant: Checking path {}", path);
    // Check if the file exists
    if !path::Path::new(path).exists() {
        println!("[debug] is_relevant: File {} does not exist", path);
        return false;
    }

    // Check with .gitignore
    let gitignore_path = path::Path::new(".gitignore");
    if !gitignore_path.exists() {
        println!("[debug] is_relevant: .gitignore not found, no files will be ignored");
        return is_relevant_norm(&normalize_path(path), &compiled);
    }

    let mut gitignore_builder = GitignoreBuilder::new(".");
    gitignore_builder.add(gitignore_path);
    let gitignore = gitignore_builder
        .build()
        .expect("Failed to parse .gitignore file");

    if gitignore.matched(path, false).is_ignore() {
        println!(
            "[debug] is_relevant: Path {} is ignored by .gitignore",
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
    let gitignore = builder.build().expect("Error while building .gitignore");

    let is_ignored = gitignore
        .matched_path_or_any_parents(path_norm, false)
        .is_ignore();

    if !is_ignored {
        // Use `policy` to check relevant prefixes
        return policy
            .relevant_prefixes_norm
            .iter()
            .any(|prefix| path_has_compiled_prefix(path_norm, prefix));
    }

    false
}

pub fn display_change_path(ch: &impl AsRef<str>) -> String {
    ch.as_ref().to_string()
}

/// Unmerged states (porcelain XY):
/// - 'U' is the canonical conflict marker
/// - 'AA' and 'DD' are also unmerged
pub fn has_merge_conflicts(changes: &[impl AsRef<str>]) -> bool {
    changes.iter().any(|c| {
        let x = c.as_ref().as_bytes()[0];
        let y = c.as_ref().as_bytes()[1];
        x == b'U' || y == b'U' || (x == b'A' && y == b'A') || (x == b'D' && y == b'D')
    })
}
