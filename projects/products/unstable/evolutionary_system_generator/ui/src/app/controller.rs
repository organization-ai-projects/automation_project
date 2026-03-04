// projects/products/unstable/evolutionary_system_generator/ui/src/app/controller.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::{apply_candidates, apply_error, apply_report};
use crate::diagnostics::ui_error::UiError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;
use common_json::Json;

pub struct Controller {
    backend: Option<BackendProcess>,
    client: Option<IpcClient>,
    backend_bin: String,
}

impl Controller {
    pub fn new() -> Self {
        let backend_bin =
            std::env::var("EVO_BACKEND_BIN").unwrap_or_else(|_| "evo-backend".to_string());
        Self {
            backend: None,
            client: None,
            backend_bin,
        }
    }

    pub fn handle(&mut self, state: &mut AppState, action: Action) -> Vec<String> {
        match action {
            Action::StartSearch {
                seed,
                population_size,
                max_generations,
                rule_pool,
            } => {
                let rule_pool_json = common_json::to_string(&rule_pool).unwrap_or_default();
                if let Err(err) = self.ensure_connected() {
                    apply_error(state, err.clone());
                    return vec![format!("Error: {err}")];
                }
                let request = format!(
                    r#"{{"type":"NewSearch","seed":{seed},"population_size":{population_size},"max_generations":{max_generations},"rule_pool":{rule_pool_json}}}"#
                );
                match self.send_request(&request) {
                    Ok(line) if line.contains(r#""type":"Ok""#) => {
                        vec!["Search started.".to_string()]
                    }
                    Ok(line) => {
                        apply_error(state, line.clone());
                        vec![format!("Error: {line}")]
                    }
                    Err(e) => {
                        apply_error(state, e.clone());
                        vec![format!("Error: {e}")]
                    }
                }
            }
            Action::StepGen => {
                if let Err(err) = self.ensure_connected() {
                    apply_error(state, err.clone());
                    return vec![format!("Error: {err}")];
                }
                match self.send_request(r#"{"type":"StepGen"}"#) {
                    Ok(line) => self.handle_report_or_error(state, &line),
                    Err(e) => {
                        apply_error(state, e.clone());
                        vec![format!("Error: {e}")]
                    }
                }
            }
            Action::RunToEnd => {
                if let Err(err) = self.ensure_connected() {
                    apply_error(state, err.clone());
                    return vec![format!("Error: {err}")];
                }
                match self.send_request(r#"{"type":"RunToEnd"}"#) {
                    Ok(line) => self.handle_report_or_error(state, &line),
                    Err(e) => {
                        apply_error(state, e.clone());
                        vec![format!("Error: {e}")]
                    }
                }
            }
            Action::ShowCandidates { top_n } => {
                if let Err(err) = self.ensure_connected() {
                    apply_error(state, err.clone());
                    return vec![format!("Error: {err}")];
                }
                let request = format!(r#"{{"type":"GetCandidates","top_n":{top_n}}}"#);
                match self.send_request(&request) {
                    Ok(line) => match common_json::from_json_str::<Json>(&line) {
                        Ok(value)
                            if json_field(&value, "type").and_then(Json::as_str)
                                == Some("Candidates") =>
                        {
                            apply_candidates(state, value);
                            vec![format!("Candidates received (top {top_n})")]
                        }
                        Ok(value)
                            if json_field(&value, "type").and_then(Json::as_str)
                                == Some("Error") =>
                        {
                            let err = json_field(&value, "message")
                                .and_then(Json::as_str)
                                .unwrap_or("unknown backend error")
                                .to_string();
                            apply_error(state, err.clone());
                            vec![format!("Error: {err}")]
                        }
                        _ => {
                            let err = "No candidates in response".to_string();
                            apply_error(state, err.clone());
                            vec![format!("Error: {err}")]
                        }
                    },
                    Err(e) => {
                        apply_error(state, e.clone());
                        vec![format!("Error: {e}")]
                    }
                }
            }
            Action::Quit => {
                self.disconnect();
                vec!["Goodbye.".to_string()]
            }
        }
    }

    fn ensure_connected(&mut self) -> Result<(), String> {
        if self.client.is_some() {
            return Ok(());
        }
        let mut backend = BackendProcess::spawn(&self.backend_bin)
            .map_err(|e| format!("backend spawn failed: {e}"))?;
        let stdin = backend
            .take_stdin()
            .map_err(|e| format!("backend stdin unavailable: {e}"))?;
        let stdout = backend
            .take_stdout()
            .map_err(|e| format!("backend stdout unavailable: {e}"))?;
        self.client = Some(IpcClient::new(stdin, stdout));
        self.backend = Some(backend);
        Ok(())
    }

    fn send_request(&mut self, request: &str) -> Result<String, String> {
        let client = self
            .client
            .as_mut()
            .ok_or(UiError::NoBackend)
            .map_err(|e| e.to_string())?;
        client.send_request(request).map_err(|e| e.to_string())
    }

    fn handle_report_or_error(&mut self, state: &mut AppState, line: &str) -> Vec<String> {
        match common_json::from_json_str::<Json>(line) {
            Ok(value) if json_field(&value, "type").and_then(Json::as_str) == Some("Report") => {
                let generation = json_field(&value, "generation")
                    .and_then(Json::as_u64)
                    .unwrap_or(0) as u32;
                let best = json_field(&value, "best_fitness")
                    .and_then(Json::as_f64)
                    .unwrap_or(0.0);
                let done = json_field(&value, "done")
                    .and_then(Json::as_bool)
                    .unwrap_or(false);
                apply_report(state, generation, best, done);
                vec![format!(
                    "Generation {} | best_fitness {:.4} | done={}",
                    generation, best, done
                )]
            }
            Ok(value) if json_field(&value, "type").and_then(Json::as_str) == Some("Error") => {
                let err = json_field(&value, "message")
                    .and_then(Json::as_str)
                    .unwrap_or("unknown backend error")
                    .to_string();
                apply_error(state, err.clone());
                vec![format!("Error: {err}")]
            }
            _ => {
                let err = "Unexpected backend response".to_string();
                apply_error(state, err.clone());
                vec![format!("Error: {err}")]
            }
        }
    }

    fn disconnect(&mut self) {
        self.client = None;
        if let Some(backend) = self.backend.as_mut() {
            let _ = backend.kill();
        }
        self.backend = None;
    }
}

fn json_field<'a>(json: &'a Json, key: &str) -> Option<&'a Json> {
    json.as_object().and_then(|obj| obj.get(key))
}
