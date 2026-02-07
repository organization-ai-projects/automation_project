use super::test_helpers::{
    assert_min_warnings, assert_valid, assert_warn_contains, assert_warn_contains_all,
    assert_warn_not_contains, create_strict_validator, create_validator, TestResult,
};


#[test]
fn test_unused_variable_detection() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let unused_var = 42;
            let used_var = 10;
            println!("{}", used_var);
        }
    "#;

    let validation = validator.validate(code)?;
    assert_valid(&validation);

    // Should warn about unused_var
    assert_warn_contains_all(&validation, &["unused_var", "never used"]);

    // Should not warn about used_var
    assert_warn_not_contains(&validation, "used_var");
    Ok(())
}

#[test]
fn test_unused_variable_with_underscore_prefix() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let _unused = 42;
        }
    "#;

    let validation = validator.validate(code)?;

    // Variables starting with underscore should not trigger warnings
    assert_warn_not_contains(&validation, "_unused");
    Ok(())
}

#[test]
fn test_unused_import_detection() -> TestResult {
    let validator = create_validator();
    let code = r#"
        use std::collections::HashMap;
        use std::vec::Vec;
        
        fn main() {
            let map = HashMap::new();
        }
    "#;

    let validation = validator.validate(code)?;

    // Should warn about unused Vec import
    assert_warn_contains(&validation, "Vec");

    // Should not warn about used HashMap
    assert_warn_not_contains(&validation, "HashMap");
    Ok(())
}

#[test]
fn test_dead_code_detection() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            return;
            let x = 42;
        }
    "#;

    let validation = validator.validate(code)?;

    // Should warn about dead code after return
    assert_warn_contains(&validation, "Unreachable");
    Ok(())
}

#[test]
fn test_dead_code_with_break() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            loop {
                break;
                let x = 42;
            }
        }
    "#;

    let validation = validator.validate(code)?;

    // Should warn about dead code after break
    assert_warn_contains(&validation, "Unreachable");
    Ok(())
}

#[test]
fn test_strict_mode_unused_variable() -> TestResult {
    let validator = create_strict_validator();

    let code = r#"
        fn main() {
            let unused = 42;
        }
    "#;

    let validation = validator.validate(code)?;

    // In strict mode, semantic issues should be reported as errors
    assert_warn_contains_all(&validation, &["error", "unused"]);
    Ok(())
}

#[test]
fn test_strict_mode_unused_import() -> TestResult {
    let validator = create_strict_validator();

    let code = r#"
        use std::collections::HashMap;
        
        fn main() {}
    "#;

    let validation = validator.validate(code)?;

    // In strict mode, unused imports should be reported as errors
    assert_warn_contains_all(&validation, &["error", "HashMap"]);
    Ok(())
}

#[test]
fn test_strict_mode_dead_code() -> TestResult {
    let validator = create_strict_validator();

    let code = r#"
        fn main() {
            return;
            let x = 1;
        }
    "#;

    let validation = validator.validate(code)?;

    // In strict mode, dead code should be reported as errors
    assert_warn_contains(&validation, "error");
    assert_warn_contains(&validation, "Unreachable");
    Ok(())
}

#[test]
fn test_multiple_semantic_issues() -> TestResult {
    let validator = create_validator();
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let unused_var = 42;
            return;
            let dead_code_var = 1;
        }
    "#;

    let validation = validator.validate(code)?;

    // Should have multiple warnings
    assert_min_warnings(&validation, 2);
    Ok(())
}

#[test]
fn test_no_semantic_issues() -> TestResult {
    let validator = create_validator();
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let map = HashMap::new();
            let x = 42;
            println!("{} {:?}", x, map);
        }
    "#;

    let validation = validator.validate(code)?;

    // Should not have semantic warnings (may have other warnings like println!)
    let semantic_warnings: Vec<_> = validation
        .warnings
        .iter()
        .filter(|w| w.contains("unused") || w.contains("Unreachable") || w.contains("dead"))
        .collect();

    assert!(
        semantic_warnings.is_empty(),
        "Should not have semantic warnings for valid code"
    );
    Ok(())
}

