//! Tests for the as_str() method generation (premium feature)
// projects/libraries/protocol_macros/tests/as_str_test.rs
mod common;

use common::{assert_contains_all, Event, HTTPStatus};

#[test]
fn test_as_str_unit_variant() {
    let ping = Event::ping();
    assert_eq!(ping.as_str(), "ping");
}

#[test]
fn test_as_str_struct_variant() {
    let created = Event::created("id".to_string(), "data".to_string());
    assert_eq!(created.as_str(), "created");
}

#[test]
fn test_as_str_tuple_variant() {
    let data_sent = Event::data_sent("msg".to_string(), 42);
    assert_eq!(data_sent.as_str(), "data_sent");
}

#[test]
fn test_as_str_with_acronyms() {
    // Premium snake_case conversion handles acronyms!
    let request = HTTPStatus::Request;
    assert_eq!(request.as_str(), "request");

    let not_found = HTTPStatus::NotFound;
    assert_eq!(not_found.as_str(), "not_found");

    let error = HTTPStatus::ServerError;
    assert_eq!(error.as_str(), "server_error");
}

#[test]
fn test_as_str_const_fn() {
    // as_str() is const, so it can be used in const contexts
    const EVENT_NAME: &str = {
        let event = Event::Ping;
        event.as_str()
    };
    assert_eq!(EVENT_NAME, "ping");
}

#[test]
fn test_as_str_for_routing() {
    // Real-world use case: routing
    fn route_event(event: &Event) -> &'static str {
        match event.as_str() {
            "ping" => "handle_ping",
            "created" => "handle_created",
            "data_sent" => "handle_data_sent",
            _ => "unknown_handler",
        }
    }

    let ping = Event::ping();
    assert_eq!(route_event(&ping), "handle_ping");

    let created = Event::created("id".into(), "data".into());
    assert_eq!(route_event(&created), "handle_created");
}

#[test]
fn test_as_str_for_logging() {
    // Real-world use case: structured logging
    fn log_event(event: &Event) {
        println!("Event type: {}", event.as_str());
        println!("Event display: {}", event);
    }

    let ping = Event::ping();
    log_event(&ping);
    // Output:
    // Event type: ping
    // Event display: ping
}

#[test]
fn test_as_str_vs_display() {
    // as_str() gives the variant name only
    // Display gives the full formatted representation
    let created = Event::created("123".into(), "test".into());

    assert_eq!(created.as_str(), "created");
    assert_contains_all(&created.to_string(), &["created", "id=123", "data=test"]);
}

#[test]
fn test_as_str_collection() {
    let events = [
        Event::ping(),
        Event::created("id1".into(), "data1".into()),
        Event::data_sent("msg".into(), 42),
    ];

    let names: Vec<&str> = events.iter().map(|e| e.as_str()).collect();
    assert_eq!(names, vec!["ping", "created", "data_sent"]);
}
