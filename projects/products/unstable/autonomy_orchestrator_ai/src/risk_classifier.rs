// projects/products/unstable/autonomy_orchestrator_ai/src/risk_classifier.rs
use crate::domain::{RiskSignal, RiskTier};

pub struct RiskClassifierInput<'a> {
    pub risk_tier_override: Option<RiskTier>,
    pub delivery_enabled: bool,
    pub delivery_branch: Option<&'a str>,
    pub delivery_dry_run: bool,
    pub executor_bin: Option<&'a str>,
    pub executor_args: &'a [String],
}

pub struct RiskClassification {
    pub tier: RiskTier,
    pub signals: Vec<RiskSignal>,
}

pub fn classify_risk(input: &RiskClassifierInput<'_>) -> RiskClassification {
    if let Some(override_tier) = input.risk_tier_override {
        return RiskClassification {
            tier: override_tier,
            signals: vec![RiskSignal {
                code: "RISK_TIER_OVERRIDE_APPLIED".to_string(),
                source: "cli_override".to_string(),
                value: format!("{:?}", override_tier).to_lowercase(),
            }],
        };
    }

    let mut signals: Vec<RiskSignal> = Vec::new();
    let mut max_tier = RiskTier::Low;

    // Rule: high on delivery to main/master/release branch
    if input.delivery_enabled && !input.delivery_dry_run {
        if let Some(branch) = input.delivery_branch {
            if is_protected_branch(branch) {
                signals.push(RiskSignal {
                    code: "DELIVERY_TO_PROTECTED_BRANCH".to_string(),
                    source: "delivery_options".to_string(),
                    value: branch.to_string(),
                });
                max_tier = max_tier.max(RiskTier::High);
            } else {
                signals.push(RiskSignal {
                    code: "DELIVERY_WRITE_ENABLED".to_string(),
                    source: "delivery_options".to_string(),
                    value: branch.to_string(),
                });
                max_tier = max_tier.max(RiskTier::Medium);
            }
        } else {
            signals.push(RiskSignal {
                code: "DELIVERY_WRITE_ENABLED".to_string(),
                source: "delivery_options".to_string(),
                value: "no_branch".to_string(),
            });
            max_tier = max_tier.max(RiskTier::Medium);
        }
    }

    // Rule: high on history rewrite intent in executor args
    if input.executor_bin.is_some() {
        let args_joined: Vec<&str> = input.executor_args.iter().map(|s| s.as_str()).collect();
        if has_history_rewrite_intent(&args_joined) {
            signals.push(RiskSignal {
                code: "HISTORY_REWRITE_INTENT".to_string(),
                source: "executor_args".to_string(),
                value: input.executor_args.join(" "),
            });
            max_tier = max_tier.max(RiskTier::High);
        } else {
            // write-capable executor without high-risk signal → medium
            signals.push(RiskSignal {
                code: "EXECUTOR_WRITE_CAPABLE".to_string(),
                source: "executor_args".to_string(),
                value: input.executor_bin.unwrap_or("").to_string(),
            });
            max_tier = max_tier.max(RiskTier::Medium);
        }
    }

    RiskClassification {
        tier: max_tier,
        signals,
    }
}

fn is_protected_branch(branch: &str) -> bool {
    let lower = branch.to_ascii_lowercase();
    lower == "main"
        || lower == "master"
        || lower.starts_with("release")
        || lower.starts_with("rel/")
        || lower.starts_with("hotfix")
}

