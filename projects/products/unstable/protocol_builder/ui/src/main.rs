// projects/products/unstable/protocol_builder/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use diagnostics::ui_error::UiError;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    // usage: protocol_builder_ui generate --schema <file> --out <dir>
    if args.len() < 2 || args[1] != "generate" {
        return Err(UiError::MissingArgument(
            "usage: protocol_builder_ui generate --schema <file> --out <dir>",
        ));
    }

    let schema_path = find_arg(&args, "--schema").ok_or(UiError::MissingArgument("--schema"))?;
    let out_dir = find_arg(&args, "--out").ok_or(UiError::MissingArgument("--out"))?;
    let backend_binary = resolve_backend_binary_path()?;

    public_api::run_generate(schema_path, out_dir, &backend_binary)?;

    Ok(())
}

fn find_arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}

fn resolve_backend_binary_path() -> Result<String, UiError> {
    if let Ok(path) = std::env::var("PROTOCOL_BUILDER_BACKEND_BIN")
        && !path.trim().is_empty()
    {
        return Ok(path);
    }

    let current_exe = std::env::current_exe().map_err(|e| UiError::SpawnFailed(e.to_string()))?;
    if let Some(parent) = current_exe.parent() {
        let sibling = parent.join("protocol_builder_backend");
        if sibling.exists() {
            return Ok(sibling.to_string_lossy().to_string());
        }
    }

    Ok("protocol_builder_backend".to_string())
}
