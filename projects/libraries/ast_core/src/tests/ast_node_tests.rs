// projects/libraries/ast_core/src/tests/ast_node_tests.rs
use crate::{AstErrorKind, AstKey, AstKind, AstNode};

#[test]
fn test_duplicate_key_detection() {
    let node = AstNode::new(AstKind::Object(vec![
        (
            AstKey::String("key".to_string()),
            AstNode::new(AstKind::Number(1.into())),
        ),
        (
            AstKey::String("key".to_string()),
            AstNode::new(AstKind::Number(2.into())),
        ),
    ]));

    let result = node.validate();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, AstErrorKind::DuplicateKey { key } if key == "key"));
}
