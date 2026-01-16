# Builder fluide pour objets JSON

Permet de construire des objets JSON de manière lisible et type-safe.

## Méthodes

| Méthode     | Description               |
| ----------- | ------------------------- |
| `field`     | Ajoute un champ           |
| `field_opt` | Ajoute si `Some`          |
| `field_if`  | Ajoute si condition vraie |
| `build`     | Finalise l'objet          |

## Exemple

```rust
use common_json::{JsonObjectBuilder, JsonAccess};

let user = JsonObjectBuilder::new()
    .field("name", "Alice")
    .field("age", 30)
    .field_opt("email", Some("alice@example.com"))
    .field_if(true, "verified", true)
    .build();

assert_eq!(user.get_field("name").unwrap().as_str(), Some("Alice"));
assert_eq!(user.get_field("age").unwrap().as_i64(), Some(30));
```

## Tests

Ce module contient des tests couvrant :

- Ajout de champs simples avec `field`
- Ajout conditionnel avec `field_opt` et `field_if`
- Finalisation et validation de l'objet avec `build`
