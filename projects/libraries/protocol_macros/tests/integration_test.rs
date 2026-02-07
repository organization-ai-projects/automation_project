//! Integration tests for protocol_macros demonstrating advanced features
// projects/libraries/protocol_macros/tests/integration_test.rs
mod common;

use common::{assert_contains_all, assert_empty_struct_format, BinaryEvent, DebugEvent, TestEvent};
use protocol_macros::EnumMethods;

#[test]
fn test_unit_variants() {
    let ping = TestEvent::ping();
    let pong = TestEvent::pong();

    assert!(matches!(ping, TestEvent::Ping));
    assert!(matches!(pong, TestEvent::Pong));
}

#[test]
fn test_struct_variant() {
    let msg = TestEvent::message("Hello".to_string());
    match msg {
        TestEvent::Message { content } => {
            assert_eq!(content, "Hello");
        }
        _ => panic!("Expected Message variant"),
    }

    let complex = TestEvent::complex(42, "Test".to_string(), true);
    match complex {
        TestEvent::Complex { id, name, active } => {
            assert_eq!(id, 42);
            assert_eq!(name, "Test");
            assert!(active);
        }
        _ => panic!("Expected Complex variant"),
    }
}

#[test]
fn test_tuple_variant() {
    let num = TestEvent::number(42);
    match num {
        TestEvent::Number(value) => {
            assert_eq!(value, 42);
        }
        _ => panic!("Expected Number variant"),
    }

    let coords = TestEvent::coordinates(45.5, -73.6);
    match coords {
        TestEvent::Coordinates(lat, lon) => {
            assert_eq!(lat, 45.5);
            assert_eq!(lon, -73.6);
        }
        _ => panic!("Expected Coordinates variant"),
    }
}

#[test]
fn test_binary_event_with_debug_mode() {
    let data = BinaryEvent::data(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let display = data.to_string();
    // Debug mode allows Vec<u8> to be displayed - check for hex values as decimals
    assert_contains_all(&display, &["data(arg0=", "222", "173", "190", "239"]);

    let empty = BinaryEvent::empty();
    assert_eq!(empty.to_string(), "empty");
}

#[test]
fn test_display_implementation() {
    let ping = TestEvent::ping();
    assert_eq!(ping.to_string(), "ping");

    let msg = TestEvent::message("Hello World".to_string());
    assert_contains_all(&msg.to_string(), &["message", "content=Hello World"]);

    let num = TestEvent::number(999);
    assert_contains_all(&num.to_string(), &["number", "arg0=999"]);

    let complex = TestEvent::complex(99, "Foo".to_string(), false);
    assert_contains_all(
        &complex.to_string(),
        &["complex", "id=99", "name=Foo", "active=false"],
    );

    let coords = TestEvent::coordinates(1.23, 4.56);
    let coords_display = coords.to_string();
    assert_contains_all(&coords_display, &["coordinates", "arg0=", "1.23", "4.56"]);
}

#[test]
fn test_debug_mode_display() {
    let simple = DebugEvent::simple();
    assert_eq!(simple.to_string(), "simple");

    let with_data = DebugEvent::with_data("test".to_string());
    // In debug mode, strings are displayed with quotes
    assert_contains_all(&with_data.to_string(), &["with_data", "value=", "\"test\""]);

    let tuple = DebugEvent::tuple(42, "hello".to_string());
    let display = tuple.to_string();
    // Debug mode shows quotes around strings
    assert_contains_all(&display, &["42", "\"hello\""]);
}

#[test]
fn test_constructor_type_inference() {
    // Verify that constructors work with type inference
    let events = [
        TestEvent::ping(),
        TestEvent::message("msg".into()),
        TestEvent::number(123),
    ];

    assert_eq!(events.len(), 3);
}

#[test]
fn test_generic_usage() {
    // Test that generated code works with generic contexts
    fn process_event<T: std::fmt::Display>(event: T) -> String {
        format!("Event: {}", event)
    }

    let ping = TestEvent::ping();
    let result = process_event(ping);
    assert_eq!(result, "Event: ping");

    let msg = TestEvent::message("test".to_string());
    let result = process_event(msg);
    assert_eq!(result, "Event: message { content=test }");
}

#[test]
fn test_clone_behavior() {
    let original = TestEvent::complex(1, "original".to_string(), true);
    let cloned = original.clone();

    assert_eq!(original.to_string(), cloned.to_string());

    // Verify they are independent
    match (original, cloned) {
        (
            TestEvent::Complex {
                id: id1,
                name: name1,
                active: active1,
            },
            TestEvent::Complex {
                id: id2,
                name: name2,
                active: active2,
            },
        ) => {
            assert_eq!(id1, id2);
            assert_eq!(name1, name2);
            assert_eq!(active1, active2);
        }
        _ => panic!("Expected Complex variants"),
    }
}

#[test]
fn test_multiple_string_fields() {
    let complex = TestEvent::complex(123, "MultipleStrings".to_string(), true);
    let display = complex.to_string();
    assert_contains_all(&display, &["id=123", "name=MultipleStrings", "active=true"]);
}

#[test]
fn test_empty_struct_edge_case() {
    // While our test enums don't have empty struct variants,
    // the macro should handle them correctly
    #[derive(Debug, EnumMethods)]
    enum EdgeCase {
        Empty {},
        Unit,
    }

    let empty = EdgeCase::empty();
    // Verify empty struct format using helper function
    assert_empty_struct_format(&empty.to_string(), "empty");

    let unit = EdgeCase::unit();
    assert_eq!(unit.to_string(), "unit");
}
