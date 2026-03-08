use crate::adjudication::adjudication_engine::AdjudicationEngine;
use crate::ai::ai_engine::AiEngine;
use crate::ai::ai_profile::AiProfile;
use crate::config::game_config::GameConfig;
use crate::diagnostics::diplo_sim_error::DiploSimError;
use crate::io::json_codec::{decode, encode};
use crate::map::map_loader::{load_map_from_file, load_map_from_str};
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::replay::event_log::EventLog;
use crate::replay::replay_event::ReplayEvent;
use crate::replay::replay_file::ReplayFile;
use crate::report::match_report::MatchReport;
use crate::report::run_hash::{canonical_json_string, compute_run_hash_from_json};
use crate::report::turn_report::TurnReport;
use crate::time::phase::Phase;
use crate::time::turn::Turn;

fn serialize_canonical_match_report(report: &MatchReport) -> Result<String, DiploSimError> {
    let json = common_json::to_json(report)
        .map_err(|e| DiploSimError::Internal(format!("Serialize error: {e}")))?;
    Ok(canonical_json_string(&json))
}

pub fn run_simulation(
    num_turns: u32,
    seed: u64,
    map_path: &str,
    num_players: u32,
    out_path: &str,
    replay_out: Option<&str>,
) -> Result<(), DiploSimError> {
    if num_turns == 0 {
        return Err(DiploSimError::Config(
            "num_turns must be greater than zero".to_string(),
        ));
    }
    if num_players == 0 {
        return Err(DiploSimError::Config(
            "num_players must be greater than zero".to_string(),
        ));
    }

    let game_config = GameConfig::new(num_turns, seed, num_players, map_path.to_string());
    let map_json = std::fs::read_to_string(&game_config.map_path).map_err(|e| {
        DiploSimError::Io(format!("Cannot read map '{}': {}", game_config.map_path, e))
    })?;
    run_simulation_from_map_json(
        num_turns,
        seed,
        num_players,
        &map_json,
        out_path,
        replay_out,
    )
}

pub fn run_simulation_with_map_id(
    num_turns: u32,
    seed: u64,
    map_id: &str,
    num_players: u32,
    out_path: &str,
    replay_out: Option<&str>,
) -> Result<(), DiploSimError> {
    if num_turns == 0 {
        return Err(DiploSimError::Config(
            "num_turns must be greater than zero".to_string(),
        ));
    }
    if num_players == 0 {
        return Err(DiploSimError::Config(
            "num_players must be greater than zero".to_string(),
        ));
    }
    let Some(map_json) = crate::map::catalog::map_json_for_id(map_id) else {
        return Err(DiploSimError::Config(format!(
            "unknown map_id '{map_id}' (expected one of: {})",
            crate::map::catalog::available_map_ids().join(", ")
        )));
    };
    run_simulation_from_map_json(num_turns, seed, num_players, map_json, out_path, replay_out)
}

fn run_simulation_from_map_json(
    num_turns: u32,
    seed: u64,
    num_players: u32,
    map_json: &str,
    out_path: &str,
    replay_out: Option<&str>,
) -> Result<(), DiploSimError> {
    let game_config = GameConfig::new(num_turns, seed, num_players, "<embedded>".to_string());

    let (map, starting_units) = load_map_from_str(map_json)?;

    // Determine factions from starting_units, cap by num_players
    let mut faction_ids: Vec<u32> = starting_units.iter().map(|su| su.faction_id).collect();
    faction_ids.sort();
    faction_ids.dedup();
    let faction_ids: Vec<u32> = faction_ids
        .into_iter()
        .take(game_config.num_players as usize)
        .collect();

    let factions: Vec<Faction> = faction_ids
        .iter()
        .map(|&fid| Faction {
            id: FactionId(fid),
            name: format!("Faction{}", fid),
        })
        .collect();

    // Build initial units for valid factions
    let mut units: Vec<Unit> = starting_units
        .iter()
        .filter(|su| faction_ids.contains(&su.faction_id))
        .enumerate()
        .map(|(i, su)| Unit {
            id: UnitId(i as u32),
            faction_id: FactionId(su.faction_id),
            territory_id: crate::map::territory_id::TerritoryId(su.territory_id),
        })
        .collect();
    units.sort_by_key(|u| u.id);

    let initial_state = GameState {
        units,
        factions,
        current_turn: Turn::new(0),
        map_graph: map.clone(),
    };

    let mut engine = AdjudicationEngine::new(initial_state);
    let ai = AiEngine::new(game_config.seed, AiProfile::default());
    let mut event_log = EventLog::new();
    let mut turn_reports: Vec<TurnReport> = Vec::new();
    let mut next_order_id: u32 = 0;

    for turn_index in 0..game_config.num_turns {
        let turn = engine.current_state().current_turn;
        let order_sets = ai.generate_all_orders(engine.current_state(), &mut next_order_id);
        let adjudication = engine.adjudicate(&order_sets);

        event_log.push(ReplayEvent {
            turn,
            order_sets: order_sets.clone(),
        });

        turn_reports.push(TurnReport {
            turn,
            order_sets,
            adjudication,
        });
        let phase = Phase::Orders;
        tracing::debug!(
            "Completed simulation turn index {} phase {:?}",
            turn_index,
            phase
        );
    }

    let match_report = MatchReport::build(map.name.clone(), game_config.seed, turn_reports);

    // Write match report
    let report_json = serialize_canonical_match_report(&match_report)?;
    std::fs::write(out_path, &report_json)
        .map_err(|e| DiploSimError::Io(format!("Cannot write '{}': {}", out_path, e)))?;

    // Write replay file if requested
    if let Some(replay_path) = replay_out {
        let map_hash = compute_run_hash_from_json(map_json);
        let replay_file = ReplayFile {
            map_hash,
            map_name: map.name.clone(),
            map_json: map_json.to_string(),
            seed: game_config.seed,
            num_factions: game_config.num_players,
            event_log,
        };
        let replay_json = encode(&replay_file)?;
        std::fs::write(replay_path, &replay_json).map_err(|e| {
            DiploSimError::Io(format!("Cannot write replay '{}': {}", replay_path, e))
        })?;
    }

    tracing::info!("Run complete. RunHash: {}", match_report.run_hash);
    Ok(())
}

