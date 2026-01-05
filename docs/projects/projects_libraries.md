# Bibliothèques et Composantes Symboliques

## 1. Rôle détaillé des composantes

### 1.1 Common (`common`)

- **Types fondamentaux** : IDs, enums, états.
- **Erreurs communes** : Gestion des erreurs partagées.
- **Utilitaires génériques** : Fonctions et outils réutilisables.
- **Aucune dépendance runtime** : Pas de dépendances comme tokio, dioxus, etc.
- `common` ne doit contenir aucune logique métier, orchestration, ou accès I/O.

> Les contrats de communication sont définis dans `protocol`.

---

### 1.2 Composante Symbolique (`symbolic`)

- **Règles de linting** : Application des bonnes pratiques.
- **Analyse statique** : Vérification de la structure, des conventions et des patterns.
- **Moteur de règles et décisions** : Gestion des workflows symboliques.
- **Orchestration symbolique** : Coordination des sous-modules spécialisés.

> `symbolic` est un **agrégateur** de sous-modules symboliques spécialisés.

---

### 1.3 Composante Neuronale (`neural`)

- **Compréhension d’intentions** : Conversion du langage naturel en structure.
- **Génération de code Rust** : Création automatique de code.
- **Ajustement par feedback** : Amélioration continue basée sur les retours.
- **Entraînement et inférence** : Utilisation de **Burn** pour les modèles neuronaux.
- La composante `neural` n’est jamais appelée directement par les produits. Elle est invoquée uniquement via l’orchestrateur `ai`.

> Activation par feature flag uniquement.

---

### 1.4 Orchestrateur IA (`ai`)

- **Coordination** : Supervision des composantes `symbolic` et `neural`.
- **Décision intelligente** : Détermine quand déléguer au neuronal.
- **Isolation stricte** : Aucun état global stocké.
- **Travail contextuel** : Fonctionne exclusivement via un `ProjectContext`.
