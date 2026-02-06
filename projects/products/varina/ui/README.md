# Varina UI Documentation

This directory contains the UI bundle source for the Varina product.

## Role in the Project

This product UI is responsible for providing the Varina interface as a runtime-loaded WASM bundle. It sends commands and receives events via Engine, avoiding direct coupling with the backend.

It interacts mainly with:

- Central UI - Loaded at runtime by central_ui
- Varina backend - Through Engine for commands and events
- Engine - For communication

## Directory Structure

```
ui/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   ├── TOC.md
│   └── usage.md
└── src/               # Source code
    └── main.rs
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Responsibilities

- Provides the product UI as a runtime-loaded WASM bundle.
- Sends commands and receives events via the Engine.
- Avoids direct coupling with the backend.

For architecture context, see `documentation/technical_documentation/ARCHITECTURE.md`.
## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/varina/ui/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
