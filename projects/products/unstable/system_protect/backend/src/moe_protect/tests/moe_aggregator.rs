use crate::moe_protect::expert_id::ExpertId;
use crate::moe_protect::expert_verdict::ExpertVerdict;
use crate::moe_protect::moe_aggregator::MoeAggregator;
use crate::moe_protect::protection_action::ProtectionAction;

#[test]
fn aggregator_returns_log_for_empty_verdicts() {
    let (action, confidence, _) = MoeAggregator::aggregate(&[]);
    assert_eq!(action, ProtectionAction::Log);
    assert!((confidence - 0.0).abs() < f64::EPSILON);
}

#[test]
fn aggregator_returns_single_verdict_directly() {
    let verdict = ExpertVerdict::new(
        ExpertId::new("av"),
        ProtectionAction::Block,
        0.95,
        "matched signature",
    );
    let (action, confidence, _) = MoeAggregator::aggregate(&[verdict]);
    assert_eq!(action, ProtectionAction::Block);
    assert!((confidence - 0.95).abs() < f64::EPSILON);
}

#[test]
fn aggregator_picks_highest_confidence_weighted_action() {
    let verdicts = vec![
        ExpertVerdict::new(ExpertId::new("av"), ProtectionAction::Block, 0.95, "virus"),
        ExpertVerdict::new(
            ExpertId::new("fw"),
            ProtectionAction::Block,
            0.9,
            "rule match",
        ),
        ExpertVerdict::new(
            ExpertId::new("sym"),
            ProtectionAction::Alert,
            0.7,
            "symbolic",
        ),
    ];
    let (action, _, _) = MoeAggregator::aggregate(&verdicts);
    assert_eq!(action, ProtectionAction::Block);
}
