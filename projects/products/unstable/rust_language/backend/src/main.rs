//! projects/products/unstable/rust_language/backend/src/main.rs
mod ai_assist;
mod app_error;
mod cli;
mod compiler;
mod diagnostics;
mod engine;
mod model;

#[cfg(test)]
mod tests;

use std::env;

use crate::{
    app_error::AppError,
    cli::{
        handle_check, handle_compile, handle_init, handle_parse_to_ast, handle_save_binary,
        handle_save_config, improve_code,
    },
};

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");

    match command {
        "compile" => handle_compile(args.get(2..).unwrap_or_default()),
        "check" => handle_check(args.get(2..).unwrap_or_default()),
        "init" => handle_init(args.get(2..).unwrap_or_default()),
        "improve" => improve_code(args.get(2..).unwrap_or_default()).map(|_| ()),
        "save-config" => handle_save_config(args.get(2..).unwrap_or_default()),
        "save-binary" => handle_save_binary(args.get(2..).unwrap_or_default()),
        "parse-ast" => handle_parse_to_ast(args.get(2..).unwrap_or_default()).map(|_| ()),
        _ => Err(AppError::Usage(
            "Usage: rust_language_backend <compile|check|init|improve|save-config|save-binary|parse-ast> [OPTIONS]",
        )),
    }
}
