# Script: sync_docs.sh

- [Back to Automation Index](TOC.md)

This document explains the intended usage of the `sync_docs.sh` script, which is dedicated to internal automation.

## Purpose

Currently, shared documentation is not synchronized into crates. The repository-level `CONTRIBUTING.md` is the single source of truth.

## Usage

Run the script from the root of the repository:

```bash
./scripts/automation/sync_docs.sh
```

## Features

1. **Documentation Synchronization**:
   - Disabled for now. Crates should link to `CONTRIBUTING.md` instead of copying it.

2. **Selective Updates**:
   - Not applicable while synchronization is disabled.

## Example Output

```bash
Synchronized files for projects/libraries/ai
Synchronized files for projects/libraries/ast_core
Synchronized files for projects/products/code_agent_sandbox
```

## Notes

- The script assumes the shared documentation files are located in `documentation/technical_documentation`.
- The list of crate directories is defined in the script and can be updated as needed.
