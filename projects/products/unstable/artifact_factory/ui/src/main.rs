mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::diagnostics::ui_error::UiError;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();
    tracing::info!("artifact-factory-ui starting");

    let args: Vec<String> = std::env::args().collect();
    let backend_bin = args
        .iter()
        .position(|a| a == "--backend")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "artifact_factory_backend".to_string());

    let input_paths: Vec<String> = args
        .iter()
        .position(|a| a == "--inputs")
        .map(|i| {
            args[i + 1..]
                .iter()
                .take_while(|a| !a.starts_with("--"))
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    public_api::run_headless(&backend_bin, input_paths)
}
