// projects/products/unstable/code_forge_engine/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    run_cli(&args)
}

fn run_cli(args: &[String]) -> Result<()> {
    use app::controller::Controller;
    use transport::backend_process::BackendProcess;

    let contract = args
        .iter()
        .position(|a| a == "--contract")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let out_dir = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let golden_dir = args
        .iter()
        .position(|a| a == "--golden-dir")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let backend_bin = std::env::var("CODE_FORGE_BACKEND_BIN")
        .unwrap_or_else(|_| "code_forge_engine_backend".to_string());

    let process = BackendProcess::new(backend_bin);

    let mut controller = Controller::new(process);
    controller.run(
        contract.as_deref(),
        out_dir.as_deref(),
        golden_dir.as_deref(),
    )?;
    Ok(())
}
