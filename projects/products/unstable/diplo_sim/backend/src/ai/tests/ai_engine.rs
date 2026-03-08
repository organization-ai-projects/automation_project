use crate::ai::ai_engine::AiEngine;
use crate::ai::ai_profile::AiProfile;
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
fn ai_engine_generates_one_order_set_per_faction() {
    let state = GameState {
        units: vec![
            Unit {
                id: UnitId(0),
                faction_id: FactionId(1),
                territory_id: TerritoryId(1),
            },
            Unit {
                id: UnitId(1),
                faction_id: FactionId(2),
                territory_id: TerritoryId(2),
            },
        ],
        factions: vec![
            Faction {
                id: FactionId(1),
                name: "Blue".to_string(),
            },
            Faction {
                id: FactionId(2),
                name: "Red".to_string(),
            },
        ],
        current_turn: Turn::new(0),
        map_graph: MapGraph {
            name: "tiny".to_string(),
            version: "1".to_string(),
            territories: vec![
                Territory {
                    id: TerritoryId(1),
                    name: "A".to_string(),
                },
                Territory {
                    id: TerritoryId(2),
                    name: "B".to_string(),
                },
            ],
            adjacencies: vec![[TerritoryId(1), TerritoryId(2)]],
        },
    };

    let mut next_order_id = 0;
    let engine = AiEngine::new(123, AiProfile::default());
    let order_sets = engine.generate_all_orders(&state, &mut next_order_id);

    assert_eq!(order_sets.len(), 2);
    assert_eq!(order_sets[0].faction_id, FactionId(1));
    assert_eq!(order_sets[1].faction_id, FactionId(2));
}
