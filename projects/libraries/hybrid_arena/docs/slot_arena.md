# `SlotArena` - Dynamic Slot-Based Arena Allocator

The `SlotArena` is a dynamic arena allocator that supports allocation, removal, and slot reuse. It is designed for use cases where objects are frequently added and removed, while maintaining stable IDs.

## Features

- **Dynamic allocation and removal**: Supports efficient allocation and deallocation.
- **Slot reuse**: Recycles freed slots for future allocations.
- **Generation tracking**: Prevents use-after-free bugs by invalidating stale IDs.
- **Stable IDs**: IDs remain valid across reallocations.

## Performance Characteristics

| Operation    | Complexity  |
| ------------ | ----------- |
| Allocation   | O(1)        |
| Removal      | O(1)        |
| Access by ID | O(1)        |
| Iteration    | O(capacity) |

## Use Cases

- Object pools where items are frequently added/removed.
- Entity-Component-System (ECS) entity storage.
- Handle-based resource management.
- Scenarios requiring stable IDs that survive reallocation.

## Example

```rust
use hybrid_arena::{SlotArena, Id};

let mut arena: SlotArena<String> = SlotArena::new();
let id = arena.alloc("hello".to_string()).expect("alloc hello");
assert_eq!(arena.get(id), Some(&"hello".to_string()));

let removed = arena.remove(id);
assert_eq!(removed, Some("hello".to_string()));
assert!(arena.get(id).is_none()); // ID is now invalid
```

## API Overview

### Allocation

- `SlotArena::alloc`: Allocates an item and returns its ID.
- `SlotArena::alloc_with`: Allocates an item using a closure that receives the ID.
- `SlotArena::alloc_extend`: Allocates multiple items from an iterator.

### Access

- `SlotArena::get`: Returns a reference to the item with the given ID.
- `SlotArena::get_mut`: Returns mutable references to two items simultaneously.
- `SlotArena::contains`: Checks if an ID refers to a valid item.

### Removal

- `SlotArena::remove`: Removes and returns the item with the given ID.
- `SlotArena::remove_drop`: Removes an item without returning it.

### Iteration

- `SlotArena::iter`: Returns an iterator over references to active items.
- `SlotArena::iter_mut`: Returns an iterator over mutable references to active items.
- `SlotArena::iter_with_ids`: Returns an iterator over (ID, &T) pairs for active items.
- `SlotArena::ids`: Returns an iterator over IDs of active items.

### Utilities

- `SlotArena::len`: Returns the number of active items.
- `SlotArena::is_empty`: Checks if the arena has no active items.
- `SlotArena::capacity`: Returns the total slot capacity (including free slots).
- `SlotArena::free_count`: Returns the number of free slots available for reuse.
- `SlotArena::reserve`: Reserves capacity for additional items.
- `SlotArena::clear`: Clears the arena, removing all items.
- `SlotArena::drain`: Removes all items but keeps the generation counters.

## Safety

The `SlotArena` ensures memory safety by:

- Invalidating IDs when their slots are freed.
- Using generation counters to prevent access to reused slots.

However, the `SlotArena` provides unsafe methods (`get_unchecked_id` and `get_unchecked_id_mut`) for advanced use cases where the caller guarantees the validity of IDs.

## Notes

- Iteration skips empty slots, making it efficient for sparse data.
- Free slots are reused in LIFO order for better cache performance.
- Generations wrap around on overflow, but this is unlikely to cause issues in practice.
