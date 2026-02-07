use super::test_helpers::{
    TestResult, assert_invalid, assert_valid, assert_warn_contains, create_strict_validator,
    create_validator,
};
use crate::validator::CodeValidator;

#[test]
fn test_validator_creation() {
    let validator = CodeValidator::new();
    assert!(validator.is_ok());
}

#[test]
fn test_valid_code() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;

    let validation = validator.validate(code)?;
    assert_valid(&validation);
    Ok(())
}

#[test]
fn test_invalid_syntax() -> TestResult {
    let validator = create_validator();
    let code = "fn main( {"; // Missing closing paren

    let validation = validator.validate(code)?;
    assert_invalid(&validation);
    Ok(())
}

#[test]
fn test_empty_code() -> TestResult {
    let validator = create_validator();
    let validation = validator.validate("")?;
    assert_invalid(&validation);
    Ok(())
}

#[test]
fn test_warnings() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            println!("test");
            let x = Some(5).expect ("Option was None");
            todo!();
            let _ = try ! (Ok::<_, ()>(()));
        }
    "#;

    let validation = validator.validate(code)?;
    assert_valid(&validation);
    assert_warn_contains(&validation, "expect");
    assert_warn_contains(&validation, "try");
    Ok(())
}

#[test]
fn test_suggest_fix_semicolon() {
    let validator = create_validator();
    let code = "let x = 5";

    let errors = vec!["expected `;`".to_string()];
    let fix = validator.suggest_fix(code, &errors);

    assert!(fix.is_some());
    assert!(fix.expect("Fix was None").contains(';'));
}

#[test]
fn test_suggest_fix_delimiters() {
    let validator = create_validator();
    let code = "fn main() { println!(\"test\"";

    let errors = vec!["unclosed delimiter".to_string()];
    let fix = validator.suggest_fix(code, &errors);

    assert!(fix.is_some());
}

#[test]
fn test_strict_mode() -> TestResult {
    let validator = create_strict_validator();

    // Test with code that has semantic issues
    let code = r#"
        fn main() {
            let unused_var = 42;
        }
    "#;

    let validation = validator.validate(code)?;

    // In strict mode, unused variables should be reported as errors
    assert_warn_contains(&validation, "unused_var");
    assert_warn_contains(&validation, "error");
    Ok(())
}

#[test]
fn test_validate_syntax_only() -> TestResult {
    let validator = create_validator();
    let code = "fn test() {}";

    let validation = validator.validate_syntax(code)?;
    assert_valid(&validation);
    Ok(())
}

#[test]
fn test_validate_syntax_only_empty_code() -> TestResult {
    let validator = create_validator();
    let validation = validator.validate_syntax("")?;
    assert_invalid(&validation);
    Ok(())
}

#[test]
fn test_suggest_fix_no_match() {
    let validator = create_validator();
    let code = "fn main() {}";

    let errors = vec!["some other error".to_string()];
    let fix = validator.suggest_fix(code, &errors);

    assert!(fix.is_none());
}
