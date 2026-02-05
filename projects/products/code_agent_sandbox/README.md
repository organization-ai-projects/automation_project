# Code Agent Sandbox Documentation

This directory contains a robust and modular infrastructure for executing various actions in a controlled environment.

## Role in the Project

This product is responsible for providing a secure sandbox environment for executing code and commands. It includes security policy management, secure execution of sensitive commands, sandboxed file system management, and action logging.

It interacts mainly with:

- AI library - For code generation and execution
- Security library - For policy enforcement
- Command runner library - For command execution

## Directory Structure

```
code_agent_sandbox/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   ├── TOC.md
│   ├── PROJECT_OVERVIEW.md
│   └── BUNKER_EXECUTION.md
├── src/               # Source code
│   ├── lib.rs
│   ├── executor/
│   ├── filesystem/
│   └── ...
└── tests/             # Integration tests
```

## Description

The **Code Agent Sandbox** crate provides a robust and modular infrastructure for executing various actions in a controlled environment. It includes advanced features such as:

- Security policy management.
- Secure execution of sensitive commands.
- File management through a sandboxed file system.
- Action logging and tracking.

## Documentation

For detailed documentation, please refer to the [documentation/ folder](https://github.com/organization-ai-projects/automation_project/tree/main/projects/products/code_agent_sandbox/documentation) or the [Project Overview](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/code_agent_sandbox/documentation/PROJECT_OVERVIEW.md).

## Installation

Add this crate to your project using `cargo`:

```bash
cargo add code_agent_sandbox
```

## Contribution

Contributions are welcome! Please open an issue or submit a pull request on the GitHub repository.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
