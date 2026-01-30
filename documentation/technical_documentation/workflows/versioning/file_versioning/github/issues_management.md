# Issue Management

- [Back to GitHub Index](TOC.md)

This document explains how to manage issues in the `automation_project` repository.

## Key Issues (Priority)

Keep this list short and focused on top priorities. Do not mirror the full issue tracker here.

- [Issue #51](https://github.com/organization-ai-projects/automation_project/issues/51) - `metadata.ron` schema and requirements.

## Creating an Issue

1. **Navigate to the Issues Tab**:
   - Go to the GitHub repository and click on the `Issues` tab.

2. **Click on `New Issue`**:
   - Provide a clear and concise title for the issue.
   - Add a detailed description, including:
     - The problem or feature request.
     - Steps to reproduce (if applicable).
     - Expected behavior or outcome.

3. **Assign Labels**:
   - Use labels to categorize the issue (e.g., `bug`, `enhancement`, `documentation`).

4. **Assign the Issue**:
   - Assign the issue to yourself or a team member responsible for resolving it.

5. **Link to Milestones**:
   - If the issue is part of a milestone, link it to the appropriate milestone.

## Linking Issues to Commits and PRs

1. **Reference the Issue in a Commit Message**:
   - Use `Refs #<issue-number>` in the commit message to link the commit to the issue.
   - Example:

     ```text
     fix: resolve null pointer exception

     - Fixed a null pointer exception in the parser.
     - Refs #123
     ```

2. **Reference the Issue in a Pull Request**:
   - Use `Closes #<issue-number>` in the pull request description to close the issue automatically when the PR is merged.
   - Example:

     ```text
     This PR fixes the null pointer exception in the parser.

     Closes #123
     ```

## Resolving an Issue

1. **Ensure the Issue is Addressed**:
   - Verify that the issue has been resolved in the code.
   - Ensure all tests pass and the solution meets the requirements.

2. **Close the Issue**:
   - If the issue is not automatically closed by a PR, close it manually with a comment explaining the resolution.

3. **Communicate with the Team**:
   - Notify the team about the resolution during stand-ups or via communication tools.

## Best Practices

- **Be Descriptive**: Provide as much detail as possible when creating an issue.
- **Use Templates**: If available, use issue templates to ensure consistency.
- **Keep Issues Updated**: Add comments or updates as progress is made.
- **Prioritize**: Focus on high-priority issues first.

By following these guidelines, the team can manage issues efficiently and maintain a clear workflow.
