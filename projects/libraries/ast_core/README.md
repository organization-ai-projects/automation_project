# `ast_core`

## Description

`ast_core` est une bibliothèque générique pour représenter, valider et transformer des arbres syntaxiques abstraits (AST). Elle est conçue pour être utilisée dans des outils runtime, compile-time (proc-macros), et des systèmes d'IA.

Ici, **AST** désigne un **arbre de données structuré** (noeuds, clés, littéraux). Il n'est pas nécessairement lié à une grammaire complète ou à un langage spécifique.

## Règles fondamentales

### Règle 1 : AST structurel, pas un AST de langage complet

- **OK** :
  - Arbres, listes, maps, littéraux, identifiants, payloads opaques.
- **PAS OK** :
  - Parsing ou représentation complète d'un langage (Rust, JS, etc.).

`ast_core` est générique, mais pas un compilateur universel.

### Règle 2 : Indépendance des formats

- **OK** :
  - `ast_core` ne dépend d'aucun format spécifique (JSON, YAML, TOML, etc.).
- **PAS OK** :
  - Pas de dépendance à `serde_json`, `syn`, ou `quote` **dans les dépendances normales**.
  - Ces dépendances peuvent être autorisées derrière une feature optionnelle (`dev`, `test`) pour des besoins internes.

Les formats spécifiques doivent être gérés dans des crates "frontend" (ex. `json_frontend`, `yaml_frontend`).

### Règle 3 : Réutilisabilité

- `ast_core` doit être utilisable par :
  - Runtime (CLI/serveur).
  - Compile-time (proc-macros via adapter).
  - IA (réécriture/validation).

`ast_core` = types + validations + transformations pures.

## Fonctionnalités principales

### Types génériques

- `AstNode` : Représente les noeuds de l'AST avec métadonnées.
- `AstKind` : Le type du noeud (Null, Bool, Number, String, Array, Object, Opaque).
- `AstKey` : Représente les clés des objets (Ident ou String).
- `Number` : Nombres avec préservation du type (Int, Uint, Float).
- `AstMeta` : Métadonnées (span, origin, flags, attrs, ext).

### Builder ergonomique

```rust
use ast_core::{AstBuilder, AstNode};

let config = AstBuilder::object(vec![
    ("name", AstBuilder::string("my-app")),
    ("version", AstBuilder::int(1)),
    ("enabled", AstBuilder::bool(true)),
    ("tags", AstBuilder::array(vec![
        AstBuilder::string("production"),
        AstBuilder::string("stable"),
    ])),
]);

// Accès aux données
assert_eq!(config.get("name").unwrap().as_str(), Some("my-app"));
assert_eq!(config.get("version").unwrap().as_number().unwrap().as_i64(), Some(1));
```

### Validation

La validation concerne la **structure** de l'AST :

- Profondeur maximale
- Taille maximale (éléments par array/object)
- Clés dupliquées

```rust
use ast_core::{AstBuilder, ValidateLimits};

let node = AstBuilder::object(vec![
    ("a", AstBuilder::int(1)),
    ("b", AstBuilder::int(2)),
]);

// Validation avec limites par défaut
node.validate().expect("AST valide");

// Validation avec limites personnalisées
let limits = ValidateLimits {
    max_depth: 10,
    max_size: 100,
};
node.validate_with(&limits).expect("AST valide");
```

Les erreurs incluent le **path** vers l'emplacement de l'erreur :

```rust
// Erreur avec path: "at outer.inner: Exceeded maximum depth: 2"
```

**Non inclus** : Validation métier (ex. email valide, clé obligatoire selon une spécification).

### Transformation et traversal

```rust
use ast_core::AstBuilder;

let numbers = AstBuilder::array(vec![
    AstBuilder::int(1),
    AstBuilder::int(2),
    AstBuilder::int(3),
]);

// Doubler tous les nombres
let doubled = numbers.transform(&|node| {
    if let Some(n) = node.as_number() {
        if let Some(i) = n.as_i64() {
            return AstBuilder::int(i * 2);
        }
    }
    node.clone()
});

// Visiter tous les noeuds
let mut count = 0;
numbers.visit(&mut |_| count += 1);
assert_eq!(count, 4); // array + 3 ints

// Métriques
assert_eq!(numbers.node_count(), 4);
assert_eq!(numbers.depth(), 2);
```

### Métadonnées

```rust
use ast_core::{AstBuilder, Origin, Span};

let node = AstBuilder::string("hello")
    .with_span(0, 7)
    .with_origin(Origin::Parser("json"));

assert_eq!(node.meta.span, Some(Span { start: 0, end: 7 }));
```

## Non-objectifs

- Fournir un parser complet d'un langage.
- Contenir des règles métier spécifiques à un domaine.
- Imposer un format de sérialisation unique.

## API Reference

### AstNode

| Méthode                                     | Description                        |
| ------------------------------------------- | ---------------------------------- |
| `new(kind)`                                 | Crée un noeud avec le kind donné   |
| `with_meta(meta)`                           | Définit les métadonnées            |
| `with_span(start, end)`                     | Définit le span                    |
| `with_origin(origin)`                       | Définit l'origine                  |
| `validate()`                                | Valide avec limites par défaut     |
| `validate_with(limits)`                     | Valide avec limites personnalisées |
| `is_null/bool/number/string/array/object()` | Type checks                        |
| `as_bool/number/str/array/object()`         | Accesseurs typés                   |
| `get(key)`                                  | Accès par clé (objects)            |
| `get_index(i)`                              | Accès par index (arrays)           |
| `transform(f)`                              | Transformation récursive           |
| `visit(f)`                                  | Traversal récursif                 |
| `node_count()`                              | Nombre total de noeuds             |
| `depth()`                                   | Profondeur maximale                |

### AstBuilder

| Méthode          | Description              |
| ---------------- | ------------------------ |
| `null()`         | Crée un noeud null       |
| `bool(v)`        | Crée un booléen          |
| `int(v)`         | Crée un entier signé     |
| `uint(v)`        | Crée un entier non signé |
| `float(v)`       | Crée un flottant         |
| `string(v)`      | Crée une chaîne          |
| `array(items)`   | Crée un tableau          |
| `object(fields)` | Crée un objet            |

## Contributions

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request sur le dépôt GitHub.

Pour plus de détails sur le workflow Git/GitHub utilisé dans ce projet, consultez la [documentation sur le versioning](../../../docs/versioning/git-github.md).

## Licence
