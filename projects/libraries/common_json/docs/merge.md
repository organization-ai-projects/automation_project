# Fusion, comparaison et transformation de JSON

Ce module fournit des utilitaires pour combiner, comparer et transformer des structures JSON.

## Contenu

| Fonction       | Description                                            |
| -------------- | ------------------------------------------------------ |
| `merge`        | Fusionne avec une stratégie choisie                    |
| `deep_merge`   | Fusion récursive des objets                            |
| `concat_merge` | Fusion avec concaténation des tableaux                 |
| `diff`         | Calcule les différences entre deux JSON                |
| `contains`     | Vérifie si un JSON contient un autre                   |
| `flatten`      | Aplatit un objet imbriqué en clés pointées             |
| `unflatten`    | Reconstruit un objet imbriqué depuis des clés pointées |

## Stratégiques de fusion

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

// deep_merge : fusionne les objets, remplace les tableaux
let merged = deep_merge(&base, &patch);
assert_eq!(merged["config"]["timeout"], 30);
assert_eq!(merged["config"]["debug"], true);
assert_eq!(merged["tags"], pjson!(["v2"])); // Remplacé

// concat_merge : fusionne les objets, concatène les tableaux
let merged = concat_merge(&base, &patch);
assert_eq!(merged["tags"].as_array().unwrap().len(), 2); // ["v1", "v2"]
```

## Diff et comparaison

```rust
use common_json::{pjson, diff, contains, JsonDiff};

let old = pjson!({ name: "v1", count: 10 });
let new = pjson!({ name: "v2", count: 10, extra: true });

let changes = diff(&old, &new);
// name: Changed { from: "v1", to: "v2" }
// extra: Added(true)

// contains vérifie l'inclusion partielle
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

Ce module contient 8 tests couvrant :

- `deep_merge` : fusion récursive
- `concat_merge` : fusion avec concaténation
- `diff` : comparaison (same, changed, added/removed)
- `contains` : inclusion partielle
- `flatten` / `unflatten` : aplatissement et reconstruction

### Non couvert

- `merge` avec `MergeStrategy::Replace`
- `PatchOp` (défini mais non implémenté)
- Diff sur tableaux avec éléments réordonnés
- Flatten de tableaux (uniquement objets supportés)
