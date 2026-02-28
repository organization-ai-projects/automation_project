use std::collections::BTreeMap;

use space_diplo_wars::config::game_config::GameConfig;
use space_diplo_wars::diagnostics::error::SpaceDiploWarsError;
use space_diplo_wars::diplomacy::treaty_kind::TreatyKind;
use space_diplo_wars::economy::resource_wallet::ResourceWallet;
use space_diplo_wars::fleets::fleet::Fleet;
use space_diplo_wars::fleets::ship_kind::ShipKind;
use space_diplo_wars::io::json_codec::JsonCodec;
use space_diplo_wars::map::star_map::StarMap;
use space_diplo_wars::map::star_system_id::StarSystemId;
use space_diplo_wars::model::empire_id::EmpireId;
use space_diplo_wars::model::fleet_id::FleetId;
use space_diplo_wars::model::game_id::GameId;
use space_diplo_wars::model::sim_state::{Empire, SimState};
use space_diplo_wars::orders::order::Order;
use space_diplo_wars::orders::order_id::OrderId;
use space_diplo_wars::orders::order_kind::OrderKind;
use space_diplo_wars::orders::order_validator::OrderValidator;
use space_diplo_wars::report::run_report::RunReport;
use space_diplo_wars::report::turn_report::TurnReport;
use space_diplo_wars::resolution::resolution_engine::ResolutionEngine;
use space_diplo_wars::tech::tech_tree::TechTree;
use space_diplo_wars::war::battle_input::BattleInput;
use space_diplo_wars::war::battle_resolver::BattleResolver;

fn make_state_with_empire(empire_id: &str) -> SimState {
    let mut state = SimState::new(GameId("test".into()), StarMap::new());
    let empire = Empire {
        id: EmpireId(empire_id.into()),
        name: empire_id.into(),
        home_system: "sys_home".into(),
        resources: ResourceWallet::new(),
        tech_tree: TechTree::new(),
    };
    state.empires.insert(EmpireId(empire_id.into()), empire);
    state
}

fn make_fleet(fleet_id: &str, empire_id: &str, location: &str, fighters: u32) -> Fleet {
    let mut ships = BTreeMap::new();
    ships.insert(ShipKind::Fighter, fighters);
    Fleet {
        id: FleetId(fleet_id.into()),
        empire_id: EmpireId(empire_id.into()),
        ships,
        location: location.into(),
        destination: None,
    }
}

fn make_move_order(order_id: &str, empire_id: &str, fleet_id: &str, dest: &str) -> Order {
    let mut params = BTreeMap::new();
    params.insert("fleet_id".into(), fleet_id.into());
    params.insert("destination".into(), dest.into());
    Order {
        id: OrderId(order_id.into()),
        empire_id: EmpireId(empire_id.into()),
        kind: OrderKind::MoveFleet,
        params,
    }
}

#[test]
fn test_order_validation_deterministic() {
    let mut state = make_state_with_empire("empire_a");
    let fleet = make_fleet("fleet_1", "empire_a", "sys_alpha", 5);
    state.fleets.insert(FleetId("fleet_1".into()), fleet);

    let order = make_move_order("order_1", "empire_a", "fleet_1", "sys_beta");

    let result1 = OrderValidator::validate(&order, &state);
    let result2 = OrderValidator::validate(&order, &state);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn test_resolution_tie_breaker_deterministic() {
    let mut state1 = make_state_with_empire("empire_a");
    let fleet1 = make_fleet("fleet_1", "empire_a", "sys_alpha", 5);
    state1.fleets.insert(FleetId("fleet_1".into()), fleet1);

    let mut state2 = make_state_with_empire("empire_a");
    let fleet2 = make_fleet("fleet_1", "empire_a", "sys_alpha", 5);
    state2.fleets.insert(FleetId("fleet_1".into()), fleet2);

    let orders = vec![make_move_order(
        "order_1", "empire_a", "fleet_1", "sys_beta",
    )];

    let report1 = ResolutionEngine::resolve_turn(&mut state1, &orders, 1);
    let report2 = ResolutionEngine::resolve_turn(&mut state2, &orders, 1);

    assert_eq!(report1.validation_errors, report2.validation_errors);
    assert_eq!(report1.battles.len(), report2.battles.len());
}

#[test]
fn test_treaty_validation_deterministic() {
    let state = make_state_with_empire("empire_a");

    let mut params = BTreeMap::new();
    params.insert("target".into(), "empire_b".into());
    params.insert("treaty_kind".into(), "TradePact".into());
    let order = Order {
        id: OrderId("order_treaty".into()),
        empire_id: EmpireId("empire_a".into()),
        kind: OrderKind::OfferTreaty,
        params,
    };

    let r1 = OrderValidator::validate(&order, &state);
    let r2 = OrderValidator::validate(&order, &state);
    assert!(r1.is_ok());
    assert!(r2.is_ok());
}

#[test]
fn test_battle_resolver_deterministic() {
    let attacker = make_fleet("fleet_att", "empire_a", "sys_x", 10);
    let defender = make_fleet("fleet_def", "empire_b", "sys_x", 3);

    let input1 = BattleInput {
        attacker: attacker.clone(),
        defender: defender.clone(),
        location: StarSystemId("sys_x".into()),
    };
    let input2 = BattleInput {
        attacker,
        defender,
        location: StarSystemId("sys_x".into()),
    };

    let r1 = BattleResolver::resolve(input1);
    let r2 = BattleResolver::resolve(input2);

    assert_eq!(r1.attacker_wins, r2.attacker_wins);
    assert_eq!(r1.attacker_power, r2.attacker_power);
    assert_eq!(r1.defender_power, r2.defender_power);
    // Attacker has 10 fighters (2 attack each = 20) vs defender 3 fighters (1 defense = 3): attacker wins
    assert!(r1.attacker_wins);
}

#[test]
fn test_canonical_json_deterministic() {
    let report = RunReport {
        game_id: "test_game".into(),
        seed: 42,
        turns_played: 5,
        turn_reports: vec![TurnReport {
            turn: 1,
            battles: vec![],
            diplomacy_events: vec!["TreatyOffered:t1".into()],
            validation_errors: vec![],
        }],
        final_snapshot_hash: "abc123".into(),
    };

    let json1 = JsonCodec::encode(&report).unwrap();
    let json2 = JsonCodec::encode(&report).unwrap();

    assert_eq!(json1, json2);
    // Verify keys are sorted: "final_snapshot_hash" < "game_id" < "seed" < "turn_reports" < "turns_played"
    let pos_final = json1.find("final_snapshot_hash").unwrap();
    let pos_game = json1.find("game_id").unwrap();
    let pos_seed = json1.find("seed").unwrap();
    assert!(pos_final < pos_game);
    assert!(pos_game < pos_seed);
}
