use crate::diagnostics::diplo_sim_error::DiploSimError;
use crate::map::map_loader::load_map_from_str;
use crate::map::territory_id::TerritoryId;
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::orders::order::Order;
use crate::orders::order_id::OrderId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;
use crate::orders::order_validator::validate_order_set;
use crate::time::turn::Turn;

fn make_tiny_state() -> (GameState, crate::map::map_graph::MapGraph) {
    let (map, starting_units) =
        load_map_from_str(include_str!("fixtures/maps/tiny_triangle_map.json")).unwrap();
    let units: Vec<Unit> = starting_units
        .iter()
        .enumerate()
        .map(|(i, su)| Unit {
            id: UnitId(i as u32),
            faction_id: FactionId(su.faction_id),
            territory_id: TerritoryId(su.territory_id),
        })
        .collect();
    let factions = vec![
        Faction {
            id: FactionId(0),
            name: "F0".into(),
        },
        Faction {
            id: FactionId(1),
            name: "F1".into(),
        },
    ];
    let state = GameState {
        units,
        factions,
        current_turn: Turn::new(0),
        map_graph: map.clone(),
    };
    (state, map)
}

#[test]
fn test_valid_hold_order_passes() {
    let (state, map) = make_tiny_state();
    let order_set = OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(0),
            kind: OrderKind::Hold,
        }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(
        errors.is_empty(),
        "hold order should be valid: {:?}",
        errors
    );
}

#[test]
fn test_move_to_non_adjacent_territory_is_rejected() {
    let (map, starting_units) =
        load_map_from_str(include_str!("fixtures/maps/small_ring_map.json")).unwrap();
    let units: Vec<Unit> = starting_units
        .iter()
        .enumerate()
        .map(|(i, su)| Unit {
            id: UnitId(i as u32),
            faction_id: FactionId(su.faction_id),
            territory_id: TerritoryId(su.territory_id),
        })
        .collect();
    let state = GameState {
        units,
        factions: vec![
            Faction {
                id: FactionId(0),
                name: "F0".into(),
            },
            Faction {
                id: FactionId(1),
                name: "F1".into(),
            },
            Faction {
                id: FactionId(2),
                name: "F2".into(),
            },
        ],
        current_turn: Turn::new(0),
        map_graph: map.clone(),
    };

    let order_set = OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(0),
            kind: OrderKind::Move {
                target: TerritoryId(3),
            },
        }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(!errors.is_empty());

    let first_err = &errors[0];
    assert!(matches!(
        first_err,
        DiploSimError::OrderValidation {
            order_id,
            unit_id,
            territory_id,
            reason: _
        } if *order_id == OrderId(0)
            && *unit_id == UnitId(0)
            && *territory_id == TerritoryId(3)
    ));
}

#[test]
fn test_invalid_unit_id_is_rejected() {
    let (state, map) = make_tiny_state();
    let order_set = OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(999),
            kind: OrderKind::Hold,
        }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(!errors.is_empty());
    assert!(matches!(
        &errors[0],
        DiploSimError::OrderValidation { unit_id, .. } if *unit_id == UnitId(999)
    ));
}
