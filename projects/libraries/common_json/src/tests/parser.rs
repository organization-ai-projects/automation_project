// projects/libraries/common_json/src/tests/parser.rs
use crate::parser::{parse_bytes, parse_reader, parse_str};

#[cfg(test)]
#[test]
fn test_parse_str() {
    let json_str = "{\"key\":\"value\"}";
    let parsed = parse_str(json_str);
    match parsed {
        Ok(json) => assert!(json.is_object()),
        Err(err) => panic!("Error parsing string: {:?}", err),
    }
}

#[test]
fn test_parse_bytes() {
    let json_bytes = b"{\"key\":\"value\"}";
    let parsed = parse_bytes(json_bytes);
    match parsed {
        Ok(json) => assert!(json.is_object()),
        Err(err) => panic!("Error parsing bytes: {:?}", err),
    }
}

#[test]
fn test_parse_reader() {
    let json_data = "{\"key\":\"value\"}".as_bytes();
    let parsed = parse_reader(json_data);
    match parsed {
        Ok(json) => assert!(json.is_object()),
        Err(err) => panic!("Error parsing reader: {:?}", err),
    }
}
