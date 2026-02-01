use crate::validator::CodeValidator;

#[test]
fn test_validator_creation() {
    let validator = CodeValidator::new();
    assert!(validator.is_ok());
}

#[test]
fn test_valid_code() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    assert!(validation.is_valid);
}

#[test]
fn test_invalid_syntax() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = "fn main( {"; // Missing closing paren

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    assert!(!validation.is_valid);
    assert!(!validation.errors.is_empty());
}

#[test]
fn test_empty_code() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let result = validator.validate("");

    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    assert!(!validation.is_valid);
}

#[test]
fn test_warnings() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            println!("test");
            let x = Some(5).expect ("Option was None");
            todo!();
            let _ = try ! (Ok::<_, ()>(()));
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    assert!(validation.is_valid);
    assert!(
        validation
            .warnings
            .iter()
            .any(|warning| warning.contains("expect calls"))
    );
    assert!(
        validation
            .warnings
            .iter()
            .any(|warning| warning.contains("deprecated try! macro"))
    );
}

#[test]
fn test_suggest_fix_semicolon() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = "let x = 5";

    let errors = vec!["expected `;`".to_string()];
    let fix = validator.suggest_fix(code, &errors);

    assert!(fix.is_some());
    assert!(fix.expect("Fix was None").contains(';'));
}

#[test]
fn test_suggest_fix_delimiters() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = "fn main() { println!(\"test\"";

    let errors = vec!["unclosed delimiter".to_string()];
    let fix = validator.suggest_fix(code, &errors);

    assert!(fix.is_some());
}

#[test]
fn test_strict_mode() {
    let validator = CodeValidator::new()
        .expect("Failed to create CodeValidator")
        .with_strict_mode(true);
    let result = validator.validate("fn main() {}");
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    assert!(
        validation
            .warnings
            .iter()
            .any(|warning| warning.contains("Strict mode enabled"))
    );
}

#[test]
fn test_validate_syntax_only() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = "fn test() {}";

    let result = validator.validate_syntax(code);
    assert!(result.is_ok());
    assert!(result.expect("Validation failed").is_valid);
}
