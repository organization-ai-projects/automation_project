# PJSON Proc Macros Library

Procedural macros for JSON handling in `automation_project`.

## Overview

This crate provides procedural macros for generating validated JSON structures at compile time using the `ast_core` crate for AST representation.

## Features

- **Compile-time Validation** - JSON structure is validated during compilation
- **AST-based Processing** - Uses `ast_core` for robust AST representation
- **Type Safety** - Generates type-safe Rust code from JSON literals

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pjson_proc_macros = { path = "../pjson_proc_macros" }
```

## Usage

```rust
use pjson_proc_macros::pjson;

// String literals
let s = pjson!("hello");

// Numbers
let n = pjson!(42);

// Booleans
let b = pjson!(true);

// Arrays
let arr = pjson!([1, 2, 3]);

// Objects (using struct syntax)
let obj = pjson!(MyStruct { name: "Alice", age: 30 });
```

## Supported Types

| Input Type    | Output                           |
| ------------- | -------------------------------- |
| String literal| `AstKind::String`                |
| Integer       | `AstKind::Number`                |
| Boolean       | `AstKind::Bool`                  |
| Array         | `AstKind::Array`                 |
| Struct        | `AstKind::Object`                |

## Note

For runtime JSON construction with more flexibility, see the `common_json` crate which provides the `pjson!` macro for runtime use.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/pjson_proc_macros/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
