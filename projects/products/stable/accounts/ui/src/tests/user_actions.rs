use crate::user_actions::parse_user_id;

#[test]
fn parse_user_id_accepts_valid_protocol_id() {
    let value =
        parse_user_id("00000000000000000000000000000000").expect("valid protocol id should parse");
    assert_eq!(value.to_string(), "00000000000000000000000000000000");
}

#[test]
fn parse_user_id_rejects_invalid_value() {
    let error = parse_user_id("invalid-id").expect_err("invalid id must fail");
    assert_eq!(error, "User ID must be 32 hex chars");
}
