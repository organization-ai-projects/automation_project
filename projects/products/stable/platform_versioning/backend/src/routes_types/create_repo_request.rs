use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRepoRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
