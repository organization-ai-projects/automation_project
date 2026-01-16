// projects/products/core/engine/src/login_request.rs
// HTTP: login (issue JWT)
#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
    pub password: String, // TODO: implement real validation
    pub role: Option<security::Role>,
    pub duration_ms: Option<u64>,
    pub session_id: Option<String>,
}
