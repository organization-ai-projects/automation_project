# Issue Template: Review Follow-up

This document describes the review follow-up template located at `.github/ISSUE_TEMPLATE/review_followup.md`.

This document is the canonical source for review follow-up issue conventions.

## Purpose

- Convert review comments into trackable issues.
- Group comments by file when applicable.
- Keep follow-up work linked to review context.

## Source File

- `.github/ISSUE_TEMPLATE/review_followup.md`

## Expected Content

- One issue per target file (when grouping rule applies)
- Copied review comment content
- PR reference (`PR : #<number>`)
- Review comment traceability (`Review Comment ID : <comment_id>`)
- Original review context reference
- File path marker kept at the end of the issue body

## Convention Notes

- Group multiple comments in one issue only if they target the same file.
- Split into separate issues when comments target different files.
- Use concise, action-oriented review issue titles.
- Apply labels consistently: `review` + type + scope labels.
- Keep issue content minimal and review-focused.

## Example

```text
Using only as_nanos() for directory naming is not guaranteed to be unique ...

PR : #268
Review Comment ID : discussion_r2777980411

Type : fix

Done when :
- [Temp directory naming includes collision-safe entropy in concurrent tests.]
- [Parallel test runs no longer share cleanup targets.]

_Originally posted by @Copilot in https://github.com/organization-ai-projects/automation_project/pull/268#discussion_r2777980411_

File : projects/products/stable/core/watcher/tests/config_tests.rs
```

## Navigation

- [Back to Templates README](README.md)
- [Back to Templates TOC](TOC.md)
