//projects/products/core/central_ui/src/login_input.rs
use serde::Deserialize;

//replace user_id issue 67
#[derive(Debug, Deserialize)]
pub(crate) struct LoginInput {
    pub(crate) user_id: String,
    pub(crate) password: String,
}
