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

### 1.4 AI Orchestrator (`ai`)

- **Coordination**: Supervision of `symbolic` and `neural` components.
- **Intelligent Decision-making**: Determines when to delegate to neural.
- **Strict Isolation**: No global state stored.
- **Contextual Work**: Operates exclusively via a `ProjectContext`.
