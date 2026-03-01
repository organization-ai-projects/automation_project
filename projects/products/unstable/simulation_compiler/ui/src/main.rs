// projects/products/unstable/simulation_compiler/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use diagnostics::error::UiError;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let dsl_path = args.get(1).cloned().unwrap_or_default();

    tracing::info!(dsl = %dsl_path, "simulation-compiler-ui starting");

    let mut state = app::app_state::AppState::default();
    let action = app::action::Action::LoadDsl {
        path: dsl_path.clone(),
    };
    app::reducer::apply(&mut state, action);

    let screen = screens::dsl_screen::DslScreen::new(dsl_path);
    screen.render(&state);

    let compile_screen = screens::compile_screen::CompileScreen::new();
    compile_screen.render(&state);

    let report_screen = screens::report_screen::ReportScreen::new();
    report_screen.render(&state);

    tracing::info!("simulation-compiler-ui finished");
    Ok(())
}
