# versioning_automation

A Rust-based automation tool for Git workflows, PR lifecycle management, and issue tracking. Designed as an internal monorepo tool, built to be installable and usable as a standalone binary.

---

## Installation

### Development (recompiles on change)

```bash
cargo run -p versioning_automation -- <subcommand> [args...]
```

### Stable usage (installs binary to `~/.cargo/bin/`)

```bash
cargo install --path tools/versioning_automation
va <subcommand> [args...]
```

Re-run `cargo install` after any changes you want reflected in the installed binary.

---

## Prerequisites

- Rust toolchain (`cargo`)
- [`gh`](https://cli.github.com/) CLI authenticated with your GitHub account
- `pnpm` (for markdown lint commands)
- `bash` (for shell syntax checks)

---

## Usage

```
va <subcommand> [args...]

Subcommands:
  automation   Run versioning automation helper flows
  git          Run git/versioning automation flow
  pr           Run PR automation engine flow
  issue        Run issue automation engine flow
  help         Show this help
```

---

## Git Hooks

`versioning_automation` powers all Git hooks in this repository. Install them with:

```bash
va automation install-hooks
```

Hooks installed:

| Hook                 | Behaviour                                                                  |
| -------------------- | -------------------------------------------------------------------------- |
| `pre-commit`         | Branch guard, assignment policy, markdown lint, shell syntax, `cargo fmt`  |
| `commit-msg`         | Conventional commit format, scope/file match, footer validation            |
| `prepare-commit-msg` | Auto-generates a commit message template from branch name and staged files |
| `pre-push`           | Root parent ref guard, assignment policy, merge conflict check             |
| `post-checkout`      | Convention warning when branch history references root parent issues       |
| `pre-branch-create`  | Branch name format validation                                              |

### Bypass variables

| Variable                               | Effect                                  |
| -------------------------------------- | --------------------------------------- |
| `SKIP_PRE_COMMIT=1`                    | Skip all pre-commit checks              |
| `SKIP_COMMIT_VALIDATION=1`             | Skip commit message validation          |
| `ALLOW_PROTECTED_BRANCH_COMMIT=1`      | Allow direct commit on `dev`/`main`     |
| `ALLOW_PART_OF_ONLY_PUSH=1`            | Bypass assignment policy check          |
| `SKIP_POST_CHECKOUT_CONVENTION_WARN=1` | Suppress post-checkout warnings         |
| `SKIP_PREPARE_COMMIT_MSG=1`            | Skip commit message template generation |

---

## Commit Message Format

```
<type>(<scope>): <description>
```

**Types:** `feat`, `feature`, `fix`, `doc`, `docs`, `refactor`, `test`, `tests`, `chore`, `perf`

**Scope:** must match the path of the touched crate or directory, resolved from staged files.

```bash
# Example — touching files in neurosymbolic_moe/backend:
git commit -m "refactor(projects/products/unstable/neurosymbolic_moe/backend): update tests"
```

---

## PR Commands

### Generate a PR description

```bash
va pr generate-description --auto --base dev --head $(git branch --show-current) --yes
```

| Option   | Description                                   |
| -------- | --------------------------------------------- |
| `--auto` | Non-interactive mode, creates the PR directly |
| `--base` | Base branch (e.g. `dev`)                      |
| `--head` | Head branch (defaults to current branch)      |
| `--yes`  | Auto-confirm prompts                          |

### Other PR commands

```bash
va pr breaking-detect [--text "..."|--stdin|--input-file path] [--labels-raw "a||b"]
va pr body-context --pr <number> [--repo owner/name]
va pr child-pr-refs --pr <number> [--repo owner/name]
va pr details --pr <number> [--repo owner/name]
va pr field --pr <number> --name <state|base-ref-name|head-ref-name|title|body|author-login|commit-messages> [--repo owner/name]
va pr directives --text "..." [--format plain|json] [--unique]
va pr directives-apply (--text "..." | --stdin)
va pr closure-refs (--text "..." | --stdin)
va pr non-closing-refs (--text "..." | --stdin)
va pr closure-marker (--text "..." | --stdin) --keyword-pattern <regex> --issue <#n> --mode <apply|remove>
va pr directives-state (--text "..." | --stdin)
va pr directive-conflicts (--text "..." | --stdin) [--source-branch-count <n>]
va pr directive-conflict-guard --pr <number> [--repo owner/name]
va pr duplicate-actions (--text "..." | --stdin) --mode <safe|auto-close> --repo owner/name [--assume-yes true|false]
va pr group-by-category (--text "..." | --stdin) --mode <resolved|reopen|conflict|directive>
va pr effective-category --labels-raw "label1||label2" (--title "..." | --title-category <name>) --default-category <name>
va pr issue-category-from-labels --labels-raw "label1||label2"
va pr issue-category-from-title --title "..."
va pr issue-close-policy --action <Closes|Reopen|Cancel-Closes> [--is-pr-ref true|false] [--non-compliance-reason "..."]
va pr issue-context --issue <number> [--repo owner/name]
va pr issue-view --issue <number> [--repo owner/name]
va pr pr-state --pr <number> [--repo owner/name]
va pr refresh-validation --pr <number> [--repo owner/name]
va pr text-payload --pr <number> [--repo owner/name]
va pr open-referencing-issue --issue <number> [--repo owner/name]
va pr issue-ref-kind --issue <number> [--repo owner/name]
va pr normalize-issue-key --raw "text containing #123"
va pr sort-bullets --input-file /path/to/bullets.txt
va pr issue-decision --action <Closes|Reopen|Cancel-Closes> --issue <#n> --default-category <name> [options]
va pr resolve-category --label-category <name> --title-category <name> --default-category <name>
va pr auto-add-closes --pr <number> [--repo owner/name]
va pr update-body --pr <number> [--repo owner/name] --body "..."
va pr upsert-comment --pr <number> [--repo owner/name] --marker "..." --body "..."
```

---

## Git Commands

```bash
va git create-branch [name] [--remote origin] [--base dev]
va git create-work-branch <type> <description> [--remote origin] [--base dev]
va git push-branch [--remote origin]
va git add-commit-push <message> [--no-verify] [--remote origin]
va git delete-branch <name> [--force] [--remote origin] [--base dev]
va git finish-branch [name] [--remote origin] [--base dev]
va git create-after-delete [--remote origin] [--base dev]
va git clean-local-gone [--remote origin]
va git clean-branches [--dry-run] [--remote origin] [--base dev]
va git cleanup-after-pr [--delete-only] [--remote origin] [--base dev]
va git branch-creation-check [git-command [args...]]
```

---

## Issue Commands

```bash
va issue create --title ... --context ... --problem ... --acceptance ... [--parent ...] [--label ...] [--assignee ...] [--related-issue ...] [--related-pr ...] [--repo ...] [--dry-run]
va issue read [--issue <number>] [--repo owner/name] [--json fields] [--jq filter] [--template tpl]
va issue update --issue <number> [--repo owner/name] [--title ...|--body ...|--add-label ...|--remove-label ...|--add-assignee ...|--remove-assignee ...]
va issue close --issue <number> [--repo owner/name] [--reason completed|not_planned] [--comment ...]
va issue reopen --issue <number> [--repo owner/name]
va issue delete --issue <number> [--repo owner/name]
va issue done-status (--on-dev-merge --pr <number> | --on-issue-closed --issue <number>) [--label <name>] [--repo owner/name]
va issue reopen-on-dev --pr <number> [--label <name>] [--repo owner/name]
va issue neutralize --pr <number> [--repo owner/name]
va issue auto-link --issue <number> [--repo owner/name]
va issue reevaluate --issue <number> [--repo owner/name]
va issue parent-guard (--issue <number> | --child <number>) [--strict-guard true|false]
va issue is-root-parent --issue <number> [--repo owner/name]
va issue closure-hygiene [--repo owner/name]
va issue required-fields-validate [--mode title|body|content] [--title ...] [--body ...] [--labels-raw ...]
va issue non-compliance-reason [--title ...] [--body ...] [--labels-raw ...]
va issue fetch-non-compliance-reason --issue <number> [--repo owner/name]
va issue validate-footer --file <path> [--repo owner/name]
va issue sync-project-status --repo owner/name --issue <number> --status <name>
va issue label-exists --repo owner/name --label <name>
va issue has-label --issue <number> --label <name> [--repo owner/name]
va issue list-by-label --label <name> [--repo owner/name]
va issue open-numbers [--repo owner/name]
va issue open-snapshots [--repo owner/name] [--limit <n>]
va issue extract-refs [--profile <hook|audit>] (--text <raw> | --file <path>)
va issue tasklist-refs --body <issue_body>
va issue subissue-refs --owner <owner> --repo <repo> --issue <number>
va issue assignee-logins --issue <number> [--repo owner/name]
va issue state --issue <number> [--repo owner/name]
va issue field --issue <number> --name <title|body|labels-raw> [--repo owner/name]
va issue upsert-marker-comment --repo owner/name --issue <number> --marker <marker> --body <body> [--announce true|false]
va issue repo-name
va issue current-login
```

---

## Automation Commands

```bash
va automation pre-commit-check
va automation pre-push-check
va automation commit-msg-check --file <path>
va automation prepare-commit-msg --file <path> [--source <source>]
va automation post-checkout-check
va automation pre-branch-create-check --branch <name>
va automation install-hooks
va automation changed-crates [ref1] [ref2] [--output-format paths]
va automation release-prepare <version> [--auto-changelog]
va automation check-merge-conflicts [--remote origin] [--base dev]
va automation check-dependencies [--skip-outdated] [--skip-unused]
va automation clean-artifacts [--skip-node-modules]
va automation audit-issue-status [--repo owner/name] [--base ref] [--head ref] [--limit n] [--output file]
va automation audit-security
va automation check-priority-issues [--repo owner/name]
va automation labels-sync [--labels-file path] [--prune]
va automation ci-watch-pr [--pr <number>] [--poll-interval <s>] [--max-wait <s>]
va automation sync-main-dev-ci [--remote origin] [--main main] [--dev dev] [--sync-branch name]
va automation test-coverage
```

---

## Troubleshooting

**`gh` not authenticated**

```bash
gh auth login
```

**Commit rejected — scope mismatch**

The commit scope must match the crate path resolved from your staged files. Example:

```bash
# staged: projects/products/unstable/neurosymbolic_moe/backend/src/foo.rs
# correct scope:
git commit -m "refactor(projects/products/unstable/neurosymbolic_moe/backend): description"
```

**Skip hooks temporarily**

```bash
SKIP_PRE_COMMIT=1 git commit ...
SKIP_COMMIT_VALIDATION=1 git commit ...
```
