use crate::config::game_config::GameConfig;
use crate::io::json_codec::{decode, encode};

#[test]
fn json_codec_roundtrip_preserves_game_config() {
    let config = GameConfig::new(5, 1234, 3, "map.json".to_string());
    let json = encode(&config).expect("encode");
    let decoded: GameConfig = decode(&json).expect("decode");
    assert_eq!(decoded, config);
}
