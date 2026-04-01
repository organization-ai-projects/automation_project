use super::lint_rule::LintRule;
use super::lint_rule_id::LintRuleId;
use crate::analysis::Finding;

/// Runs all enabled lint rules against a source string.
pub struct LintEngine;

impl LintEngine {
    pub fn lint(source: &str) -> Vec<Finding> {
        let rules = vec![
            LintRule {
                id: LintRuleId::TrailingWhitespace,
            },
            LintRule {
                id: LintRuleId::LineTooLong,
            },
            LintRule {
                id: LintRuleId::TodoComment,
            },
            LintRule {
                id: LintRuleId::UnusedImport,
            },
            LintRule {
                id: LintRuleId::MissingDocComment,
            },
        ];

        let mut findings = Vec::new();
        for rule in &rules {
            findings.extend(rule.check(source));
        }
        findings
    }
}
