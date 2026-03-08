use crate::adjudication::adjudication_engine::AdjudicationEngine;
use crate::adjudication::resolution_step::ResolutionOutcome;
use crate::map::map_graph::MapGraph;
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
use crate::time::turn::Turn;

fn make_triangle_map() -> MapGraph {
    crate::map::map_loader::load_map_from_str(include_str!("fixtures/maps/tiny_triangle_map.json"))
        .expect("tiny triangle map should parse")
        .0
}

fn make_two_unit_state(map: MapGraph) -> GameState {
    GameState {
        units: vec![
            Unit {
                id: UnitId(0),
                faction_id: FactionId(0),
                territory_id: TerritoryId(0),
            },
            Unit {
                id: UnitId(1),
                faction_id: FactionId(1),
                territory_id: TerritoryId(1),
            },
        ],
        factions: vec![
            Faction {
                id: FactionId(0),
                name: "F0".into(),
            },
            Faction {
                id: FactionId(1),
                name: "F1".into(),
            },
        ],
        current_turn: Turn::new(0),
        map_graph: map,
    }
}

#[test]
fn test_adjudication_deterministic_same_orders() {
    let map = make_triangle_map();
    let state1 = make_two_unit_state(map.clone());
    let state2 = make_two_unit_state(map);

    let order_sets = vec![
        OrderSet::new(
            FactionId(0),
            vec![Order {
                id: OrderId(0),
                unit_id: UnitId(0),
                kind: OrderKind::Hold,
            }],
        ),
        OrderSet::new(
            FactionId(1),
            vec![Order {
                id: OrderId(1),
                unit_id: UnitId(1),
                kind: OrderKind::Hold,
            }],
        ),
    ];

    let mut engine1 = AdjudicationEngine::new(state1);
    let mut engine2 = AdjudicationEngine::new(state2);

    let report1 = engine1.adjudicate(&order_sets);
    let report2 = engine2.adjudicate(&order_sets);

    assert_eq!(
        report1, report2,
        "same orders should produce identical reports"
    );
}

#[test]
fn test_tie_breaking_bounces_both_units() {
    let order_sets = vec![
        OrderSet::new(
            FactionId(0),
            vec![Order {
                id: OrderId(0),
                unit_id: UnitId(0),
                kind: OrderKind::Move {
                    target: TerritoryId(2),
                },
            }],
        ),
        OrderSet::new(
            FactionId(1),
            vec![Order {
                id: OrderId(1),
                unit_id: UnitId(1),
                kind: OrderKind::Move {
                    target: TerritoryId(2),
                },
            }],
        ),
    ];

    let mut engine = AdjudicationEngine::new(make_two_unit_state(make_triangle_map()));
    let report = engine.adjudicate(&order_sets);

    let step0 = report
        .steps
        .iter()
        .find(|s| s.unit_id == UnitId(0))
        .unwrap();
    let step1 = report
        .steps
        .iter()
        .find(|s| s.unit_id == UnitId(1))
        .unwrap();

    assert_eq!(step0.outcome, ResolutionOutcome::Bounced);
    assert_eq!(step1.outcome, ResolutionOutcome::Bounced);
}

#[test]
fn test_move_succeeds_against_empty_territory() {
    let state = GameState {
        units: vec![Unit {
            id: UnitId(0),
            faction_id: FactionId(0),
            territory_id: TerritoryId(0),
        }],
        factions: vec![Faction {
            id: FactionId(0),
            name: "F0".into(),
        }],
        current_turn: Turn::new(0),
        map_graph: make_triangle_map(),
    };

    let order_sets = vec![OrderSet::new(
        FactionId(0),
        vec![Order {
            id: OrderId(0),
            unit_id: UnitId(0),
            kind: OrderKind::Move {
                target: TerritoryId(1),
            },
        }],
    )];

    let mut engine = AdjudicationEngine::new(state);
    let report = engine.adjudicate(&order_sets);

    let step = report
        .steps
        .iter()
        .find(|s| s.unit_id == UnitId(0))
        .unwrap();
    assert_eq!(step.outcome, ResolutionOutcome::Moved);
    assert_eq!(step.to, TerritoryId(1));
    assert_eq!(engine.state.units[0].territory_id, TerritoryId(1));
}
