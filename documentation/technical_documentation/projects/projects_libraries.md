# Libraries and Symbolic Components

- [Back to Projects Index](../TOC.md)

## 1. Detailed Role of Components

### 1.1 Common (`common`)

- **Fundamental Types**: IDs, enums, states.
- **Common Errors**: Shared error management.
- **Generic Utilities**: Reusable functions and tools.
- **No Runtime Dependencies**: No dependencies like tokio, dioxus, etc.
- `common` must not contain any business logic, orchestration, or I/O access.

> Communication contracts are defined in `protocol`.

---

### 1.2 Symbolic Component (`symbolic`)

- **Linting Rules**: Application of best practices.
- **Static Analysis**: Verification of structure, conventions, and patterns.
- **Rule and Decision Engine**: Management of symbolic workflows.
- **Symbolic Domain Logic**: Domain-level symbolic analysis and validation capabilities.

> `symbolic` is a domain library. Product-level AI orchestration is handled by `ai`.

---

### 1.3 Neural Component (`neural`)

- **Intent Understanding**: Conversion of natural language into structure.
- **Rust Code Generation**: Automatic code creation.
- **Feedback-based Adjustment**: Continuous improvement based on feedback.
- **Training and Inference**: Use of **Burn** for neural models.
- The `neural` component is not intended as a direct product dependency. It is consumed through the `ai` orchestrator.

> Activation via feature flag only.

---

### 1.4 Library Catalog (workspace overview)

Current libraries under `projects/libraries`:

- `core/foundation/*`: Internal technical libraries and shared primitives.
- `core/contracts/*`: Cross-cutting contracts and protocol crates.
- `layers/domain/identity`: Identity types and store helpers.
- `layers/domain/security`: Auth, tokens, claims, and verification helpers.
- `layers/domain/symbolic`: Symbolic analysis/validation domain.
- `layers/domain/neural`: Neural domain (training/inference/generation).
- `layers/domain/ui`: Shared UI components for product UIs.
- `layers/domain/versioning`: Versioning and revision helpers.
- `layers/orchestration/ai`: Orchestrator for symbolic + neural workflows.

---

### 1.5 Example: Using the `common` Library

Below is an example of how to use the `common` library to define and handle errors:

#### Code Example

```rust
use common::errors::{AppError, Result};

fn perform_action() -> Result<()> {
    // Example logic
    if some_condition {
        Err(AppError::new("An error occurred"))
    } else {
        Ok(())
    }
}

fn main() {
    match perform_action() {
        Ok(_) => println!("Action performed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

#### Explanation

- `AppError`: A shared error type defined in the `common` library.
- `Result`: A type alias for `Result<T, AppError>` to simplify error handling.

This demonstrates how the `common` library provides reusable utilities for consistent error management.

---

### 1.6 AI Orchestrator (`ai`)

- **Coordination**: Supervision of `symbolic` and `neural` components.
- **Intelligent Decision-making**: Determines when to delegate to neural.
- **Strict Isolation**: No global state stored.
- **Contextual Work**: Operates exclusively via a `ProjectContext`.
