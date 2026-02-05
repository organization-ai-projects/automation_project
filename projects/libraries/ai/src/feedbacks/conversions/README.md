# Conversions Documentation

This directory contains implementations of conversions between different types used in the AI library feedback system.

## Role in the Project

This module is responsible for centralizing all type conversions between different domains (public ↔ internal, public ↔ symbolic, etc.) in the feedback system. It ensures consistent and maintainable conversion logic across the AI library.

It interacts mainly with:

- Public API feedback types
- Internal feedback types
- Symbolic feedback types

## Directory Structure

```
conversions/
├── README.md                      # This file
├── mod.rs                         # Module exports
├── feedback_input/                # FeedbackInput conversions
├── feedback_verdict/              # FeedbackVerdict conversions
├── internal_feedback_meta/        # InternalFeedbackMeta conversions
├── internal_feedback_verdict/     # InternalFeedbackVerdict conversions
└── tests/                         # Conversion tests
```

## Files

- `README.md`: This file.
- `feedback_input/`: FeedbackInput conversions.
- `feedback_verdict/`: FeedbackVerdict conversions.
- `internal_feedback_meta/`: InternalFeedbackMeta conversions.
- `internal_feedback_verdict/`: InternalFeedbackVerdict conversions.
- `mod.rs`: Module exports.
- `tests/`: Tests.


## Conventions

This folder contains the implementations of conversions between different types used in the project. To maintain clear and consistent organization, the following conventions have been adopted:

## File Structure

- Each source type has its own folder in `conversions/`.
  - Example: `feedback_verdict/`, `internal_feedback_verdict/`.
- Inside each folder, files are named according to the target type.
  - Example:
    - `feedback_verdict/internal_feedback_verdict.rs`: Contains conversions from `FeedbackVerdict` to `InternalFeedbackVerdict`.
    - `feedback_verdict/symbolic_feedback.rs`: Contains conversions from `FeedbackVerdict` to `SymbolicFeedback`.

## General Rules

1. **One file per conversion**: Each file must contain the conversion implementations for a specific target type.
2. **Documentation**: Each implementation must be accompanied by comments explaining its role and specifics.
3. **Tests**: Unit tests for conversions must be placed in the corresponding files or in a `tests/` folder if necessary.
4. **Centralization**: Any conversion between different domain types (public ↔ internal, public ↔ symbolic, etc.) must be implemented exclusively in this folder. `From` / `Into` implementations outside of `conversions/` are prohibited.

## Example

### Conversion from `FeedbackVerdict` to `InternalFeedbackVerdict`

File: `feedback_verdict/internal_feedback_verdict.rs`

```rust
impl<'a> From<FeedbackVerdict<'a>> for InternalFeedbackVerdict {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => InternalFeedbackVerdict::Correct,
            FeedbackVerdict::Rejected => InternalFeedbackVerdict::Rejected,
            FeedbackVerdict::NoFeedback => InternalFeedbackVerdict::NoFeedback,
            FeedbackVerdict::Incorrect { expected_output } => InternalFeedbackVerdict::Incorrect {
                expected_output: expected_output.into_owned(),
            },
            FeedbackVerdict::Partial { correction } => InternalFeedbackVerdict::Partial {
                correction: correction.into_owned(),
            },
        }
    }
}
```

This structure allows for quick localization and easy maintenance of conversions.

---

By following these conventions, we ensure better readability and increased scalability of the code.
