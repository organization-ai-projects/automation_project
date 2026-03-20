use crate::moe_core::ExpertError;

#[test]
fn expert_error_variants_are_constructible() {
    let error = ExpertError::InvalidInput("bad".to_string());
    assert!(matches!(error, ExpertError::InvalidInput(_)));
}
