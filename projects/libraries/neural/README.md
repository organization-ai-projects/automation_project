# Neural Library Documentation

This directory contains neural network-based code generation and inference capabilities for the automation project.

## Role in the Project

This library is responsible for providing machine learning capabilities for code generation across the automation project. It supports training, inference, feedback collection, and model adjustment, integrating with the symbolic library for hybrid AI approaches.

It interacts mainly with:

- AI library - As the neural solver component
- Symbolic library - For hybrid AI approaches
- Common tokenize library - For text tokenization
- Training systems - For model updates

## Directory Structure

```
neural/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
└── src/               # Source code
    ├── lib.rs
    ├── generation.rs
    ├── inference.rs
    ├── training.rs
    ├── feedback.rs
    ├── network.rs
    └── ...
```

## Overview

This library provides machine learning capabilities for code generation, with support for training, inference, feedback collection, and model adjustment. It integrates with the symbolic library for hybrid AI approaches.

## Features

- **Code Generation** - Generate Rust code from natural language prompts
- **Model Training** - Train models on code examples
- **Inference** - Run predictions with confidence scoring
- **Feedback System** - Collect user feedback to improve generation
- **Tokenization** - Rust-specific tokenizer for code understanding

## Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                    NeuralSolver                         │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
│  │ CodeGenerator│  │  Tokenizer  │  │FeedbackAdjuster│ │
│  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘ │
│         │                │                   │          │
│         ▼                ▼                   ▼          │
│  ┌─────────────────────────────────────────────────────┐│
│  │                  NeuralNetwork                      ││
│  └─────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
neural = { path = "../neural" }
```

## Usage

### Load and use the solver

```rust
use neural::{NeuralSolver, NeuralError};
use std::path::Path;

// Load a trained model
let mut solver = NeuralSolver::load(
    Path::new("models/code_gen.bin"),
    Path::new("models/tokenizer.json"),
)?;

// Generate code from a prompt
let result = solver.solve("Create a function that sorts a vector")?;

println!("Generated: {}", result.output);
println!("Confidence: {:.2}", result.confidence);
```

### Train a model

```rust
use neural::NeuralSolver;

let training_data = vec![
    "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
    "fn multiply(a: i32, b: i32) -> i32 { a * b }".to_string(),
];

solver.train(training_data, Path::new("models/trained.bin"))?;
```

### Collect feedback

```rust
use neural::feedback::{UserFeedback, FeedbackType};

// Record positive feedback
solver.record_feedback_if_new(
    "hash123",
    "Create add function",
    "fn add(a: i32, b: i32) -> i32 { a + b }",
    FeedbackType::Positive,
)?;

// Apply feedback to improve the model
if solver.pending_since_last_adjust() >= solver.min_feedback_for_adjustment() {
    solver.adjust_from_feedback(Path::new("models/adjusted.bin"))?;
}
```

### Evaluate model performance

```rust
let test_data = vec!["fn test() {}".to_string()];
let accuracy = solver.evaluate_model(test_data)?;
println!("Model accuracy: {:.2}%", accuracy * 100.0);
```

## Modules

| Module         | Description                               |
| -------------- | ----------------------------------------- |
| `generation`   | Code generation with sampling strategies  |
| `inference`    | Model inference and prediction            |
| `training`     | Model training utilities                  |
| `feedback`     | User feedback collection and processing   |
| `network`      | Neural network implementation             |
| `tokenization` | Rust-specific tokenizer                   |

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/neural/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
