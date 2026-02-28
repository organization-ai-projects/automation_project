use anyhow::Result;
use crate::app::app_state::AppState;
use crate::app::action::Action;
use crate::app::reducer;
use crate::transport::backend_process::BackendProcess;
use crate::screens;

pub fn dispatch(state: &mut AppState, action: Action) -> Result<()> {
    let mut backend = BackendProcess::spawn()?;

    let request_json = match &action {
        Action::Scan { root, .. } => {
            format!(r#"{{"type":"scan_forbidden","root":"{}"}}"#, root)
        }
        Action::Stability { cmd, runs, .. } => {
            format!(r#"{{"type":"run_stability_harness","cmd":"{}","runs":{}}}"#, cmd, runs)
        }
        Action::Report { .. } => {
            r#"{"type":"get_report"}"#.to_string()
        }
    };

    backend.send_line(&request_json)?;
    let response_line = backend.recv_line()?;
    reducer::apply_response(state, &response_line);

    let json_mode = match &action {
        Action::Scan { json, .. } => *json,
        Action::Stability { json, .. } => *json,
        Action::Report { json } => *json,
    };

    match &action {
        Action::Scan { .. } => screens::scan_screen::display(&response_line, json_mode),
        Action::Stability { .. } => screens::stability_screen::display(&response_line, json_mode),
        Action::Report { .. } => screens::report_screen::display(&response_line, json_mode),
    }

    Ok(())
}
