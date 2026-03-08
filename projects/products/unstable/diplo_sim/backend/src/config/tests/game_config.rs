use crate::config::game_config::GameConfig;

#[test]
fn game_config_new_sets_all_fields() {
    let config = GameConfig::new(10, 999, 4, "maps/classic.json".to_string());
    assert_eq!(config.num_turns, 10);
    assert_eq!(config.seed, 999);
    assert_eq!(config.num_players, 4);
    assert_eq!(config.map_path, "maps/classic.json");
}
