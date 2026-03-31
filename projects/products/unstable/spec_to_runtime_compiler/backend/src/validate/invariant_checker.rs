use crate::diagnostics::error::CompilerError;
use crate::dsl::ast::SpecAst;

pub struct InvariantChecker;

impl InvariantChecker {
    pub fn check(&self, ast: &SpecAst) -> Result<(), CompilerError> {
        self.check_transition_states_exist(ast)?;
        self.check_invariants(ast)?;
        Ok(())
    }

    fn check_transition_states_exist(&self, ast: &SpecAst) -> Result<(), CompilerError> {
        let state_names: Vec<&str> = ast.states.iter().map(|s| s.name.as_str()).collect();

        for t in &ast.transitions {
            if !state_names.contains(&t.from.as_str()) {
                return Err(CompilerError::Validation(format!(
                    "transition references unknown source state '{}'",
                    t.from
                )));
            }
            if !state_names.contains(&t.to.as_str()) {
                return Err(CompilerError::Validation(format!(
                    "transition references unknown target state '{}'",
                    t.to
                )));
            }
        }
        Ok(())
    }

    fn check_invariants(&self, ast: &SpecAst) -> Result<(), CompilerError> {
        for inv in &ast.invariants {
            match inv.name.as_str() {
                "no_self_loop" => {
                    for t in &ast.transitions {
                        if t.from == t.to {
                            return Err(CompilerError::Invariant(format!(
                                "no_self_loop: transition from '{}' to '{}' on event '{}' is a self-loop",
                                t.from, t.to, t.event
                            )));
                        }
                    }
                }
                _ => {
                    // Unknown invariants are ignored (forward-compatible)
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::ast::{InvariantNode, StateNode, TransitionNode};

    #[test]
    fn detects_self_loop() {
        let ast = SpecAst {
            states: vec![StateNode {
                name: "Idle".to_string(),
                fields: vec![],
            }],
            transitions: vec![TransitionNode {
                from: "Idle".to_string(),
                to: "Idle".to_string(),
                event: "noop".to_string(),
                guard_fields: vec![],
            }],
            invariants: vec![InvariantNode {
                name: "no_self_loop".to_string(),
                description: "no state transitions to itself".to_string(),
            }],
        };
        let checker = InvariantChecker;
        let result = checker.check(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("self-loop"));
    }

    #[test]
    fn detects_unknown_state() {
        let ast = SpecAst {
            states: vec![StateNode {
                name: "Idle".to_string(),
                fields: vec![],
            }],
            transitions: vec![TransitionNode {
                from: "Idle".to_string(),
                to: "Missing".to_string(),
                event: "go".to_string(),
                guard_fields: vec![],
            }],
            invariants: vec![],
        };
        let checker = InvariantChecker;
        let result = checker.check(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing"));
    }
}
