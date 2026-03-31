use crate::ability::ability::Ability;
use crate::ability::ability_id::AbilityId;
use crate::ability::ability_kind::AbilityKind;
use crate::ai::tactics_ai::TacticsAi;
use crate::grid::grid_map::GridMap;
use crate::grid::position::Position;
use crate::rng::seed::Seed;
use crate::rng::seeded_rng::SeededRng;
use crate::turn::action_entry::ActionEntry;
use crate::unit::team::Team;
use crate::unit::unit::Unit;
use crate::unit::unit_id::UnitId;
use std::collections::BTreeMap;

fn melee_ability() -> Ability {
    Ability {
        id: AbilityId(1),
        name: "Slash".to_string(),
        kind: AbilityKind::MeleeAttack,
        range: 1,
        power: 5,
    }
}

fn make_unit(id: u32, team: Team, x: i32, y: i32) -> Unit {
    Unit {
        id: UnitId(id),
        name: format!("Unit{id}"),
        team,
        position: Position::new(x, y),
        hp: 10,
        max_hp: 10,
        attack: 5,
        defense: 2,
        speed: 3,
        move_range: 3,
        abilities: vec![AbilityId(1)],
        alive: true,
    }
}

#[test]
fn ai_attacks_adjacent_enemy() {
    let units = vec![
        make_unit(1, Team::Player, 0, 0),
        make_unit(2, Team::Enemy, 1, 0),
    ];
    let mut abilities = BTreeMap::new();
    abilities.insert(AbilityId(1), melee_ability());
    let grid = GridMap::new(8, 8);
    let mut rng = SeededRng::new(Seed(42));

    let actions = TacticsAi::decide(UnitId(1), &units, &abilities, &grid, &mut rng);
    assert!(actions.iter().any(
        |a| matches!(a, ActionEntry::UseAbility { target_id, .. } if *target_id == UnitId(2))
    ));
}

#[test]
fn ai_moves_toward_distant_enemy() {
    let units = vec![
        make_unit(1, Team::Player, 0, 0),
        make_unit(2, Team::Enemy, 7, 0),
    ];
    let mut abilities = BTreeMap::new();
    abilities.insert(AbilityId(1), melee_ability());
    let grid = GridMap::new(8, 8);
    let mut rng = SeededRng::new(Seed(42));

    let actions = TacticsAi::decide(UnitId(1), &units, &abilities, &grid, &mut rng);
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, ActionEntry::Move { .. }))
    );
}

#[test]
fn ai_is_deterministic() {
    let units = vec![
        make_unit(1, Team::Player, 0, 0),
        make_unit(2, Team::Enemy, 3, 3),
    ];
    let mut abilities = BTreeMap::new();
    abilities.insert(AbilityId(1), melee_ability());
    let grid = GridMap::new(8, 8);

    let mut rng1 = SeededRng::new(Seed(42));
    let actions1 = TacticsAi::decide(UnitId(1), &units, &abilities, &grid, &mut rng1);

    let mut rng2 = SeededRng::new(Seed(42));
    let actions2 = TacticsAi::decide(UnitId(1), &units, &abilities, &grid, &mut rng2);

    assert_eq!(actions1.len(), actions2.len());
}
