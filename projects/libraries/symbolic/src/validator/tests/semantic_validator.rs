use crate::validator::CodeValidator;

#[test]
fn test_unused_variable_detection() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            let unused_var = 42;
            let used_var = 10;
            println!("{}", used_var);
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    assert!(validation.is_valid);
    
    // Should warn about unused_var (check for exact variable name in quotes)
    let has_unused_var_warning = validation
        .warnings
        .iter()
        .any(|w| w.contains("'unused_var'") && w.contains("never used"));
    
    assert!(
        has_unused_var_warning,
        "Expected warning for unused variable"
    );
    
    // Should not warn about used_var (check for exact variable name in quotes)
    let has_used_var_warning = validation
        .warnings
        .iter()
        .any(|w| w.contains("'used_var'") && w.contains("never used"));
    
    assert!(
        !has_used_var_warning,
        "Should not warn about used variable"
    );
}

#[test]
fn test_unused_variable_with_underscore_prefix() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            let _unused = 42;
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Variables starting with underscore should not trigger warnings
    assert!(
        !validation
            .warnings
            .iter()
            .any(|w| w.contains("_unused")),
        "Should not warn about variables prefixed with underscore"
    );
}

#[test]
fn test_unused_import_detection() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        use std::collections::HashMap;
        use std::vec::Vec;
        
        fn main() {
            let map = HashMap::new();
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Should warn about unused Vec import
    assert!(
        validation
            .warnings
            .iter()
            .any(|w| w.contains("Vec")),
        "Expected warning for unused import Vec"
    );
    
    // Should not warn about used HashMap
    assert!(
        !validation
            .warnings
            .iter()
            .any(|w| w.contains("HashMap") && w.contains("never used")),
        "Should not warn about used HashMap import"
    );
}

#[test]
fn test_dead_code_detection() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            return;
            let x = 42;
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Should warn about dead code after return
    assert!(
        validation
            .warnings
            .iter()
            .any(|w| w.contains("Unreachable") || w.contains("dead")),
        "Expected warning for dead code after return"
    );
}

#[test]
fn test_dead_code_with_break() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            loop {
                break;
                let x = 42;
            }
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Should warn about dead code after break
    assert!(
        validation
            .warnings
            .iter()
            .any(|w| w.contains("Unreachable") || w.contains("dead")),
        "Expected warning for dead code after break"
    );
}

#[test]
fn test_strict_mode_unused_variable() {
    let validator = CodeValidator::new()
        .expect("Failed to create CodeValidator")
        .with_strict_mode(true);
    
    let code = r#"
        fn main() {
            let unused = 42;
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // In strict mode, semantic issues should be reported as errors
    assert!(
        validation
            .warnings
            .iter()
            .any(|w| w.contains("error") && w.contains("unused")),
        "Expected error-level warning in strict mode"
    );
}

#[test]
fn test_strict_mode_unused_import() {
    let validator = CodeValidator::new()
        .expect("Failed to create CodeValidator")
        .with_strict_mode(true);
    
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {}
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // In strict mode, unused imports should be reported as errors
    assert!(
        validation
            .warnings
            .iter()
            .any(|w| w.contains("error") && w.contains("HashMap")),
        "Expected error-level warning for unused import in strict mode"
    );
}

#[test]
fn test_strict_mode_dead_code() {
    let validator = CodeValidator::new()
        .expect("Failed to create CodeValidator")
        .with_strict_mode(true);
    
    let code = r#"
        fn main() {
            return;
            let x = 1;
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // In strict mode, dead code should be reported as errors
    assert!(
        validation
            .warnings
            .iter()
            .any(|w| w.contains("error") && (w.contains("Unreachable") || w.contains("dead"))),
        "Expected error-level warning for dead code in strict mode"
    );
}

#[test]
fn test_multiple_semantic_issues() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let unused_var = 42;
            return;
            let dead_code_var = 1;
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Should have multiple warnings
    assert!(
        validation.warnings.len() >= 2,
        "Expected multiple semantic warnings"
    );
}

#[test]
fn test_no_semantic_issues() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let map = HashMap::new();
            let x = 42;
            println!("{} {:?}", x, map);
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Should not have semantic warnings (may have other warnings like println!)
    let semantic_warnings: Vec<_> = validation
        .warnings
        .iter()
        .filter(|w| {
            w.contains("unused") || w.contains("Unreachable") || w.contains("dead")
        })
        .collect();
    
    assert!(
        semantic_warnings.is_empty(),
        "Should not have semantic warnings for valid code"
    );
}

#[test]
fn test_dead_code_in_loop_doesnt_propagate() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        fn main() {
            loop {
                break;
                let x = 1;
            }
            let y = 2;
            println!("{}", y);
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // Should have dead code warning for x but not for y
    let dead_code_warnings: Vec<_> = validation
        .warnings
        .iter()
        .filter(|w| w.contains("Unreachable") || w.contains("dead"))
        .collect();
    
    // Should have at least one warning for the dead code inside the loop
    assert!(
        !dead_code_warnings.is_empty(),
        "Should detect dead code in loop"
    );
    
    // y should not be reported as unused (it's used in println!)
    assert!(
        !validation
            .warnings
            .iter()
            .any(|w| w.contains("'y'") && w.contains("never used")),
        "Should not report y as unused"
    );
}

#[test]
fn test_variable_detection_with_similar_names() {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let map = HashMap::new();
            println!("{:?}", map);
        }
    "#;

    let result = validator.validate(code);
    assert!(result.is_ok());
    let validation = result.expect("Validation failed");
    
    // map should not be reported as unused (it's used in println!)
    assert!(
        !validation
            .warnings
            .iter()
            .any(|w| w.contains("'map'") && w.contains("never used")),
        "Should not report map as unused despite being substring of HashMap"
    );
}
