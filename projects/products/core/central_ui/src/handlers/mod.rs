//projects/products/core/central_ui/src/handlers/mod.rs
mod accounts_proxy;
mod login;
mod response_with_status;
mod setup_admin;
mod setup_status;

pub(crate) use accounts_proxy::handle_accounts_proxy;
pub(crate) use login::handle_login;
pub(crate) use response_with_status::response_with_status;
pub(crate) use setup_admin::handle_setup_admin;
pub(crate) use setup_status::handle_setup_status;
