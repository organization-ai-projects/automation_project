# JSON Base Types and Builders

This module defines type aliases for JSON values and provides builder functions to easily create JSON values.

## Types

| Type         | Description                       |
| ------------ | --------------------------------- |
| `Json`       | Generic JSON value                |
| `JsonMap`    | Key-value map for objects         |
| `JsonArray`  | Array of JSON values              |
| `JsonObject` | Alias for `JsonMap<String, Json>` |
| `JsonNumber` | JSON number (integer or float)    |

## Builders

Builders allow creating JSON values explicitly without relying on internal implementation. They are also more readable for simple cases.

### Examples

```rust
use common_json::value::*;

let obj = object();           // {}
let arr = array();            // []
let n = null();               // null
let b = boolean(true);        // true
let s = string("hello");      // "hello"
let i = number_i64(42);       // 42
let u = number_u64(100);      // 100
let f = number_f64(3.14);     // Some(3.14) or None if NaN/Infinity
```

## Why Builders?

Builders allow creating JSON values explicitly without relying on the `Json::Object(...)` syntax that exposes internal implementation. They are also more readable for simple cases.

## Tests

This module includes tests covering:

- Creating empty objects
- Creating empty arrays
- Creating null values
- Creating booleans (true/false)
- Creating strings
- Creating numbers (i64, u64, f64, and NaN cases)
