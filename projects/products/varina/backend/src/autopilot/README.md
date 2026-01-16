# Module Autopilot

Le module `autopilot` est conçu pour automatiser les tâches Git tout en respectant des politiques de sécurité strictes. Ce document explique les différentes parties du module et leur rôle.

## Structure du module

### 1. `autopilot_error.rs`

Ce fichier définit la structure `AutopilotError`, utilisée pour encapsuler les erreurs spécifiques à l'autopilot. Il permet de convertir facilement des erreurs externes (comme `CommandError`) en erreurs internes.

### 2. `autopilot_mode.rs`

Contient l'énumération `AutopilotMode`, qui définit les modes d'exécution de l'autopilot :

- `DryRun` : Ne modifie rien, génère uniquement un plan.
- `ApplySafe` : Applique les changements uniquement si les vérifications de sécurité passent.

### 3. `autopilot_plan.rs`

Définit la structure `AutopilotPlan`, qui représente un plan d'action généré par l'autopilot. Les champs incluent :

- `branch` : La branche cible.
- `will_stage` : Les fichiers à ajouter.
- `will_commit` : Indique si un commit sera effectué.
- `commit_message` : Le message de commit.
- `will_push` : Indique si un push sera effectué.
- `notes` : Notes supplémentaires.

### 4. `autopilot_policy.rs`

Définit la structure `AutopilotPolicy`, qui contient les règles de sécurité pour l'autopilot. Par exemple :

- Branches protégées.
- Préfixes pertinents et bloqués.
- Autorisation de push automatique.

### 5. `autopilot_report.rs`

Contient la structure `AutopilotReport`, qui combine le plan, les changements classifiés, et les logs d'exécution. Permet de suivre ce qui a été fait ou refusé.

## Tests

Chaque fichier inclut des tests unitaires pour valider son comportement. Ces tests couvrent les cas d'utilisation principaux et les valeurs par défaut.

## Utilisation

Le module est conçu pour être utilisé dans des scénarios où des changements Git doivent être automatisés tout en respectant des politiques strictes. Les principales étapes sont :

1. Générer un plan avec `AutopilotPlan`.
2. Appliquer les changements si le mode est `ApplySafe`.
3. Générer un rapport avec `AutopilotReport`.

## Contribution

Si vous ajoutez de nouvelles fonctionnalités, assurez-vous de :

- Ajouter des commentaires explicatifs.
- Écrire des tests unitaires.
- Mettre à jour ce fichier si nécessaire.
