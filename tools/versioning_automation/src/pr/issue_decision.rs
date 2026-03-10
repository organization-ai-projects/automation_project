use crate::pr::contracts::cli::pr_issue_decision_options::PrIssueDecisionOptions;

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
pub(crate) struct DecisionOutcome {
    pub(crate) kind: String,
    pub(crate) reason: String,
    pub(crate) final_action: String,
    pub(crate) category: String,
    pub(crate) force_category: bool,
}

pub(crate) fn decide(opts: PrIssueDecisionOptions) -> DecisionOutcome {
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
