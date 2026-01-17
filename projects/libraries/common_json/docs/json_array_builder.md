# Fluent Builder for JSON Arrays

Allows constructing JSON arrays in a readable and type-safe manner.

## Methods

| Method        | Description               |
| ------------- | ------------------------- |
| `element`     | Adds an element           |
| `element_opt` | Adds if `Some`            |
| `element_if`  | Adds if condition is true |
| `extend`      | Adds multiple elements    |
| `build`       | Finalizes the array       |

## Example

```rust
use common_json::JsonArrayBuilder;

let arr = JsonArrayBuilder::new()
    .element(1)
    .element("two")
    .element(true)
    .extend(vec![4, 5, 6])
    .build();

assert_eq!(arr.as_array().unwrap().len(), 6);
```

## Tests

This module includes tests covering:

- Adding simple elements with `element`
- Conditional addition with `element_opt` and `element_if`
- Extending with multiple elements via `extend`
- Finalizing and validating the array with `build`
