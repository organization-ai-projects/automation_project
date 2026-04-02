//! projects/products/unstable/rust_language/backend/src/ai_assist/tests/code_improver.rs
use crate::ai_assist::code_improver::CodeImprover;
use crate::diagnostics::Error;

#[test]
fn test_improve_rhl_code() -> Result<(), Error> {
    let mut improver = CodeImprover::new()?;
    let source_code = "fn main() { let x = 42; }";
    let improved_code = improver.improve_rhl_code(source_code)?;
    assert!(improved_code.contains("fn main"));
    Ok(())
}

#[test]
fn test_optimize_transpiled_rust() -> Result<(), Error> {
    let mut improver = CodeImprover::new()?;
    let rust_code = "fn main() { let x = 42; println!(\"{}\", x); }";
    let optimized_code = improver.optimize_transpiled_rust(rust_code)?;
    assert!(optimized_code.contains("println!"));
    Ok(())
}
