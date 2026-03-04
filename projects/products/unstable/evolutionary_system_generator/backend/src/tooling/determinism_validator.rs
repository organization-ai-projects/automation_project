// projects/products/unstable/evolutionary_system_generator/backend/src/tooling/determinism_validator.rs
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

use serde_json::Value;

use crate::tooling::determinism_result::DeterminismResult;
use crate::tooling::tooling_error::ToolingError;
use crate::tooling::validator_config::ValidatorConfig;

pub struct DeterminismValidator;

impl DeterminismValidator {
    pub fn validate(
        config: ValidatorConfig,
        backend_bin: &str,
    ) -> Result<DeterminismResult, ToolingError> {
        let hash1 = Self::run_once(&config, backend_bin)?;
        let hash2 = Self::run_once(&config, backend_bin)?;
        let ok = hash1 == hash2;
        Ok(DeterminismResult {
            determinism_ok: ok,
            manifest_hash: Some(hash1),
        })
    }

    fn run_once(config: &ValidatorConfig, backend_bin: &str) -> Result<String, ToolingError> {
        let mut child = spawn_backend(backend_bin)?;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or(ToolingError::Io(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "no stdin",
            )))?;
        let rule_pool_json = serde_json::to_string(&config.rule_pool)
            .map_err(|e| ToolingError::Validation(e.to_string()))?;
        writeln!(
            stdin,
            r#"{{"type":"NewSearch","seed":{},"population_size":{},"max_generations":{},"rule_pool":{}}}"#,
            config.seed, config.population_size, config.max_generations, rule_pool_json
        )?;
        writeln!(stdin, r#"{{"type":"RunToEnd"}}"#)?;
        writeln!(stdin, r#"{{"type":"GetCandidates","top_n":5}}"#)?;
        drop(child.stdin.take());

        let stdout = child
            .stdout
            .take()
            .ok_or(ToolingError::Io(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "no stdout",
            )))?;
        let lines: Vec<String> = BufReader::new(stdout)
            .lines()
            .map_while(Result::ok)
            .collect();
        child.wait()?;

        extract_manifest_hash(&lines)
    }
}

pub fn spawn_backend(backend_bin: &str) -> Result<Child, ToolingError> {
    Command::new(backend_bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| ToolingError::Io(e))
}

pub fn extract_manifest_hash(lines: &[String]) -> Result<String, ToolingError> {
    for line in lines {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            if v.get("type").and_then(|t| t.as_str()) == Some("Candidates") {
                if let Some(hash) = v
                    .get("manifest")
                    .and_then(|m| m.get("manifest_hash"))
                    .and_then(|h| h.as_str())
                {
                    return Ok(hash.to_string());
                }
            }
        }
    }
    Err(ToolingError::Validation(
        "No Candidates response found in backend output".to_string(),
    ))
}
