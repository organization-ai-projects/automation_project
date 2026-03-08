use crate::model::faction_id::FactionId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_parser::parse_order_set_from_str;

#[test]
fn order_parser_builds_order_set_from_json() {
    let json = r#"{
        "faction_id":1,
        "orders":[
            {"unit_id":4,"kind":"Hold"}
        ]
    }"#;

    let mut next_order_id = 10;
    let order_set = parse_order_set_from_str(json, &mut next_order_id).expect("parse");

    assert_eq!(order_set.faction_id, FactionId(1));
    assert_eq!(order_set.orders.len(), 1);
    assert_eq!(order_set.orders[0].id.0, 10);
    assert!(matches!(order_set.orders[0].kind, OrderKind::Hold));
    assert_eq!(next_order_id, 11);
}
