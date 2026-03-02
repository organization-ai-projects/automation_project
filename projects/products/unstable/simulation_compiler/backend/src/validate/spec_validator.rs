// projects/products/unstable/simulation_compiler/backend/src/validate/spec_validator.rs
use crate::diagnostics::error::CompilerError;
use crate::dsl::ast::Ast;

pub struct SpecValidator;

impl SpecValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, ast: &Ast) -> Result<(), CompilerError> {
        for comp in &ast.components {
            if comp.name.is_empty() {
                return Err(CompilerError::Validation(
                    "component name must not be empty".to_string(),
                ));
            }
        }
        for sys in &ast.systems {
            if sys.name.is_empty() {
                return Err(CompilerError::Validation(
                    "system name must not be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::parser::Parser;

    #[test]
    fn valid_ast_passes() {
        let mut p = Parser::new("component Sensor { value: u32 }");
        let ast = p.parse().unwrap();
        let v = SpecValidator::new();
        assert!(v.validate(&ast).is_ok());
    }
}
