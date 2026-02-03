# JSON Merging, Comparison, and Transformation

- [Retour à Index de Documentation](TOC.md)

This module provides utilities for combining, comparing, and transforming JSON structures.

## Contents

| API                       | Description                                   |
| ------------------------- | --------------------------------------------- |
| `merge`                   | Merges with a chosen strategy                 |
| `JsonComparison::compare` | Computes differences between two JSON values  |
| `contains`                | Checks if one JSON contains another           |
| `flatten`                 | Flattens a nested object into dotted keys     |
| `unflatten`               | Reconstructs a nested object from dotted keys |

## Merging Strategies

```rust
use common_json::{pjson, merge, MergeStrategy, JsonAccess};

let base = pjson!({
    name: "app",
    config: { timeout: 30 },
    tags: ["v1"]
});

let patch = pjson!({
    config: { debug: true },
    tags: ["v2"]
});

// MergeStrategy::DeepMerge: merges objects, replaces arrays
let merged = merge(&base, &patch, MergeStrategy::DeepMerge);
assert_eq!(merged.get_path("config.timeout").expect("timeout").as_i64(), Some(30));
assert_eq!(merged.get_path("config.debug").expect("debug").as_bool(), Some(true));
assert_eq!(merged.get_field("tags").expect("tags"), &pjson!(["v2"])); // Replaced

// MergeStrategy::Concat: merges objects, concatenates arrays
let merged = merge(&base, &patch, MergeStrategy::Concat);
assert_eq!(merged.get_field("tags").expect("tags").as_array().expect("array").len(), 2); // ["v1", "v2"]
```

## Diff and Comparison

```rust
use common_json::{pjson, contains, JsonComparison};

let old = pjson!({ name: "v1", count: 10 });
let new = pjson!({ name: "v2", count: 10, extra: true });

let changes = JsonComparison::compare(&old, &new);
assert!(changes.object_differences.contains_key("name"));
assert!(changes.object_differences.contains_key("extra"));

// contains checks for partial inclusion
let haystack = pjson!({ user: { name: "Alice", age: 30 }, active: true });
assert!(contains(&haystack, &pjson!({ user: { name: "Alice"} })));
```

## Flatten / Unflatten

```rust
use common_json::{pjson, flatten, unflatten, JsonAccess};

let nested = pjson!({
    user: {
        profile: {
            name: "Alice"
        }
    }
});

let flat = flatten(&nested);
assert_eq!(
    flat.get_field("user.profile.name").expect("Missing 'user.profile.name'").as_str(),
    Some("Alice")
);

let restored = unflatten(&flat);
assert_eq!(
    restored.get_path("user.profile.name").expect("Missing 'user.profile.name'").as_str(),
    Some("Alice")
);
```

## Tests

This module contains 8 tests covering :

- `merge` with `MergeStrategy::DeepMerge`
- `merge` with `MergeStrategy::Concat`
- `JsonComparison::compare` (same, changed, added/removed)
- `contains` : partial inclusion
- `flatten` / `unflatten` : flattening and reconstruction

### Not covered

- `merge` with `MergeStrategy::Replace`
- `PatchOp` (defined but not implemented)
- Diff on arrays with reordered elements
- Flattening of arrays (only objects supported)
