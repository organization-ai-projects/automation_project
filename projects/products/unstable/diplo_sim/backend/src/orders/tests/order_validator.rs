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
use crate::orders::order_validator::validate_order_set;
use crate::time::turn::Turn;

#[test]
fn order_validator_rejects_non_adjacent_move() {
    let map = MapGraph {
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
        adjacencies: vec![],
    };

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
        map_graph: map.clone(),
    };

    let mut next_order_id = 0;
    let orders = OrderSet::from_raw(
        FactionId(1),
        vec![(
            UnitId(0),
            OrderKind::Move {
                target: TerritoryId(2),
            },
        )],
        &mut next_order_id,
    );

    let errors = validate_order_set(&orders, &state, &map);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("not adjacent"));
}
