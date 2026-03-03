# Table of Contents

Language: **English** | [Francais](i18n/fr/TOC.md)

This document provides an overview of all documentation files in this directory.

## Documentation

- [README.md](README.md): Main documentation for file versioning utilities
- [conventions.sh](conventions.sh): Shared commit/PR title validation contract
- [git/README.md](git/README.md): Documentation for local VCS (`vcs_local_*`) helpers
- [github/README.md](github/README.md): Documentation for shared GitHub issue helper functions

## Git Helpers

- [git/commands.sh](git/commands.sh): Single local VCS command backend (`vcs_local_*`)
- [git/branch.sh](git/branch.sh): Branch management utilities
- [git/commit.sh](git/commit.sh): Commit operations
- [git/repo.sh](git/repo.sh): Repository validation utilities
- [git/staging.sh](git/staging.sh): Staging/index operations
- [git/synch.sh](git/synch.sh): Synchronization utilities
- [git/working_tree.sh](git/working_tree.sh): Working tree state validation

## GitHub Helpers

- [github/commands.sh](github/commands.sh): Single remote VCS command backend (`vcs_remote_*`)
- [github/issue_helpers.sh](github/issue_helpers.sh): Shared issue reference parsing and marker-comment upsert helpers
- [github/pull_request_lookup.sh](github/pull_request_lookup.sh): Shared PR lookup helpers
