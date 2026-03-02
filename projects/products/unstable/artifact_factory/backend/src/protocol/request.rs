use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    LoadInputs { paths: Vec<String> },
    Analyze,
    RenderDocs,
    BuildBundle,
    GetBundle,
}
