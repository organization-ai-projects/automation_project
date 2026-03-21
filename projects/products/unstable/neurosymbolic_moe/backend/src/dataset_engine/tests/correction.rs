use crate::dataset_engine::Correction;
use protocol::ProtocolId;
use std::str::FromStr;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

#[test]
fn correction_fields_round_trip() {
    let correction = Correction {
        entry_id: protocol_id(1),
        corrected_output: "new output".to_string(),
        reason: "manual review".to_string(),
        corrected_at: 456,
    };

    assert_eq!(correction.entry_id, protocol_id(1));
    assert_eq!(correction.corrected_output, "new output");
    assert_eq!(correction.reason, "manual review");
    assert_eq!(correction.corrected_at, 456);
}
