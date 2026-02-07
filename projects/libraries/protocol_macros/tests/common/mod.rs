//! Common test utilities and shared definitions for protocol_macros tests
#![allow(dead_code)]

use protocol_macros::EnumMethods;

/// Standard test enum with unit, tuple, and struct variants (Display mode)
/// Used across multiple test files for consistency
#[derive(Debug, Clone, EnumMethods)]
pub enum TestEvent {
    Ping,
    Pong,
    Message { content: String },
    Number(u32),
    Complex { id: u32, name: String, active: bool },
    Coordinates(f64, f64),
}

/// Standard event enum for as_str testing
#[derive(Debug, Clone, EnumMethods)]
pub enum Event {
    Ping,
    Created { id: String, data: String },
    DataSent(String, u32),
}

/// Test enum with Vec<u8> using debug mode (since Vec doesn't impl Display)
#[derive(Debug, Clone, EnumMethods)]
#[enum_methods(mode = "debug")]
pub enum BinaryEvent {
    Data(Vec<u8>),
    Empty,
}

/// Test enum with debug mode display
#[derive(Debug, Clone, EnumMethods)]
#[enum_methods(mode = "debug")]
pub enum DebugEvent {
    Simple,
    WithData { value: String },
    Tuple(i32, String),
}

/// HTTP status enum for testing acronyms
#[derive(Debug, Clone, EnumMethods)]
pub enum HTTPStatus {
    Request,
    NotFound,
    ServerError,
}

/// Helper function to assert a string contains expected patterns
/// More resilient to formatting changes than exact equality
pub fn assert_contains_all(actual: &str, expected_parts: &[&str]) {
    for part in expected_parts {
        assert!(
            actual.contains(part),
            "Expected '{}' to contain '{}', but it didn't",
            actual,
            part
        );
    }
}

/// Helper to validate empty struct variant formatting
/// Ensures the variant has braces with only whitespace between them
pub fn assert_empty_struct_format(display: &str, variant_name: &str) {
    assert!(
        display.starts_with(variant_name),
        "Expected display to start with '{}', got: '{}'",
        variant_name,
        display
    );
    assert!(
        display.contains('{') && display.contains('}'),
        "Expected '{}' to contain braces",
        display
    );

    // Verify content between braces is only whitespace
    let content = display
        .split('{')
        .nth(1)
        .expect("Expected to find content after opening brace")
        .split('}')
        .next()
        .expect("Expected to find content before closing brace");
    assert!(
        content.trim().is_empty(),
        "Empty struct should only contain whitespace between braces, but found: '{}'",
        content
    );
}
