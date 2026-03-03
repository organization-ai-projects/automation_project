use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RepoSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}
