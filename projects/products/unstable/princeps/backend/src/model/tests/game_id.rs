use crate::model::game_id::GameId;

#[test]
fn game_id_new_uses_seed_and_days() {
    let id = GameId::new(42, 30);
    assert_eq!(id.to_string(), "game-s42-d30");
}
