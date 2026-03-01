mod config;
mod diagnostics;
mod events;
mod hauling;
mod io;
mod jobs;
mod map;
mod model;
mod mood;
mod needs;
mod public_api;
mod replay;
mod report;
mod rng;
mod scenario;
mod sim_engine;
mod snapshot;
mod time;

#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(2);
    }
    match args[1].as_str() {
        "run" => {
            if let Err(e) = cmd_run(&args[2..]) {
                eprintln!("Error: {e}");
                process::exit(match e {
                    diagnostics::error::ColonyManagerError::Io(_) => 3,
                    diagnostics::error::ColonyManagerError::Json(_) => 3,
                    diagnostics::error::ColonyManagerError::Sim(_) => 4,
                    diagnostics::error::ColonyManagerError::ReplayMismatch(_) => 5,
                    diagnostics::error::ColonyManagerError::InvalidScenario(_) => 2,
                });
            }
        }
        "replay" => {
            if let Err(e) = cmd_replay(&args[2..]) {
                eprintln!("Error: {e}");
                process::exit(match e {
                    diagnostics::error::ColonyManagerError::Io(_) => 3,
                    diagnostics::error::ColonyManagerError::Json(_) => 3,
                    diagnostics::error::ColonyManagerError::Sim(_) => 4,
                    diagnostics::error::ColonyManagerError::ReplayMismatch(_) => 5,
                    diagnostics::error::ColonyManagerError::InvalidScenario(_) => 2,
                });
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(2);
        }
    }
}

fn cmd_run(args: &[String]) -> Result<(), diagnostics::error::ColonyManagerError> {
    let mut ticks: u64 = 100;
    let mut seed: u64 = 42;
    let mut scenario_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut replay_out: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ticks" => { i += 1; ticks = args[i].parse().unwrap_or(100); }
            "--seed" => { i += 1; seed = args[i].parse().unwrap_or(42); }
            "--scenario" => { i += 1; scenario_path = Some(PathBuf::from(&args[i])); }
            "--out" => { i += 1; out_path = Some(PathBuf::from(&args[i])); }
            "--replay-out" => { i += 1; replay_out = Some(PathBuf::from(&args[i])); }
            _ => {}
        }
        i += 1;
    }

    let out = out_path.ok_or_else(|| diagnostics::error::ColonyManagerError::Sim("--out required".to_string()))?;

    let scenario = if let Some(p) = scenario_path {
        scenario::scenario_loader::ScenarioLoader::load(&p)?
    } else {
        scenario::scenario_loader::ScenarioLoader::default_scenario("hauling_basic")
    };

    let (report, rng_draws) = sim_engine::SimEngine::run(&scenario, ticks, seed)?;
    io::json_codec::JsonCodec::save(&report, &out)?;

    if let Some(rp) = replay_out {
        let rf = replay::replay_file::ReplayFile {
            seed: rng::seed::Seed(seed),
            ticks,
            scenario_name: scenario.name.clone(),
            rng_draws,
        };
        replay::replay_codec::ReplayCodec::save(&rf, &rp)?;
    }

    println!("Run complete. RunHash: {}", report.run_hash.0);
    Ok(())
}

fn cmd_replay(args: &[String]) -> Result<(), diagnostics::error::ColonyManagerError> {
    let mut replay_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--replay" => { i += 1; replay_path = Some(PathBuf::from(&args[i])); }
            "--out" => { i += 1; out_path = Some(PathBuf::from(&args[i])); }
            _ => {}
        }
        i += 1;
    }

    let rp = replay_path.ok_or_else(|| diagnostics::error::ColonyManagerError::Sim("--replay required".to_string()))?;
    let out = out_path.ok_or_else(|| diagnostics::error::ColonyManagerError::Sim("--out required".to_string()))?;

    let replay_file = replay::replay_codec::ReplayCodec::load(&rp)?;
    let report = replay::replay_engine::ReplayEngine::replay(&replay_file)?;
    io::json_codec::JsonCodec::save(&report, &out)?;

    println!("Replay complete. RunHash: {}", report.run_hash.0);
    Ok(())
}

fn print_usage() {
    println!("colony_manager - deterministic colony simulation");
    println!();
    println!("Commands:");
    println!("  run --ticks N --seed S [--scenario <path>] --out <path> [--replay-out <path>]");
    println!("  replay --replay <path> --out <path>");
}
