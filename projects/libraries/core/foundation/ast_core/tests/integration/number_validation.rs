// projects/libraries/ast_core/tests/integration/number_validation.rs
use ast_core::{AstKind, AstNode, Number, ValidateLimits};

#[test]
fn test_integration_number_validation() {
    let node = AstNode {
        kind: AstKind::Number(Number::Int(42)),
        meta: Default::default(),
    };

    let limits = ValidateLimits::default();
    let result = node.validate_with(&limits);
    assert!(result.is_ok());
}
