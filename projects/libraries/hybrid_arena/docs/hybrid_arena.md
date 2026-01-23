# `hybrid_arena` - High-Performance Arena Allocators

Fast, type-safe arena allocators with generation-checked IDs for Rust.

## Features

- **Type-safe IDs**: Compile-time guarantee that `Id<Foo>` can't access `Arena<Bar>`
- **Generation tracking**: Prevents use-after-free bugs with ABA problem prevention
- **Cache-friendly**: IDs packed into single `u64` for optimal cache performance
- **Zero-cost abstractions**: All bounds checks optimize away in release builds
- **Flexible allocation**: Choose between bump allocation or slot reuse
- **Rich API**: Iterators, batch allocation, Index traits, and more

## Arena Types

| Arena         | Allocation | Removal | Use Case                       |
| ------------- | ---------- | ------- | ------------------------------ |
| [`BumpArena`] | O(1)       | ❌      | ASTs, graphs, interned strings |
| [`SlotArena`] | O(1)       | O(1)    | ECS entities, object pools     |

## Quick Start

```rust
use hybrid_arena::{BumpArena, SlotArena, Id};

// BumpArena: fast append-only allocation
let mut bump: BumpArena<String> = BumpArena::new();
let id = bump.alloc("hello".to_string()).expect("alloc hello");
assert_eq!(bump[id], "hello");

// SlotArena: supports removal and reuse
let mut slots: SlotArena<i32> = SlotArena::new();
let id1 = slots.alloc(42).expect("alloc 42");
let id2 = slots.alloc(100).expect("alloc 100");
slots.remove(id1); // Slot is recycled
let id3 = slots.alloc(200).expect("alloc 200"); // Reuses slot 0
assert_eq!(id3.index(), id1.index()); // Same slot
assert_ne!(id3.generation(), id1.generation()); // Different generation
```

## ID System

IDs are packed into a single `u64` for cache efficiency:

- **Bits 0-31**: Index (supports up to 4 billion items)
- **Bits 32-63**: Generation (for use-after-free detection)

```rust
use hybrid_arena::Id;

let id: Id<String> = Id::new(42, 1);
assert_eq!(id.index(), 42);
assert_eq!(id.generation(), 1);
assert_eq!(std::mem::size_of::<Id<String>>(), 8); // Single u64
```

## Iteration

Both arenas support efficient iteration:

```rust
use hybrid_arena::BumpArena;

let mut arena: BumpArena<i32> = BumpArena::new();
arena.alloc_extend([1, 2, 3, 4, 5]).expect("extend arena");

// Reference iteration
for item in arena.iter() {
    println!("{}", item);
}

// Mutable iteration
for item in arena.iter_mut() {
    *item *= 2;
}

// Iteration with IDs
for (id, item) in arena.iter_with_ids() {
    println!("{}: {}", id, item);
}
```

## Self-Referential Structures

Use `alloc_with` to create items that know their own ID:

```rust
use hybrid_arena::{BumpArena, Id};

struct Node {
    id: Id<Node>,
    value: i32,
}

let mut arena: BumpArena<Node> = BumpArena::new();
let id = arena
    .alloc_with(|id| Node { id, value: 42 })
    .expect("alloc node");
assert_eq!(arena[id].id, id);
```

## Simultaneous Mutable Access

Safely mutate two items at once with `get_mut`:

```rust
use hybrid_arena::SlotArena;

let mut arena: SlotArena<i32> = SlotArena::new();
let id1 = arena.alloc(10).expect("alloc 10");
let id2 = arena.alloc(20).expect("alloc 20");

let (a, b) = arena.get_mut(id1, id2);
let a = a.expect("first id exists");
let b = b.expect("second id exists");
*a += 5;
*b += 5;
```

## Feature Flags

- **`serde`**: Enable serialization/deserialization support
- **`stable`**: Use only stable Rust features (default)

## Performance Tips

1. **Pre-allocate**: Use `with_capacity()` when size is known
2. **Batch allocation**: Use `alloc_extend()` for multiple items
3. **Use Index syntax**: `arena[id]` is as fast as `get().expect("present")`
4. **Choose wisely**: `BumpArena` for read-heavy, `SlotArena` for dynamic
