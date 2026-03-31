use crate::moe_protect::protection_action::ProtectionAction;

#[test]
fn protection_actions_are_distinct() {
    assert_ne!(ProtectionAction::Allow, ProtectionAction::Block);
    assert_ne!(ProtectionAction::Quarantine, ProtectionAction::Alert);
}
