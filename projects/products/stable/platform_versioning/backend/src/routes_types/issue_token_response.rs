use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IssueTokenResponse {
    pub token: String,
}
