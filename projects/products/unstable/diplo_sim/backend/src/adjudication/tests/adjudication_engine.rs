use crate::adjudication::adjudication_engine::AdjudicationEngine;
use crate::adjudication::resolution_outcome::ResolutionOutcome;
use crate::map::map_graph::MapGraph;
use crate::map::territory::Territory;
use crate::map::territory_id::TerritoryId;
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;
use crate::time::turn::Turn;

#[test]
fn adjudication_engine_processes_hold_order_and_advances_turn() {
    let state = GameState {
        units: vec![Unit {
            id: UnitId(0),
            faction_id: FactionId(1),
            territory_id: TerritoryId(1),
        }],
        factions: vec![Faction {
            id: FactionId(1),
            name: "Blue".to_string(),
        }],
        current_turn: Turn::new(0),
        map_graph: MapGraph {
            name: "tiny".to_string(),
            version: "1".to_string(),
            territories: vec![Territory {
                id: TerritoryId(1),
                name: "A".to_string(),
            }],
            adjacencies: vec![],
        },
    };

    let mut next_order_id = 0;
    let order_set = OrderSet::from_raw(
        FactionId(1),
        vec![(UnitId(0), OrderKind::Hold)],
        &mut next_order_id,
    );

    let mut engine = AdjudicationEngine::new(state);
    let report = engine.adjudicate(&[order_set]);

    assert_eq!(report.steps.len(), 1);
    assert_eq!(report.steps[0].outcome, ResolutionOutcome::Stayed);
    assert_eq!(engine.current_state().current_turn, Turn::new(1));
}
