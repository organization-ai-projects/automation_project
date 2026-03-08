use crate::model::unit_id::UnitId;

#[test]
fn unit_id_display_is_numeric() {
    assert_eq!(UnitId(7).to_string(), "7");
}
