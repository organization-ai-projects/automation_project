mod config;
mod diagnostics;
mod generator;
mod io;
mod model;
mod replay;
mod report;
mod rng;
mod runner;
mod shrinker;
mod targets;

#[cfg(test)]
mod tests;

use std::{env, process};

use crate::config::RunConfig;
use crate::diagnostics::FuzzHarnessError;
use crate::io::JsonCodec;
use crate::replay::ReplayEngine;
use crate::replay::ReplayFile;
use crate::runner::FuzzRunner;
use crate::shrinker::InputShrinker;
use crate::targets::resolve_target;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(2);
    }

    let result = match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        "shrink" => cmd_shrink(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(2);
        }
    };

    match result {
        Ok(msg) => {
            println!("{msg}");
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}

fn cmd_run(args: &[String]) -> Result<String, FuzzHarnessError> {
    let config = RunConfig::parse_run(args)?;
    let target = resolve_target(&config.target_name)?;

    let report = FuzzRunner::run(target.as_ref(), config.seed, config.iterations)?;

    if let (Some(replay_out), Some(first_failure)) =
        (&config.replay_out_path, report.failures.first())
    {
        let replay_file = ReplayFile {
            target_name: config.target_name.clone(),
            seed: config.seed,
            input: first_failure.input.clone(),
            failure_message: first_failure.message.clone(),
        };
        JsonCodec::save_replay_file(&replay_file, replay_out)?;
    }

    if config.json_output {
        let json = JsonCodec::to_json_pretty(&report)?;
        return Ok(json);
    }

    Ok(format!(
        "Run complete. Seed: {}, Iterations: {}, Failures: {}, Hash: {}",
        report.seed,
        report.iterations_run,
        report.failures.len(),
        report.run_hash.0
    ))
}

fn cmd_replay(args: &[String]) -> Result<String, FuzzHarnessError> {
    let config = RunConfig::parse_replay(args)?;
    let file_path = config.file_path.as_ref().ok_or_else(|| {
        FuzzHarnessError::InvalidConfig("--target <replay_file> is required for replay".to_string())
    })?;

    let replay_file = JsonCodec::load_replay_file(file_path)?;
    let target = resolve_target(&replay_file.target_name)?;

    let result = ReplayEngine::replay(target.as_ref(), &replay_file)?;

    Ok(format!("Replay complete. Result: {result}"))
}

fn cmd_shrink(args: &[String]) -> Result<String, FuzzHarnessError> {
    let config = RunConfig::parse_shrink(args)?;
    let file_path = config.file_path.as_ref().ok_or_else(|| {
        FuzzHarnessError::InvalidConfig("--target <replay_file> is required for shrink".to_string())
    })?;
    let out_path = config.out_path.as_ref().ok_or_else(|| {
        FuzzHarnessError::InvalidConfig("--out <file> is required for shrink".to_string())
    })?;

    let replay_file = JsonCodec::load_replay_file(file_path)?;
    let target = resolve_target(&replay_file.target_name)?;

    let shrink_report = InputShrinker::shrink(target.as_ref(), &replay_file)?;
    JsonCodec::save_shrink_report(&shrink_report, out_path)?;

    Ok(format!(
        "Shrink complete. Original size: {}, Shrunk size: {}, Steps: {}",
        shrink_report.original_size,
        shrink_report.shrunk_input.data.len(),
        shrink_report.shrink_steps
    ))
}

fn print_usage() {
    println!("fuzz_harness - deterministic fuzz runner for parsers/DSLs");
    println!();
    println!("Commands:");
    println!("  run --target <name> --seed <u64> --iters N [--json] [--replay-out <file>]");
    println!("  replay --target <file>");
    println!("  shrink --target <file> --out <file>");
}
