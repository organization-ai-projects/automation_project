# Conventions pour les conversions

Ce dossier contient les implémentations des conversions entre différents types utilisés dans le projet. Afin de maintenir une organisation claire et cohérente, les conventions suivantes ont été adoptées :

## Structure des fichiers

- Chaque type source possède son propre dossier dans `conversions/`.
  - Exemple : `feedback_verdict/`, `internal_feedback_verdict/`.
- À l'intérieur de chaque dossier, les fichiers sont nommés selon le type cible.
  - Exemple :
    - `feedback_verdict/internal_feedback_verdict.rs` : Contient les conversions de `FeedbackVerdict` vers `InternalFeedbackVerdict`.
    - `feedback_verdict/symbolic_feedback.rs` : Contient les conversions de `FeedbackVerdict` vers `SymbolicFeedback`.

## Règles générales

1. **Un fichier par conversion** : Chaque fichier doit contenir les implémentations de conversion pour un type cible spécifique.
2. **Documentation** : Chaque implémentation doit être accompagnée de commentaires expliquant son rôle et ses particularités.
3. **Tests** : Les tests unitaires pour les conversions doivent être placés dans les fichiers correspondants ou dans un dossier `tests/` si nécessaire.

## Exemple

### Conversion de `FeedbackVerdict` vers `InternalFeedbackVerdict`

Fichier : `feedback_verdict/internal_feedback_verdict.rs`

```rust
impl<'a> From<FeedbackVerdict<'a>> for InternalFeedbackVerdict {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => InternalFeedbackVerdict::Correct,
            FeedbackVerdict::Rejected => InternalFeedbackVerdict::Rejected,
            FeedbackVerdict::Incorrect { expected_output } => InternalFeedbackVerdict::Incorrect {
                expected_output: expected_output.into_owned(),
            },
            FeedbackVerdict::Partial { correction } => InternalFeedbackVerdict::Partial {
                correction: correction.into_owned(),
            },
        }
    }
}
```

Cette structure permet de localiser rapidement les conversions et de les maintenir facilement.

---

En suivant ces conventions, nous assurons une meilleure lisibilité et une évolutivité accrue du code.
