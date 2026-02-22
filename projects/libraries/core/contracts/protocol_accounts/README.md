# Protocol Accounts Contracts

This crate contains account-domain request/response contracts used by products such as `accounts-backend`, `accounts-ui`, `engine`, and `central_ui`.

## Scope

- Account setup/login DTOs
- Account CRUD/update DTOs
- Account status/summary DTOs

## Non-goals

- Transport protocol primitives (`Command`, `Event`, `Payload`)
- Cross-domain protocol metadata and validation types

Those types remain in the `protocol` crate.
