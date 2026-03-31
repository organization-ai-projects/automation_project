use crate::decision::CandidateAction;

#[test]
fn canonical_order_is_sell_hold_buy() {
    let order = CandidateAction::canonical_order();
    assert_eq!(
        order,
        vec![
            CandidateAction::Sell,
            CandidateAction::Hold,
            CandidateAction::BuyMore
        ]
    );
}

#[test]
fn serialization_roundtrip() {
    let action = CandidateAction::BuyMore;
    let json = common_json::to_json_string(&action).unwrap();
    let restored: CandidateAction = common_json::from_str(&json).unwrap();
    assert_eq!(action, restored);
}
