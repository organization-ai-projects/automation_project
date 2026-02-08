// projects/libraries/ast_core/tests/integration/duplicate_keys.rs
use crate::helpers::assert_error_matches;
use ast_core::{AstKey, AstKind, AstNode, ValidateLimits};

#[test]
fn test_integration_duplicate_keys() {
    let node = AstNode {
        kind: AstKind::Object(vec![
            (
                AstKey::String("key1".to_string()),
                AstNode {
                    kind: AstKind::Null,
                    meta: Default::default(),
                },
            ),
            (
                AstKey::String("key1".to_string()),
                AstNode {
                    kind: AstKind::Null,
                    meta: Default::default(),
                },
            ),
        ]),
        meta: Default::default(),
    };

    let limits = ValidateLimits::default();
    let result = node.validate_with(&limits);
    assert_error_matches(result, "Duplicate key found: key1");
}
