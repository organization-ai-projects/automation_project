// projects/products/unstable/simulation_compiler/backend/src/dsl/ast.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ast {
    pub components: Vec<ComponentNode>,
    pub systems: Vec<SystemNode>,
    pub events: Vec<EventNode>,
    pub reports: Vec<ReportNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemNode {
    pub name: String,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNode {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportNode {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub ty: String,
}
