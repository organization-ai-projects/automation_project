# Bot CI Harness (local)

This directory contains a local harness that tests the CI script for syncing main→dev **without act**, **without GitHub**, by mocking `gh`.

## Role in the Project

This harness is responsible for validating the sync automation logic in a controlled, repeatable environment without external dependencies.
It interacts mainly with:

- Sync scripts under `scripts/versioning/file_versioning/orchestrators/read/`
- Mocked GitHub CLI behaviors
- Scenario definitions in `scenarios/`

## Run all tests

```bash
tools/bot_ci_harness/run_all.sh
```

## Choose a different script

```bash
SCRIPT_UNDER_TEST=./scripts/.../synch_main_dev_ci.sh tools/bot_ci_harness/run_all.sh
```

## Add a scenario

Create a file `tools/bot_ci_harness/scenarios/XX_name.env` with:

- SCENARIO_NAME
- SETUP (noop|main_ahead|conflict)
- EXPECT_EXIT (0|1)
- MOCK\_\* (optional)
- BACKGROUND_MAIN_COMMIT_DELAY (optional, e.g., 1)
- BACKGROUND_MAIN_COMMIT_MSG (optional)
- BACKGROUND_DEV_COMMIT_DELAY (optional, e.g., 1)
- BACKGROUND_DEV_COMMIT_MSG (optional)
- STABLE_TIMEOUT_SECS (optional, e.g., 5)

## Directory Structure

```plaintext
tools/bot_ci_harness/
├── README.md
├── run_all.sh              # Main runner
├── run_failfast.sh         # Stop on first failure
├── run_parallel.sh         # Run scenarios in parallel
├── lib/
│   ├── assert.sh           # Assertion functions
│   └── git_sandbox.sh      # Temporary git repo creation
├── mocks/
│   └── gh                  # Mock GitHub CLI
├── scenarios/              # Scenario definitions
│   ├── 01_noop_dev_up_to_date.env
│   ├── 02_sync_needed_happy_path.env
│   ├── 03_merge_conflict.env
│   ├── 04_pr_already_exists.env
│   ├── 05_unstable_then_ok.env
│   ├── 06_main_advances_midrun.env
│   ├── 07_automerge_enable_fail.env
│   ├── 08_pr_exists_automerge_fail.env
│   ├── 09_dev_advances_midrun.env
│   └── 10_stable_timeout.env
├── scenario_generator.sh   # Scenario generator script
└── show_timing.sh           # Display scenario timing summary
```

## Files

- `README.md`: This file.
- `run_all.sh`: Main runner.
- `run_failfast.sh`: Stop on first failure.
- `run_parallel.sh`: Run scenarios in parallel.
- `show_timing.sh`: Display scenario timing summary.
- `scenario_generator.sh`: Scenario generator script.
- `lib/`: Assertion and sandbox helpers.
- `mocks/`: Mocked external commands.
- `scenarios/`: Scenario definitions.

## Covered scenarios

- ✅ **noop_dev_up_to_date**: dev already up-to-date, nothing to do
- ✅ **sync_needed_happy_path**: main ahead → PR creation + auto-merge
- ✅ **merge_conflict**: merge conflict detected
- ✅ **pr_already_exists**: existing PR reused
- ✅ **unstable_then_ok**: mergeable UNKNOWN then OK after a few calls
- ✅ **main_advances_midrun**: main advances during execution
- ✅ **automerge_enable_fail**: auto-merge enable fails
- ✅ **pr_exists_automerge_fail**: existing PR + auto-merge fails
- ✅ **dev_advances_midrun**: dev advances during execution
- ✅ **stable_timeout**: mergeable remains UNKNOWN until timeout

## What it covers

- The main→dev sync logic
- Git commands (fetch, merge, push)
- `gh` orchestration (PR creation, auto-merge, polling)
- Error scenarios (conflicts, existing PR, unstable states)

## What it does not cover (by design)

- Rulesets / bypass actors (GitHub backend side)
- Real GitHub auto-merge
- Real permissions and authentication

For these aspects, keep a real smoke test on GitHub.

## Why this approach?

This testing approach is superior to `act` for this use case because:

1. **Realism**: tests the actual logic without relying on complex Docker images
2. **Speed**: no Docker image pulls, instant execution
3. **Control**: precise mocking of GitHub behaviors (unstable states, conflicts, etc.)
4. **Maintainability**: simple bash code, easy to extend
5. **Determinism**: no surprises related to the Docker environment

## Generate a scenario automatically

A utility script is available to easily generate custom scenarios:

```bash
./tools/bot_ci_harness/scenario_generator.sh
```

This script guides you to create a `.env` file with the necessary parameters (name, setup type, mock states, etc.). The file will be automatically placed in the `scenarios/` folder with an incremented number.
