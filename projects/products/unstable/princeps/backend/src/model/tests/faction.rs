use crate::model::faction::Faction;

#[test]
fn faction_new_starts_without_candidates() {
    let faction = Faction::new("centrist", "Centrist Bloc");
    assert_eq!(faction.id, "centrist");
    assert_eq!(faction.name, "Centrist Bloc");
    assert!(faction.candidates.is_empty());
}
