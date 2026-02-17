# Bot CI Harness (local)

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient un harness local pour tester le script de synchronisation main->dev **sans act**, **sans GitHub reel**, via un mock `gh`.

## Role dans le projet

Le harness valide la logique d'automatisation dans un environnement controle, reproductible et sans dependances externes.
Il interagit principalement avec:

- Les scripts de sync dans `scripts/versioning/file_versioning/orchestrators/read/`
- Les comportements GitHub CLI mockes
- Les scenarios definis dans `scenarios/`

## Pre-requis

- Bash 4.0+
- Git 2.0+
- Utilitaires Unix standards (`mktemp`, `date`, `grep`, `sed`, ...)

Pas de Docker, pas d'auth GitHub necessaires.

## Demarrage rapide

Lancer tous les tests d'integration:

```bash
tools/bot_ci_harness/run_all.sh
```

Lancer les tests unitaires:

```bash
tools/bot_ci_harness/tests/unit_tests.sh
```

Lancer un scenario precis:

```bash
tools/bot_ci_harness/run_all.sh --scenario 2
```

Mode verbeux:

```bash
tools/bot_ci_harness/run_all.sh --verbose
```

## Choisir un autre script sous test

```bash
SCRIPT_UNDER_TEST=./scripts/.../synch_main_dev_ci.sh tools/bot_ci_harness/run_all.sh
```

## Ajouter un scenario

Creer `tools/bot_ci_harness/scenarios/XX_name.env` avec:

- `SCENARIO_NAME`
- `SETUP` (`noop|main_ahead|conflict`)
- `EXPECT_EXIT` (`0|1`)
- `MOCK_*` (optionnel)
- `BACKGROUND_MAIN_COMMIT_DELAY` / `BACKGROUND_MAIN_COMMIT_MSG` (optionnel)
- `BACKGROUND_DEV_COMMIT_DELAY` / `BACKGROUND_DEV_COMMIT_MSG` (optionnel)
- `STABLE_TIMEOUT_SECS` (optionnel)

## Structure du repertoire

```plaintext
tools/bot_ci_harness/
├── README.md
├── run_all.sh
├── run_failfast.sh
├── run_parallel.sh
├── lib/
│   ├── assert.sh
│   ├── git_sandbox.sh
│   ├── git_operations.sh
│   ├── logging.sh
│   └── mock_setup.sh
├── mocks/
│   └── gh
├── scenarios/
│   ├── 01_noop_dev_up_to_date.env
│   ├── 02_sync_needed_happy_path.env
│   ├── ...
│   └── 10_stable_timeout.env
├── scenario_generator.sh
└── show_timing.sh
```

## Points forts

- **Isolation sandbox**: chaque test dans un repo Git temporaire dedie
- **Mock GitHub CLI**: simulation complete des operations PR
- **Logs structures**: timestamps + niveaux de log
- **Design modulaire**: librairies reutilisables et extension facile

## Scenarios couverts

- `noop_dev_up_to_date`
- `sync_needed_happy_path`
- `merge_conflict`
- `pr_already_exists`
- `unstable_then_ok`
- `main_advances_midrun`
- `automerge_enable_fail`
- `pr_exists_automerge_fail`
- `dev_advances_midrun`
- `stable_timeout`

## Ce que couvre le harness

- Logique de sync main->dev
- Orchestration Git (`fetch`, `merge`, `push`)
- Orchestration `gh` (creation PR, auto-merge, polling)
- Cas d'erreur (conflits, PR existante, etats instables)

## Ce qu'il ne couvre pas volontairement

- Rulesets GitHub reels / bypass actors
- Auto-merge GitHub reel
- Permissions et auth reelles

Ces points doivent etre verifies via smoke tests GitHub reels.
