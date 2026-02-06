# Security Library Documentation

This directory contains role-based access control (RBAC) and token authentication for the automation project.

## Role in the Project

This library is responsible for security primitives across the automation project. It provides roles, permissions, token-based authentication, and password hashing. User management and storage are intentionally out of scope (handled by the identity crate).

It interacts mainly with:

- Identity library - For user management
- Protocol library - For ProtocolId types
- Engine product - For authorization enforcement
- All products - For authentication and authorization

## Directory Structure

```
security/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
└── src/               # Source code
    ├── lib.rs
    ├── role.rs
    ├── permission.rs
    ├── token.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Overview

This library provides user-agnostic security primitives: roles, permissions, token-based authentication, and password hashing. It is used by Engine to enforce authorization on all commands.

User management and storage are intentionally out of scope; use the `identity` crate for users and stores.

## Features

- **Role-Based Access Control** - Four hierarchical roles with predefined permissions
- **Permission System** - Eight granular permissions for fine-grained control
- **Token Authentication** - JWT-like token generation and validation
- **Permission Utilities** - Helper functions for checking and filtering permissions
- **Password Hashing** - Argon2 hashing and verification utilities

## Roles

| Role       | Level | Permissions                                            |
| ---------- | ----- | ------------------------------------------------------ |
| Admin      | 4     | All permissions                                        |
| Moderator  | 3     | Read, Write, Execute, Train, ViewLogs                  |
| User       | 2     | Read, Write, Execute                                   |
| Guest      | 1     | Read only                                              |

## Permissions

| Permission        | Description                              |
| ----------------- | ---------------------------------------- |
| `Read`            | Read code, view projects                 |
| `Write`           | Write/modify code                        |
| `Execute`         | Execute code generation, analysis        |
| `Delete`          | Delete projects/files                    |
| `Admin`           | Manage users, permissions, settings      |
| `Train`           | Train/adjust models                      |
| `ViewLogs`        | Access logs and metrics                  |
| `ConfigureSystem` | Modify system configuration              |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
security = { path = "../security" }
```

## Usage

### Check role permissions

```rust
use security::{Role, Permission, has_permission, check_permission};

let role = Role::User;

// Boolean check
if has_permission(&role, Permission::Write) {
    println!("User can write");
}

// Result-based check (for error handling)
match check_permission(&role, Permission::Delete) {
    Ok(()) => println!("Access granted"),
    Err(e) => println!("Access denied: {:?}", e),
}
```

### Token-based authentication

```rust
use security::{Token, TokenService, Role, check_token_permission, Permission};
use protocol::ProtocolId;
use std::str::FromStr;

// Create a token service
let service = TokenService::new_hs256("your-secret-key")?;

// Generate a token for a subject
let subject_id = ProtocolId::from_str("00000000000000000000000000000001")?;
let token = service.issue(subject_id, Role::User, 3600, None)?;

// Note: Token.value represents the JWT ID (`jti`) as a ProtocolId.

// Validate and check permissions
check_token_permission(&token, Permission::Write)?;
```

### Role comparison

```rust
use security::Role;

let admin = Role::Admin;
let user = Role::User;

if admin.has_higher_privilege_than(&user) {
    println!("Admin outranks User");
}
```

### Filter allowed permissions

```rust
use security::{Role, Permission, filter_allowed_permissions, missing_permissions};

let required = vec![Permission::Read, Permission::Write, Permission::Delete];

// Get what the user CAN do
let allowed = filter_allowed_permissions(&Role::User, &required);
// Returns: [Read, Write]

// Get what the user CANNOT do
let missing = missing_permissions(&Role::User, &required);
// Returns: [Delete]
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/security/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
