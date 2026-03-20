use crate::dataset_engine::Correction;
use protocol::ProtocolId;

fn protocol_id(_byte: u8) -> ProtocolId {
    ProtocolId::default()
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
