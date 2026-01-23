#[cfg(test)]
mod tests {
    use crate::rules::{RulesEngine, RulesError};

    #[test]
    fn test_rules_engine_creation() {
        let engine = RulesEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_struct_generation() {
        let engine = RulesEngine::new().unwrap();
        let result = engine.generate("create struct User with name and email", None);

        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("struct User"));
        assert!(code.contains("pub name"));
        assert!(code.contains("pub email"));
    }

    #[test]
    fn test_enum_generation() {
        let engine = RulesEngine::new().unwrap();
        let result = engine.generate("create enum Status", None);

        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("enum Status"));
    }

    #[test]
    fn test_function_generation() {
        let engine = RulesEngine::new().unwrap();
        let result = engine.generate("create function calculate", None);

        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("fn calculate"));
        assert!(code.contains("todo!()"));
    }

    #[test]
    fn test_match_confidence() {
        let engine = RulesEngine::new().unwrap();

        let confidence = engine.match_confidence("create struct User");
        assert!(confidence > 0.8);

        let no_match = engine.match_confidence("something random");
        assert_eq!(no_match, 0.0);
    }

    #[test]
    fn test_extract_name_via_public_interface() {
        let engine = RulesEngine::new().unwrap();

        let result = engine.generate("create struct User", None);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("struct User"));
    }

    #[test]
    fn test_extract_fields_via_public_interface() {
        let engine = RulesEngine::new().unwrap();

        let result = engine.generate("create struct User", None);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("struct User"));
    }

    #[test]
    fn test_refactoring() {
        let engine = RulesEngine::new().unwrap();

        let code = "struct User {\n    name: String,\n}";
        let result = engine.apply_refactoring(code, "add debug derive");

        assert!(result.is_ok());
        let refactored = result.unwrap();
        assert!(refactored.code.contains("#[derive(Debug)]"));
        assert!(!refactored.changes_applied.is_empty());
    }

    #[test]
    fn test_template_not_found() {
        let engine = RulesEngine::new().unwrap();
        let result = engine.generate("something completely random", None);

        assert!(result.is_err());
        assert!(matches!(result, Err(RulesError::TemplateNotFound(_))));
    }
}
