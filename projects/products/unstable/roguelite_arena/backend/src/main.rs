mod combat;
mod config;
mod diagnostics;
mod events;
mod io;
mod loot;
mod model;
mod replay;
mod report;
mod rng;
mod scenarios;
mod snapshot;
mod time;

#[cfg(test)]
mod tests;

use std::{env, process};

use crate::combat::CombatEngine;
use crate::config::RunConfig;
use crate::diagnostics::RogueliteArenaError;
use crate::io::JsonCodec;
use crate::replay::ReplayCodec;
use crate::replay::ReplayEngine;
use crate::replay::ReplayFile;
use crate::rng::Seed;
use crate::scenarios::ScenarioLoader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(2);
    }

    let result = match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
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

fn cmd_run(args: &[String]) -> Result<String, RogueliteArenaError> {
    let config = RunConfig::parse(args)?;
    let scenario = if let Some(path) = &config.scenario_path {
        ScenarioLoader::load(path)?
    } else {
        ScenarioLoader::default_scenario("arena_basic")
    };

    let (report, draws) = CombatEngine::run(&scenario, config.ticks, config.seed)?;

    if let Some(ref out_path) = config.out_path {
        JsonCodec::save(&report, out_path)?;
    }

    if let Some(ref replay_path) = config.replay_out_path {
        let replay_file = ReplayFile {
            seed: Seed(config.seed),
            ticks: config.ticks,
            scenario_name: scenario.name.clone(),
            rng_draws: draws,
        };
        ReplayCodec::save(&replay_file, replay_path)?;
    }

    Ok(format!("Run complete. Hash: {}", report.run_hash.0))
}

fn cmd_replay(args: &[String]) -> Result<String, RogueliteArenaError> {
    let (replay_path, out_path) = parse_replay_args(args)?;
    let replay_file = ReplayCodec::load(&replay_path)?;
    let report = ReplayEngine::replay(&replay_file)?;

    if let Some(ref out) = out_path {
        JsonCodec::save(&report, out)?;
    }

    Ok(format!("Replay complete. Hash: {}", report.run_hash.0))
}

fn parse_replay_args(
    args: &[String],
) -> Result<(std::path::PathBuf, Option<std::path::PathBuf>), RogueliteArenaError> {
    let mut replay_path = None;
    let mut out_path = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--replay" => {
                i += 1;
                replay_path = Some(std::path::PathBuf::from(args.get(i).ok_or_else(|| {
                    RogueliteArenaError::InvalidConfig("missing --replay value".to_string())
                })?));
            }
            "--out" => {
                i += 1;
                out_path = Some(std::path::PathBuf::from(args.get(i).ok_or_else(|| {
                    RogueliteArenaError::InvalidConfig("missing --out value".to_string())
                })?));
            }
            _ => {}
        }
        i += 1;
    }
    let path = replay_path
        .ok_or_else(|| RogueliteArenaError::InvalidConfig("--replay is required".to_string()))?;
    Ok((path, out_path))
}

fn print_usage() {
    println!("roguelite_arena - deterministic roguelite arena simulation");
    println!();
    println!("Commands:");
    println!("  run --ticks N --seed S [--scenario <path>] --out <path> [--replay-out <path>]");
    println!("  replay --replay <path> --out <path>");
}
