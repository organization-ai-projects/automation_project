use crate::ai::ai_engine::AiEngine;
use crate::ai::ai_profile::AiProfile;
use crate::map::map_loader::load_map_from_str;
use crate::map::territory_id::TerritoryId;
use crate::model::faction::Faction;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit::Unit;
use crate::model::unit_id::UnitId;
use crate::time::turn::Turn;

fn make_tiny_state() -> GameState {
    let (map, _) =
        load_map_from_str(include_str!("fixtures/maps/tiny_triangle_map.json")).expect("map");
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
fn test_same_seed_produces_same_orders() {
    let state1 = make_tiny_state();
    let state2 = make_tiny_state();

    let seed = 12345u64;
    let engine1 = AiEngine::new(seed, AiProfile::default());
    let engine2 = AiEngine::new(seed, AiProfile::default());

    let mut oid1 = 0u32;
    let mut oid2 = 0u32;
    let orders1 = engine1.generate_all_orders(&state1, &mut oid1);
    let orders2 = engine2.generate_all_orders(&state2, &mut oid2);

    assert_eq!(orders1.len(), orders2.len());
    for (os1, os2) in orders1.iter().zip(orders2.iter()) {
        assert_eq!(os1.faction_id, os2.faction_id);
        assert_eq!(os1.orders.len(), os2.orders.len());
        for (o1, o2) in os1.orders.iter().zip(os2.orders.iter()) {
            assert_eq!(o1.kind, o2.kind, "same seed must produce same order kinds");
        }
    }
}