#[test]
fn test_dead_code_in_loop_doesnt_propagate() -> TestResult {
    let validator = create_validator();
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

    let validation = validator.validate(code)?;

    // Should have dead code warning for x but not for y
    assert_warn_contains(&validation, "Unreachable");

    // y should not be reported as unused (it's used in println!)
    assert_warn_not_contains(&validation, "'y'");
    Ok(())
}

#[test]
fn test_variable_detection_with_similar_names() -> TestResult {
    let validator = create_validator();
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let map = HashMap::new();
            println!("{:?}", map);
        }
    "#;

    let validation = validator.validate(code)?;

    // map should not be reported as unused (it's used in println!)
    assert_warn_not_contains(&validation, "'map'");
    Ok(())
}

#[test]
fn test_tuple_destructuring_detection() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let (a, b) = (1, 2);
            let (unused_x, used_y) = (3, 4);
            println!("{} {}", a, b);
            println!("{}", used_y);
        }
    "#;

    let validation = validator.validate(code)?;

    // Should warn about unused_x but not about a, b, or used_y
    assert_warn_contains_all(&validation, &["unused_x", "never used"]);
    assert_warn_not_contains(&validation, "'a'");
    assert_warn_not_contains(&validation, "'b'");
    assert_warn_not_contains(&validation, "used_y");
    Ok(())
}

#[test]
fn test_if_match_dead_code_detection() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let x = 5;
            if x > 0 {
                return;
            } else {
                return;
            }
            let dead = "unreachable";
        }
    "#;

    let validation = validator.validate(code)?;

    // Should detect dead code after if-else that both return
    assert_warn_contains(&validation, "Unreachable");
    Ok(())
}

#[test]
fn test_import_in_type_position() -> TestResult {
    let validator = create_validator();
    let code = r#"
        use std::collections::HashMap;
        
        fn process(map: HashMap<String, i32>) -> usize {
            map.len()
        }
        
        fn main() {
            let m = HashMap::new();
            println!("{}", process(m));
        }
    "#;

    let validation = validator.validate(code)?;

    // HashMap should not be reported as unused (it's used in type position)
    assert_warn_not_contains(&validation, "HashMap");
    Ok(())
}

#[test]
fn test_type_inconsistency_detection() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let x: i32 = "hello";
        }
    "#;

    let validation = validator.validate(code)?;

    // Should warn about type mismatch
    assert_warn_contains_all(&validation, &["Type mismatch", "'x'"]);
    Ok(())
}

#[test]
fn test_type_inconsistency_in_assignment() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let mut x: i32 = 42;
            x = "not an integer";
        }
    "#;

    let validation = validator.validate(code)?;

    // Should warn about type mismatch in assignment
    assert_warn_contains_all(&validation, &["Type mismatch", "'x'"]);
    Ok(())
}

#[test]
fn test_compatible_types_no_warning() -> TestResult {
    let validator = create_validator();
    let code = r#"
        fn main() {
            let x: i32 = 42;
            let y: i64 = 100;
        }
    "#;

    let validation = validator.validate(code)?;

    // Should not warn about compatible integer types
    let type_warnings: Vec<_> = validation
        .warnings
        .iter()
        .filter(|w| w.contains("Type mismatch"))
        .collect();

    assert!(
        type_warnings.is_empty(),
        "Should not warn about compatible integer types"
    );
    Ok(())
}

#[test]
fn test_strict_mode_type_inconsistency() -> TestResult {
    let validator = create_strict_validator();

    let code = r#"
        fn main() {
            let x: bool = 123;
        }
    "#;

    let validation = validator.validate(code)?;

    // In strict mode, type inconsistencies should be reported as errors
    assert_warn_contains(&validation, "error");
    assert_warn_contains(&validation, "Type mismatch");
    Ok(())
}

#[test]
fn test_multiple_validation_types() -> TestResult {
    let validator = create_validator();
    let code = r#"
        use std::collections::HashMap;
        
        fn main() {
            let unused_var = 42;
            let x: bool = 10;
            return;
            let dead = 1;
        }
    "#;

    let validation = validator.validate(code)?;

    // Should have all types of warnings
    assert_warn_contains(&validation, "unused");
    assert_warn_contains(&validation, "Type mismatch");
    assert_warn_contains(&validation, "Unreachable");
    Ok(())
}
