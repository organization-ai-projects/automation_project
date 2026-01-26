# Git Workflow Pipeline Overview

- [Back to Git Index](TOC.md)

This document provides an overview of the Git workflow pipeline for the `automation_project`. Each step links to a detailed guide.

Related scripts:

- [Git Scripts Index](scripts/TOC.md)
- [File Versioning Scripts](../scripts/TOC.md)

## Table of Contents

1. [Branch Creation](branch_creation.md)
2. [Commit Guidelines](commit.md)
3. [Push Guidelines](push.md)
4. [Pull Request and Merging](pull_request.md)
5. [Issue Management](../github/issues_management.md)

## Workflow Pipeline

0. **Check Existing Issues**:
   - Before starting any work, check the [Issues](../github/issues_management.md) to see if there is already an issue for the task.
   - If no issue exists, create one following the [Issue Management Guidelines](../github/issues_management.md).
   - Assign the issue to yourself and link it to your branch or pull request.

1. **Create a New Branch**:
   - Follow the [Branch Creation Guidelines](branch_creation.md) to create a new working branch.

2. **Make Changes and Commit**:
   - Follow the [Commit Guidelines](commit.md) to stage and commit your changes.

3. **Push Changes**:
   - Follow the [Push Guidelines](push.md) to push your changes to the remote repository.

4. **Create a Pull Request**:
   - Open a pull request to merge your branch into `dev`.
   - Ensure all tests pass and the PR is approved before merging. CI enforces
     tests on `dev` and `main`.

5. **Wait for CI Validation**:
   - Ensure that all Continuous Integration (CI) checks pass before proceeding with the merge.

6. **Merge into `main`**:
   - Once the PR to `dev` is merged and validated, create a PR to merge `dev` into `main`.
   - Wait for CI validation on the `main` branch before merging.

7. **Synchronize `main` and `dev`**:
   - Use the `sync_main_dev.sh` script to ensure `main` and `dev` are synchronized:

     ```bash
     ./scripts/versioning/file_versioning/synch_main_dev.sh
     ```

8. **Create a New Working Branch**:
   - After synchronization, you can create a new working branch following the [Branch Creation Guidelines](branch_creation.md).

9. **Manage Issues**:
   - Follow the [Issue Management Guidelines](../github/issues_management.md) to create, assign, and link issues to commits or pull requests.

## Merging Process Summary

- **From `dev` to `main`**:
  1. Stabilize the `dev` branch.
  2. Perform thorough tests.
  3. Create a PR from `dev` to `main`.
  4. Once approved, merge into `main`.
- **From a working branch to `dev`**:
  1. Create a PR from the working branch to `dev`.
  2. Wait for approval and ensure tests pass.
  3. Merge into `dev`.
