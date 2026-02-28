mod actions;
mod debate;
mod diagnostics;
mod events;
mod model;
mod poll;
mod public_api;
mod replay;
mod report;
mod sim;

use crate::diagnostics::error::PrincepsError;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::sim::sim_engine::SimEngine;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), PrincepsError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        "export" => cmd_export(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            Ok(())
        }
    }
}

fn cmd_run(args: &[String]) -> Result<(), PrincepsError> {
    let mut days: u32 = 30;
    let mut seed: u64 = 42;
    let mut json_output = false;
    let mut replay_out: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--days" => {
                i += 1;
                days = args.get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30);
            }
            "--seed" => {
                i += 1;
                seed = args.get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(42);
            }
            "--json" => json_output = true,
            "--replay-out" => {
                i += 1;
                replay_out = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let mut engine = SimEngine::with_defaults(seed);
    let report = engine.run(days)?;

    if json_output {
        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        println!("{json}");
    } else {
        println!("{}", report.to_markdown());
    }

    if let Some(path) = replay_out {
        let json = serde_json::to_string_pretty(&engine.replay)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        std::fs::write(&path, json)?;
        eprintln!("Replay saved to {path}");
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
    let replay_file: ReplayFile = serde_json::from_str(&content)
        .map_err(|e| PrincepsError::Replay(e.to_string()))?;

    let engine = ReplayEngine;
    let report = engine.replay(&replay_file)?;

    if json_output {
        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        println!("{json}");
    } else {
        println!("{}", report.to_markdown());
    }

    Ok(())
}

fn cmd_export(args: &[String]) -> Result<(), PrincepsError> {
    let mut format = "markdown".to_string();
    let mut seed: u64 = 42;
    let mut days: u32 = 30;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--format" => {
                i += 1;
                format = args.get(i).cloned().unwrap_or_else(|| "markdown".to_string());
            }
            "--seed" => {
                i += 1;
                seed = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(42);
            }
            "--days" => {
                i += 1;
                days = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(30);
            }
            _ => {}
        }
        i += 1;
    }

    let mut engine = SimEngine::with_defaults(seed);
    let report = engine.run(days)?;

    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&report)
                .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
            println!("{json}");
        }
        _ => {
            println!("{}", report.to_markdown());
        }
    }

    Ok(())
}

fn print_usage() {
    println!("princeps — deterministic political campaign satire game");
    println!();
    println!("Commands:");
    println!("  run [--days N] [--seed S] [--json] [--replay-out <file>]");
    println!("  replay <replay_file.json> [--json]");
    println!("  export [--format markdown|json] [--seed S] [--days N]");
}
