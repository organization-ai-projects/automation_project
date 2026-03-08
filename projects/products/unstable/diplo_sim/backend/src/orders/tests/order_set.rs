use crate::model::faction_id::FactionId;
use crate::model::unit_id::UnitId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;

#[test]
fn order_set_from_raw_assigns_incremental_ids() {
    let mut next_order_id = 5;
    let order_set = OrderSet::from_raw(
        FactionId(2),
        vec![
            (UnitId(10), OrderKind::Hold),
            (
                UnitId(11),
                OrderKind::Move {
                    target: crate::map::territory_id::TerritoryId(3),
                },
            ),
        ],
        &mut next_order_id,
    );

    assert_eq!(order_set.faction_id, FactionId(2));
    assert_eq!(order_set.orders.len(), 2);
    assert_eq!(order_set.orders[0].id.0, 5);
    assert_eq!(order_set.orders[1].id.0, 6);
    assert_eq!(next_order_id, 7);
}
