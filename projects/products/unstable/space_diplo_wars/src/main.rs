#![allow(dead_code)]

use std::path::PathBuf;
use std::process;

use space_diplo_wars::config::game_config::GameConfig;
use space_diplo_wars::diagnostics::error::SpaceDiploWarsError;
use space_diplo_wars::io::json_codec::JsonCodec;
use space_diplo_wars::orders;
use space_diplo_wars::replay::replay_file::ReplayFile;
use space_diplo_wars::report::run_hash::RunHash;
use space_diplo_wars::report::run_report::RunReport;
use space_diplo_wars::report::turn_report::TurnReport;
use space_diplo_wars::resolution::resolution_engine::ResolutionEngine;
use space_diplo_wars::scenario::scenario::Scenario;
use space_diplo_wars::scenario::scenario_loader::ScenarioLoader;
use space_diplo_wars::snapshot::snapshot_hash::SnapshotHash;
use space_diplo_wars::snapshot::state_snapshot::StateSnapshot;
use space_diplo_wars::time;

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();

    // Parse subcommand
    if args.len() < 2 || args[1] != "run" {
        eprintln!("Usage: space_diplo_wars run [OPTIONS]");
        process::exit(2);
    }

    match run_command(&args[2..]) {
        Ok(()) => process::exit(0),
        Err(SpaceDiploWarsError::InvalidCli(msg)) => {
            eprintln!("Invalid CLI: {}", msg);
            process::exit(2);
        }
        Err(
            SpaceDiploWarsError::InvalidScenario(msg) | SpaceDiploWarsError::InvalidOrders(msg),
        ) => {
            eprintln!("Invalid scenario/config/orders: {}", msg);
            process::exit(3);
        }
        Err(SpaceDiploWarsError::ReplayMismatch(msg)) => {
            eprintln!("Replay mismatch: {}", msg);
            process::exit(4);
        }
        Err(e) => {
            eprintln!("Internal error: {}", e);
            process::exit(5);
        }
    }
}

fn run_command(args: &[String]) -> Result<(), SpaceDiploWarsError> {
    // Parse: --turns N --ticks-per-turn K --seed S --scenario <file> --out <file>
    let mut turns: u64 = 10;
    let mut ticks_per_turn: u64 = 4;
    let mut seed: u64 = 42;
    let mut scenario_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--turns" => {
                i += 1;
                turns = args
                    .get(i)
                    .ok_or_else(|| {
                        SpaceDiploWarsError::InvalidCli("--turns requires value".into())
                    })?
                    .parse()
                    .map_err(|_| {
                        SpaceDiploWarsError::InvalidCli("--turns must be a number".into())
                    })?;
            }
            "--ticks-per-turn" => {
                i += 1;
                ticks_per_turn = args
                    .get(i)
                    .ok_or_else(|| {
                        SpaceDiploWarsError::InvalidCli("--ticks-per-turn requires value".into())
                    })?
                    .parse()
                    .map_err(|_| {
                        SpaceDiploWarsError::InvalidCli("--ticks-per-turn must be a number".into())
                    })?;
            }
            "--seed" => {
                i += 1;
                seed = args
                    .get(i)
                    .ok_or_else(|| SpaceDiploWarsError::InvalidCli("--seed requires value".into()))?
                    .parse()
                    .map_err(|_| {
                        SpaceDiploWarsError::InvalidCli("--seed must be a number".into())
                    })?;
            }
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(args.get(i).ok_or_else(|| {
                    SpaceDiploWarsError::InvalidCli("--scenario requires value".into())
                })?));
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(args.get(i).ok_or_else(|| {
                    SpaceDiploWarsError::InvalidCli("--out requires value".into())
                })?));
            }
            flag => {
                return Err(SpaceDiploWarsError::InvalidCli(format!(
                    "Unknown flag: {}",
                    flag
                )));
            }
        }
        i += 1;
    }

    let scenario_file = scenario_path
        .ok_or_else(|| SpaceDiploWarsError::InvalidCli("--scenario is required".into()))?;

    let scenario = ScenarioLoader::load_from_file(&scenario_file)
        .map_err(|e| SpaceDiploWarsError::InvalidScenario(e.to_string()))?;

    let config = GameConfig {
        turns,
        ticks_per_turn,
        seed,
    };
    let (run_report, replay_file) = execute_run(&scenario, &config)?;

    // Compute run hash
    let run_hash = RunHash::compute(&run_report)?;
    tracing::info!("RunHash: {}", run_hash.0);

    // Serialize outputs
    let report_json = JsonCodec::encode(&run_report)?;
    let replay_json = JsonCodec::encode(&replay_file)?;

    let output = format!(
        "{{\"run_report\":{},\"replay_file\":{},\"run_hash\":\"{}\"}}",
        report_json, replay_json, run_hash.0
    );

    if let Some(path) = out_path {
        std::fs::write(&path, &output)?;
        tracing::info!("Output written to {:?}", path);
    } else {
        println!("{}", output);
    }

    Ok(())
}

fn execute_run(
    scenario: &Scenario,
    config: &GameConfig,
) -> Result<(RunReport, ReplayFile), SpaceDiploWarsError> {
    let mut state = scenario.build_initial_state();

    // Compute scenario hash
    let scenario_json = JsonCodec::encode(scenario)?;
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    sha2::Digest::update(&mut hasher, scenario_json.as_bytes());
    let scenario_hash = hex::encode(sha2::Digest::finalize(hasher));

    let mut replay_file = ReplayFile::new(config.seed, scenario_hash);
    let mut turn_reports: Vec<TurnReport> = Vec::new();

    let total_turns = config.turns;

    for turn in 1..=total_turns {
        let orders = scenario.orders_for_turn(turn);

        // Record orders in replay
        let order_set = orders::order_set::OrderSet {
            turn,
            orders: orders.clone(),
        };
        replay_file
            .orders_per_turn
            .insert(turn.to_string(), order_set);

        let res_report = ResolutionEngine::resolve_turn(&mut state, &orders, turn);

        // Verify checkpoints
        for cp in &scenario.checkpoints {
            if cp.turn == turn {
                if let Some(expected) = &cp.expected_snapshot_hash {
                    let snap = StateSnapshot::from_state(&state);
                    let computed = SnapshotHash::compute(&snap)?;
                    if computed.0 != *expected {
                        return Err(SpaceDiploWarsError::ReplayMismatch(format!(
                            "Checkpoint at turn {}: expected {}, got {}",
                            turn, expected, computed.0
                        )));
                    }
                }
            }
        }

        turn_reports.push(TurnReport {
            turn,
            battles: res_report.battles,
            diplomacy_events: res_report.diplomacy_events,
            validation_errors: res_report.validation_errors,
        });

        state.current_turn = time::turn::Turn(turn);
        state.current_tick = time::tick::Tick(turn * config.ticks_per_turn);
    }

    let final_snapshot = StateSnapshot::from_state(&state);
    let snapshot_hash = SnapshotHash::compute(&final_snapshot)?;

    let run_report = RunReport {
        game_id: state.game_id.0.clone(),
        seed: config.seed,
        turns_played: total_turns,
        turn_reports,
        final_snapshot_hash: snapshot_hash.0,
    };

    Ok((run_report, replay_file))
}
