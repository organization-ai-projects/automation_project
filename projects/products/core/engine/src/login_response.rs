#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}