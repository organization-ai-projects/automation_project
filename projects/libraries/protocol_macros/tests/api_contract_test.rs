//! API Contract Tests - Verify public behavior of the macro
//!
//! These tests ensure that the public API of EnumMethods works as documented
//! and catches regressions in the generated code behavior.
// projects/libraries/protocol_macros/tests/api_contract_test.rs
mod common;

use common::{assert_contains_all, assert_empty_struct_format};
use protocol_macros::EnumMethods;

/// Test 1: Debug mode actually uses Debug formatting
#[test]
fn test_debug_mode_uses_debug_formatting() {
    #[derive(Debug, EnumMethods)]
    #[enum_methods(mode = "debug")]
    enum Event {
        Data(Vec<u8>),
    }

    let event = Event::data(vec![0xFF, 0xAA, 0xBB]);
    let display_str = event.to_string();

    // Debug formatting should show the bytes as [255, 170, 187]
    assert!(
        display_str.contains("[255, 170, 187]"),
        "Expected Debug formatting with decimal values, got: {}",
        display_str
    );
}

/// Test 2: Case-insensitive mode attribute
#[test]
fn test_mode_is_case_insensitive() {
    // Test lowercase
    #[derive(Debug, EnumMethods)]
    #[enum_methods(mode = "debug")]
    enum Event1 {
        Data(Vec<u8>),
    }

    // Test uppercase
    #[derive(Debug, EnumMethods)]
    #[enum_methods(mode = "Debug")]
    enum Event2 {
        Data(Vec<u8>),
    }

    // Test all caps
    #[derive(Debug, EnumMethods)]
    #[enum_methods(mode = "DEBUG")]
    enum Event3 {
        Data(Vec<u8>),
    }

    let e1 = Event1::data(vec![1, 2, 3]);
    let e2 = Event2::data(vec![1, 2, 3]);
    let e3 = Event3::data(vec![1, 2, 3]);

    // All should use Debug formatting
    assert!(e1.to_string().contains("[1, 2, 3]"));
    assert!(e2.to_string().contains("[1, 2, 3]"));
    assert!(e3.to_string().contains("[1, 2, 3]"));
}

/// Test 3: Numbers in variant names are handled correctly
#[test]
fn test_numbers_in_variant_names() {
    #[derive(Debug, EnumMethods)]
    enum Protocol {
        HTTP2Request,
        UTF8String,
        Base64Encoder,
    }

    let http2 = Protocol::http2_request();
    let utf8 = Protocol::utf8_string();
    let base64 = Protocol::base64_encoder();

    assert_eq!(http2.as_str(), "http2_request");
    assert_eq!(utf8.as_str(), "utf8_string");
    assert_eq!(base64.as_str(), "base64_encoder");
}

/// Test 4: Const as_str() can be used in const contexts
#[test]
fn test_as_str_in_const_context() {
    #[derive(Debug, EnumMethods)]
    enum Event {
        Ping,
        Created { id: &'static str },
    }

    const PING_NAME: &str = {
        let event = Event::Ping;
        event.as_str()
    };

    const CREATED_NAME: &str = {
        let event = Event::Created { id: "123" };
        event.as_str()
    };

    assert_eq!(PING_NAME, "ping");
    assert_eq!(CREATED_NAME, "created");
}

/// Test 5: Display format matches documentation
#[test]
fn test_display_format_matches_docs() {
    #[derive(Debug, EnumMethods)]
    enum Event {
        Ping,
        Created { id: String, name: String },
        Data(String, u32),
    }

    let ping = Event::ping();
    let created = Event::created("123".into(), "test".into());
    let data = Event::data("info".into(), 42);

    // Unit variant: just the name
    assert_eq!(ping.to_string(), "ping");

    // Struct variant: name { field1=value1, field2=value2 }
    assert_contains_all(&created.to_string(), &["created", "id=123", "name=test"]);

    // Tuple variant: name(arg0=value0, arg1=value1)
    assert_contains_all(&data.to_string(), &["data", "arg0=info", "arg1=42"]);
}

/// Test 6: Acronyms are handled correctly in generated method names
#[test]
fn test_acronym_handling_in_methods() {
    #[derive(Debug, EnumMethods)]
    enum Status {
        HTTPRequest,
        XMLParser,
        JSONData,
    }

    // Methods should be snake_case with proper acronym handling
    let http = Status::http_request();
    let xml = Status::xml_parser();
    let json = Status::json_data();

    assert_eq!(http.as_str(), "http_request");
    assert_eq!(xml.as_str(), "xml_parser");
    assert_eq!(json.as_str(), "json_data");
}

/// Test 7: as_str() returns only variant name, not full Display output
#[test]
fn test_as_str_vs_display_distinction() {
    #[derive(Debug, EnumMethods)]
    enum Event {
        Created { id: String, data: String },
        Ping,
        Data(String, u32),
    }

    let created = Event::created("123".into(), "test".into());
    let ping = Event::ping();
    let data = Event::data("info".into(), 42);

    // as_str() should return just the variant name
    assert_eq!(created.as_str(), "created");
    assert_eq!(ping.as_str(), "ping");
    assert_eq!(data.as_str(), "data");

    // Display should return the full formatted output
    assert_contains_all(&created.to_string(), &["created", "id=123", "data=test"]);
    assert_eq!(ping.to_string(), "ping");
    assert_contains_all(&data.to_string(), &["data", "arg0=info", "arg1=42"]);
}

/// Test 8: Constructor type inference works correctly
#[test]
fn test_constructor_type_inference() {
    #[derive(Debug, EnumMethods)]
    enum Event {
        Message { content: String },
        Data(String, u32),
    }

    // Type inference should work with .into()
    let msg = Event::message("hello".into());
    let data = Event::data("world".into(), 42);

    assert_eq!(msg.as_str(), "message");
    assert_eq!(data.as_str(), "data");
}

/// Test 9: Empty struct variant edge case
#[test]
fn test_empty_struct_variant() {
    #[derive(Debug, EnumMethods)]
    enum Event {
        Empty {},
    }

    let empty = Event::empty();
    // Verify empty struct format using helper function
    assert_empty_struct_format(&empty.to_string(), "empty");
}

/// Test 10: Multiple constructors work independently
#[test]
fn test_multiple_constructors_independent() {
    #[derive(Debug, EnumMethods)]
    enum Event {
        A,
        B,
        C,
    }

    let a = Event::a();
    let b = Event::b();
    let c = Event::c();

    assert_eq!(a.as_str(), "a");
    assert_eq!(b.as_str(), "b");
    assert_eq!(c.as_str(), "c");
}
