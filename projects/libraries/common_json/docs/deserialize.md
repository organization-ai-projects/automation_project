# Désérialisation JSON avec trait et fonctions utilitaires

Ce module fournit deux approches pour désérialiser du JSON :

- Le trait `JsonDeserializable` pour une API orientée objet
- Des fonctions standalone pour une utilisation directe

## Architecture

| API               | Description                    | Source      |
| ----------------- | ------------------------------ | ----------- |
| `parse`           | Parse une chaîne → `Json`      | `&str`      |
| `parse_bytes`     | Parse des bytes → `Json`       | `&[u8]`     |
| `parse_reader`    | Parse un reader → `Json`       | `impl Read` |
| `from_json`       | Désérialise → `T`              | `&Json`     |
| `from_json_owned` | Désérialise (sans clone) → `T` | `Json`      |
| `from_str`        | Parse et désérialise → `T`     | `&str`      |
| `from_bytes`      | Parse et désérialise → `T`     | `&[u8]`     |
| `from_reader`     | Parse et désérialise → `T`     | `impl Read` |

## Parse vs Désérialisation

- **Parse** : Convertit du texte/bytes en `Json` (valeur générique)
- **Désérialisation** : Convertit du JSON en type Rust typé

### Exemple pour Parse vs Désérialisation

```rust
use common_json::{parse, from_str, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct User { name: String }

let json_str = r#"{"name": "Alice"}"#;

// Parse → Json générique
let json: Json = parse(json_str).unwrap();
assert_eq!(json["name"], "Alice");

// Désérialisation → Type concret
let user: User = from_str(json_str).unwrap();
assert_eq!(user.name, "Alice");
```

## Trait JsonDeserializable

Le trait est automatiquement implémenté pour tout type `T: DeserializeOwned`.

### Exemple pour Trait JsonDeserializable

```rust
use common_json::JsonDeserializable;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config { port: u16 }

let config = Config::from_json_str(r#"{"port": 8080}"#).unwrap();
assert_eq!(config.port, 8080);
```

## Fonctions standalone

### `parse`

```rust
use common_json::parse;

let json = parse(r#"{"name": "Alice", "age": 30}"#).unwrap();
assert_eq!(json["name"], "Alice");
assert_eq!(json["age"], 30);
```

### `parse_bytes`

```rust
use common_json::parse_bytes;

let json = parse_bytes(br#"[1, 2, 3]"#).unwrap();
assert_eq!(json[0], 1);
```

### `parse_reader`

```rust
use common_json::parse_reader;
use std::io::Cursor;

let cursor = Cursor::new(br#"{"key": "value"}"#);
let json = parse_reader(cursor).unwrap();
assert_eq!(json["key"], "value");
```

### `from_json`

```rust
use common_json::{from_json, pjson};
use serde::Deserialize;

#[derive(Deserialize)]
struct User { name: String }

let json = pjson!({ name: "Alice" });
let user: User = from_json(&json).unwrap();
assert_eq!(user.name, "Alice");
```

### `from_str`

```rust
use common_json::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct Point { x: i32, y: i32 }

let p: Point = from_str(r#"{"x": 10, "y": 20}"#).unwrap();
assert_eq!(p.x, 10);
```

## Alias legacy

Pour la compatibilité avec l'ancien code :

- `from_value` → `from_json_owned`
- `from_json_str` (fonction) → `from_str`

## Tests

Ce module contient 5 tests couvrant :

- `parse` : parsing de chaîne vers `Json`
- `parse_bytes` : parsing de bytes vers `Json`
- `from_str` : désérialisation directe depuis chaîne
- `from_json` : désérialisation depuis valeur `Json`
- Méthodes du trait `JsonDeserializable`

### Non couvert

- `parse_reader` / `from_reader` (lecture depuis `Read`)
- `from_json_owned` (sans clone)
- Gestion d'erreurs sur JSON malformé
