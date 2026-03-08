use crate::orders::order_id::OrderId;

#[test]
fn order_id_display_is_numeric() {
    assert_eq!(OrderId(12).to_string(), "12");
}
