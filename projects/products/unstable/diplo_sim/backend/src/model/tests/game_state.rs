use crate::map::map_graph::MapGraph;
use crate::map::territory::Territory;
use crate::map::territory_id::TerritoryId;
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::time::turn::Turn;

#[test]
fn game_state_finds_units_by_id_and_territory() {
    let state = GameState {
        units: vec![Unit {
            id: UnitId(10),
            faction_id: FactionId(1),
            territory_id: TerritoryId(2),
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
                id: TerritoryId(2),
                name: "B".to_string(),
            }],
            adjacencies: vec![],
        },
    };

    assert!(state.unit_by_id(UnitId(10)).is_some());
    assert!(state.unit_at(TerritoryId(2)).is_some());
    assert!(state.unit_at(TerritoryId(99)).is_none());
}
