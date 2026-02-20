// projects/libraries/common_json/src/tests/parser.rs
use crate::Json;
use crate::parser::{parse_bytes, parse_reader, parse_str};

#[test]
fn test_parse_str() -> crate::JsonResult<()> {
    let json_str = "{\"key\":\"value\"}";
    let parsed = parse_str(json_str)?;
    assert!(matches!(parsed, Json::Object(_)));
    Ok(())
}

#[test]
fn test_parse_bytes() -> crate::JsonResult<()> {
    let json_bytes = b"{\"key\":\"value\"}";
    let parsed = parse_bytes(json_bytes)?;
    assert!(matches!(parsed, Json::Object(_)));
    Ok(())
}

#[test]
fn test_parse_reader() -> crate::JsonResult<()> {
    let json_data = "{\"key\":\"value\"}".as_bytes();
    let parsed = parse_reader(json_data)?;
    assert!(matches!(parsed, Json::Object(_)));
    Ok(())
}
