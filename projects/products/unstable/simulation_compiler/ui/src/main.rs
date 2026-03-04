// projects/products/unstable/simulation_compiler/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use diagnostics::ui_error::UiError;
use transport::backend_process::BackendProcess;
use transport::ipc_client::IpcClient;
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
    controller.dispatch(app::action::Action::DryRun);
    controller.dispatch(app::action::Action::GetReport);

    let backend = BackendProcess::new("simulation-compiler-backend");
    let ipc_client = IpcClient::new();
    let request = format!(
        "{{\"kind\":\"CompileDryRun\",\"dsl_path\":\"{}\"}}",
        dsl_path
    );
    match ipc_client.send_request(&request) {
        Ok(report) => controller.dispatch(app::action::Action::SetReport(report)),
        Err(e) => controller.dispatch(app::action::Action::SetError(e.to_string())),
    }

    let screen = screens::dsl_screen::DslScreen::new(dsl_path);
    screen.render(&controller.state);

    let compile_screen = screens::compile_screen::CompileScreen::new();
    compile_screen.render(&controller.state);

    let report_screen = screens::report_screen::ReportScreen::new();
    report_screen.render(&controller.state);

    let diff_widget = DiffWidget::new("Compile report");
    let current_report = controller.state.last_report.clone().unwrap_or_default();
    diff_widget.render("", &current_report)?;

    tracing::info!(backend = %backend.binary_path, "backend process configured");

    tracing::info!("simulation-compiler-ui finished");
    Ok(())
}
