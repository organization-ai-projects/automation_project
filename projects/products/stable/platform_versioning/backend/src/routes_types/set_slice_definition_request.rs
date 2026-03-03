use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SetSliceDefinitionRequest {
    pub paths: Vec<String>,
}
