# AI Library - Public API Documentation

A Rust library providing a unified interface for neuro-symbolic AI operations, combining neural networks with symbolic reasoning.

## Overview

The `ai` library provides a high-level API through the `AiBody` struct, which orchestrates neural and symbolic solvers to handle various AI tasks such as code generation, analysis, and refactoring.

## Quick Start

```rust
use ai::{AiBody, Task};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AiBody::new()?;
    let task = Task::new_code_generation("write a fibonacci function".to_string());
    let result = ai.solve(&task)?;
    println!("Generated: {}", result.output);
    Ok(())
}
```

## Public API

### Core Types

#### `AiBody`

The main entry point for the library. All operations should go through this interface.

```rust
use ai::ai_body::AiBody;

let mut ai = AiBody::new()?;
```

#### `Task`

Represents a task to be solved by the AI system. Although `AiBody` is the primary entry point, `Task` is part of the public API and can be directly constructed by advanced users for fine-grained control.

```rust
use ai::Task;

// Create different types of tasks
let task = Task::new_code_generation("write a function to sort an array".to_string());
let task = Task::new_code_analysis("analyze this code for bugs".to_string());
let task = Task::new_refactoring(
    "fn old_code() {}".to_string(),
    "make it more efficient".to_string()
);
```

**Public methods:**

- `new_code_generation(input: String) -> Self`
- `new_code_analysis(input: String) -> Self`
- `new_refactoring(input: String, instruction: String) -> Self`
- `task_type(&self) -> &TaskType`
- `input(&self) -> &str`
- `context(&self) -> Option<&str>`

#### `TaskResult`

The result of solving a task.

```rust
pub struct TaskResult {
    pub output: String,
    pub confidence: f64,
    pub strategy_used: SolverStrategy,
    pub metadata: Option<String>,
}
```

**Public methods:**

- `new(output: String, confidence: f64, strategy: SolverStrategy, metadata: Option<String>) -> Self`

#### `TaskType`

Defines the type of task to execute.

```rust
pub enum TaskType {
    CodeAnalysis,
    Linting,
    Documentation,
    SimpleGeneration,
    ComplexGeneration,
    Refactoring,
    IntentParsing,
}
```

#### `SolverStrategy`

Defines how the neuro-symbolic orchestration should work.

```rust
pub enum SolverStrategy {
    /// Use only the symbolic solver
    SymbolicOnly,

    /// Use only the neural solver
    NeuralOnly,

    /// Try symbolic first, if it fails or has low confidence → neural
    SymbolicThenNeural,

    /// Neural generates, symbolic validates and corrects if necessary
    NeuralWithSymbolicValidation,

    /// Execute both in parallel, choose the best result
    Hybrid,
}
```

#### `AiError`

Error type for all AI operations.

```rust
pub enum AiError {
    SymbolicError(symbolic::SymbolicError),
    NeuralError(neural::NeuralError),
    TaskError(String),
}
```

### Feedback API

#### `FeedbackInput`

Public API for providing feedback to the AI system.

```rust
use ai::feedbacks::{FeedbackInput, FeedbackMeta, FeedbackVerdict};

// Correct output
let feedback = FeedbackInput::correct("task", "input", "generated_output");

// Incorrect with expected output
let feedback = FeedbackInput::incorrect_expected(
    "task",
    "input",
    "wrong_output",
    "expected_output"
);

// Partial correction
let feedback = FeedbackInput::partial_correction(
    "task",
    "input",
    "output_with_issues",
    "corrected_part"
);

// Rejected
let feedback = FeedbackInput::rejected("task", "input", "bad_output");

// With metadata
let feedback = FeedbackInput::correct("task", "input", "output")
    .meta(FeedbackMeta::new()
        .confidence(0.95)
        .rationale("well structured")
        .source("human reviewer"));
```

#### `FeedbackVerdict`

Defines the verdict for a feedback.

```rust
pub enum FeedbackVerdict<'a> {
    Correct,
    Incorrect { expected_output: Cow<'a, str> },
    Partial { correction: Cow<'a, str> },
    Rejected,
}
```

#### `FeedbackMeta`

Metadata for feedback.

```rust
pub struct FeedbackMeta<'a> {
    pub confidence: Option<f32>,
    pub rationale: Option<Cow<'a, str>>,
    pub source: Option<Cow<'a, str>>,
}
```

