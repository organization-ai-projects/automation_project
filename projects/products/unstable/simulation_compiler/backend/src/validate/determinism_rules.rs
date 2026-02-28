// projects/products/unstable/simulation_compiler/backend/src/validate/determinism_rules.rs
use crate::diagnostics::error::CompilerError;
use crate::dsl::ast::Ast;

/// Ruleset that rejects nondeterministic DSL patterns.
#[derive(Default)]
pub struct DeterminismRules;

const FORBIDDEN_TYPES: &[&str] = &["Instant", "SystemTime", "HashMap", "HashSet"];

impl DeterminismRules {
    pub fn check(&self, ast: &Ast) -> Result<(), CompilerError> {
        for comp in &ast.components {
            for field in &comp.fields {
                if FORBIDDEN_TYPES.contains(&field.ty.as_str()) {
                    return Err(CompilerError::Determinism(format!(
                        "component `{}` field `{}` uses forbidden type `{}`",
                        comp.name, field.name, field.ty
                    )));
                }
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
    fn rejects_instant_field() {
        let mut p = Parser::new("component Bad { ts: Instant }");
        let ast = p.parse().unwrap();
        let rules = DeterminismRules::default();
        assert!(rules.check(&ast).is_err());
    }

    #[test]
    fn allows_u32_field() {
        let mut p = Parser::new("component Good { x: u32 }");
        let ast = p.parse().unwrap();
        let rules = DeterminismRules::default();
        assert!(rules.check(&ast).is_ok());
    }
}
