use auto_manager_ai::{Action, ActionStatus, ActionTarget, Policy, PolicyDecisionType, RiskLevel};

#[test]
fn test_confidence_threshold_default() {
    let policy = Policy::default();
    
    // Default threshold should be 0.6
    assert_eq!(policy.min_confidence, 0.6);
}

#[test]
fn test_confidence_below_threshold_denied() {
    let policy = Policy::default();
    
    // Create action with confidence below threshold
    let action = Action {
        id: "confidence_test_1".to_string(),
        action_type: "analyze_repository".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "Low confidence test".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.5, // Below 0.6 threshold
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    assert_eq!(decision.decision, PolicyDecisionType::Deny);
    assert!(decision.reason.contains("Confidence"));
}

#[test]
fn test_confidence_at_threshold_allowed() {
    let policy = Policy::default();
    
    // Create action with confidence at threshold
    let action = Action {
        id: "confidence_test_2".to_string(),
        action_type: "analyze_repository".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "At threshold test".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.6, // At threshold
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    // Should be allowed (read-only action at threshold)
    assert_eq!(decision.decision, PolicyDecisionType::Allow);
}

#[test]
fn test_confidence_above_threshold_allowed() {
    let policy = Policy::default();
    
    // Create action with high confidence
    let action = Action {
        id: "confidence_test_3".to_string(),
        action_type: "analyze_repository".to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "High confidence test".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence: 0.95, // Well above threshold
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    };
    
    let decision = policy.evaluate(&action);
    
    assert_eq!(decision.decision, PolicyDecisionType::Allow);
}

#[test]
fn test_confidence_range_validation() {
    let policy = Policy::default();
    
    // Test various confidence levels
    let test_cases = vec![
        (0.0, false),  // Should be denied
        (0.3, false),  // Should be denied
        (0.59, false), // Should be denied (just below)
        (0.6, true),   // Should be allowed (at threshold)
        (0.7, true),   // Should be allowed
        (0.9, true),   // Should be allowed
        (1.0, true),   // Should be allowed
    ];
    
    for (confidence, should_allow) in test_cases {
        let action = Action {
            id: format!("confidence_test_{}", confidence),
            action_type: "analyze_repository".to_string(),
            status: ActionStatus::Proposed,
            target: ActionTarget::Repo {
                reference: "test/repo".to_string(),
            },
            justification: format!("Test with confidence {}", confidence),
            risk_level: RiskLevel::Low,
            required_checks: vec![],
            confidence,
            evidence: vec![],
            depends_on: None,
            missing_inputs: None,
            dry_run: None,
        };
        
        let decision = policy.evaluate(&action);
        
        if should_allow {
            assert_eq!(
                decision.decision,
                PolicyDecisionType::Allow,
                "Confidence {} should be allowed",
                confidence
            );
        } else {
            assert_eq!(
                decision.decision,
                PolicyDecisionType::Deny,
                "Confidence {} should be denied",
                confidence
            );
        }
    }
}
