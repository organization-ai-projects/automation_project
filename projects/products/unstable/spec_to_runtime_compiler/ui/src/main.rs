// projects/products/unstable/spec_to_runtime_compiler/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use diagnostics::error::UiError;
use transport::backend_process::BackendProcess;
use transport::ipc_client::{CompilerRequest, CompilerResponse, IpcClient};
use widgets::table_widget::TableWidget;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let spec_path = args.get(1).cloned().unwrap_or_default();

    tracing::info!(spec = %spec_path, "spec_to_runtime_compiler_ui starting");

    let mut controller = app::controller::Controller::new();
    controller.dispatch(app::action::Action::LoadSpec {
        path: spec_path.clone(),
    });
    controller.dispatch(app::action::Action::Compile);

    let source = if spec_path.is_empty() {
        "state Idle {}\nstate Running { tick: u64 }\ntransition Idle -> Running on start {}"
            .to_string()
    } else {
        std::fs::read_to_string(&spec_path)
            .map_err(|e| UiError::Internal(format!("failed to read spec file: {e}")))?
    };
    let backend_binary = resolve_backend_binary_path()?;
    let mut backend = BackendProcess::new(backend_binary)?;
    let mut ipc_client = IpcClient::new();

    ipc_client.send_request(&mut backend, CompilerRequest::LoadSpec { source })?;
    controller.dispatch(app::action::Action::DryRun);

    match ipc_client.send_request(&mut backend, CompilerRequest::CompileDryRun)? {
        CompilerResponse::CompileReport { report_json, .. } => {
            controller.dispatch(app::action::Action::SetReport(report_json));
            controller.dispatch(app::action::Action::GetReport);
        }
        CompilerResponse::Error { message } => {
            controller.dispatch(app::action::Action::SetError(message));
        }
        CompilerResponse::Ok => {}
    }

    let screen = screens::spec_screen::SpecScreen::new(spec_path);
    screen.render(&controller.state);

    let compile_screen = screens::compile_screen::CompileScreen::new();
    compile_screen.render(&controller.state);

    let report_screen = screens::report_screen::ReportScreen::new();
    report_screen.render(&controller.state);

    let table_widget = TableWidget::new("Compile report");
    let current_report = controller.state.last_report.clone().unwrap_or_default();
    let headers = ["field", "value"];
    let rows = vec![vec!["report".to_string(), current_report]];
    table_widget.render(&headers, &rows)?;

    tracing::info!(backend = %backend.binary_path, "backend process configured");

    tracing::info!("spec_to_runtime_compiler_ui finished");
    Ok(())
}

fn resolve_backend_binary_path() -> Result<String, UiError> {
    if let Ok(path) = std::env::var("SPEC_TO_RUNTIME_COMPILER_BACKEND_BIN")
        && !path.trim().is_empty()
    {
        return Ok(path);
    }

    let current_exe = std::env::current_exe()
        .map_err(|e| UiError::Internal(format!("failed to resolve current executable: {e}")))?;
    if let Some(parent) = current_exe.parent() {
        let sibling = parent.join("spec_to_runtime_compiler_backend");
        if sibling.exists() {
            return Ok(sibling.to_string_lossy().to_string());
        }
    }

    Ok("spec_to_runtime_compiler_backend".to_string())
}
