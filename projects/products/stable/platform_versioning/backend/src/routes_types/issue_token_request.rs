use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IssueTokenRequest {
    pub subject: String,
    pub repo_id: Option<String>,
    pub permission: crate::auth::Permission,
    pub expires_at: Option<u64>,
}
