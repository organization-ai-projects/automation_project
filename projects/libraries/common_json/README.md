# Common JSON Library

Comprehensive JSON library for `automation_project` with ergonomic APIs and rich utilities.

## Overview

A custom JSON implementation providing parsing, serialization, deserialization, merging, and comparison utilities. Unlike `serde_json`, this library offers additional features like merge strategies, JSON patching, and builder patterns.

## Features

- **Parsing** - Fast JSON parsing from strings, bytes, or readers ([example](#parse-and-serialize))
- **Serialization** - Convert Rust types to JSON with pretty-print options ([example](#parse-and-serialize))
- **Deserialization** - Type-safe JSON to Rust conversion ([example](#parse-and-serialize))
- **Macros** - `pjson!` macro for ergonomic JSON construction ([example](#construct-json-with-macros))
- **Merging** - Flexible merge strategies for combining JSON values ([example](#merge-json-values))
- **Comparison** - Deep equality and diff utilities
- **Access** - Path-based JSON traversal (mutable and immutable) ([example](#access-nested-values))
- **Builders** - Fluent API for constructing arrays and objects ([example](#builder-pattern))

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
common_json = { path = "../common_json" }
```

## Usage

### Construct JSON with macros

```rust
use common_json::{pjson, Json};

// Object with various types
let user = pjson!({
    "name": "Alice",
    "age": 30,
    "active": true,
    "tags": ["admin", "user"],
    "metadata": {
        "created": "2024-01-01",
        "version": 2
    }
});

// Identifier keys (no quotes needed)
let config = pjson!({
    host: "localhost",
    port: 8080,
    debug: false
});

// Dynamic keys
let key = "dynamic";
let obj = pjson!({
    (key): "value"
});
```

### Parse and serialize

```rust
use common_json::{parse, to_string_pretty, from_str};

// Parse string to Json
let json = parse(r#"{"name": "Bob", "age": 25}"#)?;

// Serialize to string
let output = to_string_pretty(&json)?;

// Deserialize to typed struct
#[derive(Deserialize)]
struct User { name: String, age: u32 }
let user: User = from_str(r#"{"name": "Bob", "age": 25}"#)?;
```

### Access nested values

```rust
use common_json::{pjson, JsonAccess};

let data = pjson!({
    "users": [
        { "name": "Alice", "role": "admin" },
        { "name": "Bob", "role": "user" }
    ]
});

// Path-based access
let name = data.get_path(&["users", "0", "name"]);
let role = data["users"][0]["role"].as_str();
```

### Merge JSON values

```rust
use common_json::{merge, pjson, MergeStrategy};

let base = pjson!({ "a": 1, "b": { "x": 10 } });
let patch = pjson!({ "b": { "y": 20 }, "c": 3 });

// Deep merge
let merged = merge(&base, &patch, MergeStrategy::Deep);
// Result: { "a": 1, "b": { "x": 10, "y": 20 }, "c": 3 }
```

### Builder pattern

```rust
use common_json::json_object_builder::JsonObjectBuilder;

let obj = JsonObjectBuilder::new()
    .insert("name", "Alice")
    .insert("age", 30)
    .insert_if(true, "admin", true)
    .build();
```

## Macros Reference

| Macro          | Description                     |
| -------------- | ------------------------------- |
| `pjson!`       | Full-featured JSON construction |
| `json_array!`  | Simple array construction       |
| `json_object!` | Simple object construction      |
| `json_value!`  | Convert expression to Json      |

### Examples for Macros

#### `json_array!`

```rust
use common_json::json_array;

let array = json_array!["apple", "banana", "cherry"];
assert_eq!(array[0].as_str(), Some("apple"));
```

#### `json_object!`

```rust
use common_json::json_object;

let object = json_object! {
    "name" => "Alice",
    "age" => 30
};
assert_eq!(object["name"].as_str(), Some("Alice"));
```

#### `json_value!`

```rust
use common_json::json_value;

let value = json_value!(42);
assert_eq!(value.as_i64(), Some(42));
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/common_json/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
