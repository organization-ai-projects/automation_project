use crate::parse_json::{format_ts, short_token};

#[test]
fn short_token_truncates_to_8_chars() {
    let value = short_token("abcdefghijklmnopqrstuvwxyz");
    assert_eq!(value, "abcdefgh...");
}

#[test]
fn short_token_keeps_short_values() {
    let value = short_token("abcd");
    assert_eq!(value, "abcd...");
}

#[test]
fn format_ts_formats_some_and_none() {
    assert_eq!(format_ts(Some(42)), "42");
    assert_eq!(format_ts(None), "-");
}
