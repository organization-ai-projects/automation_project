use crate::model::faction_id::FactionId;

#[test]
fn faction_id_display_is_numeric() {
    assert_eq!(FactionId(3).to_string(), "3");
}
