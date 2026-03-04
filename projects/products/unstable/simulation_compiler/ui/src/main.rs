// projects/products/unstable/simulation_compiler/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use diagnostics::ui_error::UiError;
use transport::backend_process::BackendProcess;
use transport::ipc_client::{CompilerRequest, CompilerResponse, IpcClient};
use widgets::diff_widget::DiffWidget;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let dsl_path = args.get(1).cloned().unwrap_or_default();

    tracing::info!(dsl = %dsl_path, "simulation-compiler-ui starting");

    let mut controller = app::controller::Controller::new();
    controller.dispatch(app::action::Action::LoadDsl {
        path: dsl_path.clone(),
    });
    controller.dispatch(app::action::Action::Compile);

    let source = if dsl_path.is_empty() {
        "component Sensor { field: u32 }".to_string()
    } else {
        std::fs::read_to_string(&dsl_path)
            .map_err(|e| UiError::Internal(format!("failed to read DSL file: {e}")))?
    };
    let backend_binary = resolve_backend_binary_path()?;
    let mut backend = BackendProcess::new(backend_binary)?;
    let mut ipc_client = IpcClient::new();

    ipc_client.send_request(&mut backend, CompilerRequest::LoadDsl { source })?;
    controller.dispatch(app::action::Action::DryRun);

    match ipc_client.send_request(&mut backend, CompilerRequest::CompileDryRun)? {
        CompilerResponse::Report { json } => {
            controller.dispatch(app::action::Action::SetReport(json));
            controller.dispatch(app::action::Action::GetReport);
        }
        CompilerResponse::Error { message } => {
            controller.dispatch(app::action::Action::SetError(message));
        }
        CompilerResponse::Ok => {}
    }

    let screen = screens::dsl_screen::DslScreen::new(dsl_path);
    screen.render(&controller.state);

    let compile_screen = screens::compile_screen::CompileScreen::new();
    compile_screen.render(&controller.state);

    let report_screen = screens::report_screen::ReportScreen::new();
    report_screen.render(&controller.state);

    let diff_widget = DiffWidget::new("Compile report");
    let current_report = controller.state.last_report.clone().unwrap_or_default();
    diff_widget.render("{}", &current_report)?;

    tracing::info!(backend = %backend.binary_path, "backend process configured");

    tracing::info!("simulation-compiler-ui finished");
    Ok(())
}

fn resolve_backend_binary_path() -> Result<String, UiError> {
    if let Ok(path) = std::env::var("SIMULATION_COMPILER_BACKEND_BIN") {
        if !path.trim().is_empty() {
            return Ok(path);
        }
    }

    let current_exe = std::env::current_exe()
        .map_err(|e| UiError::Internal(format!("failed to resolve current executable: {e}")))?;
    if let Some(parent) = current_exe.parent() {
        let sibling = parent.join("simulation-compiler-backend");
        if sibling.exists() {
            return Ok(sibling.to_string_lossy().to_string());
        }
    }

    Ok("simulation-compiler-backend".to_string())
}
