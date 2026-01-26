# Multi-project Organization

- [Back to Projects Index](TOC.md)

## Introduction

This document details the organization of projects within the `automation_project` workspace.

---

## 1. Managing Internal Project State

Each project is responsible for managing its own internal state. This includes elements such as configuration files, caches, logs, and temporary data.

### Key Principles

- **Independence**: Each project must manage its state in isolation, without relying on other projects.
- **Flexibility**: The internal structure of a project is left to its discretion, so it can adapt to its specific needs.
- **Strict Isolation**: No internal files or data should "leak" to other projects.

### Recommendations

- Use dedicated files or folders to organize internal state (e.g., `state/`, `cache/`, `logs/`).
- Include a `schema_version` field in persisted files (like `project.toml`) to ensure future compatibility.
- Document the chosen internal structure to facilitate maintenance and collaboration.

### Prohibitions

- No project should write to the internal state of another project.
- No global shared state outside of mechanisms explicitly provided by the Engine.
- No direct access to another project via relative or absolute paths.

### Scope of Internal State

The internal state of a project includes all persisted or semi-persisted data necessary for its operation but does not include the source code itself.

> These principles ensure that each project remains autonomous and maintainable while allowing great flexibility in its internal organization.
