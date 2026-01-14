# Builder fluide pour tableaux JSON

Permet de construire des tableaux JSON de manière lisible et type-safe.

## Méthodes

| Méthode       | Description               |
| ------------- | ------------------------- |
| `element`     | Ajoute un élément         |
| `element_opt` | Ajoute si `Some`          |
| `element_if`  | Ajoute si condition vraie |
| `extend`      | Ajoute plusieurs éléments |
| `build`       | Finalise le tableau       |

## Exemple

```rust
use common_json::JsonArrayBuilder;

let arr = JsonArrayBuilder::new()
    .element(1)
    .element("two")
    .element(true)
    .extend(vec![4, 5, 6])
    .build();

assert_eq!(arr.as_array().unwrap().len(), 6);
```

## Tests

Ce module contient des tests couvrant :

- Ajout d'éléments simples avec `element`
- Ajout conditionnel avec `element_opt` et `element_if`
- Extension avec plusieurs éléments via `extend`
- Finalisation et validation du tableau avec `build`
