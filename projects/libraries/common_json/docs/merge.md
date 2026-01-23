# JSON Merging, Comparison, and Transformation

This module provides utilities for combining, comparing, and transforming JSON structures.

## Contents

| Function       | Description                                   |
| -------------- | --------------------------------------------- |
| `merge`        | Merges with a chosen strategy                 |
| `deep_merge`   | Recursive merging of objects                  |
| `concat_merge` | Merging with array concatenation              |
| `diff`         | Computes differences between two JSONs        |
| `contains`     | Checks if one JSON contains another           |
| `flatten`      | Flattens a nested object into dotted keys     |
| `unflatten`    | Reconstructs a nested object from dotted keys |

## Merging Strategies

```rust
use common_json::{pjson, deep_merge, concat_merge};

let base = pjson!({
    name: "app",
    config: { timeout: 30 },
    tags: ["v1"]
});

let patch = pjson!({
    config: { debug: true },
    tags: ["v2"]
});

// deep_merge: merges objects, replaces arrays
let merged = deep_merge(&base, &patch);
assert_eq!(merged["config"]["timeout"], 30);
assert_eq!(merged["config"]["debug"], true);
assert_eq!(merged["tags"], pjson!(["v2"])); // Replaced

// concat_merge: merges objects, concatenates arrays
let merged = concat_merge(&base, &patch);
assert_eq!(merged["tags"].as_array().unwrap().len(), 2); // ["v1", "v2"]
```

## Diff and Comparison

```rust
use common_json::{pjson, diff, contains, JsonDiff};

let old = pjson!({ name: "v1", count: 10 });
let new = pjson!({ name: "v2", count: 10, extra: true });

let changes = diff(&old, &new);
// name: Changed { from: "v1", to: "v2" }
// extra: Added(true)

// contains checks for partial inclusion
let haystack = pjson!({ user: { name: "Alice", age: 30 }, active: true });
assert!(contains(&haystack, &pjson!({ user: { name: "Alice"} })));
```

## Flatten / Unflatten

```rust
use common_json::{pjson, flatten, unflatten};

let nested = pjson!({
    user: {
        profile: {
            name: "Alice"
        }
    }
});

let flat = flatten(&nested);
assert_eq!(flat["user.profile.name"], "Alice");

let restored = unflatten(&flat);
assert_eq!(restored["user"]["profile"]["name"], "Alice");
```

## Tests

This module contains 8 tests covering :

- `deep_merge` : recursive merging
- `concat_merge` : merging with concatenation
- `diff` : comparison (same, changed, added/removed)
- `contains` : partial inclusion
- `flatten` / `unflatten` : flattening and reconstruction

### Not covered

- `merge` with `MergeStrategy::Replace`
- `PatchOp` (defined but not implemented)
- Diff on arrays with reordered elements
- Flattening of arrays (only objects supported)
