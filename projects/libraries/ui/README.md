# UI Library Documentation

This directory contains reusable UI components for the automation project.

## Role in the Project

This library is responsible for providing shared UI components and widgets used by product UIs across the automation project. Components are built with Dioxus and designed to be compiled to WASM for use in UI bundles.

It interacts mainly with:

- Product UIs - For shared components
- Central UI - For common widgets
- Accounts UI, Varina UI - For UI elements

## Directory Structure

```
ui/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
└── src/               # Source code
    ├── lib.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Overview

This library provides shared UI components and widgets used by product UIs. Components are built with Dioxus and designed to be compiled to WASM for use in UI bundles.

## Status

This library is under active development. Components will be added as the UI system matures.

## Planned Features

- Common widgets (buttons, forms, tables)
- Theme system and styling utilities
- Layout components
- Navigation elements

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ui = { path = "../ui" }
```

## Usage

```rust
use ui::init;

// Initialize the UI library
init();
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/ui/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
