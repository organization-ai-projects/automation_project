# Issue Decomposition Conventions

This document defines stable, automation-friendly rules for splitting work into
issues, bundles (parent issues), sub-issues (UI), and pull requests.

The goal is to maximize determinism and automation, while keeping human judgment
only where it is strictly required.

---

## Core Principle

> **Consistency over tooling.**
> These conventions define how the team reasons about work decomposition.

The purpose of this document is to establish **shared conventions**, not to
specify automation or tooling details.

How these conventions are automated (or not) is intentionally left out of scope
and may evolve independently.

---

## Definitions

### Bundle (Parent Issue)

A **bundle** (parent issue) represents a **coherent set of tasks that must be
completed together to be considered correct**.

A bundle is **not**:

* a long-lived workstream
* a thematic grouping
* a release or delivery plan

Those concerns belong to **milestones**.

---

### Child Issue

A **child issue** is a concrete, actionable task that belongs to a bundle.

Child issues are:

* individually implementable
* individually reviewable
* **not acceptable to deliver alone** when part of a bundle

---

## When to Use a Bundle (Parent Issue)

Create a bundle **only when all conditions are true**:

* Tasks are **functionally dependent**
* Delivering any single task alone would leave the system:

  * incomplete
  * misleading
  * inconsistent (code, docs, tests, or behavior)
* Tasks are expected to be handled in the **same working context**

Typical examples:

* Script change + documentation update
* Breaking change + all required consumer updates
* Refactor + required CI or test adaptations
* Multiple changes required to keep a single file or crate coherent

If tasks can be delivered independently without breaking coherence,
**do not create a bundle**.

---

## When NOT to Use a Bundle

Do **not** create a bundle when:

* Tasks merely live in the same directory or crate
* Tasks are similar but independent
* Each task can be merged safely on its own
* Grouping would be purely organizational or visual

In those cases, use:

* standalone issues
* shared labels
* milestones (when time-based grouping is needed)

---

## Examples Matrix

| Situation | Recommended Structure | Why |
|---|---|---|
| Script behavior changes and related docs/tests must stay aligned | Bundle (parent) + required child issues | Partial delivery would leave behavior and guidance inconsistent |
| Two review comments on the same file | One review follow-up issue | Review convention is one issue per file |
| Review comments across different files | One issue per file | Preserves strict file-level ownership and closure clarity |
| Independent low-risk docs edits in different areas | Standalone issues (can share one PR) | No functional dependency between tasks |
| Multiple tasks targeted for the same release window only | Milestone only (no bundle) | Time grouping is a milestone concern, not dependency |
| Breaking API change + mandatory consumer updates | Bundle (parent) + blocking children | Change is not correct until all consumers are adapted |

### Anti-Examples

* Do not create a bundle only because issues share the same directory.
* Do not use a bundle as a backlog container for unrelated future work.
* Do not replace milestone planning with parent/child links when tasks are independent.

---

## Review Follow-up Convention (Strict Scope)

This section applies **only** to issues created from pull request review comments.

### Rules

* One review issue per file when comments target different files
* All review comments for the same file are grouped into one issue
* Review follow-up issues **must declare the affected file explicitly**

Required field (review issues only):

```
File : <path/to/file>
```

This field:

* is mandatory **only** for review follow-up issues
* is used exclusively for consistent team-level grouping
* must not be used for non-review issues

---

## Sub-Issues (GitHub UI)

GitHub sub-issues may be used for **visual organization and clarity**.

Their usage is governed by these conventions, not by tooling guarantees.

How sub-issues are created (manually or otherwise) is out of scope for this
document.

---

## Convention Scope

This document defines **what** conventions are followed, not **how** they are
implemented or enforced.

Automation, scripts, or bots may rely on these conventions, but are not part of
this specification.

The conventions are designed to be clear and deterministic for humans first.

---

## Branch and Pull Request Strategy

### Bundle Flow

* Default: one branch and one PR per child issue
* Allowed (solo / atomic delivery):

  * one branch
  * multiple commits
  * one commit per child issue
  * one final PR

### Standalone Issue Flow

* Multiple standalone, low-risk issues may be resolved in a single PR
* Only when changes are homogeneous and reviewable together

---

## PR Risk Guardrail

* A PR must not mix:

  * high-risk changes (breaking, API, infra, security)
  * with low-risk-only changes (formatting, docs)

* If mixing is unavoidable:

  * compatibility impact must be explicit in the PR body
  * high-risk content must be reviewed first

---

## Parent (Bundle) Closure Policy

* A bundle may be closed **only** when all required child issues are closed
* Bundles must explicitly distinguish:

  * **Required**: blocking child issues
  * **Optional**: non-blocking follow-ups

If closed manually, the closure comment must be traceable, for example:

```
Closed as completed. Delivered in PR #<number>.
```

---

## Milestones vs Bundles

* **Bundles** answer: *"what must be done together to be correct?"*
* **Milestones** answer: *"by when should this be delivered?"*

They serve different purposes and must not be treated as duplicates.
