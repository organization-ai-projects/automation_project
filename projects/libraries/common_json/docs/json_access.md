# Accès fluide aux valeurs JSON

Ce module fournit des traits et des builders pour naviguer et construire des structures JSON de manière ergonomique.

## Contenu

| Type                | Description                    |
| ------------------- | ------------------------------ |
| `JsonAccess`        | Trait pour l'accès en lecture  |
| `JsonAccessMut`     | Trait pour l'accès en écriture |
| `JsonObjectBuilder` | Builder fluide pour objets     |
| `JsonArrayBuilder`  | Builder fluide pour tableaux   |

## Navigation par chemin

Le trait `JsonAccess` permet de naviguer dans des structures imbriquées avec la méthode `get_path` :

```rust
use common_json::{pjson, JsonAccess};

let data = pjson!({
    user: {
        profile: {
            name: "Alice"
        }
    },
    tags: ["admin", "user"]
});

// Navigation par points
assert_eq!(data.get_path("user.profile.name").unwrap().as_str(), Some("Alice"));

// Accès aux tableaux avec [index]
assert_eq!(data.get_path("tags[0]").unwrap().as_str(), Some("admin"));
assert_eq!(data.get_path("tags[1]").unwrap().as_str(), Some("user"));
```

## Accesseurs stricts

Les méthodes `as_*_strict()` retournent une erreur si le type ne correspond pas, au lieu de retourner `None` :

```rust
use common_json::{pjson, JsonAccess, JsonError};

let data = pjson!({ count: 42 });

// as_i64() retourne Option<i64>
assert_eq!(data.get_field("count").unwrap().as_i64(), Some(42));

// as_i64_strict() retourne Result<i64, JsonError>
assert_eq!(data.get_field("count").unwrap().as_i64_strict().unwrap(), 42);

// Erreur si mauvais type
let err = data.get_field("count").unwrap().as_str_strict();
assert!(matches!(err, Err(JsonError::TypeMismatch { .. })));
```

## Builders fluides

Les builders permettent de construire du JSON de manière lisible :

```rust
use common_json::{JsonObjectBuilder, JsonArrayBuilder};

let obj = JsonObjectBuilder::new()
    .field("name", "Alice")
    .field("age", 30)
    .field_opt("nickname", Some("Ali"))  // Ajouté car Some
    .field_opt::<_, &str>("email", None) // Ignoré car None
    .field_if(true, "active", true)      // Ajouté car condition vraie
    .build();

let arr = JsonArrayBuilder::new()
    .element(1)
    .element(2)
    .extend(vec![3, 4, 5])
    .build();
```

## Tests

Ce module contient 8 tests couvrant :

- `get_field` : accès aux champs d'objets
- `get_index` : accès aux éléments de tableaux
- `get_path` : navigation par chemin (dot notation + indices)
- `as_*_strict` : accesseurs stricts avec erreurs typées
- `type_name` : identification du type JSON
- `is_truthy` : évaluation booléenne des valeurs
- Mutations : `set_field`, `push`, etc.
- Builders : `JsonObjectBuilder` et `JsonArrayBuilder`

### Non couvert

- `get_field_mut`, `get_index_mut` (accesseurs mutables)
- `remove_field`, `remove_at` (suppressions)
- `insert_at` (insertion à un index)
- Chemins invalides complexes
