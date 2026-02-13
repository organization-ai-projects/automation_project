# Templates Overview

This document explains how GitHub templates and script-generated PR bodies are articulated in this repository.

## Scope

- Pull request template definitions in `.github/PULL_REQUEST_TEMPLATE/`
- Issue template definitions in `.github/ISSUE_TEMPLATE/`
- PR body generation behavior in `scripts/versioning/file_versioning/github/generate_pr_description.sh`

## Articulation

- Templates define the expected structure and writing contract for humans.
- The generator produces a pre-filled PR body aligned with that structure.
- Structural and semantic alignment is required across both supports.

## Placeholder Convention

- Template placeholders are human-editable markers (for example `<head-branch>`).
- Generated output uses concrete values with backticks (for example `` `feature/x` ``).
- This difference is intentional and does not indicate a mismatch.

## Navigation

- [Back to Templates README](README.md)
- [Back to Templates TOC](TOC.md)
