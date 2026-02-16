use super::test_helpers::TestResult;
use crate::rules::{RulesEngine, RulesError};

/// Helper to create a RulesEngine, panicking with helpful message if creation fails.
fn create_engine() -> RulesEngine {
    RulesEngine::new().expect("Failed to create RulesEngine")
}

#[test]
fn test_rules_engine_creation() {
    let engine = RulesEngine::new();
    assert!(engine.is_ok());
}

#[test]
fn test_struct_generation() -> TestResult {
    let engine = create_engine();
    let code = engine.generate("create struct User with name and email", None)?;

    assert!(code.contains("struct User"));
    assert!(code.contains("pub name"));
    assert!(code.contains("pub email"));
    Ok(())
}

#[test]
fn test_enum_generation() -> TestResult {
    let engine = create_engine();
    let code = engine.generate("create enum Status", None)?;

    assert!(code.contains("enum Status"));
    Ok(())
}

#[test]
fn test_function_generation() -> TestResult {
    let engine = create_engine();
    let code = engine.generate("create function calculate", None)?;

    assert!(code.contains("fn calculate"));
    assert!(code.contains("todo!()"));
    Ok(())
}

#[test]
fn test_match_confidence() {
    let engine = create_engine();

    let confidence = engine.match_confidence("create struct User");
    assert!(confidence > 0.8);

    let no_match = engine.match_confidence("something random");
    assert_eq!(no_match, 0.0);
}

#[test]
fn test_extract_name_via_public_interface() -> TestResult {
    let engine = create_engine();

    let code = engine.generate("create struct User", None)?;
    assert!(code.contains("struct User"));
    Ok(())
}

#[test]
fn test_extract_fields_via_public_interface() -> TestResult {
    let engine = create_engine();

    let code = engine.generate("create struct User", None)?;
    assert!(code.contains("struct User"));
    assert!(!code.contains("TODO"));
    assert!(code.contains("pub field1"));
    Ok(())
}

#[test]
fn test_trait_generation_has_default_method_without_todo() -> TestResult {
    let engine = create_engine();
    let code = engine.generate("create trait Runner", None)?;

    assert!(code.contains("trait Runner"));
    assert!(code.contains("fn execute(&self);"));
    assert!(!code.contains("TODO"));
    Ok(())
}

#[test]
fn test_trait_generation_extracts_methods_from_prompt() -> TestResult {
    let engine = create_engine();
    let code = engine.generate("create trait Runner with methods: start, stop", None)?;

    assert!(code.contains("fn start(&self);"));
    assert!(code.contains("fn stop(&self);"));
    assert!(!code.contains("TODO"));
    Ok(())
}

#[test]
fn test_refactoring() -> TestResult {
    let engine = create_engine();

    let code = "struct User {\n    name: String,\n}";
    let refactored = engine.apply_refactoring(code, "add debug derive")?;

    assert!(refactored.code.contains("#[derive(Debug)]"));
    assert!(!refactored.changes_applied.is_empty());
    Ok(())
}

#[test]
fn test_template_not_found() {
    let engine = create_engine();
    let result = engine.generate("something completely random", None);

    assert!(result.is_err());
    assert!(matches!(result, Err(RulesError::TemplateNotFound(_))));
}
