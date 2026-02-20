# Feedback Semantics Documentation

This directory contains feedback data structures and semantics used for training and evaluation in the AI library.

## Role in the Project

This module is responsible for defining feedback data structures and their semantic meanings. It ensures consistent handling of feedback between public and internal APIs for training and evaluation.

It interacts mainly with:

- AI body - For feedback processing
- Neural library - For training adjustments
- Symbolic library - For rule adjustments

## Directory Structure

```
feedbacks/
├── README.md              # This file
├── mod.rs                 # Module exports
├── ai_feedback.rs         # Main feedback processing
├── conversions/           # Type conversions
├── internal/              # Internal feedback types
└── public_api_feedback/   # Public API types
```

## Files

- `README.md`: This file.
- `ai_feedback.rs`: Additional module or asset.
- `conversions/`: Type conversion modules.
- `internal/`: Internal types and helpers.
- `mod.rs`: Module exports.
- `public_api_feedback/`: Public API feedback types.


## Feedback Semantics

This module defines feedback data used for training and evaluation. The meanings
below are intended to keep public and internal handling consistent.

## Verdicts

- `Correct`: The output is correct.
- `Incorrect`: The output is wrong; includes the expected answer.
- `Partial`: The output is partially correct; includes a suggested correction.
- `Rejected`: The evaluator explicitly refuses to judge (policy, safety, or other
  refusal). No learning should be applied.
- `NoFeedback`: The evaluator provides no judgment (abstains or skips). No
  learning should be applied.

## Notes

- `Rejected` and `NoFeedback` are distinct to preserve intent, even though both
  result in no training adjustments.
