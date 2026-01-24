# JSON Serialization

This module provides tools for serializing data into JSON, with two main approaches:

- **Trait [`JsonSerializable`]**: Object-oriented API for types implementing `serde::Serialize`.
- **Standalone Functions**: Direct usage without trait implementation.

## Contents

| API                           | Description                 | Returns                   |
| ----------------------------- | --------------------------- | ------------------------- |
| [`JsonSerializable::to_json`] | Trait method → JSON value   | `Result<Json, JsonError>` |
| [`to_json`]                   | Function → JSON value       | `Result<Json, JsonError>` |
| [`to_string`]                 | Function → Compact string   | `JsonResult<String>`      |
| [`to_string_pretty`]          | Function → Formatted string | `JsonResult<String>`      |
| [`to_bytes`]                  | Function → Compact bytes    | `JsonResult<Vec<u8>>`     |
| [`to_bytes_pretty`]           | Function → Formatted bytes  | `JsonResult<Vec<u8>>`     |
| [`write_to`]                  | Function → Write to `Write` | `JsonResult<()>`          |
| [`write_to_pretty`]           | Function → Formatted write  | `JsonResult<()>`          |

## Usage Example

### Trait vs Functions

The [`JsonSerializable`] trait is automatically implemented for any type implementing `serde::Serialize`. Both approaches are equivalent:

```rust
use common_json::{to_string, JsonSerializable};
use serde::Serialize;

#[derive(Serialize)]
struct User { name: String }

let user = User { name: "Alice".into() };

// Function approach
let s1 = to_string(&user).expect("Failed to serialize user to string");

// Trait approach
let s2 = user.to_json_string().expect("Failed to serialize user to JSON string");
```

### Full Example

```rust
use common_json::{to_json, to_string, to_string_pretty, to_bytes, JsonSerializable};
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    name: String,
    debug: bool,
    max_connections: u32,
}

let config = Config {
    name: "my-app".into(),
    debug: true,
    max_connections: 100,
};

// To a JSON value
let json = to_json(&config).expect("Failed to serialize config to JSON value");
assert_eq!(json["name"], "my-app");

// To a compact string
let compact = to_string(&config).expect("Failed to serialize config to compact string");
// {"name":"my-app","debug":true,"max_connections":100}

// To a formatted string
let pretty = to_string_pretty(&config).expect("Failed to serialize config to formatted string");
// {
//   "name": "my-app",
//   "debug": true,
//   "max_connections": 100
// }

// To bytes (for network I/O)
let bytes = to_bytes(&config).expect("Failed to serialize config to bytes");
```

## Legacy Aliases

For compatibility with old code, the following aliases are available:

- [`to_value`] → [`to_json`]
- [`to_json_string`] → [`to_string`]
- [`to_json_string_pretty`] → [`to_string_pretty`]

## Tests

This module contains tests covering:

- `to_json`: conversion to JSON value
- `to_string`: conversion to string
- `to_bytes`: conversion to bytes
- Methods of the `JsonSerializable` trait

### Not Covered

- `write_to` / `write_to_pretty` (writing to a `Write`)
- `to_string_pretty` / `to_bytes_pretty` (formatted variants)
- Error handling for non-serializable types
