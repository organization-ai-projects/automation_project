# Accounts Product Documentation

This directory contains the Accounts product that handles first-admin bootstrap and normal login functionality.

## Role in the Project

This product is responsible for user authentication and account management in the automation project. It provides setup functionality for creating the first admin user and normal login/account management features.

It interacts mainly with:

- Engine - For command handling and authorization
- Central UI - As a proxy for UI communication
- Identity library - For user storage
- Security library - For authentication tokens

## Directory Structure

```
accounts/
├── README.md           # This file
├── metadata.ron        # Product metadata
├── backend/           # Backend service (WebSocket)
├── ui/                # Dioxus WASM UI bundle
└── data/              # Storage directory (JSON)
```

## Description

Accounts is the product that handles first-admin bootstrap and normal login.

Modes:

- **Setup**: visible only when no admin exists. Uses `owner.claim` to create the first admin.
- **Normal**: login and account management.

## Notes

- UI bundle and backend are separate crates.
- The UI communicates with `engine` through `central_ui` (proxy).
- Backend registers with Engine via WebSocket (`backend.hello`).
- Storage (JSON) lives in `projects/products/accounts/data/` by default (override with `ACCOUNTS_DATA_DIR`).
- `accounts-backend` is a product binary (not a library crate).
- Migration: existing stored `user_id` values must be 32-char hex ProtocolId strings.

## Backend Actions

Backend (WS actions handled by accounts-backend):

- `accounts.setup_status`
- `accounts.setup_admin`
- `accounts.login`
- `accounts.list`
- `accounts.get`
- `accounts.create`
- `accounts.update`
- `accounts.update_status`
- `accounts.reset_password`

## Admin Endpoints

Admin endpoints (via Engine, requires `Authorization: Bearer <jwt>`).
`{user_id}` is a 32-char hex ProtocolId:

- `GET /accounts/users`
- `GET /accounts/users/{user_id}`
- `POST /accounts/users`
- `PATCH /accounts/users/{user_id}`
- `POST /accounts/users/{user_id}/status`
- `POST /accounts/users/{user_id}/reset_password`
