use crate::diagnostics::error::CompilerError;
use crate::dsl::ast::SpecAst;

const FORBIDDEN_TYPES: &[&str] = &["Instant", "SystemTime", "HashMap", "HashSet"];
const DEFAULT_ABLE_TYPES: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64", "bool", "String",
];

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
                if !DEFAULT_ABLE_TYPES.contains(&field.ty.as_str()) {
                    return Err(CompilerError::Determinism(format!(
                        "state '{}' field '{}' uses type '{}' which may not implement Default; \
                         allowed types: {}",
                        state.name,
                        field.name,
                        field.ty,
                        DEFAULT_ABLE_TYPES.join(", ")
                    )));
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
            initial_state: None,
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
            initial_state: None,
        };
        let checker = DeterminismChecker;
        let result = checker.check(&ast);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_non_default_type() {
        let ast = SpecAst {
            states: vec![StateNode {
                name: "Running".to_string(),
                fields: vec![FieldDef {
                    name: "data".to_string(),
                    ty: "CustomType".to_string(),
                }],
            }],
            transitions: vec![],
            invariants: vec![],
            initial_state: None,
        };
        let checker = DeterminismChecker;
        let result = checker.check(&ast);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("may not implement Default"));
    }
}
