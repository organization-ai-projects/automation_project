# Symbolic Library

Rule-based code analysis, generation, and refactoring for `automation_project`.

## Overview

This library provides deterministic, rule-based code manipulation capabilities. Unlike the neural library which uses ML, symbolic processing relies on pattern matching, templates, and explicit rules for predictable results.

## Features

- **Code Analysis** - Static analysis of Rust code structure
- **Code Generation** - Template-based code generation
- **Linting** - Rule-based code quality checks
- **Refactoring** - Automated code transformations
- **Validation** - Code correctness verification
- **Workflow Orchestration** - Multi-step processing pipelines

## Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                   SymbolicSolver                        │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
│  │ CodeAnalyzer│  │ RulesEngine │  │  CodeValidator  │ │
│  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘ │
│         │                │                   │          │
│         ▼                ▼                   ▼          │
│    Analysis         Generation          Validation      │
│    Linting          Refactoring                         │
└─────────────────────────────────────────────────────────┘
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
symbolic = { path = "../symbolic" }
```

## Usage

### Create a solver

```rust
use symbolic::symbolic_solver::SymbolicSolver;

let solver = SymbolicSolver::new()?;
```

### Analyze code

```rust
let result = solver.solve(code, "analysis", None)?;
println!("Analysis: {}", result.output);
println!("Confidence: {:.2}", result.confidence);
```

### Generate code from templates

```rust
let result = solver.solve(
    "Create a struct with name and age fields",
    "generation",
    Some("user context"),
)?;
println!("Generated:\n{}", result.output);
```

### Lint code

```rust
let result = solver.solve(code, "linting", None)?;
println!("Lint results: {}", result.output);
```

### Refactor code

```rust
let result = solver.solve(
    code,
    "refactoring",
    Some("rename variable 'x' to 'count'"),
)?;
println!("Refactored:\n{}", result.output);
```

### Validate code

```rust
let validation = solver.validate_code(code)?;
if validation.is_valid {
    println!("Code is valid");
} else {
    println!("Errors: {:?}", validation.errors);
}
```

### Adjust rules based on feedback

```rust
use symbolic::feedback_symbolic::SymbolicFeedback;

let feedback = SymbolicFeedback::positive("Good refactoring");
solver.adjust_rules(input, feedback)?;
```

## Task Types

| Task Type       | Description                              |
| --------------- | ---------------------------------------- |
| `analysis`      | Analyze code structure and patterns      |
| `generation`    | Generate code from prompts               |
| `linting`       | Check code for issues                    |
| `documentation` | Generate documentation for code          |
| `refactoring`   | Apply code transformations               |

## Modules

| Module      | Description                                |
| ----------- | ------------------------------------------ |
| `analyzer`  | Code structure analysis                    |
| `rules`     | Rules engine and templates                 |
| `validator` | Code validation and error checking         |
| `linter`    | Code quality analysis                      |
| `generation`| Template-based code generation             |
| `workflow`  | Multi-step processing orchestration        |

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/symbolic/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
