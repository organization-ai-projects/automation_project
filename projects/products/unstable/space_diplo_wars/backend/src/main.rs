#![allow(dead_code)]

mod ai;
mod config;
mod diagnostics;
mod diplomacy;
mod economy;
mod events;
mod fleets;
mod io;
mod map;
mod model;
mod orders;
mod queues;
mod replay;
mod report;
mod resolution;
mod scenarios;
mod snapshot;
mod tech;
mod time;
mod war;

use std::path::PathBuf;
use std::process;

use crate::ai::ai_engine::AiEngine;
use crate::ai::ai_profile::AiProfile;
use crate::config::game_config::GameConfig;
use crate::diagnostics::error::SpaceDiploWarsError;
use crate::economy::economy_engine::EconomyEngine;
use crate::io::json_codec::JsonCodec;
use crate::orders::order_set::OrderSet;
use crate::queues::queue_engine::QueueEngine;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_hash::RunHash;
use crate::report::run_report::RunReport;
use crate::report::turn_report::TurnReport;
use crate::resolution::resolution_engine::ResolutionEngine;
use crate::scenarios::scenario::Scenario;
use crate::scenarios::scenario_loader::ScenarioLoader;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::tech::tech_engine::TechEngine;
use crate::time::phase::Phase;

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");

    let result = match command {
        "run" => handle_run(&args[2..]),
        "replay" => handle_replay(&args[2..]),
        "snapshot" => handle_snapshot(&args[2..]),
        "validate" => handle_validate(&args[2..]),
        _ => Err(SpaceDiploWarsError::InvalidCli(
            "Usage: space_diplo_wars <run|replay|snapshot|validate> [OPTIONS]".into(),
        )),
    };

    match result {
        Ok(()) => process::exit(0),
        Err(SpaceDiploWarsError::InvalidCli(msg)) => {
            tracing::error!("Invalid CLI: {msg}");
            process::exit(2);
        }
        Err(
            SpaceDiploWarsError::InvalidScenario(msg) | SpaceDiploWarsError::InvalidOrders(msg),
        ) => {
            tracing::error!("Invalid scenario/config/orders: {msg}");
            process::exit(3);
        }
        Err(SpaceDiploWarsError::ReplayMismatch(msg)) => {
            tracing::error!("Replay mismatch: {msg}");
            process::exit(4);
        }
        Err(e) => {
            tracing::error!("Internal error: {e}");
            process::exit(5);
        }
    }
}

fn handle_run(args: &[String]) -> Result<(), SpaceDiploWarsError> {
    let mut turns: u64 = 10;
    let mut ticks_per_turn: u64 = 4;
    let mut seed: u64 = 42;
    let mut scenario_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut replay_out: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--turns" => {
                i += 1;
                turns = parse_u64_arg(args.get(i), "--turns")?;
            }
            "--ticks-per-turn" => {
                i += 1;
                ticks_per_turn = parse_u64_arg(args.get(i), "--ticks-per-turn")?;
            }
            "--seed" => {
                i += 1;
                seed = parse_u64_arg(args.get(i), "--seed")?;
            }
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(require_arg(args.get(i), "--scenario")?));
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(require_arg(args.get(i), "--out")?));
            }
            "--replay-out" => {
                i += 1;
                replay_out = Some(PathBuf::from(require_arg(args.get(i), "--replay-out")?));
            }
            flag => {
                return Err(SpaceDiploWarsError::InvalidCli(format!(
                    "Unknown flag: {flag}"
                )));
            }
        }
        i += 1;
    }

    let scenario_file = scenario_path
        .ok_or_else(|| SpaceDiploWarsError::InvalidCli("--scenario is required".into()))?;
    let report_file =
        out_path.ok_or_else(|| SpaceDiploWarsError::InvalidCli("--out is required".into()))?;

    let scenario = ScenarioLoader::load_from_file(&scenario_file)
        .map_err(|e| SpaceDiploWarsError::InvalidScenario(e.to_string()))?;

    let config = GameConfig {
        turns,
        ticks_per_turn,
        seed,
    };
    let (run_report, replay_file) = execute_run(&scenario, &config)?;

    let report_json = JsonCodec::encode(&run_report)?;
    std::fs::write(&report_file, report_json)?;

    if let Some(path) = replay_out {
        let replay_json = ReplayCodec::encode(&replay_file)?;
        std::fs::write(path, replay_json)?;
    }

    Ok(())
}

