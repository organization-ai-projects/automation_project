# System Processes

- [Back to Technical TOC](TOC.md)

## Introduction

This document explains how the core system processes are launched, supervised, and coordinated in the `automation_project`.

---

## 1. Core Process Flow

### 1.1 Objectives

The Engine is the single hub for commands/events and coordinates all execution. Core services are started by the Launcher and supervised by the Watcher.

### 1.2 Startup and Supervision (Launcher)

For workspace users and operators, the Launcher is the entry point:

1. Start the system with the Launcher (`cargo run -p launcher` from the repo root).
2. The Launcher boots core services: `engine`, `central_ui`, and `watcher`.
3. The Watcher supervises core services and restarts them if they crash.
4. The Engine becomes the single hub for commands and events.

### 1.3 Command/Event Flow

1. A UI bundle sends a Command to `central_ui`, which forwards it to the Engine.
2. The Engine validates auth/permissions.
3. The Engine routes the command to the target backend.
4. The backend emits Events (logs, progress, results).
5. The Engine forwards Events to `central_ui`, which displays them.

### 1.4 First Launch Checklist

- Ensure the registry is available (`.automation_project/registry.json`).
- Appliance-style admin bootstrap (one-time, no terminal):
  - Engine generates `~/.automation_project/owner.claim` on first start (permissions 0600).
  - On non-Unix platforms, strict 0600 permissions may not be enforceable; treat the claim file as sensitive and restrict access via OS-specific ACLs when possible.
  - Engine stays in setup mode until the claim is consumed.
  - Central UI reads `owner.claim` locally and calls `POST /setup/owner/admin` with:
    - `claim` (file secret)
    - `user_id` (32 hex chars)
    - `password`
  - Engine verifies the claim, creates the first admin, then consumes the claim and writes `owner.claim.used`.
  - Setup mode is permanently disabled after first admin creation.
  - Claims expire after 24 hours; expired claims are regenerated on next start.
- Login validates credentials against the identity store and rejects invalid credentials.
- Role escalation via the login request is ignored (role is derived from the identity store).
