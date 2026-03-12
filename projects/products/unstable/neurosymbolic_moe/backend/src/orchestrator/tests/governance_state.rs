use crate::orchestrator::GovernanceState;

#[test]
fn ensure_checksum_populates_and_verifies() {
    let mut state = GovernanceState::from_components(3, None, None, None);
    state.state_checksum.clear();
    state.ensure_checksum();
    assert!(state.verify_checksum());
}

#[test]
fn tampered_checksum_fails_verification() {
    let mut state = GovernanceState::from_components(5, None, None, None);
    state.state_checksum = "tampered".to_string();
    assert!(!state.verify_checksum());
}