fn handle_replay(args: &[String]) -> Result<(), SpaceDiploWarsError> {
    let mut replay_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--replay" => {
                i += 1;
                replay_path = Some(PathBuf::from(require_arg(args.get(i), "--replay")?));
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(require_arg(args.get(i), "--out")?));
            }
            flag => {
                return Err(SpaceDiploWarsError::InvalidCli(format!(
                    "Unknown flag: {flag}"
                )));
            }
        }
        i += 1;
    }

    let replay_file = replay_path
        .ok_or_else(|| SpaceDiploWarsError::InvalidCli("--replay is required".into()))?;
    let out_file =
        out_path.ok_or_else(|| SpaceDiploWarsError::InvalidCli("--out is required".into()))?;

    let replay_json = std::fs::read_to_string(&replay_file)?;
    let replay = ReplayCodec::decode(&replay_json)
        .map_err(|e| SpaceDiploWarsError::ReplayMismatch(e.to_string()))?;

    let run_report = ReplayEngine::replay(&replay)?;
    std::fs::write(out_file, JsonCodec::encode(&run_report)?)?;
    Ok(())
}

fn handle_snapshot(args: &[String]) -> Result<(), SpaceDiploWarsError> {
    let mut replay_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut at_turn: Option<u64> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--replay" => {
                i += 1;
                replay_path = Some(PathBuf::from(require_arg(args.get(i), "--replay")?));
            }
            "--at-turn" => {
                i += 1;
                at_turn = Some(parse_u64_arg(args.get(i), "--at-turn")?);
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(require_arg(args.get(i), "--out")?));
            }
            flag => {
                return Err(SpaceDiploWarsError::InvalidCli(format!(
                    "Unknown flag: {flag}"
                )));
            }
        }
        i += 1;
    }

    let replay_file = replay_path
        .ok_or_else(|| SpaceDiploWarsError::InvalidCli("--replay is required".into()))?;
    let out_file =
        out_path.ok_or_else(|| SpaceDiploWarsError::InvalidCli("--out is required".into()))?;
    let turn =
        at_turn.ok_or_else(|| SpaceDiploWarsError::InvalidCli("--at-turn is required".into()))?;

    let replay_json = std::fs::read_to_string(&replay_file)?;
    let replay = ReplayCodec::decode(&replay_json)
        .map_err(|e| SpaceDiploWarsError::ReplayMismatch(e.to_string()))?;

    let snapshot = ReplayEngine::snapshot_at_turn(&replay, turn)?;
    std::fs::write(out_file, JsonCodec::encode(&snapshot)?)?;
    Ok(())
}

fn handle_validate(args: &[String]) -> Result<(), SpaceDiploWarsError> {
    let mut scenario_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(require_arg(args.get(i), "--scenario")?));
            }
            flag => {
                return Err(SpaceDiploWarsError::InvalidCli(format!(
                    "Unknown flag: {flag}"
                )));
            }
        }
        i += 1;
    }

    let scenario_file = scenario_path
        .ok_or_else(|| SpaceDiploWarsError::InvalidCli("--scenario is required".into()))?;

    let scenario = ScenarioLoader::load_from_file(&scenario_file)
        .map_err(|e| SpaceDiploWarsError::InvalidScenario(e.to_string()))?;

    if scenario.empires.is_empty() {
        return Err(SpaceDiploWarsError::InvalidScenario(
            "scenario must define at least one empire".into(),
        ));
    }
    if scenario.star_map.systems.is_empty() {
        return Err(SpaceDiploWarsError::InvalidScenario(
            "scenario must define at least one star system".into(),
        ));
    }

    Ok(())
}

