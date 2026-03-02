# protocol_builder

A protocol schema compiler and code generator with a backend/ui/tooling split.

## Crates

- **backend**: IPC server that reads requests from stdin and writes responses to stdout (newline-delimited JSON).
- **ui**: CLI frontend (`protocol_builder_ui generate --schema <file> --out <dir>`) that spawns the backend.
- **tooling**: Validation utilities (`validate-emitted`, `validate-transcript`).

## Schema Format

```json
{
  "name": "MyProtocol",
  "version": "1.0.0",
  "messages": [{"name": "Ping", "fields": [{"name": "id", "type_spec": "U64"}]}],
  "endpoints": [{"name": "ping", "request": "Ping", "response": "Pong"}]
}
```
