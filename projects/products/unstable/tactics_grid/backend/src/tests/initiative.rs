use crate::grid::position::Position;
use crate::turn::initiative::Initiative;
use crate::unit::team::Team;
use crate::unit::unit::Unit;
use crate::unit::unit_id::UnitId;

fn make_unit(id: u32, speed: i32) -> Unit {
    Unit {
        id: UnitId(id),
        name: format!("Unit{id}"),
        team: Team::Player,
        position: Position::new(0, 0),
        hp: 10,
        max_hp: 10,
        attack: 5,
        defense: 2,
        speed,
        move_range: 3,
        abilities: vec![],
        alive: true,
    }
}

#[test]
fn initiative_orders_by_speed_descending() {
    let units = vec![make_unit(1, 3), make_unit(2, 5), make_unit(3, 1)];
    let order = Initiative::compute_order(&units);
    assert_eq!(order, vec![UnitId(2), UnitId(1), UnitId(3)]);
}

#[test]
fn initiative_tie_break_by_lowest_id() {
    let units = vec![make_unit(3, 5), make_unit(1, 5), make_unit(2, 5)];
    let order = Initiative::compute_order(&units);
    assert_eq!(order, vec![UnitId(1), UnitId(2), UnitId(3)]);
}

#[test]
fn initiative_excludes_dead_units() {
    let mut units = vec![make_unit(1, 5), make_unit(2, 3)];
    units[0].alive = false;
    let order = Initiative::compute_order(&units);
    assert_eq!(order, vec![UnitId(2)]);
}
