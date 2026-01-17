# Non-Negotiable Principles

## Introduction

This document lists the fundamental principles that guide the development of the `automation_project`. For an overview, see [Overview](overview.md).

---

## 1. Non-Negotiable Principles

These principles apply from the earliest versions and take precedence over any considerations of speed or convenience.

- Multi-project **from design**
- Strict state isolation
- Symbolic priority
- Neural optional
- Clear and stable APIs
- Long-term architecture planning
- No circular dependencies between crates:
  - `engine` never depends on `ui`
  - `ai` never depends on `engine`
  - `symbolic` and `neural` do not know the workspace

Any project evolution must preserve these principles or explicitly justify their revision.
