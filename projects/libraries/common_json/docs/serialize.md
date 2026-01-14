# Sérialisation JSON

Ce module fournit des outils pour sérialiser des données en JSON, avec deux approches principales :

- **Trait [`JsonSerializable`]** : API orientée objet pour les types implémentant `serde::Serialize`.
- **Fonctions standalone** : Utilisation directe sans implémentation de trait.

## Contenu

| API                           | Description                      | Retourne                          |
| ----------------------------- | -------------------------------- | --------------------------------- |
| [`JsonSerializable::to_json`] | Méthode trait → valeur JSON      | `Result<Json, JsonError>`         |
| [`to_json`]                   | Fonction → valeur JSON           | `Result<Json, JsonError>`         |
| [`to_string`]                 | Fonction → chaîne compacte       | `JsonResult<String>`              |
| [`to_string_pretty`]          | Fonction → chaîne formatée       | `JsonResult<String>`              |
| [`to_bytes`]                  | Fonction → bytes compacts        | `JsonResult<Vec<u8>>`             |
| [`to_bytes_pretty`]           | Fonction → bytes formatés        | `JsonResult<Vec<u8>>`             |
| [`write_to`]                  | Fonction → écriture dans `Write` | `JsonResult<()>`                  |
| [`write_to_pretty`]           | Fonction → écriture formatée     | `JsonResult<()>`                  |

## Exemple d'utilisation

### Trait vs Fonctions

Le trait [`JsonSerializable`] est automatiquement implémenté pour tout type implémentant `serde::Serialize`. Les deux approches sont équivalentes :

```rust
use common_json::{to_string, JsonSerializable};
use serde::Serialize;

#[derive(Serialize)]
struct User { name: String }

let user = User { name: "Alice".into() };

// Approche fonction
let s1 = to_string(&user).unwrap();

// Approche trait
let s2 = user.to_json_string().unwrap();
```

### Exemple complet

```rust
use common_json::{to_json, to_string, to_string_pretty, to_bytes, JsonSerializable};
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    name: String,
    debug: bool,
    max_connections: u32,
}

let config = Config {
    name: "my-app".into(),
    debug: true,
    max_connections: 100,
};

// Vers une valeur JSON
let json = to_json(&config).unwrap();
assert_eq!(json["name"], "my-app");

// Vers une chaîne compacte
let compact = to_string(&config).unwrap();
// {"name":"my-app","debug":true,"max_connections":100}

// Vers une chaîne formatée
let pretty = to_string_pretty(&config).unwrap();
// {
//   "name": "my-app",
//   "debug": true,
//   "max_connections": 100
// }

// Vers des bytes (pour I/O réseau)
let bytes = to_bytes(&config).unwrap();
```

## Alias legacy

Pour la compatibilité avec l'ancien code, les alias suivants sont disponibles :

- [`to_value`] → [`to_json`]
- [`to_json_string`] → [`to_string`]
- [`to_json_string_pretty`] → [`to_string_pretty`]

## Tests

Ce module contient des tests couvrant :

- `to_json` : conversion vers valeur JSON
- `to_string` : conversion vers chaîne
- `to_bytes` : conversion vers bytes
- Méthodes du trait `JsonSerializable`

### Non couvert

- `write_to` / `write_to_pretty` (écriture vers un `Write`)
- `to_string_pretty` / `to_bytes_pretty` (variantes formatées)
- Gestion d'erreurs sur types non sérialisables
