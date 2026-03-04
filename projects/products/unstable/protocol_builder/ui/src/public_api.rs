// projects/products/unstable/protocol_builder/ui/src/public_api.rs
use crate::app::controller::Controller;
use crate::diagnostics::ui_error::UiError;
use crate::screens::{
    generate_screen::GenerateScreen, report_screen::ReportScreen, schema_screen::SchemaScreen,
};
use crate::transport::{BackendProcess, IpcClient};
use crate::widgets::table_widget::TableWidget;

/// Runs the generate workflow: spawn backend, send IPC, print report.
pub fn run_generate(schema_path: &str, out_dir: &str, backend_binary: &str) -> Result<(), UiError> {
    let process = BackendProcess::spawn(backend_binary)?;
    let client = IpcClient::new(process);
    let mut controller = Controller::new(client);
    controller.generate(schema_path, out_dir)?;

    let state = &controller.state;
    let schema_screen = SchemaScreen {
        schema_path: state.schema_path.clone().unwrap_or_default(),
    };
    let generate_screen = GenerateScreen {
        out_dir: state.out_dir.clone().unwrap_or_default(),
    };
    let report_screen = ReportScreen {
        manifest_hash: state.manifest_hash.clone().unwrap_or_default(),
        report_json: state.report_json.clone().unwrap_or_default(),
    };
    let mut table = TableWidget::new();
    table.insert("backend", backend_binary.to_string());
    table.insert("schema", schema_screen.schema_path.clone());
    table.insert("output", generate_screen.out_dir.clone());

    println!("{}", schema_screen.render());
    println!("{}", generate_screen.render());
    println!("{}", table.render());
    if let Some(hash) = &state.manifest_hash {
        println!("manifest_hash: {}", hash);
    }
    if let Some(report) = &state.report_json {
        println!("report: {}", report);
    }
    if state.manifest_hash.is_some() || state.report_json.is_some() {
        println!("{}", report_screen.render());
    }
    if let Some(err) = &state.last_error {
        return Err(UiError::IpcError(err.to_string()));
    }
    Ok(())
}
