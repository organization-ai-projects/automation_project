# BumpArena

A bump arena for fast, append-only allocation.

## Performance Characteristics

- **Allocation**: O(1) amortized
- **Access by ID**: O(1)
- **Iteration**: O(n), cache-friendly
- **Memory**: Contiguous, no fragmentation

## When to Use

- Parse trees / ASTs that are built once and read many times
- Interned strings or symbols
- ECS entity storage (without removal)
- Graph nodes allocated in bulk

## Example

```rust
use hybrid_arena::{BumpArena, Id};

let mut arena: BumpArena<String> = BumpArena::new();
let id = arena.alloc("hello".to_string()).unwrap();
assert_eq!(arena.get(id), Some(&"hello".to_string()));
```
