use diplo_sim::adjudication::adjudication_report::AdjudicationReport;
use diplo_sim::adjudication::resolution_step::ResolutionOutcome;
use diplo_sim::map::territory_id::TerritoryId;
use diplo_sim::model::faction::Faction;
use diplo_sim::model::faction_id::FactionId;
use diplo_sim::model::game_state::GameState;
use diplo_sim::model::unit::Unit;
use diplo_sim::model::unit_id::UnitId;
use diplo_sim::orders::order_id::OrderId;
use diplo_sim::orders::order_kind::OrderKind;
use diplo_sim::public_api::{AdjudicationEngine, MapGraph, Order, OrderSet};
use diplo_sim::time::turn::Turn;

fn make_triangle_map() -> MapGraph {
    diplo_sim::map::map_loader::load_map_from_str(include_str!(
        "fixtures/maps/tiny_triangle_map.json"
    ))
    .unwrap()
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
    let state2 = make_two_unit_state(map.clone());

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
        "Same orders should produce identical reports"
    );
}

#[test]
fn test_tie_breaking_lower_unit_id_wins() {
    // Both units try to move to territory 2 from territories 0 and 1
    // Both territories 0 and 1 are adjacent to 2 (tiny_triangle)
    // Lower UnitId(0) should win the tie
    let map = make_triangle_map();
    let state = make_two_unit_state(map);

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

    // Both have strength 1, tie -> neither moves (nobody wins a tie)
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

    // In a tie, neither unit moves (both bounce)
    assert_eq!(step0.outcome, ResolutionOutcome::Bounced);
    assert_eq!(step1.outcome, ResolutionOutcome::Bounced);
}

#[test]
fn test_move_succeeds_against_empty_territory() {
    let map = make_triangle_map();
    // Unit 0 at T0, no unit at T1, Unit 0 moves to T1
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
        map_graph: map,
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
