use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::{apply_candidates, apply_error};

pub struct Controller {
    child: Option<Child>,
    backend_bin: String,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            child: None,
            backend_bin: "evo-backend".to_string(),
        }
    }

    pub fn with_backend_bin(path: impl Into<String>) -> Self {
        Self {
            child: None,
            backend_bin: path.into(),
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
                let rule_pool_json = serde_json::to_string(&rule_pool).unwrap_or_default();
                let requests = vec![format!(
                    r#"{{"type":"NewSearch","seed":{seed},"population_size":{population_size},"max_generations":{max_generations},"rule_pool":{rule_pool_json}}}"#
                )];
                match self.run_requests_in_fresh_backend(&requests) {
                    Ok(lines) => {
                        if lines.iter().any(|l| l.contains(r#""type":"Ok""#)) {
                            vec!["Search started.".to_string()]
                        } else {
                            let err = lines.join("; ");
                            apply_error(state, err.clone());
                            vec![format!("Error: {}", err)]
                        }
                    }
                    Err(e) => {
                        apply_error(state, e.clone());
                        vec![format!("Error: {}", e)]
                    }
                }
            }
            Action::StepGen => {
                apply_error(
                    state,
                    "StepGen requires an active session; use run instead.".to_string(),
                );
                vec!["Use 'run' to run to completion.".to_string()]
            }
            Action::RunToEnd => {
                // Handled via StartSearch + RunToEnd + GetCandidates in one session
                vec!["Use 'new-search' first, then 'candidates'.".to_string()]
            }
            Action::ShowCandidates { top_n } => {
                // Spawns backend, runs a default search, gets candidates
                let requests = vec![
                    r#"{"type":"NewSearch","seed":42,"population_size":8,"max_generations":4,"rule_pool":["rule_a","rule_b","rule_c","rule_d"]}"#.to_string(),
                    r#"{"type":"RunToEnd"}"#.to_string(),
                    format!(r#"{{"type":"GetCandidates","top_n":{top_n}}}"#),
                ];
                match self.run_requests_in_fresh_backend(&requests) {
                    Ok(lines) => {
                        for line in &lines {
                            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                                if v.get("type").and_then(|t| t.as_str()) == Some("Candidates") {
                                    apply_candidates(state, v.clone());
                                    return vec![format!("Candidates received (top {top_n})")];
                                }
                            }
                        }
                        let err = "No candidates in response".to_string();
                        apply_error(state, err.clone());
                        vec![format!("Error: {}", err)]
                    }
                    Err(e) => {
                        apply_error(state, e.clone());
                        vec![format!("Error: {}", e)]
                    }
                }
            }
            Action::Quit => vec!["Goodbye.".to_string()],
        }
    }

    fn run_requests_in_fresh_backend(
        &mut self,
        requests: &[String],
    ) -> Result<Vec<String>, String> {
        let mut child = Command::new(&self.backend_bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to spawn evo-backend: {e}"))?;

        {
            let stdin = child.stdin.as_mut().ok_or("no stdin")?;
            for req in requests {
                writeln!(stdin, "{req}").map_err(|e| e.to_string())?;
            }
        }
        drop(child.stdin.take());

        let stdout = child.stdout.take().ok_or("no stdout")?;
        let lines: Vec<String> = BufReader::new(stdout)
            .lines()
            .map_while(Result::ok)
            .collect();
        child.wait().map_err(|e| e.to_string())?;
        Ok(lines)
    }
}
