// projects/products/unstable/simulation_compiler/backend/src/model/report_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSpec {
    pub name: String,
    pub fields: Vec<crate::model::component_spec::FieldSpec>,
}
