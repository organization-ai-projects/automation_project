// projects/libraries/ast_core/src/tests/ast_builder_tests.rs
#[cfg(test)]
mod tests {
    use crate::AstBuilder;

    #[test]
    fn test_builder_basics() {
        let node = AstBuilder::object(vec![
            ("name", AstBuilder::string("test")),
            ("value", AstBuilder::int(42)),
            ("active", AstBuilder::bool(true)),
        ]);

        assert!(node.is_object());
        assert_eq!(
            node.get("name").expect("Missing 'name'").as_string(),
            Some("test")
        );
        assert_eq!(
            node.get("value")
                .expect("Missing 'value'")
                .as_number()
                .expect("'value' is not a number")
                .as_i64(),
            Some(42)
        );
        assert_eq!(
            node.get("active").expect("Missing 'active'").as_bool(),
            Some(true)
        );
    }
}
