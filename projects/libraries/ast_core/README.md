# AST Core Documentation

This directory contains a generic library for representing, validating, and transforming Abstract Syntax Trees (ASTs).

## Role in the Project

This library is responsible for providing generic AST data structures for use across runtime tools, compile-time tools (proc-macros), and AI systems. It provides the foundational types and operations for working with structured data trees.

It interacts mainly with:

- AI library - For AST rewriting and validation
- Proc-macro libraries - For compile-time AST manipulation
- Protocol libraries - For structured data representation

## Directory Structure

```
ast_core/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
├── benches/           # Performance benchmarks
├── src/               # Source code
│   ├── lib.rs
│   ├── ast_node.rs
│   ├── builder.rs
│   └── ...
└── tests/             # Integration tests
```

## Description

`ast_core` is a generic library for representing, validating, and transforming Abstract Syntax Trees (ASTs). It is designed for use in runtime tools, compile-time tools (proc-macros), and AI systems.

Here, **AST** refers to a **structured data tree** (nodes, keys, literals). It is not necessarily tied to a complete grammar or specific language.

## Fundamental Rules

### Rule 1: Structural AST, not a complete language AST

- **Allowed**:
  - Trees, lists, maps, literals, identifiers, opaque payloads.
- **Not Allowed**:
  - Parsing or complete representation of a language (Rust, JS, etc.).

`ast_core` is generic but not a universal compiler.

### Rule 2: Format Independence

- **Allowed**:
  - `ast_core` does not depend on any specific format (JSON, YAML, TOML, etc.).
- **Not Allowed**:
  - No dependency on `serde_json`, `syn`, or `quote` **in normal dependencies**.
  - These dependencies may be allowed behind an optional feature (`dev`, `test`) for internal needs.

Specific formats should be handled in "frontend" crates (e.g., `json_frontend`, `yaml_frontend`).

### Rule 3: Reusability

- `ast_core` must be usable by:
  - Runtime (CLI/server).
  - Compile-time (proc-macros via adapter).
  - AI (rewriting/validation).

`ast_core` = types + validations + pure transformations.

## Key Features

### Generic Types

- `AstNode`: Represents AST nodes with metadata.
- `AstKind`: The type of the node (Null, Bool, Number, String, Array, Object, Opaque).
- `AstKey`: Represents object keys (Ident or String).
- `Number`: Numbers with type preservation (Int, Uint, Float).
- `AstMeta`: Metadata (span, origin, flags, attrs, ext).

### Ergonomic Builder

```rust
use ast_core::{AstBuilder, AstNode};

let config = AstBuilder::object(vec![
    ("name", AstBuilder::string("my-app")),
    ("version", AstBuilder::int(1)),
    ("enabled", AstBuilder::bool(true)),
    ("tags", AstBuilder::array(vec![
        AstBuilder::string("production"),
        AstBuilder::string("stable"),
    ])),
]);

// Access data
assert_eq!(config.get("name").expect("Missing 'name' key").as_str(), Some("my-app"));
assert_eq!(config.get("version").expect("Missing 'version' key").as_number().expect("'version' is not a number").as_i64(), Some(1));
```

### Validation

Validation concerns the **structure** of the AST:

- Maximum depth
- Maximum size (elements per array/object)
- Duplicate keys

```rust
use ast_core::{AstBuilder, ValidateLimits};

let node = AstBuilder::object(vec![
    ("a", AstBuilder::int(1)),
    ("b", AstBuilder::int(2)),
]);

// Validation with default limits
node.validate().expect("Valid AST");

// Validation with custom limits
let limits = ValidateLimits {
    max_depth: 10,
    max_size: 100,
};
node.validate_with(&limits).expect("Valid AST");
```

Errors include the **path** to the error location:

```rust
// Error with path: "at outer.inner: Exceeded maximum depth: 2"
```

**Not included**: Business validation (e.g., valid email, required key according to a specification).

### Transformation and Traversal

```rust
use ast_core::AstBuilder;

let numbers = AstBuilder::array(vec![
    AstBuilder::int(1),
    AstBuilder::int(2),
    AstBuilder::int(3),
]);

// Double all numbers
let doubled = numbers.transform(&|node| {
    if let Some(n) = node.as_number() {
        if let Some(i) = n.as_i64() {
            return AstBuilder::int(i * 2);
        }
    }
    node.clone()
});

// Visit all nodes
let mut count = 0;
numbers.visit(&mut |_| count += 1);
assert_eq!(count, 4); // array + 3 ints

// Metrics
assert_eq!(numbers.node_count(), 4);
assert_eq!(numbers.depth(), 2);
```

### Metadata

```rust
use ast_core::{AstBuilder, Origin, Span};

let node = AstBuilder::string("hello")
    .with_span(0, 7)
    .with_origin(Origin::Parser("json"));

assert_eq!(node.meta.span, Some(Span { start: 0, end: 7 }));
```

## Non-Goals

- Provide a complete parser for a language.
- Contain domain-specific business rules.
- Impose a unique serialization format.

## API Reference

### AstNode

| Method                                      | Description                        |
| ------------------------------------------- | ---------------------------------- |
| `new(kind)`                                 | Creates a node with the given kind |
| `with_meta(meta)`                           | Sets metadata                      |
| `with_span(start, end)`                     | Sets the span                      |
| `with_origin(origin)`                       | Sets the origin                    |
| `validate()`                                | Validates with default limits      |
| `validate_with(limits)`                     | Validates with custom limits       |
| `is_null/bool/number/string/array/object()` | Type checks                        |
| `as_bool/number/str/array/object()`         | Typed accessors                    |
| `get(key)`                                  | Access by key (objects)            |
| `get_index(i)`                              | Access by index (arrays)           |
| `transform(f)`                              | Recursive transformation           |
| `visit(f)`                                  | Recursive traversal                |
| `node_count()`                              | Total number of nodes              |
| `depth()`                                   | Maximum depth                      |

### AstBuilder

| Method           | Description              |
| ---------------- | ------------------------ |
| `null()`         | Creates a null node      |
| `bool(v)`        | Creates a boolean        |
| `int(v)`         | Creates a signed integer |
| `uint(v)`        | Creates an unsigned int  |
| `float(v)`       | Creates a float          |
| `string(v)`      | Creates a string         |
| `array(items)`   | Creates an array         |
| `object(fields)` | Creates an object        |

## Contributions

Contributions are welcome! Please open an issue or pull request on the GitHub repository.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/ast_core/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
