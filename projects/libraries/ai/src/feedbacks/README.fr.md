# Feedback semantics

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
