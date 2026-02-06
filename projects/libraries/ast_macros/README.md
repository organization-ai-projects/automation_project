# ast_macros

Reusable declarative macros for building Abstract Syntax Tree (AST) nodes.

## Overview

This crate provides a collection of declarative macros that make it easier to construct and validate AST nodes from the `ast_core` library. The macros have been extracted into a separate crate to promote reusability across different projects while maintaining a clean separation of concerns.

## Features

- **`build_array!`** - Construct AST arrays using familiar `[]` syntax
- **`build_object!`** - Construct AST objects using familiar `{}` syntax  
- **`key!`** - Create AST keys from identifiers, literals, or expressions
- **`value!`** - Construct various AST values (null, bool, numbers, arrays, objects)
- **`validate_preset!`** - Apply validation presets (strict, unbounded, default)
- **`apply_cfg!`** - Configure validation limits

## Usage

To use these macros, add both `ast_macros` and `ast_core` to your dependencies:

```toml
[dependencies]
ast_core = { workspace = true }
ast_macros = { workspace = true }
```

### Examples

```rust
use ast_core::{AstNode, AstKey, AstBuilder};
use ast_macros::{value, key, build_object, build_array};

// Create a simple value
let num = value!(42);

// Create an array
let arr = build_array!([1, 2, 3]);

// Create an object
let obj = build_object!({
    name: "test",
    count: 42,
    active: true
});

// Create a key
let k = key!(field_name);
```

## Compatibility

These macros use fully qualified paths to reference `ast_core` types (e.g., `::ast_core::AstNode`). This means:

- You must have `ast_core` in your dependencies to use these macros
- The macros maintain compatibility with `ast_core` types without creating circular dependencies
- You can use these macros in any crate that depends on `ast_core`

## Note

For the high-level `past!` macro that provides additional features like metadata and validation chaining, see the `ast_core` crate directly.
