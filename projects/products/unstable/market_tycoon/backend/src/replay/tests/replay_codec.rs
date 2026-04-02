use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_file::ReplayFile;

#[test]
fn encode_decode_roundtrip() {
    let rf = ReplayFile::new(42, 10, vec![]);
    let encoded = ReplayCodec::encode(&rf).unwrap();
    let decoded = ReplayCodec::decode(&encoded).unwrap();
    assert_eq!(decoded.seed, 42);
    assert_eq!(decoded.ticks, 10);
}
