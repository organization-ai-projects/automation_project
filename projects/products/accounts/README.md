# Accounts Product

Accounts is the product that handles first-admin bootstrap and normal login.

Modes:

- **Setup**: visible only when no admin exists. Uses `owner.claim` to create the first admin.
- **Normal**: login and account management.

Notes:

- UI bundle and backend are separate crates.
- The UI communicates with `engine` through `central_ui` (proxy).
- Backend registers with Engine via WebSocket (`backend.hello`).
- Storage (JSON) lives in `projects/products/accounts/data/` by default (override with `ACCOUNTS_DATA_DIR`).
- `accounts-backend` is a product binary (not a library crate).

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

Admin endpoints (via Engine, requires `Authorization: Bearer <jwt>`).
`{user_id}` is a 32-char hex ProtocolId:

- `GET /accounts/users`
- `GET /accounts/users/{user_id}`
- `POST /accounts/users`
- `PATCH /accounts/users/{user_id}`
- `POST /accounts/users/{user_id}/status`
- `POST /accounts/users/{user_id}/reset_password`