**Builder methods:**

- `new() -> Self`
- `confidence(self, v: f32) -> Self`
- `rationale(self, v: impl Into<Cow<'a, str>>) -> Self`
- `source(self, v: impl Into<Cow<'a, str>>) -> Self`

## Public Methods on `AiBody`

### Initialization

```rust
/// Create a new AiBody instance
pub fn new() -> Result<Self, AiError>
```

```rust
/// Load a neural model from disk
pub fn load_neural_model(
    &mut self,
    model_path: &Path,
    tokenizer_path: &Path,
) -> Result<(), AiError>
```

### Task Solving

```rust
/// Solve a task using automatic strategy selection
pub fn solve(&mut self, task: &Task) -> Result<TaskResult, AiError>
```

```rust
/// Force symbolic-then-neural strategy
pub fn solve_symbolic_then_neural(&mut self, task: &Task) -> Result<TaskResult, AiError>
```

```rust
/// Force neural-with-validation strategy
pub fn solve_neural_with_validation(&mut self, task: &Task) -> Result<TaskResult, AiError>
```

```rust
/// Force hybrid strategy
pub fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError>
```

### Code Operations

```rust
/// Generate code from a prompt
pub fn generate_code(&mut self, prompt: &str) -> Result<String, AiError>
```

```rust
/// Analyze code and return insights
pub fn analyze_code(&mut self, code: &str) -> Result<String, AiError>
```

```rust
/// Refactor code according to instructions
pub fn refactor_code(&mut self, code: &str, instruction: &str) -> Result<String, AiError>
```

### Training and Feedback

```rust
/// Train with simple verdict (ok/not ok)
pub fn train_with_verdict(
    &mut self,
    task: &Task,
    input: &str,
    generated_output: &str,
    ok: bool,
) -> Result<(), AiError>
```

```rust
/// Adjust the system with detailed feedback
pub fn adjust(&mut self, req: FeedbackInput<'_>) -> Result<(), AiError>
```

```rust
/// Train the neural model with examples
pub fn train_neural(
    &mut self,
    training_data: Vec<String>,
    model_path: &Path,
) -> Result<(), AiError>
```

### Model Persistence

```rust
/// Save the neural model to disk
pub fn save_neural_model(
    &self,
    model_path: &Path,
    tokenizer_path: &Path,
) -> Result<(), AiError>
```

```rust
/// Append a training example to the replay buffer
pub fn append_training_example(
    &self,
    replay_path: &Path,
    example_json: &str,
) -> Result<(), AiError>
```

```rust
/// Load training examples from the replay buffer
pub fn load_training_examples(
    &self,
    replay_path: &Path,
) -> Result<Vec<String>, AiError>
```

### Evaluation

```rust
/// Evaluate the model on test data
pub fn evaluate_model(&mut self, test_data: Vec<String>) -> Result<f64, AiError>
```

### Utilities

```rust
/// Create a task from a prompt (convenience method)
pub fn create_task(&self, prompt: &str) -> Task
```

## Usage Examples

### Basic Code Generation

```rust
use ai::{AiBody, Task};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AiBody::new()?;

    // Load neural model (optional)
    ai.load_neural_model(
        std::path::Path::new("model.safetensors"),
        std::path::Path::new("tokenizer.json")
    )?;

    // Generate code
    let code = ai.generate_code("write a function to calculate fibonacci")?;
    println!("Generated: {}", code);

    Ok(())
}
```

### Task-Based Solving

```rust
use ai::{AiBody, Task};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AiBody::new()?;

    // Create and solve a task
    let task = Task::new_code_generation("implement a binary search".to_string());
    let result = ai.solve(&task)?;

    println!("Output: {}", result.output);
    println!("Confidence: {:.2}", result.confidence);
    println!("Strategy: {:?}", result.strategy_used);

    Ok(())
}
```

### With Feedback

```rust
use ai::{AiBody, Task};
use ai::feedbacks::{FeedbackInput, FeedbackMeta};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AiBody::new()?;

    let task = Task::new_code_generation("write a sort function".to_string());
    let result = ai.solve(&task)?;

    // Provide feedback
    let feedback = FeedbackInput::correct(
        task.input(),
        task.input(),
        &result.output
    ).meta(
        FeedbackMeta::new()
            .confidence(0.9)
            .source("user")
    );

    ai.adjust(feedback)?;

    Ok(())
}
```

