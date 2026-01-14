# Types de base JSON et constructeurs

Ce module définit les types alias pour les valeurs JSON et fournit des fonctions constructeurs pour créer facilement des valeurs JSON.

## Types

| Type         | Description                        |
| ------------ | ---------------------------------- |
| `Json`       | Valeur JSON générique              |
| `JsonMap`    | Map clé-valeur pour objets         |
| `JsonArray`  | Tableau de valeurs JSON            |
| `JsonObject` | Alias pour `JsonMap<String, Json>` |
| `JsonNumber` | Nombre JSON (entier ou flottant)   |

## Constructeurs

Les constructeurs permettent de créer des valeurs JSON de manière explicite sans dépendre d'une implémentation interne. Ils sont aussi plus lisibles pour les cas simples.

### Exemples

```rust
use common_json::value::*;

let obj = object();           // {}
let arr = array();            // []
let n = null();               // null
let b = boolean(true);        // true
let s = string("hello");      // "hello"
let i = number_i64(42);       // 42
let u = number_u64(100);      // 100
let f = number_f64(3.14);     // Some(3.14) ou None si NaN/Infinity
```

## Pourquoi des constructeurs ?

Les constructeurs permettent de créer des valeurs JSON de manière explicite sans dépendre de la syntaxe `Json::Object(...)` qui expose l'implémentation interne. Ils sont aussi plus lisibles pour les cas simples.

## Tests

Ce module contient des tests couvrant :

- Création d'objets vides
- Création de tableaux vides
- Création de valeurs null
- Création de booléens (true/false)
- Création de chaînes
- Création de nombres (i64, u64, f64, et cas NaN)