fn require_arg<'a>(value: Option<&'a String>, flag: &str) -> Result<&'a str, SpaceDiploWarsError> {
    value
        .map(String::as_str)
        .ok_or_else(|| SpaceDiploWarsError::InvalidCli(format!("{flag} requires a value")))
}

fn parse_u64_arg(value: Option<&String>, flag: &str) -> Result<u64, SpaceDiploWarsError> {
    require_arg(value, flag)?
        .parse::<u64>()
        .map_err(|_| SpaceDiploWarsError::InvalidCli(format!("{flag} must be a number")))
}

fn execute_run(
    scenario: &Scenario,
    config: &GameConfig,
) -> Result<(RunReport, ReplayFile), SpaceDiploWarsError> {
    let mut state = scenario.build_initial_state();

    let scenario_json = JsonCodec::encode(scenario)?;
    let mut hasher = <sha2::Sha256 as sha2::Digest>::new();
    sha2::Digest::update(&mut hasher, scenario_json.as_bytes());
    let scenario_hash = hex::encode(sha2::Digest::finalize(hasher));

    let mut replay_file = ReplayFile::new(
        config.seed,
        config.ticks_per_turn,
        scenario_hash,
        scenario.clone(),
    );
    let mut turn_reports: Vec<TurnReport> = Vec::new();

    for turn in 1..=config.turns {
        for _ in 0..config.ticks_per_turn {
            state.current_phase = Phase::EconomyTick;
            EconomyEngine::tick(&mut state);
            QueueEngine::tick(&mut state);
            TechEngine::tick(&mut state);
            state.current_tick = crate::time::tick::Tick(state.current_tick.0 + 1);
        }

        state.current_phase = Phase::OrdersSubmit;
        let mut orders = scenario.orders_for_turn(turn);
        if orders.is_empty() {
            let mut empire_ids: Vec<_> = state.empires.keys().cloned().collect();
            empire_ids.sort_by(|a, b| a.0.cmp(&b.0));
            for empire_id in empire_ids {
                let ai_orders =
                    AiEngine::generate_orders(&empire_id, &AiProfile::default(), &state, turn);
                orders.extend(ai_orders);
            }
        }
        replay_file.orders_per_turn.insert(
            turn.to_string(),
            OrderSet {
                turn,
                orders: orders.clone(),
            },
        );
        if let Some(choices) = scenario.scripted_treaty_choices.get(&turn.to_string()) {
            replay_file
                .treaty_decisions
                .insert(turn.to_string(), choices.clone());
        }

        state.current_phase = Phase::OrdersResolve;
        let res_report = ResolutionEngine::resolve_turn(&mut state, &orders, turn);
        state.current_phase = Phase::Aftermath;

        for cp in &scenario.checkpoints {
            if cp.turn == turn
                && let Some(expected) = &cp.expected_snapshot_hash
            {
                let snapshot = StateSnapshot::from_state(&state);
                let computed = SnapshotHash::compute(&snapshot)?;
                if computed.0 != *expected {
                    return Err(SpaceDiploWarsError::ReplayMismatch(format!(
                        "checkpoint at turn {turn}: expected {expected}, got {}",
                        computed.0
                    )));
                }
            }
        }

        turn_reports.push(TurnReport {
            turn,
            battles: res_report.battles,
            diplomacy_events: res_report.diplomacy_events,
            validation_errors: res_report.validation_errors,
        });

        state.current_turn = crate::time::turn::Turn(turn);
    }

    let final_snapshot = StateSnapshot::from_state(&state);
    let snapshot_hash = SnapshotHash::compute(&final_snapshot)?;
    let run_report = RunReport {
        game_id: state.game_id.0.clone(),
        seed: config.seed,
        turns_played: config.turns,
        turn_reports,
        final_snapshot_hash: snapshot_hash.0,
    };

    // Validate canonical JSON can be hashed deterministically.
    let run_hash = RunHash::compute(&run_report)?;
    tracing::info!("RunHash: {}", run_hash.0);

    Ok((run_report, replay_file))
}
