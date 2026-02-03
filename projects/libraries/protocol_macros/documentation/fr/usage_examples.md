# Usage Examples

- [Retour à Index de Documentation](TOC.md)


This document provides detailed examples of how to use the `protocol_macros` crate.

## Basic Example

```rust
use protocol_macros::EnumMethods;

#[derive(Debug, Clone, EnumMethods)]
enum Event {
    Ping,
    Created { id: String, data: String },
    Data(String, u32),
}

// Generated constructors (snake_case):
let ping = Event::ping();
let created = Event::created("id".to_string(), "data".to_string());
let data = Event::data("info".to_string(), 42);

// Generated Display implementation:
assert_eq!(ping.to_string(), "ping");
assert_eq!(created.to_string(), "created { id=id, data=data }");
assert_eq!(data.to_string(), "data(arg0=info, arg1=42)");
```

## Debug Mode

```rust
#[derive(Debug, Clone, EnumMethods)]
#[enum_methods(mode = "debug")]
enum BinaryEvent {
    Data(Vec<u8>),
    Empty,
}

let data = BinaryEvent::data(vec![0xDE, 0xAD]);
println!("{}", data); // Output: data(arg0=[222, 173])
```

For more examples, refer to the [README](../../README.fr.md).
