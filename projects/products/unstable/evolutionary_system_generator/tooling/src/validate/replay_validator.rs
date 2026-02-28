use std::io::{BufRead, BufReader, Write};

use crate::diagnostics::error::ToolingError;
use crate::validate::determinism_validator::{ValidatorConfig, spawn_backend, extract_manifest_hash};

#[derive(Debug)]
pub struct ReplayValidatorResult {
    pub replay_ok: bool,
}

pub struct ReplayValidator;

impl ReplayValidator {
    pub fn validate(
        config: ValidatorConfig,
        replay_path: &str,
        backend_bin: &str,
    ) -> Result<ReplayValidatorResult, ToolingError> {
        // Phase 1: run a search, save replay, capture manifest hash
        let original_hash = {
            let mut child = spawn_backend(backend_bin)?;
            let stdin = child.stdin.as_mut().ok_or(ToolingError::Io(
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "no stdin"),
            ))?;
            let rule_pool_json = serde_json::to_string(&config.rule_pool)
                .map_err(|e| ToolingError::Validation(e.to_string()))?;
            writeln!(
                stdin,
                r#"{{"type":"NewSearch","seed":{},"population_size":{},"max_generations":{},"rule_pool":{}}}"#,
                config.seed, config.population_size, config.max_generations, rule_pool_json
            )?;
            writeln!(stdin, r#"{{"type":"RunToEnd"}}"#)?;
            writeln!(stdin, r#"{{"type":"SaveReplay","path":"{}"}}"#, replay_path)?;
            writeln!(stdin, r#"{{"type":"GetCandidates","top_n":5}}"#)?;
            drop(child.stdin.take());

            let stdout = child.stdout.take().ok_or(ToolingError::Io(
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "no stdout"),
            ))?;
            let lines: Vec<String> = BufReader::new(stdout)
                .lines()
                .map_while(Result::ok)
                .collect();
            child.wait()?;
            extract_manifest_hash(&lines)?
        };

        // Phase 2: load replay and get candidates
        let replayed_hash = {
            let mut child = spawn_backend(backend_bin)?;
            let stdin = child.stdin.as_mut().ok_or(ToolingError::Io(
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "no stdin"),
            ))?;
            let rule_pool_json = serde_json::to_string(&config.rule_pool)
                .map_err(|e| ToolingError::Validation(e.to_string()))?;
            writeln!(
                stdin,
                r#"{{"type":"LoadReplay","path":"{}","rule_pool":{}}}"#,
                replay_path, rule_pool_json
            )?;
            drop(child.stdin.take());

            let stdout = child.stdout.take().ok_or(ToolingError::Io(
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "no stdout"),
            ))?;
            let lines: Vec<String> = BufReader::new(stdout)
                .lines()
                .map_while(Result::ok)
                .collect();
            child.wait()?;
            extract_manifest_hash(&lines)?
        };

        Ok(ReplayValidatorResult { replay_ok: original_hash == replayed_hash })
    }
}