fn has_history_rewrite_intent(args: &[&str]) -> bool {
    let has = |needle: &str| args.contains(&needle);
    let has_prefix = |prefix: &str| args.iter().any(|a| a.starts_with(prefix));

    has("--force")
        || has("-f")
        || has("--force-with-lease")
        || has_prefix("--force-with-lease=")
        || has("rebase")
        || (has("reset") && (has("--hard") || has("--mixed")))
        || (has("push") && (has("--force") || has("-f") || has("--force-with-lease")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_args() -> Vec<String> {
        Vec::new()
    }

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn low_risk_when_no_write_signals() {
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: false,
            delivery_branch: None,
            delivery_dry_run: false,
            executor_bin: None,
            executor_args: &no_args(),
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::Low);
        assert!(result.signals.is_empty());
    }

    #[test]
    fn medium_risk_on_delivery_without_protected_branch() {
        let branch = "feature/my-feature".to_string();
        let args = no_args();
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: false,
            executor_bin: None,
            executor_args: &args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::Medium);
        assert!(
            result
                .signals
                .iter()
                .any(|s| s.code == "DELIVERY_WRITE_ENABLED")
        );
    }

    #[test]
    fn high_risk_on_delivery_to_main() {
        let branch = "main".to_string();
        let args = no_args();
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: false,
            executor_bin: None,
            executor_args: &args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::High);
        assert!(
            result
                .signals
                .iter()
                .any(|s| s.code == "DELIVERY_TO_PROTECTED_BRANCH")
        );
    }

    #[test]
    fn high_risk_on_delivery_to_release_branch() {
        let branch = "release/1.0".to_string();
        let args = no_args();
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: false,
            executor_bin: None,
            executor_args: &args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::High);
    }

    #[test]
    fn high_risk_on_force_push_executor_arg() {
        let executor_args = args(&["push", "--force"]);
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: false,
            delivery_branch: None,
            delivery_dry_run: false,
            executor_bin: Some("git"),
            executor_args: &executor_args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::High);
        assert!(
            result
                .signals
                .iter()
                .any(|s| s.code == "HISTORY_REWRITE_INTENT")
        );
    }

    #[test]
    fn high_risk_on_rebase_arg() {
        let executor_args = args(&["rebase", "-i", "HEAD~3"]);
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: false,
            delivery_branch: None,
            delivery_dry_run: false,
            executor_bin: Some("git"),
            executor_args: &executor_args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::High);
    }

    #[test]
    fn high_risk_dominates_over_medium() {
        let branch = "main".to_string();
        let executor_args = args(&["do-something"]);
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: false,
            executor_bin: Some("my-tool"),
            executor_args: &executor_args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::High);
    }

    #[test]
    fn cli_override_low_applies_regardless_of_delivery() {
        let branch = "main".to_string();
        let args = no_args();
        let input = RiskClassifierInput {
            risk_tier_override: Some(RiskTier::Low),
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: false,
            executor_bin: None,
            executor_args: &args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::Low);
        assert!(
            result
                .signals
                .iter()
                .any(|s| s.code == "RISK_TIER_OVERRIDE_APPLIED")
        );
    }

    #[test]
    fn cli_override_high_applies_regardless_of_inputs() {
        let args = no_args();
        let input = RiskClassifierInput {
            risk_tier_override: Some(RiskTier::High),
            delivery_enabled: false,
            delivery_branch: None,
            delivery_dry_run: false,
            executor_bin: None,
            executor_args: &args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::High);
        assert!(
            result
                .signals
                .iter()
                .any(|s| s.code == "RISK_TIER_OVERRIDE_APPLIED")
        );
    }

    #[test]
    fn medium_risk_on_executor_without_rewrite_args() {
        let executor_args = args(&["cargo", "test"]);
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: false,
            delivery_branch: None,
            delivery_dry_run: false,
            executor_bin: Some("cargo"),
            executor_args: &executor_args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::Medium);
        assert!(
            result
                .signals
                .iter()
                .any(|s| s.code == "EXECUTOR_WRITE_CAPABLE")
        );
    }

    #[test]
    fn delivery_dry_run_is_not_write_capable() {
        let branch = "main".to_string();
        let args = no_args();
        let input = RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: true,
            executor_bin: None,
            executor_args: &args,
        };
        let result = classify_risk(&input);
        assert_eq!(result.tier, RiskTier::Low);
        assert!(result.signals.is_empty());
    }

    #[test]
    fn identical_inputs_produce_same_classification() {
        let branch = "feature/x".to_string();
        let executor_args = args(&["cargo", "build"]);
        let make_input = || RiskClassifierInput {
            risk_tier_override: None,
            delivery_enabled: true,
            delivery_branch: Some(&branch),
            delivery_dry_run: false,
            executor_bin: Some("cargo"),
            executor_args: &executor_args,
        };
        let r1 = classify_risk(&make_input());
        let r2 = classify_risk(&make_input());
        assert_eq!(r1.tier, r2.tier);
        assert_eq!(r1.signals.len(), r2.signals.len());
        for (s1, s2) in r1.signals.iter().zip(r2.signals.iter()) {
            assert_eq!(s1.code, s2.code);
            assert_eq!(s1.source, s2.source);
            assert_eq!(s1.value, s2.value);
        }
    }
}
