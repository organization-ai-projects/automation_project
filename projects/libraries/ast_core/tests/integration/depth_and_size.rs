use ast_core::{AstKind, AstNode, ValidateLimits};

#[test]
fn test_integration_depth_and_size() {
    let node = AstNode {
        kind: AstKind::Array(vec![
            AstNode {
                kind: AstKind::Array(vec![AstNode {
                    kind: AstKind::Null,
                    meta: Default::default(),
                }]),
                meta: Default::default(),
            },
            AstNode {
                kind: AstKind::Null,
                meta: Default::default(),
            },
        ]),
        meta: Default::default(),
    };

    let limits = ValidateLimits {
        max_depth: 2,
        max_size: 1,
    };

    let result = node.validate_with(&limits);
    assert!(result.is_err());
    let error_message = format!("{}", result.unwrap_err());
    assert!(
        error_message.contains("Exceeded maximum depth")
            || error_message.contains("Exceeded maximum size")
    );
}

#[test]
fn test_integration_max_depth() {
    let mut node = AstNode {
        kind: AstKind::Object(vec![]),
        meta: Default::default(),
    };

    // Create a deeply nested structure
    for _ in 0..11 {
        node = AstNode {
            kind: AstKind::Object(vec![("key".into(), node)]),
            meta: Default::default(),
        };
    }

    let limits = ValidateLimits {
        max_depth: 10,
        ..Default::default()
    };

    let result = node.validate_with(&limits);
    assert!(result.is_err());
    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("Exceeded maximum depth: 10 (got: 11)"));
}

#[test]
fn test_integration_max_size() {
    let node = AstNode {
        kind: AstKind::Object(vec![
            (
                "key1".into(),
                AstNode {
                    kind: AstKind::Null,
                    meta: Default::default(),
                },
            );
            101
        ]),
        meta: Default::default(),
    };

    let limits = ValidateLimits {
        max_size: 100,
        ..Default::default()
    };

    let result = node.validate_with(&limits);
    assert!(result.is_err());
    assert_eq!(
        format!("{}", result.unwrap_err()),
        "Exceeded maximum size for object: 100"
    );
}
