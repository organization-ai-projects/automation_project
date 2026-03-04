use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DiffRequest {
    pub from: String,
    pub to: String,
}
