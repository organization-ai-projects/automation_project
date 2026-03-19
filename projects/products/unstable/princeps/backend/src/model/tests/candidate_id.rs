use crate::model::candidate_id::CandidateId;

#[test]
fn candidate_id_display_roundtrip() {
    let id = CandidateId::new("cand-42");
    assert_eq!(id.to_string(), "cand-42");
}
