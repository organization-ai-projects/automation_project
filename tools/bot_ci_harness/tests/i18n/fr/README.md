# Tests Bot CI Harness

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les tests du framework de test `bot_ci_harness` lui-meme.

## Tests unitaires

Les tests unitaires valident les fonctions de librairie individuellement, sans lancer tout le harness.

### Lancer les tests unitaires

```bash
./tools/bot_ci_harness/tests/unit_tests.sh
```

### Ce qui est teste

- **assert.sh**: fonctions d'assertion (`assert_eq`, `assert_ne`, `assert_contains`, ...)
- **validation.sh**: validation d'entrees (`validate_numeric`, `validate_enum`, ...)
- **git_operations.sh**: utilitaires Git (`git_branches_identical`, `git_get_sha`, ...)

### Ajouter un nouveau test unitaire

1. Creer une fonction dans `unit_tests.sh` avec le format `test_<module>_<fonction>()`
2. Utiliser `test_assert` pour l'executer
3. Retourner `0` en succes, `1` en echec

Exemple:

```bash
test_my_new_function() {
  if my_function "input"; then
    return 0
  else
    return 1
  fi
}

# In main():
test_assert "my_function with valid input" test_my_new_function
```

## Tests d'integration

Les tests d'integration sont les scenarios du repertoire parent.
Ils couvrent le workflow complet:

- Creation/initialisation de repository Git
- Mock GitHub CLI
- Execution des scripts
- Validation via assertions

Voir `../scenarios/` pour les definitions.
