//! projects/products/unstable/rust_language/backend/src/engine/tests/rhl_engine.rs
use crate::{
    engine::RhlEngine,
    model::{ProjectConfig, SourceFile},
};

#[test]
fn test_compile_source() {
    let config = ProjectConfig::new("test_project".into(), "0.1.0".into(), "main.rhl".into());
    let engine = RhlEngine::from_config(config);
    let source = SourceFile::new("test.rhl".into(), "fn main() {}".into());

    let result = engine.compile_source(&source);
    assert!(result.is_ok());
}
