# JSON Error Types with Rich Context

This module provides error types specific to JSON operations, enabling precise diagnosis of issues when handling JSON data.

## Error Types

| Variant                       | Description                           | Example Usage                |
| ----------------------------- | ------------------------------------- | ---------------------------- |
| `JsonError::Serialize`        | Serialization/deserialization error   | Malformed JSON               |
| `JsonError::TypeMismatch`     | Expected type differs from found type | `as_str()` on a number       |
| `JsonError::MissingField`     | Missing field in an object            | `obj["missing"]`             |
| `JsonError::IndexOutOfBounds` | Index out of bounds in an array       | `arr[10]` on array of size 3 |
| `JsonError::InvalidPath`      | Invalid path expression               | `"user..name"`               |
| `JsonError::UnexpectedNull`   | Unexpected null value                 | Required field is null       |
| `JsonError::ParseError`       | Parsing error with position           | Line/column of the error     |
| `JsonError::Custom`           | Custom error                          | Specific business messages   |

## Example

```rust
use common_json::{pjson, JsonAccess, JsonError};

let data = pjson!({ name: "test" });

// TypeMismatch: trying to read a string as i64
let result = data.get_field("name").unwrap().as_i64_strict();
assert!(matches!(result, Err(JsonError::TypeMismatch { .. })));

// MissingField: non-existent field
let result = data.get_field("missing");
assert!(matches!(result, Err(JsonError::MissingField { .. })));
```

## Additional Examples

### TypeMismatch Error Example

```rust
use common_json::JsonError;

let err = JsonError::type_mismatch("string", "number");
assert!(err.to_string().contains("expected string"));
```

### MissingField Error Example

```rust
use common_json::JsonError;

let err = JsonError::missing_field("username");
assert!(err.to_string().contains("username"));
```

### IndexOutOfBounds Error Example

```rust
use common_json::JsonError;

let err = JsonError::index_out_of_bounds(10, 3);
// Message: "Index out of bounds: 10 (array length: 3)"
```

### Custom Error Example

```rust
use common_json::JsonError;

let err = JsonError::custom("Invalid user configuration");
```

## Tests

This module contains 3 tests covering:

- Creation and verification of `TypeMismatch` errors
- Creation and verification of `MissingField` errors
- Creation and formatting of `IndexOutOfBounds` errors

### Not Covered

- `InvalidPath`, `UnexpectedNull`, `ParseError`, `Custom` (constructors tested indirectly)
- `is_serialize()` (tested indirectly through deserialization modules)
