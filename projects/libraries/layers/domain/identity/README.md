# Identity Library Documentation

This directory contains user identity primitives and in-memory storage for the automation project.

## Role in the Project

This library is responsible for user identity and storage concerns. It provides user-related types and in-memory storage without mixing into the security crate, while relying on security for password hashing and role definitions.

It interacts mainly with:

- Security library - For password hashing and role definitions
- Common library - For ID types
- Protocol library - For ProtocolId
- Products - For user management

## Directory Structure

```
identity/
├── Cargo.toml          # Package configuration
├── README.md           # This file
└── src/               # Source code
    ├── lib.rs
    ├── user_id.rs
    ├── store.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `src/`: Source code.
- `tests/`: Tests.


## Overview

This crate provides user-related types and storage without mixing into the `security` crate.
It relies on `security` for password hashing and role definitions.

## Features

- **UserId** - Validated user identifier type
- **User Store** - In-memory user storage with password verification
- **Identity Errors** - Clear error types for identity concerns

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
identity = { path = "../identity" }
```

## Usage

```rust
use identity::{UserId, UserStore};
use protocol::ProtocolId;
use security::Role;
use std::str::FromStr;

let store = UserStore::new();
let user_id = UserId::new(ProtocolId::from_str("00000000000000000000000000000001")?)?;

store.add_user(user_id.clone(), "secure_password", Role::User).await?;
let role = store.authenticate(&user_id, "secure_password").await?;

// Conversions from Id128 now use TryFrom to enforce validation.
```

## Scope

- This crate owns user identity and storage concerns.
- Token issuance and authorization checks live in the `security` crate.
