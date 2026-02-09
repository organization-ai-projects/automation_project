// projects/libraries/ast_core/src/tests/validate_limits_tests.rs
use crate::{AstBuilder, AstErrorKind, ValidateLimits};

#[test]
fn test_validation_with_path() {
    let node = AstBuilder::object(vec![(
        "outer",
        AstBuilder::object(vec![(
            "inner",
            AstBuilder::array(vec![
                AstBuilder::null(),
                AstBuilder::null(),
                AstBuilder::null(),
            ]),
        )]),
    )]);

    let limits = ValidateLimits {
        max_depth: 2,
        max_size: 100,
    };

    let result = node.validate_with(&limits);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, AstErrorKind::MaxDepth { .. }));
    // Path should point to where the error occurred
    assert!(!err.path.0.is_empty());
}
