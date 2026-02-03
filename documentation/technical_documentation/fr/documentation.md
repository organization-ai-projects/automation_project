# Documentation Automatisée

- [Retour à TOC Technique](TOC.md)

## Introduction

Ce document describes the objectives and features of automated documentation in the `automation_project`.

---

## 1. Documentation Automatisée

### 1.0 Convention

Crate-level documentation lives in a `documentation/` folder at the crate root.
Contribution guidelines live at the repository root (`CONTRIBUTING.md`).

### 1.1 Objectives

Documentation is an essential component of the project and must be generated automatically to ensure it is up-to-date and consistent with the code.

### 1.2 Ownership

- Workspace documentation lives in `documentation/technical_documentation/`.
- Crate documentation lives in `projects/**/documentation/` and should focus on crate-specific usage.

### 1.3 Detailed Features

The following features are planned for automated documentation:

1. **Automatic Generation**:
   - Use `cargo doc` to produce standard Rust documentation.
   - Enriched documentation with code examples, diagrams, and detailed explanations.

2. **Multi-format Export (optional)**:
   - **HTML**: For online viewing.
   - **Markdown**: For integration into Git repositories.
   - **PDF**: For deliverables or offline distribution.

   Export to other formats can be added later as needed.

3. **Integration with Workflows (planned)**:
   - Automatic generation of documentation for modules added or modified in workflows.
   - Enrichment of examples through symbolic and neural workflows.

4. **Compatibility and Standards**:
   - Adherence to standardized formats like Markdown and HTML.
   - Documentation of critical dependencies and minimum required versions.

5. **Verification and Quality (planned)**:
   - Define project-specific linting rules.
   - Automate code convention checks via:
     - **Clippy**: for standard Rust rules.
     - Custom rules tailored to the project.
   - Generate detailed reports on detected violations and improvement suggestions.
   - Propose automatic corrections where possible.

> These features ensure comprehensive, up-to-date documentation tailored to the diverse needs of users and developers.

Automated documentation is considered a first-class artifact of the system, on par with code or workflows.

### 1.3 Examples

#### Example 1: Generating Documentation

To generate documentation for the entire workspace, use the following command:

```bash
cargo doc --workspace --open
```

This will produce HTML documentation for all crates in the workspace and open it in your default browser.

#### Example 2: Exporting to PDF

Currently, exporting to PDF requires a third-party tool. Use the following steps:

1. Generate HTML documentation using `cargo doc`.
2. Convert the HTML to PDF using a tool like `wkhtmltopdf`:

```bash
wkhtmltopdf target/doc/index.html documentation.pdf
```

#### Example 3: Linting Documentation

To ensure documentation adheres to standards, run the following command:

```bash
cargo clippy -- -D warnings
```

This will enforce all linting rules and highlight any issues in the documentation.
