use diplo_sim::diagnostics::error::DiploSimError;
use diplo_sim::map::map_loader::load_map_from_str;
use diplo_sim::map::territory_id::TerritoryId;
use diplo_sim::model::faction::Faction;
use diplo_sim::model::faction_id::FactionId;
use diplo_sim::model::game_state::GameState;
use diplo_sim::model::unit::Unit;
use diplo_sim::model::unit_id::UnitId;
use diplo_sim::orders::order_id::OrderId;
use diplo_sim::orders::order_kind::OrderKind;
use diplo_sim::orders::order::Order;
use diplo_sim::orders::order_set::OrderSet;
use diplo_sim::orders::order_validator::validate_order_set;
use diplo_sim::time::turn::Turn;

fn make_tiny_state() -> (GameState, diplo_sim::map::map_graph::MapGraph) {
    let (map, starting_units) = load_map_from_str(
        include_str!("fixtures/maps/tiny_triangle_map.json")
    ).unwrap();
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
        Faction { id: FactionId(0), name: "F0".into() },
        Faction { id: FactionId(1), name: "F1".into() },
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
        vec![Order { id: OrderId(0), unit_id: UnitId(0), kind: OrderKind::Hold }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(errors.is_empty(), "Hold order should be valid: {:?}", errors);
}

#[test]
fn test_valid_move_to_adjacent_territory_passes() {
    let (state, map) = make_tiny_state();
    // Unit 0 is at T0, T1 is adjacent
    let order_set = OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(0),
            kind: OrderKind::Move { target: TerritoryId(1) },
        }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(errors.is_empty(), "Move to adjacent territory should be valid: {:?}", errors);
}

#[test]
fn test_move_to_non_adjacent_territory_is_rejected() {
    // tiny_triangle has T0-T1, T1-T2, T0-T2 - all connected, so let's use small_ring
    let (map, starting_units) = load_map_from_str(
        include_str!("fixtures/maps/small_ring_map.json")
    ).unwrap();
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
            Faction { id: FactionId(0), name: "F0".into() },
            Faction { id: FactionId(1), name: "F1".into() },
            Faction { id: FactionId(2), name: "F2".into() },
        ],
        current_turn: Turn::new(0),
        map_graph: map.clone(),
    };

    // Unit 0 is at T0, T3 is NOT adjacent (ring: T0-T1-T2-T3-T4-T5-T0)
    let order_set = OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(0),
            kind: OrderKind::Move { target: TerritoryId(3) },
        }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(!errors.is_empty(), "Move to non-adjacent territory should be rejected");

    // Verify error is structured with order_id, unit_id, territory_id
    let first_err = &errors[0];
    assert!(matches!(
        first_err,
        DiploSimError::OrderValidation { order_id, unit_id, territory_id, reason: _ }
        if *order_id == OrderId(0) && *unit_id == UnitId(0) && *territory_id == TerritoryId(3)
    ), "Error should reference the correct order_id, unit_id, and territory_id. Got: {:?}", first_err);
}

#[test]
fn test_invalid_unit_id_is_rejected() {
    let (state, map) = make_tiny_state();
    let order_set = OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(999),  // doesn't exist
            kind: OrderKind::Hold,
        }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(!errors.is_empty(), "Order with invalid unit_id should be rejected");
    let first = &errors[0];
    assert!(matches!(first, DiploSimError::OrderValidation { unit_id, .. } if *unit_id == UnitId(999)));
}

#[test]
fn test_order_for_wrong_faction_is_rejected() {
    let (state, map) = make_tiny_state();
    // Unit 0 belongs to faction 0, but orders come from faction 1
    let order_set = OrderSet::new(
        FactionId(1),
        vec![Order { id: OrderId(0), unit_id: UnitId(0), kind: OrderKind::Hold }],
    );
    let errors = validate_order_set(&order_set, &state, &map);
    assert!(!errors.is_empty(), "Ordering a unit from wrong faction should be rejected");
}
