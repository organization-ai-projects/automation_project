# Automated Documentation

- [Back to Technical TOC](TOC.md)

## Introduction

This document describes the objectives and features of automated documentation in the `automation_project`.

---

## 1. Automated Documentation

### 1.0 Convention

Crate-level documentation lives in a `documentation/` folder at the crate root.
Contribution guidelines live at the repository root (`CONTRIBUTING.md`).

### 1.1 Objectives

Documentation is an essential component of the project and must be generated automatically to ensure it is up-to-date and consistent with the code.

### 1.2 Ownership

- Workspace documentation lives in `documentation/technical_documentation/`.
- Crate documentation lives in `projects/**/documentation/` and should focus on crate-specific usage.

#### 1.1.1 Detailed Features

The following features are proposed for automated documentation:

1. **Automatic Generation**:
   - Use `cargo doc` to produce standard Rust documentation.
   - Enriched documentation with code examples, diagrams, and detailed explanations.

2. **Multi-format Export**:
   - **HTML**: For online viewing.
   - **Markdown**: For integration into Git repositories.
   - **PDF**: For deliverables or offline distribution.

   Export to other formats can be added later as needed.

3. **Integration with Workflows**:
   - Automatic generation of documentation for modules added or modified in workflows.
   - Enrichment of examples through symbolic and neural workflows.

4. **Compatibility and Standards**:
   - Adherence to standardized formats like Markdown and HTML.
   - Documentation of critical dependencies and minimum required versions.

5. **Verification and Quality**:
   - Define project-specific linting rules.
   - Automate code convention checks via:
     - **Clippy**: for standard Rust rules.
     - Custom rules tailored to the project.
   - Generate detailed reports on detected violations and improvement suggestions.
   - Propose automatic corrections where possible.

> These features ensure comprehensive, up-to-date documentation tailored to the diverse needs of users and developers.

Automated documentation is considered a first-class artifact of the system, on par with code or workflows.
