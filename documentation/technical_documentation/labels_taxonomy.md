# Labels Taxonomy (Issues and Pull Requests)

This document defines how labels are used across Issues and Pull Requests.

## Source of Truth

- Label definitions live in `.github/labels.json`.
- Synchronization script:
  - `bash scripts/versioning/file_versioning/orchestrators/execute/labels_sync.sh`

## Naming Rules

- Lowercase only.
- Use `kebab-case` or existing scoped paths (`projects/...`, `tools/...`).
- One label must have one clear meaning.
- No synonyms with identical scope and intent.

## Label Families

- Type: `bug`, `fix`, `enhancement`, `feature`, `refactor`, `chore`, `documentation`, `testing`, `security`.
- Workflow: `automation`, `automation-failed`, `review`, `sync_branch`.
- Scope: `projects/...`, `tools/...`, `workspace`, `scripts`.
- Triage: `high priority`, `question`, `duplicate`, `invalid`, `wontfix`, `help wanted`, `good first issue`.
- Artifact target: `issue`, `pull-request`.

## Usage Guidelines

- Issues:
  - Use one type label and relevant scope/workflow labels.
  - Use triage labels only when needed.
  - Optional: use `issue` when explicitly tracking issue-process work.
- Pull Requests:
  - Mirror the dominant type and scope labels from resolved work.
  - Use `pull-request` for PR process/template/automation changes.
  - Avoid label inflation; keep only labels that improve routing/review.

## Clarification Rules (Non-Ambiguous Intent)

- `bug`: Problem report or defect tracking issue.
- `fix`: Code/config changes that implement a correction.
- `enhancement`: Incremental improvement.
- `feature`: New capability.

This separation avoids conflating issue classification with implementation intent.
