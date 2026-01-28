# GitHub CLI Usage

- [Back to GitHub Index](TOC.md)

This document explains how to use the GitHub CLI (`gh`) to automate tasks in the `automation_project`.

## Creating a Pull Request

Use the following command to create a pull request:

```bash
gh pr create --title "<PR Title>" --body "<PR Description>" --base dev
```

## Managing Labels

To create or update labels:

```bash
gh label create <label-name> --color <hex-color>
```

## Watching PR Checks

To monitor the status of PR checks:

```bash
gh pr checks
```

## Additional Commands

- Use `gh issue create` to create issues directly from the CLI.
- Use `gh repo clone` to clone repositories.
- Use `gh auth login` to authenticate with GitHub.
