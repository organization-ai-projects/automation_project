// projects/libraries/symbolic/src/tests/symbolic_solver.rs
use super::test_helpers::TestResult;
use crate::symbolic_error::SymbolicError;
use crate::symbolic_solver::SymbolicSolver;

#[test]
fn test_solver_initialization() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let result = solver.solve("fn main() {}", "analysis", None)?;
    assert_eq!(result.output, "Analysis successful");
    Ok(())
}

#[test]
fn test_solve_unknown_task_type() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let err = solver.solve("input", "unknown_task", None).unwrap_err();
    assert!(matches!(err, SymbolicError::AnalysisError(_)));
    assert!(err.to_string().contains("Unknown task type: unknown_task"));
    Ok(())
}

#[test]
fn test_solve_analysis_path() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let result = solver.solve("fn main() {}", "analysis", None)?;
    assert_eq!(result.output, "Analysis successful");
    assert!((result.confidence - 0.95).abs() < f64::EPSILON);
    assert_eq!(result.metadata.as_deref(), Some("Symbolic analysis"));
    Ok(())
}

#[test]
fn test_solve_generation_success() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let result = solver.solve("create struct MyType", "generation", None)?;
    assert!(result.output.contains("struct MyType"));
    assert_eq!(result.confidence, 0.9);
    assert_eq!(
        result.metadata.as_deref(),
        Some("Template-based generation")
    );
    Ok(())
}

#[test]
fn test_solve_generation_no_template() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let err = solver
        .solve("nonsense prompt", "generation", None)
        .unwrap_err();
    assert!(matches!(err, SymbolicError::GenerationError(_)));
    Ok(())
}

#[test]
fn test_solve_linting_path() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let result = solver.solve("fn main() {}", "linting", None)?;
    assert!(result.output.contains("Found 2 issues"));
    assert_eq!(result.metadata.as_deref(), Some("2 issues"));
    Ok(())
}

#[test]
fn test_solve_documentation_path() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let result = solver.solve("fn main() {}", "documentation", None)?;
    assert_eq!(result.output, "Documentation generated");
    assert_eq!(result.metadata.as_deref(), Some("Generated documentation"));
    Ok(())
}

#[test]
fn test_solve_refactor_requires_instruction() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let err = solver
        .solve("struct Foo {}", "refactoring", None)
        .unwrap_err();
    assert!(matches!(err, SymbolicError::GenerationError(_)));
    assert!(err.to_string().contains("Refactoring requires instruction"));
    Ok(())
}

#[test]
fn test_solve_refactor_success() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let code = "struct Foo {\n    a: i32,\n}\n";
    let result = solver.solve(code, "refactoring", Some("add debug derive"))?;
    assert!(result.output.contains("#[derive(Debug)]"));
    assert!(result.metadata.is_some());
    assert_eq!(result.confidence, 0.85);
    Ok(())
}

#[test]
fn test_solve_refactor_no_applicable_rules() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let code = "struct Foo {\n    a: i32,\n}\n";
    let err = solver
        .solve(code, "refactoring", Some("non matching instruction"))
        .unwrap_err();
    assert!(matches!(err, SymbolicError::GenerationError(_)));
    assert!(err.to_string().contains("No applicable refactoring rules"));
    Ok(())
}

#[test]
fn test_validate_code_empty_invalid() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let validation = solver.validate_code("")?;
    assert!(!validation.is_valid);
    assert!(validation.errors.iter().any(|e| e == "Code is empty"));
    Ok(())
}

#[test]
fn test_validate_code_valid() -> TestResult {
    let solver = SymbolicSolver::new()?;
    let validation = solver.validate_code("fn main() {}")?;
    assert!(validation.is_valid);
    Ok(())
}
