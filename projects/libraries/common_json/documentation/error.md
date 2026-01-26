# JSON Error Types with Rich Context

- [Back to Documentation Index](TOC.md)

This module provides error types specific to JSON operations, enabling precise diagnosis of issues when handling JSON data.

## Error Types

The full list of error codes lives in `JsonErrorCode`. The table below highlights the most common ones.

| Code                              | Description                           | Example Usage                |
| --------------------------------- | ------------------------------------- | ---------------------------- |
| `JsonErrorCode::Serialize`        | Serialization/deserialization error   | Malformed JSON               |
| `JsonErrorCode::TypeMismatch`     | Expected type differs from found type | `as_str()` on a number       |
| `JsonErrorCode::FieldNotFound`    | Missing field in an object            | Accessing a missing key      |
| `JsonErrorCode::IndexOutOfBounds` | Index out of bounds in an array       | `arr[10]` on array of size 3 |
| `JsonErrorCode::InvalidPath`      | Invalid path expression               | `"user..name"`               |
| `JsonErrorCode::UnexpectedNull`   | Unexpected null value                 | Required field is null       |
| `JsonErrorCode::ParseError`       | Parsing error with position           | Line/column of the error     |
| `JsonErrorCode::Custom`           | Custom error                          | Specific business messages   |

## Example

```rust
use common_json::{pjson, JsonAccess, JsonErrorCode};

let data = pjson!({ name: "test" });

// TypeMismatch: trying to read a string as i64
let result = data
    .get_field("name")
    .expect("field exists")
    .as_i64_strict();
assert!(matches!(result, Err(e) if e.code == JsonErrorCode::TypeMismatch));

// MissingField: non-existent field
let result = data.get_field("missing");
assert!(matches!(result, Err(e) if e.code == JsonErrorCode::FieldNotFound));
```

## Additional Examples

### TypeMismatch Error Example

```rust
use common_json::{JsonError, JsonErrorCode};

let err = JsonError::new(JsonErrorCode::TypeMismatch)
    .context("expected string, found number");
assert!(err.to_string().contains("type mismatch"));
```

### MissingField Error Example

```rust
use common_json::{JsonError, JsonErrorCode};

let err = JsonError::new(JsonErrorCode::FieldNotFound).context("username");
assert!(err.to_string().contains("username"));
```

### IndexOutOfBounds Error Example

```rust
use common_json::{JsonError, JsonErrorCode};

let err = JsonError::new(JsonErrorCode::IndexOutOfBounds)
    .context("Index 10 out of bounds for array of length 3");
```

### Custom Error Example

```rust
use common_json::{JsonError, JsonErrorCode};

let err = JsonError::new(JsonErrorCode::Custom).context("Invalid user configuration");
```

## Tests

This module contains 3 tests covering:

- Creation and verification of `TypeMismatch` errors
- Creation and verification of `MissingField` errors
- Creation and formatting of `IndexOutOfBounds` errors

### Not Covered

- `InvalidPath`, `UnexpectedNull`, `ParseError`, `Custom` (constructors tested indirectly)
- `is_serialize()` (tested indirectly through deserialization modules)
