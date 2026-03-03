use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FetchSummary {
    pub object_ids: Vec<String>,
}
