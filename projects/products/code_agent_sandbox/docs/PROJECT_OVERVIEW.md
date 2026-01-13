# Project Overview

## Table of Contents

- [Context](#context)
- [Key Features](#key-features)
  - [1. Security Policy Management](#1-security-policy-management)
  - [2. Secure Command Execution](#2-secure-command-execution)
  - [3. File Management with `SandboxFs`](#3-file-management-with-sandboxfs)
  - [4. Action Logging and Tracking](#4-action-logging-and-tracking)
- [Use Cases](#use-cases)
- [Architecture](#architecture)
- [Conclusion](#conclusion)

## Context

This project provides a robust and modular infrastructure for executing various actions in a controlled environment. It includes advanced features such as security policy management, secure command execution, and file management through a sandboxed file system.

## Key Features

### 1. Security Policy Management

The `Policy` module allows defining and enforcing security rules for executed actions. This includes:

- Validating authorized commands.
- Restricting access to sensitive resources.
- Applying specific rules for certain actions.

### 2. Secure Command Execution

The project includes a mechanism to execute commands in a secure environment (bunker) when necessary. This ensures that sensitive commands do not compromise system integrity.

### 3. File Management with `SandboxFs`

The sandboxed file system (`SandboxFs`) allows:

- Controlled file reading and writing.
- Path validation.
- Managing symbolic links to prevent unauthorized access.

### 4. Action Logging and Tracking

The `Journal` module tracks all executed actions, recording:

- Important events.
- Action results.
- Policy violations, if any.

## Use Cases

- **Task Automation**: Running scripts and commands in a controlled environment.
- **Enhanced Security**: Enforcing strict policies for sensitive actions.
- **Resource Management**: Controlling access to files and directories.

## Architecture

The project is structured into several modules, each with a specific responsibility:

- `Policy`: Security rule management.
- `CommandRunner`: Command execution.
- `SandboxFs`: File management in an isolated environment.
- `Journal`: Action tracking and logging.

## Conclusion

This project offers a comprehensive solution for managing actions in a secure and controlled environment. Its modularity allows for easy extension to meet specific needs.