pub fn replay_simulation(replay_path: &str, out_path: &str) -> Result<(), DiploSimError> {
    let replay_json = std::fs::read_to_string(replay_path)
        .map_err(|e| DiploSimError::Io(format!("Cannot read replay '{}': {}", replay_path, e)))?;

    let replay_file: ReplayFile =
        decode(&replay_json).map_err(|e| DiploSimError::Replay(e.to_string()))?;

    let match_report = crate::replay::replay_engine::replay(&replay_file)?;

    let report_json = serialize_canonical_match_report(&match_report)?;
    std::fs::write(out_path, &report_json)
        .map_err(|e| DiploSimError::Io(format!("Cannot write '{}': {}", out_path, e)))?;

    tracing::info!("Replay complete. RunHash: {}", match_report.run_hash);
    Ok(())
}

pub fn validate_map(map_path: &str) -> Result<(), DiploSimError> {
    let map_json = std::fs::read_to_string(map_path)
        .map_err(|e| DiploSimError::Io(format!("Cannot read map '{}': {}", map_path, e)))?;
    let (map_graph_validated, starting_units) = load_map_from_file(map_path)?;
    tracing::debug!(
        "Validated map territories={} units={}",
        map_graph_validated.territory_count(),
        starting_units.len()
    );
    tracing::debug!("Loaded map bytes: {}", map_json.len());
    tracing::info!("Map '{}' is valid", map_path);
    Ok(())
}

pub fn validate_orders_cmd(map_path: &str, orders_path: &str) -> Result<(), DiploSimError> {
    let (loaded_map, starting_units) = load_map_from_file(map_path)?;
    let orders_json = std::fs::read_to_string(orders_path)
        .map_err(|e| DiploSimError::Io(format!("Cannot read orders '{}': {}", orders_path, e)))?;

    let mut next_order_id: u32 = 0;
    let order_set =
        crate::orders::order_parser::parse_order_set_from_str(&orders_json, &mut next_order_id)?;

    let (map_graph, loaded_units) = load_map_from_file(map_path)?;
    tracing::debug!(
        "Loaded map for order validation: territories={} units={}",
        loaded_map.territories.len(),
        loaded_units.len()
    );
    let factions: Vec<Faction> = starting_units
        .iter()
        .map(|su| su.faction_id)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|fid| Faction {
            id: FactionId(fid),
            name: format!("Faction{}", fid),
        })
        .collect();

    let units: Vec<Unit> = starting_units
        .iter()
        .enumerate()
        .map(|(i, su)| Unit {
            id: UnitId(i as u32),
            faction_id: FactionId(su.faction_id),
            territory_id: crate::map::territory_id::TerritoryId(su.territory_id),
        })
        .collect();

    let state = GameState {
        units,
        factions,
        current_turn: Turn::new(0),
        map_graph: map_graph.clone(),
    };

    let errors = crate::orders::order_validator::validate_order_set(&order_set, &state, &map_graph);
    if errors.is_empty() {
        tracing::info!("Orders are valid");
        Ok(())
    } else {
        for e in &errors {
            tracing::error!("Validation error: {}", e);
        }
        Err(DiploSimError::OrderValidation {
            order_id: crate::orders::order_id::OrderId(0),
            unit_id: UnitId(0),
            territory_id: crate::map::territory_id::TerritoryId(0),
            reason: format!("{} validation error(s)", errors.len()),
        })
    }
}
