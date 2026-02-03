# Fluent Builder for JSON Objects

- [Retour à Index de Documentation](TOC.md)


Allows constructing JSON objects in a readable and type-safe manner.

## Methods

| Method      | Description               |
| ----------- | ------------------------- |
| `field`     | Adds a field              |
| `field_opt` | Adds if `Some`            |
| `field_if`  | Adds if condition is true |
| `build`     | Finalizes the object      |

## Example

```rust
use common_json::{JsonObjectBuilder, JsonAccess};

let user = JsonObjectBuilder::new()
    .field("name", "Alice")
    .field("age", 30)
    .field_opt("email", Some("alice@example.com"))
    .field_if(true, "verified", true)
    .build();

assert_eq!(
    user.get_field("name").expect("field exists").as_str(),
    Some("Alice")
);
assert_eq!(
    user.get_field("age").expect("field exists").as_i64(),
    Some(30)
);
```

## Tests

This module includes tests covering:

- Adding simple fields with `field`
- Conditional addition with `field_opt` and `field_if`
- Finalizing and validating the object with `build`
