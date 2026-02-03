# `past!` Macro Documentation

- [Retour à Index de Documentation](TOC.md)


The `past!` macro is a high-level utility for building and validating Abstract Syntax Tree (AST) nodes. This document provides detailed usage examples and explanations.

## Modes

### Object

```rust
past!({ key: value, ... })
```

### Array

```rust
past!([1, 2, 3])
```

### Scalar

```rust
past!(42)
past!("string")
```

## Validation

### Default

```rust
past!({ ... }, validate)
```

### Preset

```rust
past!({ ... }, validate: preset: strict)
```

### Config

```rust
past!({ ... }, validate: cfg: { max_depth: 10, max_size: 100 })
```

## Metadata

### Origin

```rust
origin: ai("agent")
origin: tool("fmt")
```

### Flags

```rust
flags: ["generated", "cached"]
```

### Attributes

```rust
attrs: { "version": "1.0" }
```

## Chaining

Combine meta and validation clauses:

```rust
past!({ data: 42 }, origin: ai("gpt"), validate: preset: strict)
```

## Validate Existing Node

```rust
past!(node, validate)
past!(node, validate: preset: strict)
```

---

For more details, refer to the inline documentation in the `ast_macro.rs` file.
