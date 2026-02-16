# Bibliotheques et composants symboliques

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour a l'index projets](TOC.md)

## 1. Role detaille des composants

### 1.1 Common (`common`)

- **Types fondamentaux** : IDs, enums, etats.
- **Erreurs communes** : gestion d'erreurs partagee.
- **Utilitaires generiques** : fonctions et outils reutilisables.
- **Sans dependances runtime** : pas de dependances comme tokio, dioxus, etc.
- `common` ne doit contenir aucune logique metier, orchestration, ou acces I/O.

> Les contrats de communication sont definis dans `protocol`.

---

### 1.2 Composant symbolique (`symbolic`)

- **Regles de linting** : application des bonnes pratiques.
- **Analyse statique** : verification de structure, conventions et patterns.
- **Moteur de regles et de decision** : gestion des workflows symboliques.
- **Orchestration symbolique** : coordination de sous-modules specialises.

> `symbolic` est un **agregateur** de sous-modules symboliques specialises.

---

### 1.3 Composant neural (`neural`)

- **Compréhension d'intention** : conversion du langage naturel en structure.
- **Generation de code Rust** : creation automatique de code.
- **Ajustement par feedback** : amelioration continue basee sur le feedback.
- **Training et inference** : utilisation de **Burn** pour les modeles neuraux.
- Le composant `neural` n'est jamais appele directement par les produits. Il est invoque uniquement via l'orchestrateur `ai`.

> Activation via feature flag uniquement.

---

### 1.4 Catalogue des bibliotheques (vue workspace)

Bibliotheques actuelles sous `projects/libraries` :

- `ai` : orchestrateur pour flux symbolic + neural.
- `ast_core` : structures AST et utilitaires de parsing.
- `command_runner` : execution de commandes avec resultats structures.
- `common` : types, erreurs et utilitaires partages.
- `common_calendar` : utilitaires calendrier/date.
- `common_json` : modele JSON + helpers.
- `common_parsing` : helpers de parsing pour formats partages.
- `common_time` : utilitaires de temps.
- `common_tokenize` : utilitaires de tokenisation.
- `hybrid_arena` : stockage style arena avec indexation hybride.
- `identity` : types d'identite et helpers de store.
- `neural` : composant inference/training neural.
- `pjson_proc_macros` : proc-macros pour outillage JSON.
- `protocol` : contrats wire et types de protocole.
- `protocol_macros` : proc-macros pour helpers protocole.
- `security` : auth, tokens, claims et helpers de verification.
- `symbolic` : moteur d'analyse/validation symbolique.
- `ui` : composants UI partages pour UIs produits.

---

### 1.5 Exemple : utiliser la bibliotheque `common`

Ci-dessous un exemple d'utilisation de `common` pour definir et gerer les erreurs :

#### Exemple de code

```rust
use common::errors::{AppError, Result};

fn perform_action() -> Result<()> {
    // Example logic
    if some_condition {
        Err(AppError::new("An error occurred"))
    } else {
        Ok(())
    }
}

fn main() {
    match perform_action() {
        Ok(_) => println!("Action performed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

#### Explication

- `AppError` : type d'erreur partage defini dans `common`.
- `Result` : alias de type pour `Result<T, AppError>` afin de simplifier la gestion d'erreurs.

Cela montre comment la bibliotheque `common` fournit des utilitaires reutilisables pour une gestion d'erreurs coherente.

---

### 1.6 Orchestrateur AI (`ai`)

- **Coordination** : supervision des composants `symbolic` et `neural`.
- **Decision intelligente** : determine quand deleguer au neural.
- **Isolation stricte** : aucun etat global stocke.
- **Travail contextuel** : opere exclusivement via un `ProjectContext`.
