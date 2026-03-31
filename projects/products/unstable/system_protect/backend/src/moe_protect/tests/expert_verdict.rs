use crate::moe_protect::expert_id::ExpertId;
use crate::moe_protect::expert_verdict::ExpertVerdict;
use crate::moe_protect::protection_action::ProtectionAction;

#[test]
fn verdict_new_sets_fields() {
    let verdict = ExpertVerdict::new(
        ExpertId::new("test"),
        ProtectionAction::Block,
        0.95,
        "test reasoning",
    );
    assert_eq!(verdict.expert_id, ExpertId::new("test"));
    assert_eq!(verdict.action, ProtectionAction::Block);
    assert!((verdict.confidence - 0.95).abs() < f64::EPSILON);
    assert_eq!(verdict.reasoning, "test reasoning");
}
