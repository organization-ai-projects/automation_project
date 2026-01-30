//projects/products/core/central_ui/src/setup_admin_input.rs
use serde::Deserialize;

//replace user_id issue 67
#[derive(Debug, Deserialize)]
pub(crate) struct SetupAdminInput {
    pub(crate) user_id: String,
    pub(crate) password: String,
}
