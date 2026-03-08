use super::replay_file::ReplayFile;
use crate::adjudication::adjudication_engine::AdjudicationEngine;
use crate::diagnostics::error::DiploSimError;
use crate::map::map_loader::load_map_from_str;
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::report::match_report::MatchReport;
use crate::report::turn_report::TurnReport;
use crate::time::turn::Turn;

/// Replay a ReplayFile to produce a MatchReport.
pub fn replay(file: &ReplayFile) -> Result<MatchReport, DiploSimError> {
    let (map, starting_units) = load_map_from_str(&file.map_json)?;

    // Build factions from starting_units
    let mut faction_ids: Vec<u32> = starting_units.iter().map(|su| su.faction_id).collect();
    faction_ids.sort();
    faction_ids.dedup();

    let factions: Vec<Faction> = faction_ids
        .iter()
        .map(|&fid| Faction {
            id: FactionId(fid),
            name: format!("Faction{}", fid),
        })
        .collect();

    // Build initial units
    let mut units: Vec<Unit> = starting_units
        .iter()
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
        map_graph: map,
    };

    let mut engine = AdjudicationEngine::new(initial_state);
    let mut turn_reports: Vec<TurnReport> = Vec::new();

    for event in &file.event_log.events {
        let adjudication = engine.adjudicate(&event.order_sets);
        turn_reports.push(TurnReport {
            turn: event.turn,
            order_sets: event.order_sets.clone(),
            adjudication,
        });
    }

    Ok(MatchReport::build(
        file.map_name.clone(),
        file.seed,
        turn_reports,
    ))
}
