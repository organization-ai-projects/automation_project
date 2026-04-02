//! projects/products/unstable/rust_language/backend/src/cli/mod.rs
mod cli_errors;
mod handlers;

#[cfg(test)]
mod tests;

pub(crate) use cli_errors::*;
pub(crate) use handlers::{
    handle_check, handle_compile, handle_init, handle_parse_to_ast, handle_save_binary,
    handle_save_config, improve_code,
};
