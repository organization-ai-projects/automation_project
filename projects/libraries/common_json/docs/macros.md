# Macros de construction JSON

Ce module fournit des macros pour créer du JSON de manière déclarative, avec des fonctionnalités supplémentaires.

## Macros disponibles

| Macro          | Description                           |
| -------------- | ------------------------------------- |
| `pjson!`       | Macro principale, syntaxe JSON-like   |
| `pjson_key!`   | Helper pour les clés (interne)        |
| `json_array!`  | Création de tableaux avec expressions |
| `json_object!` | Création d'objets avec `=>`           |

## Syntaxe de `pjson!`

### Valeurs primitives

```rust
use common_json::pjson;

let null = pjson!(null);        // null
let t = pjson!(true);           // true
let f = pjson!(false);          // false
let n = pjson!(42);             // 42
let pi = pjson!(3.14);          // 3.14
let s = pjson!("hello");        // "hello"
```

### Tableaux

```rust
use common_json::pjson;

let empty = pjson!([]);
let numbers = pjson!([1, 2, 3]);
let mixed = pjson!([1, "two", true, null]);
let nested = pjson!([[1, 2], [3, 4]]);
```

### Objets

```rust
use common_json::pjson;

// Clés comme identifiants
let obj = pjson!({ name: "test", value: 42 });

// Clés comme chaînes (pour caractères spéciaux)
let obj = pjson!({ "key-with-dash": "value" });

// Clés dynamiques avec parenthèses
let key = "dynamic";
let obj = pjson!({ (key): "value" });
```

### Interpolation de variables

```rust
use common_json::pjson;

let name = "Alice";
let age = 30;

// Utilisez des parenthèses pour interpoler des variables
let user = pjson!({
    name: (name),
    age: (age)
});
```

### Structures imbriquées

```rust
use common_json::pjson;

let config = pjson!({
    server: {
        host: "localhost",
        port: 8080
    },
    features: ["auth", "api"],
    debug: true
});
```

### Trailing commas

Les virgules finales sont acceptées :

```rust
use common_json::pjson;

let obj = pjson!({ a: 1, b: 2, });  // OK
let arr = pjson!([1, 2, 3,]);       // OK
```

## Tests

Ce module contient 16 tests couvrant :

- Primitives : null, booléens, nombres, chaînes
- Tableaux : vides, simples, mixtes
- Objets : vides, simples, imbriqués
- Fonctionnalités : clés dynamiques, interpolation, trailing commas
- Macros alternatives : `json_array!`, `json_object!`

### Non couvert

- Gestion d'erreurs (types non sérialisables)
- Cas limites avec caractères Unicode dans les clés
