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
            node.get("name").expect("name field").as_string(),
            Some("test")
        );
        assert_eq!(
            node.get("value")
                .expect("value field")
                .as_number()
                .expect("value number")
                .as_i64(),
            Some(42)
        );
        assert_eq!(
            node.get("active").expect("active field").as_bool(),
            Some(true)
        );
    }
}
