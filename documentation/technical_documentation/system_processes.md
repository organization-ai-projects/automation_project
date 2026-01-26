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

1. A UI or backend sends a Command to the Engine.
2. The Engine validates auth/permissions.
3. The Engine routes the command to the target backend (if any).
4. The backend emits Events (logs, progress, results).
5. The Engine forwards Events to connected clients.

### 1.4 First Launch Checklist

- Ensure the registry is available (`.automation_project/registry.json`).
- Authentication bootstrap is not secured yet.
  - Current login accepts any `user_id` with a non-empty password and defaults to `Role::User` if no role is provided.
  - A proper admin bootstrap flow and password validation are planned but not implemented (see issue #53).
