// projects/libraries/ast_core/src/tests/ast_builder_tests.rs
use super::test_helpers::{assert_bool_key, assert_int_key, assert_string_key};
use crate::AstBuilder;

#[test]
fn test_builder_basics() {
    let node = AstBuilder::object(vec![
        ("name", AstBuilder::string("test")),
        ("value", AstBuilder::int(42)),
        ("active", AstBuilder::bool(true)),
    ]);

    assert!(node.is_object());
    assert_string_key(&node, "name", "test");
    assert_int_key(&node, "value", 42);
    assert_bool_key(&node, "active", true);
}
