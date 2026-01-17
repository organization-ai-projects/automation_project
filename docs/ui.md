# UI

## 1. Rules

- Every product UI is a **WASM bundle** loaded by `central_ui`.
- A UI never depends on a product backend.
- All actions go through `engine` via `protocol`.

## 2. Minimal UI Contract

- WS connection to `engine`
- User session authentication
- Send Command / receive Events
