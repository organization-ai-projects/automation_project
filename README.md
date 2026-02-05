# Automatic Project

`automation_project` is an advanced automation workspace, designed to orchestrate multiple projects simultaneously with features such as code generation, linting, automated documentation, and workflow orchestration.

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

## Repository structure

- `projects/products/core/`: core binaries (engine, launcher, watcher, central UI).
- `projects/products/<product>/`: product backends and UI bundles.
- `projects/libraries/`: shared libraries (protocol, common, security, symbolic, neural, ai).
- `documentation/`: technical documentation and guides.

Internal documentation is available in the `documentation/` folder. Start with the [Documentation Index](documentation/TOC.md), or jump directly to the [Technical Documentation TOC](documentation/technical_documentation/TOC.md).

Scripts documentation is indexed in [scripts/TOC.md](scripts/TOC.md).

## Contribute

Contributions are welcome! Please open an issue or a pull request on the GitHub repository.

For contribution guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Licenses

Each crate in this workspace may have its own license. Please refer to the `LICENSE` file or the `README.md` in each crate's directory for specific licensing details.
