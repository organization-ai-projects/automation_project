use crate::map::territory_id::TerritoryId;

#[test]
fn territory_id_display_is_numeric() {
    assert_eq!(TerritoryId(42).to_string(), "42");
}
