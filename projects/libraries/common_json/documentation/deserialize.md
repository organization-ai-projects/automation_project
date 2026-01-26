# JSON Deserialization with Trait and Utility Functions

- [Back to Documentation Index](TOC.md)


This module provides two approaches for deserializing JSON:

- The `JsonDeserializable` trait for an object-oriented API
- Standalone functions for direct usage

## Architecture

| API               | Description                  | Source      |
| ----------------- | ---------------------------- | ----------- |
| `parse`           | Parse a string → `Json`      | `&str`      |
| `parse_bytes`     | Parse bytes → `Json`         | `&[u8]`     |
| `parse_reader`    | Parse a reader → `Json`      | `impl Read` |
| `from_json`       | Deserialize → `T`            | `&Json`     |
| `from_json_owned` | Deserialize (no clone) → `T` | `Json`      |
| `from_str`        | Parse and deserialize → `T`  | `&str`      |
| `from_bytes`      | Parse and deserialize → `T`  | `&[u8]`     |
| `from_reader`     | Parse and deserialize → `T`  | `impl Read` |

## Parse vs Deserialization

- **Parse**: Converts text/bytes into `Json` (generic value)
- **Deserialization**: Converts JSON into a typed Rust structure

### Example for Parse vs Deserialization

```rust
use common_json::{parse, from_str, Json, JsonAccess};
use serde::Deserialize;

#[derive(Deserialize)]
struct User { name: String }

let json_str = r#"{"name": "Alice"}"#;

// Parse → Generic Json
let json: Json = parse(json_str).expect("Failed to parse JSON string");
assert_eq!(
    json.get_field("name").expect("Missing 'name'").as_str(),
    Some("Alice")
);

// Deserialize → Concrete Type
let user: User = from_str(json_str).expect("Failed to deserialize JSON into User struct");
assert_eq!(user.name, "Alice");
```

## JsonDeserializable Trait

The trait is automatically implemented for any type `T: DeserializeOwned`.

### Example for JsonDeserializable Trait

```rust
use common_json::JsonDeserializable;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config { port: u16 }

let config = Config::from_json_str(r#"{"port": 8080}"#).expect("Failed to deserialize JSON into Config struct");
assert_eq!(config.port, 8080);
```

## Standalone Functions

### `parse`

```rust
use common_json::{parse, JsonAccess};

let json = parse(r#"{"name": "Alice", "age": 30}"#).expect("Failed to parse JSON string");
assert_eq!(json.get_field("name").expect("Missing 'name'").as_str(), Some("Alice"));
assert_eq!(json.get_field("age").expect("Missing 'age'").as_i64(), Some(30));
```

### `parse_bytes`

```rust
use common_json::parse_bytes;

let json = parse_bytes(br#"[1, 2, 3]"#).expect("parse bytes");
assert_eq!(json[0], 1);
```

### `parse_reader`

```rust
use common_json::{parse_reader, JsonAccess};
use std::io::Cursor;

let cursor = Cursor::new(br#"{"key": "value"}"#);
let json = parse_reader(cursor).expect("Failed to parse JSON from reader");
assert_eq!(json.get_field("key").expect("Missing 'key'").as_str(), Some("value"));
```

### `from_json`

```rust
use common_json::{from_json, pjson};
use serde::Deserialize;

#[derive(Deserialize)]
struct User { name: String }

let json = pjson!({ name: "Alice" });
let user: User = from_json(&json).expect("deserialize user");
assert_eq!(user.name, "Alice");
```

### `from_str`

```rust
use common_json::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct Point { x: i32, y: i32 }

let p: Point = from_str(r#"{"x": 10, "y": 20}"#).expect("Failed to deserialize JSON into Point struct");
assert_eq!(p.x, 10);
```

## Legacy Alias

For compatibility with old code:

- `from_value` → `from_json_owned`
- `from_json_str` (function) → `from_str`

## Tests

This module contains 5 tests covering :

- `parse`: parsing from string to `Json`
- `parse_bytes`: parsing from bytes to `Json`
- `from_str`: direct deserialization from string
- `from_json`: deserialization from `Json` value
- Methods of the `JsonDeserializable` trait

### Not Covered

- `parse_reader` / `from_reader` (reading from `Read`)
- `from_json_owned` (no clone)
- Error handling on malformed JSON
