#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleEngine;

impl RuleEngine {
    pub fn evaluate_product(
        product_dir: &std::path::Path,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::reports::violation::Violation> {
        let product_name = product_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_product");

        let rules = vec![
            crate::rules::rule::Rule {
                id: crate::rules::rule_id::RuleId::Structure,
            },
            crate::rules::rule::Rule {
                id: crate::rules::rule_id::RuleId::Crate,
            },
            crate::rules::rule::Rule {
                id: crate::rules::rule_id::RuleId::Naming,
            },
            crate::rules::rule::Rule {
                id: crate::rules::rule_id::RuleId::Layering,
            },
            crate::rules::rule::Rule {
                id: crate::rules::rule_id::RuleId::Determinism,
            },
        ];

        let mut out = Vec::new();
        for rule in rules {
            let mut next = match rule.id {
                crate::rules::rule_id::RuleId::Structure => {
                    crate::rules::structure_rules::StructureRules::evaluate(
                        product_dir,
                        scope,
                        mode,
                    )
                }
                crate::rules::rule_id::RuleId::Crate => {
                    crate::rules::crate_rules::CrateRules::evaluate(
                        product_dir,
                        product_name,
                        scope,
                        mode,
                    )
                }
                crate::rules::rule_id::RuleId::Naming => {
                    crate::rules::naming_rules::NamingRules::evaluate(product_dir, scope, mode)
                }
                crate::rules::rule_id::RuleId::Layering => {
                    crate::rules::layering_rules::LayeringRules::evaluate(
                        product_dir,
                        product_name,
                        scope,
                        mode,
                    )
                }
                crate::rules::rule_id::RuleId::Determinism => {
                    crate::rules::determinism_rules::DeterminismRules::evaluate(
                        product_dir,
                        scope,
                        mode,
                    )
                }
            };
            out.append(&mut next);
        }

        out
    }

    pub fn evaluate_tool(
        tool_dir: &std::path::Path,
        scope: crate::config::path_classification::PathClassification,
        mode: crate::config::enforcement_mode::EnforcementMode,
    ) -> Vec<crate::reports::violation::Violation> {
        crate::rules::tool_rules::ToolRules::evaluate(tool_dir, scope, mode)
    }
}
