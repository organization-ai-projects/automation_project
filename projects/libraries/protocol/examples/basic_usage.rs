// Example: Basic usage of the protocol library
//
// Run with: cargo run --example basic_usage -p protocol

use protocol::{Command, CommandType, Event, EventType, ValidationError};

fn main() {
    println!("=== Protocol Library Basic Usage ===\n");
    println!("Protocol version: {}\n", protocol::version());

    // Example 1: Create and validate a valid command
    println!("--- Example 1: Valid Command ---");
    let cmd = Command::new(
        "execute_task".to_string(),
        CommandType::Execute,
        r#"{"task": "process_data", "params": {"input": "data.csv"}}"#.to_string(),
    );

    match cmd.validate() {
        Ok(()) => {
            println!("✓ Command is valid!");
            println!("  Name: {}", cmd.name);
            println!("  Type: {}", cmd.command_type);
            println!("  Payload: {}", cmd.payload);
            println!("  Timestamp: {}", cmd.metadata.timestamp);
            println!("  ID: {}", cmd.metadata.id);
        }
        Err(e) => println!("✗ Validation failed: {}", e),
    }

    // Example 2: Create and validate a valid event
    println!("\n--- Example 2: Valid Event ---");
    let event = Event::new(
        "task_completed".to_string(),
        EventType::Completed,
        r#"{"result": "success", "processed": 1500, "duration_ms": 2345}"#.to_string(),
    );

    match event.validate() {
        Ok(()) => {
            println!("✓ Event is valid!");
            println!("  Name: {}", event.name);
            println!("  Type: {}", event.event_type);
            println!("  Data: {}", event.data);
            println!("  Time: {}", event.metadata.timestamp_to_string());
        }
        Err(e) => println!("✗ Validation failed: {}", e),
    }

    // Example 3: Invalid command - empty name
    println!("\n--- Example 3: Invalid Command (Empty Name) ---");
    let invalid_cmd = Command::new("".to_string(), CommandType::Query, "payload".to_string());

    match invalid_cmd.validate() {
        Ok(()) => println!("✓ Command is valid"),
        Err(ValidationError::EmptyName) => {
            println!("✗ Expected error caught: {}", ValidationError::EmptyName);
        }
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // Example 4: Invalid command - invalid name format
    println!("\n--- Example 4: Invalid Command (Invalid Name Format) ---");
    let invalid_cmd = Command::new(
        "test command with spaces!".to_string(),
        CommandType::Create,
        "payload".to_string(),
    );

    match invalid_cmd.validate() {
        Ok(()) => println!("✓ Command is valid"),
        Err(ValidationError::InvalidNameFormat(name)) => {
            println!("✗ Expected error caught: Invalid name format '{}'", name);
            println!("  (Only alphanumeric, underscore, hyphen, and dot are allowed)");
        }
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // Example 5: Invalid event - payload too large
    println!("\n--- Example 5: Invalid Event (Payload Too Large) ---");
    let large_payload = "x".repeat(11 * 1024 * 1024); // 11 MB (exceeds 10 MB limit)
    let invalid_event = Event::new("large_event".to_string(), EventType::Error, large_payload);

    match invalid_event.validate() {
        Ok(()) => println!("✓ Event is valid"),
        Err(ValidationError::PayloadTooLarge { size, max }) => {
            println!("✗ Expected error caught: Payload too large");
            println!("  Size: {} bytes, Max: {} bytes", size, max);
        }
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // Example 6: Different command types
    println!("\n--- Example 6: Different Command Types ---");
    let command_types = vec![
        (CommandType::Execute, "Execute a task"),
        (CommandType::Query, "Query for information"),
        (CommandType::Update, "Update existing data"),
        (CommandType::Delete, "Delete resources"),
        (CommandType::Create, "Create new resources"),
        (CommandType::Subscribe, "Subscribe to updates"),
        (CommandType::Unsubscribe, "Unsubscribe from updates"),
        (CommandType::Configure, "Configure settings"),
        (CommandType::Custom, "Custom command"),
    ];

    for (cmd_type, description) in command_types {
        let _cmd = Command::new(
            format!("{}_command", cmd_type.as_str()),
            cmd_type,
            "{}".to_string(),
        );
        println!("  {} - {}", cmd_type, description);
    }

    // Example 7: Different event types
    println!("\n--- Example 7: Different Event Types ---");
    let event_types = vec![
        (EventType::Started, "System started"),
        (EventType::Stopped, "System stopped"),
        (EventType::Created, "Data created"),
        (EventType::Updated, "Data updated"),
        (EventType::Deleted, "Data deleted"),
        (EventType::Error, "Error occurred"),
        (EventType::Warning, "Warning issued"),
        (EventType::Info, "Informational message"),
        (EventType::Completed, "Task completed"),
        (EventType::Failed, "Task failed"),
        (EventType::Progress, "Progress update"),
        (EventType::StateChanged, "State changed"),
        (EventType::Custom, "Custom event"),
    ];

    for (evt_type, description) in event_types {
        let _evt = Event::new(
            format!("{}_event", evt_type.as_str()),
            evt_type,
            "{}".to_string(),
        );
        println!("  {} - {}", evt_type, description);
    }

    println!("\n=== Examples Complete ===");
}
