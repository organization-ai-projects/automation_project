// projects/products/unstable/protocol_builder/ui/src/public_api.rs
use anyhow::Result;

use crate::app::Controller;
use crate::transport::{BackendProcess, IpcClient};

/// Runs the generate workflow: spawn backend, send IPC, print report.
pub fn run_generate(schema_path: &str, out_dir: &str, backend_binary: &str) -> Result<()> {
    let process = BackendProcess::spawn(backend_binary)?;
    let client = IpcClient::new(process);
    let mut controller = Controller::new(client);
    controller.generate(schema_path, out_dir)?;

    let state = &controller.state;
    if let Some(hash) = &state.manifest_hash {
        println!("manifest_hash: {}", hash);
    }
    if let Some(report) = &state.report_json {
        println!("report: {}", report);
    }
    Ok(())
}
