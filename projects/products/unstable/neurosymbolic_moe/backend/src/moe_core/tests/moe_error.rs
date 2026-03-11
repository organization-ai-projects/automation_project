use crate::moe_core::{ExpertError, MoeError};

#[test]
fn moe_error_from_expert_error() {
    let expert_error = ExpertError::ExecutionFailed("boom".to_string());
    let moe_error = MoeError::from(expert_error);
    assert!(matches!(moe_error, MoeError::ExpertError(_)));
}
