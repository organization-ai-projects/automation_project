use crate::app::controller::Controller;
use crate::diagnostics::ui_error::UiError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

pub fn run_cli(args: &[String]) -> Result<i32, UiError> {
    let contract = args
        .iter()
        .position(|a| a == "--contract")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .ok_or_else(|| UiError::Usage("missing --contract <file>".to_string()))?;

    let out_dir = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .ok_or_else(|| UiError::Usage("missing --out <dir>".to_string()))?;

    let dry_run = args.iter().any(|arg| arg == "--dry-run");

    let backend_bin = std::env::var("CODE_FORGE_BACKEND_BIN")
        .unwrap_or_else(|_| "code_forge_engine_backend".to_string());

    let process = BackendProcess::spawn(&backend_bin)?;
    let client = IpcClient::new(process);
    let mut controller = Controller::new(client);
    controller.run(&contract, &out_dir, dry_run)?;

    Ok(0)
}
