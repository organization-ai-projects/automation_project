use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub destination: Option<String>,
    pub policy: Option<String>,
}
