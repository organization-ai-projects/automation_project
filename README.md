# Automatic Project

This directory contains the `automation_project` workspace, designed to orchestrate multiple projects simultaneously with features such as code generation, linting, automated documentation, and workflow orchestration.

## Role in the Project

This repository serves as the main workspace for the automation project. It is responsible for coordinating all project components, products, and libraries into a cohesive automation system.

It interacts mainly with:

- `projects/products/core/`: Core binaries including engine, launcher, watcher, and central UI
- `projects/libraries/`: Shared libraries for protocol, common utilities, security, and AI
- `documentation/`: Technical documentation and architectural guides
- `.github/workflows/`: CI/CD automation and workflow orchestration

## Architecture at a glance

- Engine is the single authority and WebSocket hub.
- Product backends are separate processes; UIs are runtime-loaded bundles.
- Central UI aggregates product UIs without compile-time coupling.

For details, see the architecture doc: `documentation/technical_documentation/ARCHITECTURE.md`.

## Directory Structure

```plaintext
./
├── .github/              # GitHub configuration and workflows
├── documentation/        # Technical documentation and guides
├── projects/             # Products and libraries
│   ├── products/         # Product backends and UI bundles
│   └── libraries/        # Shared libraries (protocol, common, security, ai)
├── scripts/              # Automation and versioning scripts
├── CONTRIBUTING.md       # Contribution guidelines
└── README.md             # This file
```

## Files

- `README.md`: This file.
- `CONTRIBUTING.md`: Contribution guidelines.
- `LICENSE`: Repository license (if present).
- `.github/`: GitHub configuration and workflows.
- `documentation/`: Technical documentation and guides.
- `projects/`: Products and libraries.
- `scripts/`: Automation and versioning scripts.

Internal documentation is available in the `documentation/` folder. Start with the [Documentation Index](documentation/TOC.md), or jump directly to the [Technical Documentation TOC](documentation/technical_documentation/TOC.md).

Scripts documentation is indexed in [scripts/TOC.md](scripts/TOC.md).

## Contribute

Contributions are welcome! Please open an issue or a pull request on the GitHub repository.

For contribution guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Licenses

Each crate in this workspace may have its own license. Please refer to the `LICENSE` file or the `README.md` in each crate's directory for specific licensing details.
