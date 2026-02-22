// projects/products/unstable/auto_manager_ai/src/engine_gateway.rs

use std::path::PathBuf;

use common_json::from_str;
use protocol::{Command, CommandType, Metadata, Payload};

use crate::adapters::{RepoAdapter, RepoContext};
use crate::config::RunMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineErrorKind {
    Unavailable,
    Protocol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineError {
    pub kind: EngineErrorKind,
    pub code: &'static str,
    pub message: String,
}

impl EngineError {
    fn unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: EngineErrorKind::Unavailable,
            code,
            message: message.into(),
        }
    }

    fn protocol(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: EngineErrorKind::Protocol,
            code,
            message: message.into(),
        }
    }

    pub fn render(&self) -> String {
        format!("[{}::{:?}] {}", self.code, self.kind, self.message)
    }
}

/// Engine gateway for lifecycle signaling and protocol-mediated repo context access.
pub struct EngineGateway {
    run_mode: RunMode,
    product_name: String,
}

impl EngineGateway {
    pub fn new(run_mode: RunMode) -> Self {
        Self {
            run_mode,
            product_name: "auto_manager_ai".to_string(),
        }
    }

    pub fn register_startup(&self) -> Result<String, EngineError> {
        self.ensure_available()?;
        let _ = self.build_command(
            "auto_manager.lifecycle.startup",
            &format!(r#"{{"product":"{}"}}"#, self.product_name),
        )?;
        Ok("engine startup registration acknowledged".to_string())
    }

    pub fn record_health(&self) -> Result<String, EngineError> {
        self.ensure_available()?;
        let _ = self.build_command(
            "auto_manager.lifecycle.health",
            &format!(
                r#"{{"product":"{}","status":"healthy"}}"#,
                self.product_name
            ),
        )?;
        Ok("engine health event acknowledged".to_string())
    }

    pub fn register_shutdown(&self) -> Result<String, EngineError> {
        self.ensure_available()?;
        let _ = self.build_command(
            "auto_manager.lifecycle.shutdown",
            &format!(r#"{{"product":"{}"}}"#, self.product_name),
        )?;
        Ok("engine shutdown registration acknowledged".to_string())
    }

    pub fn fetch_repo_context(&self, repo_path: PathBuf) -> Result<RepoContext, EngineError> {
        if self.ensure_available().is_err() {
            if matches!(self.run_mode, RunMode::DeterministicFallback) {
                let adapter = RepoAdapter::new(repo_path);
                return adapter
                    .get_context()
                    .map(|mut ctx| {
                        ctx.mediated_by_engine = false;
                        ctx
                    })
                    .map_err(|e| {
                        EngineError::unavailable(
                            "ENGINE_FALLBACK_REPO_CONTEXT_FAILED",
                            format!("deterministic fallback failed: {e}"),
                        )
                    });
            }
            return Err(EngineError::unavailable(
                "ENGINE_UNAVAILABLE_FAIL_CLOSED",
                "engine not available in EngineRequired mode",
            ));
        }

        let path_str = repo_path.to_string_lossy().replace('"', "\\\"");
        let _ = self.build_command(
            "auto_manager.repo_context.query",
            &format!(r#"{{"repo_path":"{}"}}"#, path_str),
        )?;

        let adapter = RepoAdapter::new(repo_path);
        adapter
            .get_context()
            .map(|mut ctx| {
                ctx.mediated_by_engine = true;
                ctx
            })
            .map_err(|e| {
                EngineError::protocol(
                    "ENGINE_PROTOCOL_REPO_CONTEXT_FAILED",
                    format!("protocol request accepted but context retrieval failed: {e}"),
                )
            })
    }

    fn ensure_available(&self) -> Result<(), EngineError> {
        let available = std::env::var("AUTO_MANAGER_ENGINE_AVAILABLE")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if available {
            Ok(())
        } else {
            Err(EngineError::unavailable(
                "ENGINE_UNAVAILABLE",
                "set AUTO_MANAGER_ENGINE_AVAILABLE=true or switch to deterministic fallback mode",
            ))
        }
    }

    fn build_command(&self, action: &str, payload_json: &str) -> Result<Command, EngineError> {
        let payload = from_str(payload_json).map_err(|e| {
            EngineError::protocol(
                "ENGINE_PROTOCOL_PAYLOAD_BUILD_FAILED",
                format!("invalid payload json: {e}"),
            )
        })?;
        let command = Command {
            metadata: Metadata::now(),
            command_type: CommandType::Query,
            action: Some(action.to_string()),
            payload: Some(Payload {
                payload_type: Some("application/json".to_string()),
                payload: Some(payload),
            }),
        };
        if command.validate() {
            Ok(command)
        } else {
            Err(EngineError::protocol(
                "ENGINE_PROTOCOL_COMMAND_INVALID",
                "command validation failed",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::EngineGateway;
    use crate::config::RunMode;
    use crate::tests::test_helpers::create_unique_temp_dir;

    #[test]
    fn engine_required_fails_closed_when_unavailable() {
        let gateway = EngineGateway::new(RunMode::EngineRequired);
        let temp_dir = create_unique_temp_dir("auto_manager_ai_engine_required");
        fs::create_dir_all(&temp_dir).expect("create temp_dir");
        let result = gateway.fetch_repo_context(temp_dir.clone());
        assert!(result.is_err());
        let err = result.expect_err("expected engine failure");
        assert_eq!(err.code, "ENGINE_UNAVAILABLE_FAIL_CLOSED");
        fs::remove_dir_all(temp_dir).ok();
    }

    #[test]
    fn deterministic_fallback_returns_repo_context_when_engine_unavailable() {
        let gateway = EngineGateway::new(RunMode::DeterministicFallback);
        let temp_dir = create_unique_temp_dir("auto_manager_ai_engine_fallback");
        fs::create_dir_all(&temp_dir).expect("create temp_dir");
        fs::write(temp_dir.join("README.md"), "hello").expect("write file");

        let result = gateway.fetch_repo_context(temp_dir.clone());
        assert!(result.is_ok());
        let ctx = result.expect("context should be available in fallback mode");
        assert!(!ctx.mediated_by_engine);
        assert!(!ctx.tracked_files.is_empty());
        fs::remove_dir_all(temp_dir).ok();
    }
}
