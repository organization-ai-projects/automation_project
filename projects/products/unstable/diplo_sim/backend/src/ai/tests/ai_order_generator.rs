use crate::ai::ai_order_generator::generate_orders_for_faction;
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
fn order_generation_is_deterministic_for_same_seed_and_state() {
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
        current_turn: Turn::new(4),
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

    let profile = AiProfile {
        move_probability: 100,
    };

    let mut next_order_id_left = 0;
    let mut next_order_id_right = 0;
    let left = generate_orders_for_faction(
        777,
        state.current_turn.number,
        FactionId(1),
        &state,
        &profile,
        &mut next_order_id_left,
    );
    let right = generate_orders_for_faction(
        777,
        state.current_turn.number,
        FactionId(1),
        &state,
        &profile,
        &mut next_order_id_right,
    );

    assert_eq!(left, right);
    assert_eq!(next_order_id_left, next_order_id_right);
}
