# Protocol Library

A Rust library for communication based on typed commands and events with validation and metadata.

## Version

Current version: **1.0.0**

## Features

- ✅ **Typed commands and events** - Defined types for better safety and clarity
- ✅ **Robust validation** - Comprehensive validation with descriptive error messages
- ✅ **Automatic metadata** - Automatically generated timestamps and unique IDs
- ✅ **Serialization** - Full serde support for JSON/binary
- ✅ **Security** - Size limits and format validation to prevent abuse
- ✅ **Complete documentation** - Inline docs and examples

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
protocol = { path = "../path/to/protocol" }
```

## Usage

### Create and validate a command

```rust
use protocol::{Command, CommandType};

// Create a new command
let cmd = Command::new(
    "execute_task".to_string(),
    CommandType::Execute,
    r#"{"task": "example", "params": {}}"#.to_string()
);

// Validate the command
match cmd.validate() {
    Ok(()) => println!("Valid command!"),
    Err(e) => eprintln!("Validation error: {}", e),
}
```

### Create and validate an event

```rust
use protocol::{Event, EventType};

// Create a new event
let event = Event::new(
    "task_completed".to_string(),
    EventType::Completed,
    r#"{"result": "success", "duration_ms": 1234}"#.to_string()
);

// Validate the event
if let Err(e) = event.validate() {
    eprintln!("Invalid event: {}", e);
}
```

### Available command types

- `Execute` - Execute a task or operation
- `Query` - Query for information
- `Update` - Update existing data
- `Delete` - Delete data or resources
- `Create` - Create new resources
- `Subscribe` - Subscribe to events or updates
- `Unsubscribe` - Unsubscribe from events or updates
- `Configure` - Configuration command
- `Custom` - Custom command type

### Available event types

- `Started` / `Stopped` - System start/stop
- `Created` / `Updated` / `Deleted` - Data modifications
- `Error` / `Warning` / `Info` - Log levels
- `Completed` / `Failed` - Task results
- `Progress` - Progress updates
- `StateChanged` - State changes
- `Custom` - Custom event type

### Metadata

Metadata is automatically generated with:

- **Timestamp**: milliseconds since UNIX epoch
- **Unique ID**: combination of timestamp and atomic counter

```rust
use protocol::Metadata;

// Create with the current timestamp
let metadata = Metadata::now();

// Access the data
println!("Timestamp: {}", metadata.timestamp);
println!("ID: {}", metadata.id);
println!("Readable format: {}", metadata.timestamp_to_string());
```

### Error handling

The library provides detailed validation errors:

```rust
use protocol::{Command, CommandType, ValidationError};

let cmd = Command::new(
    "test command with spaces!".to_string(), // Invalid name
    CommandType::Execute,
    "payload".to_string()
);

match cmd.validate() {
    Err(ValidationError::InvalidNameFormat(name)) => {
        println!("Invalid name: {}", name);
    }
    Err(e) => println!("Other error: {}", e),
    Ok(()) => println!("OK"),
}
```

Error types:

- `EmptyName` - Name is empty or contains only spaces
- `EmptyPayload` - Payload/data is empty
- `InvalidNameFormat` - Name contains invalid characters
- `PayloadTooLarge` - Payload exceeds the maximum size (10 MB)
- `NameTooLong` - Name exceeds the maximum length (256 characters)
- `InvalidTimestamp` - Invalid timestamp (too far in the future)

## Security limits

To prevent abuse and attacks:

### Commands

- Maximum name length: **256 characters**
- Maximum payload size: **10 MB**
- Allowed characters in the name: alphanumeric, `_`, `-`, `.`

### Events

- Maximum name length: **256 characters**
- Maximum data size: **10 MB**
- Allowed characters in the name: alphanumeric, `_`, `-`, `.`

### Timestamps

- Maximum drift into the future: **1 hour**

## Tests

Run the tests:

```bash
cargo test -p protocol
```

## Possible future developments

- Support for structured payloads with `serde_json::Value`
- Compression of large payloads
- Encryption of sensitive data
- Cryptographic signatures for authenticity
- Support for custom validation schemas

## Contribute

Contributions are welcome! Please open an issue or pull request on the GitHub repository.

For more details on the Git/GitHub workflow used in this project, see the [versioning documentation](../../../docs/versioning/git-github.md).

## License

To be defined
