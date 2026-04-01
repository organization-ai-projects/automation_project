use serde::{Deserialize, Serialize};

use super::invariant_spec::InvariantSpec;
use super::state_spec::StateSpec;
use super::transition_spec::TransitionSpec;
use crate::dsl::ast::SpecAst;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeSpec {
    pub states: Vec<StateSpec>,
    pub transitions: Vec<TransitionSpec>,
    pub invariants: Vec<InvariantSpec>,
    pub initial_state: Option<String>,
}

impl RuntimeSpec {
    pub fn from_ast(ast: &SpecAst) -> Self {
        let mut states: Vec<StateSpec> = ast
            .states
            .iter()
            .map(|s| StateSpec {
                name: s.name.clone(),
                fields: s
                    .fields
                    .iter()
                    .map(|f| FieldSpec {
                        name: f.name.clone(),
                        ty: f.ty.clone(),
                    })
                    .collect(),
            })
            .collect();
        states.sort_by(|a, b| a.name.cmp(&b.name));

        let mut transitions: Vec<TransitionSpec> = ast
            .transitions
            .iter()
            .map(|t| TransitionSpec {
                from: t.from.clone(),
                to: t.to.clone(),
                event: t.event.clone(),
                guard_fields: t
                    .guard_fields
                    .iter()
                    .map(|f| FieldSpec {
                        name: f.name.clone(),
                        ty: f.ty.clone(),
                    })
                    .collect(),
            })
            .collect();
        transitions.sort_by(|a, b| (&a.from, &a.event, &a.to).cmp(&(&b.from, &b.event, &b.to)));

        let mut invariants: Vec<InvariantSpec> = ast
            .invariants
            .iter()
            .map(|i| InvariantSpec {
                name: i.name.clone(),
                description: i.description.clone(),
            })
            .collect();
        invariants.sort_by(|a, b| a.name.cmp(&b.name));

        Self {
            states,
            transitions,
            invariants,
            initial_state: ast.initial_state.clone(),
        }
    }
}
