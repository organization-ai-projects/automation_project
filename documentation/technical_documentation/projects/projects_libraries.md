# Libraries and Symbolic Components

- [Back to Projects Index](TOC.md)

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
- **Symbolic Orchestration**: Coordination of specialized submodules.

> `symbolic` is an **aggregator** of specialized symbolic submodules.

---

### 1.3 Neural Component (`neural`)

- **Intent Understanding**: Conversion of natural language into structure.
- **Rust Code Generation**: Automatic code creation.
- **Feedback-based Adjustment**: Continuous improvement based on feedback.
- **Training and Inference**: Use of **Burn** for neural models.
- The `neural` component is never called directly by products. It is invoked only via the `ai` orchestrator.

> Activation via feature flag only.

---

### 1.4 Library Catalog (workspace overview)

Current libraries under `projects/libraries`:

- `ai`: Orchestrator for symbolic + neural flows.
- `ast_core`: AST structures and parsing utilities.
- `command_runner`: Execute commands with structured results.
- `common`: Shared types, errors, and utilities.
- `common_calendar`: Calendar/date utilities.
- `common_json`: JSON model + helpers.
- `common_parsing`: Parsing helpers for shared formats.
- `common_time`: Time utilities.
- `common_tokenize`: Tokenization utilities.
- `hybrid_arena`: Arena-style storage with hybrid indexing.
- `identity`: Identity types and store helpers.
- `neural`: Neural inference/training component.
- `pjson_proc_macros`: Proc-macros for JSON tooling.
- `protocol`: Wire contracts and protocol types.
- `protocol_macros`: Proc-macros for protocol helpers.
- `security`: Auth, tokens, claims, and verification helpers.
- `symbolic`: Symbolic analysis/validation engine.
- `ui`: Shared UI components for product UIs.

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
