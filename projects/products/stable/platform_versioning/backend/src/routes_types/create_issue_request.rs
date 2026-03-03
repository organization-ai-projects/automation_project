use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateIssueRequest {
    pub id: String,
    pub repo_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
}
