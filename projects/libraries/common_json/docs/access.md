# Fluent Access to JSON Values and Builders

This module provides traits and builders for navigating and constructing JSON structures ergonomically.

## Contents

| Type                | Description                |
| ------------------- | -------------------------- |
| `JsonAccess`        | Trait for read access      |
| `JsonAccessMut`     | Trait for write access     |
| `JsonObjectBuilder` | Fluent builder for objects |
| `JsonArrayBuilder`  | Fluent builder for arrays  |

## Path Navigation

The `JsonAccess` trait allows navigating nested structures with the `get_path` method:

```rust
use common_json::{pjson, JsonAccess};

let data = pjson!({
    user: {
        profile: {
            name: "Alice"
        }
    },
    tags: ["admin", "user"]
});

// Dot navigation
assert_eq!(data.get_path("user.profile.name").unwrap().as_str(), Some("Alice"));

// Array access with [index]
assert_eq!(data.get_path("tags[0]").unwrap().as_str(), Some("admin"));
assert_eq!(data.get_path("tags[1]").unwrap().as_str(), Some("user"));
```

## Strict Accessors

The `as_*_strict()` methods return an error if the type does not match, instead of returning `None`:

```rust
use common_json::{pjson, JsonAccess, JsonError};

let data = pjson!({ count: 42 });

// as_i64() returns Option<i64>
assert_eq!(data.get_field("count").unwrap().as_i64(), Some(42));

// as_i64_strict() returns Result<i64, JsonError>
assert_eq!(data.get_field("count").unwrap().as_i64_strict().unwrap(), 42);

// Error if wrong type
let err = data.get_field("count").unwrap().as_str_strict();
assert!(matches!(err, Err(JsonError::TypeMismatch { .. })));
```

## Fluent Builders

The builders allow constructing JSON in a readable way:

```rust
use common_json::{JsonObjectBuilder, JsonArrayBuilder};

let obj = JsonObjectBuilder::new()
    .field("name", "Alice")
    .field("age", 30)
    .field_opt("nickname", Some("Ali"))  // Added because Some
    .field_opt::<_, &str>("email", None) // Ignored because None
    .field_if(true, "active", true)      // Added because condition is true
    .build();

let arr = JsonArrayBuilder::new()
    .element(1)
    .element(2)
    .extend(vec![3, 4, 5])
    .build();
```

## Modifying JSON Values

The `JsonAccessMut` trait allows modifying JSON objects and arrays directly.

### Main Methods

| Method          | Description                                            |
| --------------- | ------------------------------------------------------ |
| `get_field_mut` | Accesses a mutable field of a JSON object              |
| `get_index_mut` | Accesses a mutable element of a JSON array             |
| `set_field`     | Sets or updates a field of a JSON object               |
| `remove_field`  | Removes a field from a JSON object                     |
| `push`          | Adds an element to a JSON array                        |
| `insert_at`     | Inserts an element at a specific index in a JSON array |
| `remove_at`     | Removes an element at a specific index in a JSON array |

### Usage Example

```rust
use common_json::{pjson, JsonAccessMut};

let mut data = pjson!({ "name": "Alice", "tags": ["admin"] });

// Modify a field
data.set_field("name", "Bob").unwrap();
assert_eq!(data.get_field("name").unwrap().as_str(), Some("Bob"));

// Add a field
data.set_field("age", 25).unwrap();
assert_eq!(data.get_field("age").unwrap().as_i64(), Some(25));

// Add to an array
let tags = data.get_field_mut("tags").unwrap();
tags.push("user").unwrap();
assert_eq!(tags.as_array().unwrap().len(), 2);
```

## Tests

This module contains 8 tests covering :

- `get_field` : access to object fields
- `get_index` : access to array elements
- `get_path` : navigation by path (dot notation + indices)
- `as_*_strict` : strict accessors with typed errors
- `type_name` : identification of the JSON type
- `is_truthy` : boolean evaluation of values
- Mutations : `set_field`, `push`, etc.
- Builders : `JsonObjectBuilder` and `JsonArrayBuilder`

### Not Covered

- `get_field_mut`, `get_index_mut` (mutable accessors)
- `remove_field`, `remove_at` (removals)
- `insert_at` (insertion at an index)
- Complex invalid paths
