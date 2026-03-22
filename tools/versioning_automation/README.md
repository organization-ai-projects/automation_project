# versioning_automation (`va`)

Rust CLI for Git workflow automation, PR lifecycle management, and issue tracking.

---

## Installation

```bash
# Development — recompiles on change
cargo run -p versioning_automation -- <subcommand> [args...]

# Stable — installs `va` to ~/.cargo/bin/
cargo install --path tools/versioning_automation
```

Re-run `cargo install` after changes you want reflected in the installed binary.

**Prerequisites:** `cargo`, [`gh`](https://cli.github.com/) authenticated, `pnpm`, `bash`

---

## Setup

Install Git hooks once per clone:

```bash
va automation install-hooks
```

This installs `pre-commit`, `commit-msg`, `prepare-commit-msg`, `pre-push`, `post-checkout`, and `pre-branch-create`.

---

## Core Workflows

### Create a PR

```bash
va pr generate-description --auto --base dev --head $(git branch --show-current) --yes
```

| Option   | Description                              |
| -------- | ---------------------------------------- |
| `--auto` | Non-interactive, creates the PR directly |
| `--base` | Base branch (default: `dev`)             |
| `--head` | Head branch (default: current branch)    |
| `--yes`  | Auto-confirm prompts                     |

### Create an issue

```bash
va issue create \
  --title "..." \
  --context "..." \
  --problem "..." \
  --acceptance "..." \
  [--parent <number>] \
  [--label <name>] \
  [--assignee <login>] \
  [--repo owner/name]
```

### Update a PR Description

```bash
va pr generate-description --refresh-pr <pr_number> --base <base_branch> --head <head_branch> --yes
```

| Option         | Description                           |
| -------------- | ------------------------------------- |
| `--refresh-pr` | Updates the specified PR directly     |
| `<pr_number>`  | The pull request number to update     |
| `--base`       | Base branch (default: `dev`)          |
| `--head`       | Head branch (default: current branch) |
| `--yes`        | Auto-confirm prompts                  |

This command regenerates the description for the specified PR and updates it automatically.

---

## Commit Message Format

```
<type>(<scope>): <description>
```

**Types:** `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`

**Scope:** path of the touched crate, resolved automatically from staged files.

```bash
# Example
git commit -m "refactor(projects/products/unstable/neurosymbolic_moe/backend): update tests"
```

---

## Hook Bypass Variables

| Variable                               | Effect                                  |
| -------------------------------------- | --------------------------------------- |
| `SKIP_PRE_COMMIT=1`                    | Skip all pre-commit checks              |
| `SKIP_COMMIT_VALIDATION=1`             | Skip commit message validation          |
| `ALLOW_PROTECTED_BRANCH_COMMIT=1`      | Allow direct commit on `dev`/`main`     |
| `ALLOW_PART_OF_ONLY_PUSH=1`            | Bypass assignment policy check          |
| `SKIP_POST_CHECKOUT_CONVENTION_WARN=1` | Suppress post-checkout warnings         |
| `SKIP_PREPARE_COMMIT_MSG=1`            | Skip commit message template generation |

---

## Troubleshooting

**`gh` not authenticated**

```bash
gh auth login
```

**Commit rejected — scope mismatch**

The scope must match the crate path resolved from staged files:

```bash
# staged: projects/products/unstable/neurosymbolic_moe/backend/src/foo.rs
git commit -m "refactor(projects/products/unstable/neurosymbolic_moe/backend): description"
```

**Skip hooks temporarily**

```bash
SKIP_PRE_COMMIT=1 git commit ...
SKIP_COMMIT_VALIDATION=1 git commit ...
```

---

## Full Command Reference

<details>
<summary>PR commands</summary>

```
va pr generate-description [--auto] [--base <ref>] [--head <ref>] [--yes]
                           [--dry-run [OUTPUT_FILE]] [--auto-edit <pr>]
                           [--refresh-pr <pr>] [--validation-only]
                           [--duplicate-mode safe|auto-close]
                           [--allow-partial-create]
va pr breaking-detect      [--text "..."|--stdin|--input-file path] [--labels-raw "a||b"]
va pr body-context         --pr <number> [--repo owner/name]
va pr child-pr-refs        --pr <number> [--repo owner/name]
va pr details              --pr <number> [--repo owner/name]
va pr field                --pr <number> --name <field> [--repo owner/name]
va pr pr-state             --pr <number> [--repo owner/name]
va pr refresh-validation   --pr <number> [--repo owner/name]
va pr text-payload         --pr <number> [--repo owner/name]
va pr update-body          --pr <number> [--repo owner/name] --body "..."
va pr upsert-comment       --pr <number> [--repo owner/name] --marker "..." --body "..."
va pr auto-add-closes      --pr <number> [--repo owner/name]
va pr issue-context        --issue <number> [--repo owner/name]
va pr issue-view           --issue <number> [--repo owner/name]
```

</details>

<details>
<summary>Issue commands</summary>

```
va issue create            --title ... --context ... --problem ... --acceptance ...
                           [--parent ...] [--label ...] [--assignee ...]
                           [--related-issue ...] [--related-pr ...] [--repo ...] [--dry-run]
va issue read              [--issue <number>] [--repo owner/name]
va issue update            --issue <number> [--repo owner/name] [--title ...|--body ...|...]
va issue close             --issue <number> [--repo owner/name] [--reason completed|not_planned]
va issue reopen            --issue <number> [--repo owner/name]
va issue delete            --issue <number> [--repo owner/name]
va issue done-status       (--on-dev-merge --pr <number> | --on-issue-closed --issue <number>)
va issue reopen-on-dev     --pr <number> [--repo owner/name]
va issue neutralize        --pr <number> [--repo owner/name]
va issue closure-hygiene   [--repo owner/name]
va issue open-numbers      [--repo owner/name]
va issue open-snapshots    [--repo owner/name] [--limit <n>]
va issue repo-name
va issue current-login
```

</details>

<details>
<summary>Git commands</summary>

```
va git create-branch       [name] [--remote origin] [--base dev]
va git create-work-branch  <type> <description> [--remote origin] [--base dev]
va git push-branch         [--remote origin]
va git add-commit-push     <message> [--no-verify] [--remote origin]
va git delete-branch       <name> [--force] [--remote origin] [--base dev]
va git finish-branch       [name] [--remote origin] [--base dev]
va git create-after-delete [--remote origin] [--base dev]
va git clean-local-gone    [--remote origin]
va git clean-branches      [--dry-run] [--remote origin] [--base dev]
va git cleanup-after-pr    [--delete-only] [--remote origin] [--base dev]
```

</details>

<details>
<summary>Automation commands</summary>

```
va automation install-hooks
va automation changed-crates        [ref1] [ref2] [--output-format paths]
va automation release-prepare       <version> [--auto-changelog]
va automation check-merge-conflicts [--remote origin] [--base dev]
va automation check-dependencies    [--skip-outdated] [--skip-unused]
va automation clean-artifacts       [--skip-node-modules]
va automation audit-security
va automation audit-issue-status    [--repo owner/name] [--base ref] [--head ref]
va automation check-priority-issues [--repo owner/name]
va automation labels-sync           [--labels-file path] [--prune]
va automation ci-watch-pr           [--pr <number>] [--poll-interval <s>] [--max-wait <s>]
va automation sync-main-dev-ci      [--remote origin] [--main main] [--dev dev]
va automation test-coverage
```

</details>
