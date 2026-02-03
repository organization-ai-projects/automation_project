# UI Library

Reusable UI components for `automation_project`.

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

- [Index de Documentation](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/ui/documentation/en/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
