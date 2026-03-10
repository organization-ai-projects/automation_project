use crate::pr::model::pr_issue_decision_options::PrIssueDecisionOptions;

pub(crate) fn run_issue_decision(opts: PrIssueDecisionOptions) -> i32 {
    let outcome = decide(opts);
    println!(
        "DECISION|{}|{}|{}|{}|{}",
        outcome.kind,
        outcome.reason,
        outcome.final_action,
        outcome.category,
        if outcome.force_category {
            "true"
        } else {
            "false"
        }
    );
    0
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DecisionOutcome {
    kind: String,
    reason: String,
    final_action: String,
    category: String,
    force_category: bool,
}

fn decide(opts: PrIssueDecisionOptions) -> DecisionOutcome {
    if opts.action == "Closes" && opts.seen_reopen {
        let category = if opts.reopen_category.is_empty() {
            opts.default_category
        } else {
            opts.reopen_category
        };
        return DecisionOutcome {
            kind: "resolve_reopen".to_string(),
            reason: "Resolved via directive decision => reopen.".to_string(),
            final_action: "reopen".to_string(),
            category,
            force_category: true,
        };
    }

    let inferred_conflict = opts.allow_inferred
        && (opts.inferred_decision.is_empty() || opts.inferred_decision == "conflict");

    if inferred_conflict {
        return DecisionOutcome {
            kind: "conflict".to_string(),
            reason: "conflicting inferred directives".to_string(),
            final_action: String::new(),
            category: String::new(),
            force_category: false,
        };
    }

    let effective_decision = if !opts.explicit_decision.is_empty() {
        opts.explicit_decision.as_str()
    } else {
        opts.inferred_decision.as_str()
    };

    if effective_decision == "close" && opts.action == "Reopen" {
        return DecisionOutcome {
            kind: "ignore".to_string(),
            reason: String::new(),
            final_action: String::new(),
            category: String::new(),
            force_category: false,
        };
    }

    if effective_decision == "reopen" {
        return DecisionOutcome {
            kind: "resolve_reopen".to_string(),
            reason: "Resolved via directive decision => reopen.".to_string(),
            final_action: "reopen".to_string(),
            category: opts.default_category,
            force_category: false,
        };
    }

    DecisionOutcome {
        kind: "continue".to_string(),
        reason: String::new(),
        final_action: String::new(),
        category: String::new(),
        force_category: false,
    }
}

#[cfg(test)]
mod tests {
    use crate::pr::model::pr_issue_decision_options::PrIssueDecisionOptions;

    use super::decide;

    fn base_opts() -> PrIssueDecisionOptions {
        PrIssueDecisionOptions {
            action: "Closes".to_string(),
            issue: "#1".to_string(),
            default_category: "Mixed".to_string(),
            seen_reopen: false,
            reopen_category: String::new(),
            inferred_decision: String::new(),
            explicit_decision: String::new(),
            allow_inferred: true,
        }
    }

    #[test]
    fn resolve_reopen_when_close_hits_seen_reopen() {
        let mut opts = base_opts();
        opts.seen_reopen = true;
        opts.reopen_category = "UI".to_string();
        let out = decide(opts);
        assert_eq!(out.kind, "resolve_reopen");
        assert_eq!(out.category, "UI");
        assert!(out.force_category);
    }

    #[test]
    fn conflict_when_allow_inferred_and_no_inferred_decision() {
        let opts = base_opts();
        let out = decide(opts);
        assert_eq!(out.kind, "conflict");
    }

    #[test]
    fn ignore_reopen_when_effective_close() {
        let mut opts = base_opts();
        opts.action = "Reopen".to_string();
        opts.explicit_decision = "close".to_string();
        opts.allow_inferred = false;
        let out = decide(opts);
        assert_eq!(out.kind, "ignore");
    }

    #[test]
    fn resolve_reopen_when_effective_reopen() {
        let mut opts = base_opts();
        opts.allow_inferred = false;
        opts.inferred_decision = "reopen".to_string();
        let out = decide(opts);
        assert_eq!(out.kind, "resolve_reopen");
        assert_eq!(out.category, "Mixed");
    }

    #[test]
    fn continue_when_no_directive_override() {
        let mut opts = base_opts();
        opts.allow_inferred = false;
        let out = decide(opts);
        assert_eq!(out.kind, "continue");
    }
}
