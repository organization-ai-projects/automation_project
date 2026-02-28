// projects/products/unstable/autonomy_orchestrator_ai/src/hard_gates.rs
use crate::domain::{HardGateCategory, HardGateMode, HardGateResult, HardGateRule};
use common_json::from_str;
use std::fs;
use std::path::Path;

pub fn builtin_rules() -> Vec<HardGateRule> {
    vec![
        // secrets exposure patterns
        HardGateRule {
            id: "builtin-secrets-1".to_string(),
            category: HardGateCategory::Secrets,
            pattern: "dump-secrets".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        HardGateRule {
            id: "builtin-secrets-2".to_string(),
            category: HardGateCategory::Secrets,
            pattern: "export-credentials".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        // auth/authz mutation patterns
        HardGateRule {
            id: "builtin-auth-1".to_string(),
            category: HardGateCategory::Auth,
            pattern: "modify-sudoers".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        HardGateRule {
            id: "builtin-auth-2".to_string(),
            category: HardGateCategory::Auth,
            pattern: "chmod-auth".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        // git history rewrite patterns
        HardGateRule {
            id: "builtin-git-history-1".to_string(),
            category: HardGateCategory::GitHistory,
            pattern: "--force-push".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        HardGateRule {
            id: "builtin-git-history-2".to_string(),
            category: HardGateCategory::GitHistory,
            pattern: "filter-branch".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        // destructive infra/data operations
        HardGateRule {
            id: "builtin-infra-1".to_string(),
            category: HardGateCategory::InfraDestructive,
            pattern: "terraform-destroy".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
        HardGateRule {
            id: "builtin-infra-2".to_string(),
            category: HardGateCategory::InfraDestructive,
            pattern: "drop-database".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        },
    ]
}

pub fn load_external_rules(path: &Path) -> Result<Vec<HardGateRule>, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read hard gates file '{}': {}", path.display(), e))?;
    let rules: Vec<HardGateRule> = from_str(&raw).map_err(|e| {
        format!(
            "Failed to parse hard gates file '{}': {:?}",
            path.display(),
            e
        )
    })?;
    for rule in &rules {
        if rule.id.is_empty() {
            return Err("External hard gate rule has empty id".to_string());
        }
        if rule.pattern.is_empty() {
            return Err(format!(
                "External hard gate rule '{}' has empty pattern",
                rule.id
            ));
        }
    }
    Ok(rules)
}

pub fn evaluate_hard_gates(
    rules: &[HardGateRule],
    invocation_tokens: &[String],
) -> Vec<HardGateResult> {
    rules
        .iter()
        .map(|rule| {
            let matched = match rule.mode {
                HardGateMode::MatchAnyInvocationArg => invocation_tokens.iter().any(|token| {
                    token
                        .to_ascii_lowercase()
                        .contains(&rule.pattern.to_ascii_lowercase())
                }),
            };
            HardGateResult {
                id: rule.id.clone(),
                passed: !matched,
                reason_code: category_reason_code(&rule.category).to_string(),
            }
        })
        .collect()
}

fn category_reason_code(category: &HardGateCategory) -> &'static str {
    match category {
        HardGateCategory::Secrets => "HARD_GATE_SECRET_POLICY_VIOLATION",
        HardGateCategory::Auth => "HARD_GATE_AUTH_POLICY_VIOLATION",
        HardGateCategory::GitHistory => "HARD_GATE_GIT_HISTORY_REWRITE_FORBIDDEN",
        HardGateCategory::InfraDestructive => "HARD_GATE_INFRA_DESTRUCTIVE_OPERATION",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn builtin_secrets_rule_triggers_on_matching_token() {
        let rules = builtin_rules();
        let results = evaluate_hard_gates(&rules, &tokens(&["dump-secrets", "--output", "/tmp/x"]));
        let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        assert!(
            failed
                .iter()
                .any(|r| r.reason_code == "HARD_GATE_SECRET_POLICY_VIOLATION"),
            "expected HARD_GATE_SECRET_POLICY_VIOLATION, got: {failed:?}"
        );
    }

    #[test]
    fn builtin_auth_rule_triggers_on_matching_token() {
        let rules = builtin_rules();
        let results = evaluate_hard_gates(&rules, &tokens(&["modify-sudoers"]));
        let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        assert!(
            failed
                .iter()
                .any(|r| r.reason_code == "HARD_GATE_AUTH_POLICY_VIOLATION"),
            "expected HARD_GATE_AUTH_POLICY_VIOLATION, got: {failed:?}"
        );
    }

    #[test]
    fn builtin_git_history_rule_triggers_on_matching_token() {
        let rules = builtin_rules();
        let results = evaluate_hard_gates(&rules, &tokens(&["git", "--force-push", "origin"]));
        let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        assert!(
            failed
                .iter()
                .any(|r| r.reason_code == "HARD_GATE_GIT_HISTORY_REWRITE_FORBIDDEN"),
            "expected HARD_GATE_GIT_HISTORY_REWRITE_FORBIDDEN, got: {failed:?}"
        );
    }

    #[test]
    fn builtin_infra_destructive_rule_triggers_on_matching_token() {
        let rules = builtin_rules();
        let results =
            evaluate_hard_gates(&rules, &tokens(&["terraform-destroy", "--auto-approve"]));
        let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        assert!(
            failed
                .iter()
                .any(|r| r.reason_code == "HARD_GATE_INFRA_DESTRUCTIVE_OPERATION"),
            "expected HARD_GATE_INFRA_DESTRUCTIVE_OPERATION, got: {failed:?}"
        );
    }

    #[test]
    fn no_match_passes_all_builtin_rules() {
        let rules = builtin_rules();
        let results = evaluate_hard_gates(&rules, &tokens(&["cargo", "build", "--release"]));
        assert!(
            results.iter().all(|r| r.passed),
            "expected all rules to pass, got failures: {:?}",
            results.iter().filter(|r| !r.passed).collect::<Vec<_>>()
        );
    }

    #[test]
    fn external_rule_appended_and_evaluated() {
        let mut rules = builtin_rules();
        rules.push(HardGateRule {
            id: "external-secrets-1".to_string(),
            category: HardGateCategory::Secrets,
            pattern: "custom-leak".to_string(),
            mode: HardGateMode::MatchAnyInvocationArg,
        });
        let results = evaluate_hard_gates(&rules, &tokens(&["custom-leak", "--flag"]));
        let failed: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        assert!(
            failed.iter().any(|r| r.id == "external-secrets-1"),
            "expected external rule to trigger, got: {failed:?}"
        );
    }

    #[test]
    fn load_external_rules_rejects_empty_id() {
        let tmp = std::env::temp_dir().join(format!(
            "hard_gates_test_empty_id_{}_{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::write(
            &tmp,
            r#"[{"id":"","category":"secrets","pattern":"foo","mode":"match_any_invocation_arg"}]"#,
        )
        .expect("write temp file");
        let result = load_external_rules(&tmp);
        std::fs::remove_file(&tmp).ok();
        assert!(
            result.is_err(),
            "expected error for empty id, got: {result:?}"
        );
    }

    #[test]
    fn load_external_rules_rejects_empty_pattern() {
        let tmp = std::env::temp_dir().join(format!(
            "hard_gates_test_empty_pattern_{}_{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::write(
            &tmp,
            r#"[{"id":"ext-1","category":"auth","pattern":"","mode":"match_any_invocation_arg"}]"#,
        )
        .expect("write temp file");
        let result = load_external_rules(&tmp);
        std::fs::remove_file(&tmp).ok();
        assert!(
            result.is_err(),
            "expected error for empty pattern, got: {result:?}"
        );
    }

    #[test]
    fn load_external_rules_valid_file_succeeds() {
        let tmp = std::env::temp_dir().join(format!(
            "hard_gates_test_valid_{}_{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::write(
            &tmp,
            r#"[{"id":"ext-2","category":"git_history","pattern":"squash-force","mode":"match_any_invocation_arg"}]"#,
        )
        .expect("write temp file");
        let result = load_external_rules(&tmp);
        std::fs::remove_file(&tmp).ok();
        let rules = result.expect("expected valid rules to load");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, "ext-2");
    }
}
