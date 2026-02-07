// projects/libraries/common_json/src/tests/parser.rs
use crate::parser::{parse_bytes, parse_reader, parse_str};
use super::test_helpers::{TestResult, assert_json_object};

#[test]
fn test_parse_str() -> TestResult {
    let json_str = "{\"key\":\"value\"}";
    let parsed = parse_str(json_str)?;
    assert_json_object(&parsed);
    Ok(())
}

#[test]
fn test_parse_bytes() -> TestResult {
    let json_bytes = b"{\"key\":\"value\"}";
    let parsed = parse_bytes(json_bytes)?;
    assert_json_object(&parsed);
    Ok(())
}

#[test]
fn test_parse_reader() -> TestResult {
    let json_data = "{\"key\":\"value\"}".as_bytes();
    let parsed = parse_reader(json_data)?;
    assert_json_object(&parsed);
    Ok(())
}
