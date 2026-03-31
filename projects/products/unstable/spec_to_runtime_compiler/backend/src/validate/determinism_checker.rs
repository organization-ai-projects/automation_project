use crate::diagnostics::error::CompilerError;
use crate::dsl::ast::SpecAst;

const FORBIDDEN_TYPES: &[&str] = &["Instant", "SystemTime", "HashMap", "HashSet"];

pub struct DeterminismChecker;

impl DeterminismChecker {
    pub fn check(&self, ast: &SpecAst) -> Result<(), CompilerError> {
        for state in &ast.states {
            for field in &state.fields {
                for forbidden in FORBIDDEN_TYPES {
                    if field.ty.contains(forbidden) {
                        return Err(CompilerError::Determinism(format!(
                            "state '{}' field '{}' uses non-deterministic type '{}'",
                            state.name, field.name, field.ty
                        )));
                    }
                }
            }
        }
        for t in &ast.transitions {
            for field in &t.guard_fields {
                for forbidden in FORBIDDEN_TYPES {
                    if field.ty.contains(forbidden) {
                        return Err(CompilerError::Determinism(format!(
                            "transition '{} -> {}' guard field '{}' uses non-deterministic type '{}'",
                            t.from, t.to, field.name, field.ty
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::ast::{FieldDef, StateNode};

    #[test]
    fn rejects_instant_field() {
        let ast = SpecAst {
            states: vec![StateNode {
                name: "Running".to_string(),
                fields: vec![FieldDef {
                    name: "started_at".to_string(),
                    ty: "Instant".to_string(),
                }],
            }],
            transitions: vec![],
            invariants: vec![],
        };
        let checker = DeterminismChecker;
        let result = checker.check(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Instant"));
    }

    #[test]
    fn allows_u32_field() {
        let ast = SpecAst {
            states: vec![StateNode {
                name: "Running".to_string(),
                fields: vec![FieldDef {
                    name: "tick".to_string(),
                    ty: "u32".to_string(),
                }],
            }],
            transitions: vec![],
            invariants: vec![],
        };
        let checker = DeterminismChecker;
        let result = checker.check(&ast);
        assert!(result.is_ok());
    }
}