### Advanced Strategy Selection

```rust
use ai::{AiBody, Task};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AiBody::new()?;
    let task = Task::new_code_generation("complex algorithm".to_string());

    // Use different strategies
    let result1 = ai.solve_symbolic_then_neural(&task)?;
    let result2 = ai.solve_neural_with_validation(&task)?;
    let result3 = ai.solve_hybrid(&task)?;

    // Compare results
    println!("Symbolic-then-Neural: {:.2}", result1.confidence);
    println!("Neural-with-Validation: {:.2}", result2.confidence);
    println!("Hybrid: {:.2}", result3.confidence);

    Ok(())
}
```

### Training and Persistence

```rust
use ai::AiBody;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AiBody::new()?;

    // Collect training examples
    let training_data = vec![
        "example 1".to_string(),
        "example 2".to_string(),
    ];

    // Train the model
    ai.train_neural(training_data, Path::new("model_v2.safetensors"))?;

    // Save the trained model
    ai.save_neural_model(
        Path::new("model_final.safetensors"),
        Path::new("tokenizer_final.json")
    )?;

    // Evaluate
    let test_data = vec!["test1".to_string(), "test2".to_string()];
    let accuracy = ai.evaluate_model(test_data)?;
    println!("Model accuracy: {:.2}%", accuracy * 100.0);

    Ok(())
}
```

## Architecture Notes

The library is organized into:

- **Public API**:
  - `AiBody` - Primary interface for all operations
  - `Task` - Task construction and configuration (part of public API for advanced users)
  - `TaskResult` - Results from task execution
  - `FeedbackInput` - Feedback submission
  - Supporting types: `TaskType`, `SolverStrategy`, `AiError`

- **Internal modules** (not for direct use):
  - `ai_orchestrator` - Internal orchestration logic
  - `feedbacks::ai_feedback` - Internal feedback processing
  - `dispatcher`, `solver`, `training` - Internal implementation details

**Best Practice**: Use `AiBody` as your primary entry point. The `Task` type is part of the public API for advanced control, but most operations can be performed through `AiBody` convenience methods (`generate_code`, `analyze_code`, etc.).

**Import Pattern**: The library re-exports all public types at the crate root, so you can use:

```rust
use ai::{AiBody, Task, TaskResult, AiError};
// Instead of:
// use ai::ai_body::AiBody;
// use ai::task::Task;
```

## Error Handling

All public methods return `Result<T, AiError>`. The `AiError` enum wraps errors from both symbolic and neural solvers:

```rust
use ai::{AiBody, AiError};

match ai.solve(&task) {
    Ok(result) => println!("Success: {}", result.output),
    Err(AiError::SymbolicError(e)) => eprintln!("Symbolic error: {}", e),
    Err(AiError::NeuralError(e)) => eprintln!("Neural error: {}", e),
    Err(AiError::TaskError(msg)) => eprintln!("Task error: {}", msg),
}
```

## Thread Safety

`AiBody` is intentionally not `Send` or `Sync`. This design choice allows the library to maintain mutable neural state without synchronization overhead, optimizing for single-threaded performance.

If you need parallel processing:

- Create separate `AiBody` instances per thread
- Use message passing to communicate results between threads
- Consider using a thread pool with one `AiBody` instance per worker

**Example**:

```rust
use ai::AiBody;
use std::thread;

let handles: Vec<_> = (0..4).map(|i| {
    thread::spawn(move || {
        let mut ai = AiBody::new().expect("Failed to create AiBody instance");
        // Each thread has its own instance
        ai.generate_code(&format!("task {}", i))
    })
}).collect();

for handle in handles {
    let result = handle.join().expect("Thread panicked while joining");
    println!("Result: {:?}", result);
}
```

## Dependencies

- `neural`: Neural network solver (internal)
- `symbolic`: Symbolic reasoning solver (internal)
- `serde`: Serialization for feedback types
- `thiserror`: Error handling

## Contribuer

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request sur le dépôt GitHub.

Pour plus de détails sur le workflow Git/GitHub utilisé dans ce projet, consultez la [documentation sur le versioning](../../../docs/versioning/git-github.md).

## Licence
