// projects/libraries/protocol_macros/src/tests/enum_utils.rs
use crate::enum_utils::to_snake_case;

#[test]
fn test_to_snake_case() {
    // Basic cases
    assert_eq!(to_snake_case("Ping"), "ping");
    assert_eq!(to_snake_case("Created"), "created");

    // CamelCase to snake_case
    assert_eq!(to_snake_case("DataReceived"), "data_received");
    assert_eq!(to_snake_case("UserLoggedIn"), "user_logged_in");

    // Acronym handling (premium feature!)
    assert_eq!(to_snake_case("HTTPRequest"), "http_request");
    assert_eq!(to_snake_case("HTTPServerError"), "http_server_error");
    assert_eq!(to_snake_case("XMLParser"), "xml_parser");
    assert_eq!(to_snake_case("JSONData"), "json_data");

    // Edge cases
    assert_eq!(to_snake_case("A"), "a");
    assert_eq!(to_snake_case("AB"), "ab");
    assert_eq!(to_snake_case("ABC"), "abc");
    assert_eq!(to_snake_case("ABCDef"), "abc_def");

    // Numbers in identifiers (regression prevention)
    assert_eq!(to_snake_case("V2"), "v2");
    assert_eq!(to_snake_case("HTTP2Request"), "http2_request");
    assert_eq!(to_snake_case("Version2Alpha"), "version2_alpha");
    assert_eq!(to_snake_case("Base64Encoder"), "base64_encoder");
    assert_eq!(to_snake_case("UTF8String"), "utf8_string");
}
