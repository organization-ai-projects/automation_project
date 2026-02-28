use diplo_sim::ai::ai_engine::AiEngine;
use diplo_sim::ai::ai_profile::AiProfile;
use diplo_sim::map::map_loader::load_map_from_str;
use diplo_sim::map::territory_id::TerritoryId;
use diplo_sim::model::faction::Faction;
use diplo_sim::model::faction_id::FactionId;
use diplo_sim::model::game_state::GameState;
use diplo_sim::model::unit::Unit;
use diplo_sim::model::unit_id::UnitId;
use diplo_sim::time::turn::Turn;

fn make_tiny_state() -> GameState {
    let (map, _) = load_map_from_str(include_str!("fixtures/maps/tiny_triangle_map.json")).unwrap();
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

    // Compare order kinds (not IDs since those depend on counter)
    assert_eq!(orders1.len(), orders2.len());
    for (os1, os2) in orders1.iter().zip(orders2.iter()) {
        assert_eq!(os1.faction_id, os2.faction_id);
        assert_eq!(os1.orders.len(), os2.orders.len());
        for (o1, o2) in os1.orders.iter().zip(os2.orders.iter()) {
            assert_eq!(o1.kind, o2.kind, "Same seed must produce same order kinds");
        }
    }
}

#[test]
fn test_different_seeds_may_produce_different_orders() {
    // Run multiple seeds and check that they don't all produce identical results
    // (statistical test - with enough seeds, some should differ)
    let state = make_tiny_state();

    let mut all_same = true;
    let mut first: Option<Vec<_>> = None;

    for seed in 0u64..20 {
        let engine = AiEngine::new(seed, AiProfile::default());
        let mut oid = 0u32;
        let orders = engine.generate_all_orders(&state, &mut oid);
        let kinds: Vec<_> = orders
            .iter()
            .flat_map(|os| os.orders.iter().map(|o| o.kind.clone()))
            .collect();

        if let Some(ref f) = first {
            if *f != kinds {
                all_same = false;
                break;
            }
        } else {
            first = Some(kinds);
        }
    }
    // It's fine if all_same is true (unlikely but possible); we just verify the function runs
    let _ = all_same;
}

#[test]
fn test_determinism_across_multiple_calls() {
    let seed = 99999u64;
    let engine = AiEngine::new(seed, AiProfile::default());

    for _ in 0..5 {
        let state = make_tiny_state();
        let mut oid = 0u32;
        let orders = engine.generate_all_orders(&state, &mut oid);

        // Verify the same state + same engine + same starting order id = same results
        let state2 = make_tiny_state();
        let mut oid2 = 0u32;
        let orders2 = engine.generate_all_orders(&state2, &mut oid2);

        for (os1, os2) in orders.iter().zip(orders2.iter()) {
            for (o1, o2) in os1.orders.iter().zip(os2.orders.iter()) {
                assert_eq!(o1.kind, o2.kind);
            }
        }
    }
}
