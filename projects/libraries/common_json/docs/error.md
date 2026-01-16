# Types d'erreurs JSON avec contexte riche

Ce module fournit des types d'erreurs spécifiques aux opérations JSON, permettant de diagnostiquer précisément les problèmes lors de la manipulation de données JSON.

## Types d'erreurs

| Variante                      | Description                             | Exemple d'utilisation       |
| ----------------------------- | --------------------------------------- | --------------------------- |
| `JsonError::Serialize`        | Erreur de sérialisation/désérialisation | JSON malformé               |
| `JsonError::TypeMismatch`     | Type attendu différent du type trouvé   | `as_str()` sur un nombre    |
| `JsonError::MissingField`     | Champ absent dans un objet              | `obj["missing"]`            |
| `JsonError::IndexOutOfBounds` | Index hors limites dans un tableau      | `arr[10]` sur tableau de 3  |
| `JsonError::InvalidPath`      | Expression de chemin invalide           | `"user..name"`              |
| `JsonError::UnexpectedNull`   | Valeur null inattendue                  | Champ requis est null       |
| `JsonError::ParseError`       | Erreur de parsing avec position         | Ligne/colonne de l'erreur   |
| `JsonError::Custom`           | Erreur personnalisée                    | Messages spécifiques métier |

## Exemple

```rust
use common_json::{pjson, JsonAccess, JsonError};

let data = pjson!({ name: "test" });

// TypeMismatch: essayer de lire un string comme i64
let result = data.get_field("name").unwrap().as_i64_strict();
assert!(matches!(result, Err(JsonError::TypeMismatch { .. })));

// MissingField: champ inexistant
let result = data.get_field("missing");
assert!(matches!(result, Err(JsonError::MissingField { .. })));
```

## Exemples supplémentaires

### Exemple d'erreur TypeMismatch

```rust
use common_json::JsonError;

let err = JsonError::type_mismatch("string", "number");
assert!(err.to_string().contains("expected string"));
```

### Exemple d'erreur MissingField

```rust
use common_json::JsonError;

let err = JsonError::missing_field("username");
assert!(err.to_string().contains("username"));
```

### Exemple d'erreur IndexOutOfBounds

```rust
use common_json::JsonError;

let err = JsonError::index_out_of_bounds(10, 3);
// Message: "Index out of bounds: 10 (array length: 3)"
```

### Exemple d'erreur personnalisée

```rust
use common_json::JsonError;

let err = JsonError::custom("Invalid user configuration");
```

## Tests

Ce module contient 3 tests couvrant :

- Création et vérification d'erreurs `TypeMismatch`
- Création et vérification d'erreurs `MissingField`
- Création et formatage d'erreurs `IndexOutOfBounds`

### Non couvert

- `InvalidPath`, `UnexpectedNull`, `ParseError`, `Custom` (constructeurs testés indirectement)
- `is_serialize()` (testé indirectement via les modules de désérialisation)
