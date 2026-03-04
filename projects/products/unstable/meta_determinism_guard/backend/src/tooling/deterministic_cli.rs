use crate::protocol::message::{write_stderr_line, write_stdout_line};
use crate::tooling::ruleset_loader;
use crate::tooling::tooling_error::ToolingError;
use anyhow::{Result, anyhow};

pub fn run_cli(args: &[String]) -> Result<i32> {
    if args.is_empty() {
        write_stderr_line("Usage: meta_determinism_guard_backend <command> [options]")?;
        write_stderr_line("Commands: validate, list-rules")?;
        return Ok(2);
    }

    match args[0].as_str() {
        "validate" => {
            let path = find_flag(args, "--ruleset").ok_or_else(|| anyhow!("--ruleset required"))?;
            let ruleset =
                ruleset_loader::load_from_file(&path).map_err(map_tooling_error_to_anyhow)?;
            write_stdout_line(&format!(
                "Ruleset '{}' is valid ({} rules).",
                ruleset.name,
                ruleset.rules.len()
            ))?;
            Ok(0)
        }
        "list-rules" => {
            let path = find_flag(args, "--ruleset").ok_or_else(|| anyhow!("--ruleset required"))?;
            let ruleset =
                ruleset_loader::load_from_file(&path).map_err(map_tooling_error_to_anyhow)?;
            write_stdout_line(&format!("Rules in '{}':", ruleset.name))?;
            for rule in &ruleset.rules {
                write_stdout_line(&format!("  - {} ({})", rule.name, rule.pattern))?;
            }
            Ok(0)
        }
        other => Err(anyhow!("Unknown command: {}", other)),
    }
}

fn find_flag(args: &[String], flag: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == flag)?;
    args.get(pos + 1).cloned()
}

fn map_tooling_error_to_anyhow(err: ToolingError) -> anyhow::Error {
    anyhow!(err.to_string())
}
