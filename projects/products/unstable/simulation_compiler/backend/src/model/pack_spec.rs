// projects/products/unstable/simulation_compiler/backend/src/model/pack_spec.rs
use serde::{Deserialize, Serialize};

use super::component_spec::{ComponentSpec, FieldSpec};
use super::event_spec::EventSpec;
use super::report_spec::ReportSpec;
use super::system_spec::SystemSpec;
use crate::dsl::ast::Ast;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PackSpec {
    pub components: Vec<ComponentSpec>,
    pub systems: Vec<SystemSpec>,
    pub events: Vec<EventSpec>,
    pub reports: Vec<ReportSpec>,
}

impl PackSpec {
    pub fn from_ast(ast: &Ast) -> Self {
        let components = ast
            .components
            .iter()
            .map(|c| ComponentSpec {
                name: c.name.clone(),
                fields: c
                    .fields
                    .iter()
                    .map(|f| FieldSpec {
                        name: f.name.clone(),
                        ty: f.ty.clone(),
                    })
                    .collect(),
            })
            .collect();

        let systems = ast
            .systems
            .iter()
            .map(|s| SystemSpec {
                name: s.name.clone(),
                reads: s.reads.clone(),
                writes: s.writes.clone(),
            })
            .collect();

        let events = ast
            .events
            .iter()
            .map(|e| EventSpec {
                name: e.name.clone(),
                fields: e
                    .fields
                    .iter()
                    .map(|f| FieldSpec {
                        name: f.name.clone(),
                        ty: f.ty.clone(),
                    })
                    .collect(),
            })
            .collect();

        let reports = ast
            .reports
            .iter()
            .map(|r| ReportSpec {
                name: r.name.clone(),
                fields: r
                    .fields
                    .iter()
                    .map(|f| FieldSpec {
                        name: f.name.clone(),
                        ty: f.ty.clone(),
                    })
                    .collect(),
            })
            .collect();

        Self {
            components,
            systems,
            events,
            reports,
        }
    }
}
