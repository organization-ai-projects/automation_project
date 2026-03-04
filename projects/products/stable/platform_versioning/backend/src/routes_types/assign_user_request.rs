use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssignUserRequest {
    pub subject: String,
}
