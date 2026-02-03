# JSON Construction Macros

- [Retour à Index de Documentation](TOC.md)


This module provides macros for creating JSON declaratively, with additional features.

## Available Macros

| Macro          | Description                    |
| -------------- | ------------------------------ |
| `pjson!`       | Main macro, JSON-like syntax   |
| `pjson_key!`   | Helper for keys (internal)     |
| `json_array!`  | Create arrays with expressions |
| `json_object!` | Create objects with `=>`       |

## `pjson!` Syntax

### Primitive Values

```rust
use common_json::pjson;

let null = pjson!(null);        // null
let t = pjson!(true);           // true
let f = pjson!(false);          // false
let n = pjson!(42);             // 42
let pi = pjson!(3.14);          // 3.14
let s = pjson!("hello");        // "hello"
```

### Arrays

```rust
use common_json::pjson;

let empty = pjson!([]);
let numbers = pjson!([1, 2, 3]);
let mixed = pjson!([1, "two", true, null]);
let nested = pjson!([[1, 2], [3, 4]]);
```

### Objects

```rust
use common_json::pjson;

// Keys as identifiers
let obj = pjson!({ name: "test", value: 42 });

// Keys as strings (for special characters)
let obj = pjson!({ "key-with-dash": "value" });

// Dynamic keys with parentheses
let key = "dynamic";
let obj = pjson!({ (key): "value" });
```

### Variable Interpolation

```rust
use common_json::pjson;

let name = "Alice";
let age = 30;

// Use parentheses to interpolate variables
let user = pjson!({
    name: (name),
    age: (age)
});
```

### Nested Structures

```rust
use common_json::pjson;

let config = pjson!({
    server: {
        host: "localhost",
        port: 8080
    },
    features: ["auth", "api"],
    debug: true
});
```

### Trailing Commas

Trailing commas are accepted:

```rust
use common_json::pjson;

let obj = pjson!({ a: 1, b: 2, });  // OK
let arr = pjson!([1, 2, 3,]);       // OK
```

## Tests

This module contains 16 tests covering:

- Primitives: null, booleans, numbers, strings
- Arrays: empty, simple, mixed
- Objects: empty, simple, nested
- Features: dynamic keys, interpolation, trailing commas
- Alternative macros: `json_array!`, `json_object!`

### Not Covered

- Error handling (non-serializable types)
- Edge cases with Unicode characters in keys
