use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer;
use crate::diagnostics::ui_error::UiError;
use crate::screens;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

pub fn dispatch(state: &mut AppState, action: Action) -> Result<(), UiError> {
    let request_json = build_request_json(&action);
    let response_line = send_request(&request_json)?;
    reducer::apply_response(state, &response_line)?;

    let json_mode = match &action {
        Action::Scan { json, .. } => *json,
        Action::Canon { json, .. } => *json,
        Action::Stability { json, .. } => *json,
        Action::Report { json } => *json,
    };

    let report_line = match action {
        Action::Report { .. } => response_line.clone(),
        _ => {
            if reducer::response_is_error(&response_line) {
                response_line.clone()
            } else {
                let get_report = r#"{"type":"get_report"}"#;
                let line = send_request(get_report)?;
                reducer::apply_response(state, &line)?;
                line
            }
        }
    };

    match &action {
        Action::Scan { .. } => screens::scan_screen::display(&report_line, json_mode),
        Action::Canon { .. } => screens::canon_screen::display(&report_line, json_mode),
        Action::Stability { .. } => screens::stability_screen::display(&report_line, json_mode),
        Action::Report { .. } => screens::report_screen::display(&report_line, json_mode),
    }

    Ok(())
}

fn build_request_json(action: &Action) -> String {
    match action {
        Action::Scan { root, .. } => {
            format!(r#"{{"type":"scan_forbidden","root":"{}"}}"#, root)
        }
        Action::Canon { path, .. } => {
            format!(r#"{{"type":"check_canonical_json","path":"{}"}}"#, path)
        }
        Action::Stability { cmd, runs, .. } => {
            format!(
                r#"{{"type":"run_stability_harness","cmd":"{}","runs":{}}}"#,
                cmd, runs
            )
        }
        Action::Report { .. } => r#"{"type":"get_report"}"#.to_string(),
    }
}

fn send_request(request_json: &str) -> Result<String, UiError> {
    if let Ok(address) = std::env::var("MDG_UI_IPC_ADDR") {
        let client = IpcClient::new(address);
        return client.send(request_json);
    }

    let mut backend = BackendProcess::spawn().map_err(|err| UiError::Transport(err.to_string()))?;
    backend
        .send_line(request_json)
        .map_err(|err| UiError::Transport(err.to_string()))?;
    backend
        .recv_line()
        .map_err(|err| UiError::Transport(err.to_string()))
}
