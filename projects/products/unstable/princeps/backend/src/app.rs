use crate::diagnostics::error::PrincepsError;
use crate::protocol::console;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::sim::sim_engine::SimEngine;

pub fn run(args: Vec<String>) -> Result<(), PrincepsError> {
    if args.len() < 2 {
        console::print_usage();
        return Ok(());
    }
    match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        "export" => cmd_export(&args[2..]),
        other => {
            console::print_error_line(&format!("Unknown command: {other}"));
            console::print_usage();
            Ok(())
        }
    }
}

fn cmd_run(args: &[String]) -> Result<(), PrincepsError> {
    let mut days: u32 = 30;
    let mut seed: u64 = 42;
    let mut json_output = false;
    let mut replay_out: Option<String> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--days" => {
                index += 1;
                days = args.get(index).and_then(|s| s.parse().ok()).unwrap_or(30);
            }
            "--seed" => {
                index += 1;
                seed = args.get(index).and_then(|s| s.parse().ok()).unwrap_or(42);
            }
            "--json" => json_output = true,
            "--replay-out" => {
                index += 1;
                replay_out = args.get(index).cloned();
            }
            _ => {}
        }
        index += 1;
    }

    let mut engine = SimEngine::with_defaults(seed);
    let report = engine.run(days)?;

    if json_output {
        let json = common_json::to_json_string_pretty(&report)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        console::print_line(&json);
    } else {
        console::print_line(&report.to_markdown());
    }

    if let Some(path) = replay_out {
        let json = common_json::to_json_string_pretty(&engine.replay)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        std::fs::write(&path, json)?;
        console::print_error_line(&format!("Replay saved to {path}"));
    }

    Ok(())
}

fn cmd_replay(args: &[String]) -> Result<(), PrincepsError> {
    if args.is_empty() {
        return Err(PrincepsError::InvalidArgument(
            "Usage: princeps replay <replay_file.json> [--json]".into(),
        ));
    }
    let path = &args[0];
    let json_output = args.iter().any(|a| a == "--json");

    let content = std::fs::read_to_string(path)?;
    let replay_file: ReplayFile =
        common_json::from_json_str(&content).map_err(|e| PrincepsError::Replay(e.to_string()))?;

    let engine = ReplayEngine;
    let report = engine.replay(&replay_file)?;

    if json_output {
        let json = common_json::to_json_string_pretty(&report)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        console::print_line(&json);
    } else {
        console::print_line(&report.to_markdown());
    }

    Ok(())
}

fn cmd_export(args: &[String]) -> Result<(), PrincepsError> {
    let mut format = "markdown".to_string();
    let mut seed: u64 = 42;
    let mut days: u32 = 30;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                index += 1;
                format = args
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| "markdown".to_string());
            }
            "--seed" => {
                index += 1;
                seed = args.get(index).and_then(|s| s.parse().ok()).unwrap_or(42);
            }
            "--days" => {
                index += 1;
                days = args.get(index).and_then(|s| s.parse().ok()).unwrap_or(30);
            }
            _ => {}
        }
        index += 1;
    }

    let mut engine = SimEngine::with_defaults(seed);
    let report = engine.run(days)?;

    if format == "json" {
        let json = common_json::to_json_string_pretty(&report)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        console::print_line(&json);
    } else {
        console::print_line(&report.to_markdown());
    }

    Ok(())
}
