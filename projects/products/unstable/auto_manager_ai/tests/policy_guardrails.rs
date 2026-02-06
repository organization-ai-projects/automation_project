use auto_manager_ai::{Action, ActionStatus, ActionTarget, Policy, PolicyDecisionType, RiskLevel};

#[test]
fn test_policy_default_deny_writes() {
    let policy = Policy::default();
    
    // Create a write action
    let action = Action {
        id: "test_001".to_string(),
        action_type: "create_issue".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "Test action".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.9,
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    // Should be denied in V0
    assert_eq!(decision.decision, PolicyDecisionType::Deny);
    assert!(decision.reason.contains("forbidden") || decision.reason.contains("V0"));
}

#[test]
fn test_policy_allows_read_only() {
    let policy = Policy::default();
    
    // Create a read-only action
    let action = Action {
        id: "test_002".to_string(),
        action_type: "analyze_repository".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "Read-only analysis".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.9,
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    // Should be allowed
    assert_eq!(decision.decision, PolicyDecisionType::Allow);
}

#[test]
fn test_policy_confidence_threshold() {
    let policy = Policy::default();
    
    // Create action with low confidence
    let action = Action {
        id: "test_003".to_string(),
        action_type: "analyze_repository".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "Low confidence action".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.3, // Below default threshold of 0.6
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    // Should be denied due to low confidence
    assert_eq!(decision.decision, PolicyDecisionType::Deny);
    assert!(decision.reason.contains("Confidence") || decision.reason.contains("threshold"));
}

#[test]
fn test_policy_missing_inputs() {
    let policy = Policy::default();
    
    // Create action with missing inputs
    let action = Action {
        id: "test_004".to_string(),
        action_type: "analyze_repository".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "Action needing input".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.9,
        evidence: vec![],
        depends_on: None,
        missing_inputs: Some(vec!["user_input".to_string()]),
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    // Should need input
    assert_eq!(decision.decision, PolicyDecisionType::NeedsInput);
    assert!(decision.reason.contains("Missing inputs"));
}

#[test]
fn test_all_write_actions_denied() {
    let policy = Policy::default();
    let write_actions = vec![
        "create_issue",
        "create_branch",
        "open_draft_pr",
        "post_pr_comment",
        "commit",
        "push",
        "merge",
        "force_push",
        "write_file",
        "delete_file",
    ];
    
    for action_type in write_actions {
        let action = Action {
            id: format!("test_{}", action_type),
            action_type: action_type.to_string(),
            status: ActionStatus::Proposed,
            target: ActionTarget::Repo {
                reference: "test/repo".to_string(),
            },
            justification: "Test write action".to_string(),
            risk_level: RiskLevel::Medium,
            required_checks: vec![],
            confidence: 0.9,
            evidence: vec![],
            depends_on: None,
            missing_inputs: None,
            dry_run: None,
        };
        
        let decision = policy.evaluate(&action);
        
        assert_eq!(
            decision.decision, 
            PolicyDecisionType::Deny,
            "Write action '{}' should be denied",
            action_type
        );
    }
}
