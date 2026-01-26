# Hybrid Arena Library

High-performance arena allocators for `automation_project`.

## Overview

This library provides two arena allocator types optimized for different use cases: `BumpArena` for append-only allocation with excellent cache locality, and `SlotArena` for generational allocation with removal support.

## Features

- **BumpArena** - O(1) append-only allocation with excellent cache locality
- **SlotArena** - Generational allocation with stable IDs and removal support
- **Type-Safe IDs** - Generic `Id<T>` type prevents mixing IDs from different arenas
- **Iterator Support** - Full iterator implementations (iter, iter_mut, drain, into_iter)

## Arena Comparison

| Feature           | BumpArena             | SlotArena                    |
| ----------------- | --------------------- | ---------------------------- |
| Allocation        | O(1)                  | O(1) amortized               |
| Removal           | Not supported         | O(1) with generation check   |
| Memory layout     | Contiguous            | Sparse with free list        |
| ID validity       | Always valid          | Generation-checked           |
| Use case          | Batch processing      | Entity management            |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hybrid_arena = { path = "../hybrid_arena" }
```

## Usage

### BumpArena - Fast append-only allocation

```rust
use hybrid_arena::BumpArena;

let mut arena = BumpArena::new();

// Allocate items
let id1 = arena.alloc("first")?;
let id2 = arena.alloc("second")?;

// Access by ID
println!("{}", arena.get(id1).unwrap());

// Iterate all items
for item in arena.iter() {
    println!("{}", item);
}

// Allocate with self-reference
let id = arena.alloc_with(|my_id| {
    format!("My ID index is {}", my_id.index())
})?;
```

### SlotArena - Allocation with removal

```rust
use hybrid_arena::SlotArena;

let mut arena = SlotArena::new();

// Allocate items
let id1 = arena.alloc("entity1")?;
let id2 = arena.alloc("entity2")?;

// Remove an item
arena.remove(id1);

// Old ID is now invalid (generation mismatch)
assert!(arena.get(id1).is_none());

// Slot can be reused
let id3 = arena.alloc("entity3")?;
```

### Using the prelude

```rust
use hybrid_arena::prelude::*;

let mut bump: BumpArena<i32> = BumpArena::new();
let mut slot: SlotArena<String> = SlotArena::new();
```

### Type-safe IDs

```rust
use hybrid_arena::{BumpArena, Id};

let mut strings: BumpArena<String> = BumpArena::new();
let mut numbers: BumpArena<i32> = BumpArena::new();

let str_id: Id<String> = strings.alloc("hello".to_string())?;
let num_id: Id<i32> = numbers.alloc(42)?;

// Compile error: can't use str_id with numbers arena
// numbers.get(str_id);
```

## Error Handling

```rust
use hybrid_arena::{BumpArena, ArenaError};

let mut arena = BumpArena::new();

match arena.alloc("item") {
    Ok(id) => println!("Allocated at index {}", id.index()),
    Err(ArenaError::Overflow) => eprintln!("Arena overflow (>2^32 items)"),
}
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [BumpArena](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/hybrid_arena/documentation/bump_arena.md)
- [SlotArena](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/hybrid_arena/documentation/slot_arena.md)
- [Table of Contents](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/hybrid_arena/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
