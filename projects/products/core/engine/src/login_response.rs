// projects/products/core/engine/src/login_response.rs
#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}
