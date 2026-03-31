use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::protection_result::ProtectionResult;
use crate::moe_protect::threat_id::ThreatId;

#[test]
fn protection_result_holds_fields() {
    let result = ProtectionResult {
        threat_id: ThreatId::from_str("test-001"),
        verdicts: Vec::new(),
        final_action: ProtectionAction::Block,
        combined_confidence: 0.95,
        summary: "Test".to_string(),
    };
    assert_eq!(result.final_action, ProtectionAction::Block);
}
