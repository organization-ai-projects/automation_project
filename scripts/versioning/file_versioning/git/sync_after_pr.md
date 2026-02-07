# Sync After PR Merge Workflow

This document explains how to synchronize your local branches after a PR merge, covering both manual cleanup steps and automated cleanup via the `cleanup_after_pr.sh` script.

## Overview

After a PR merges into the base branch (typically `dev`), your local branches may become outdated. You need to:

1. Update your base branch (`dev`) to get the latest merged changes
2. Identify local branches that are now behind the base branch
3. Delete outdated branches (and optionally recreate them from the updated base)

## When to Use Each Approach

### Use Manual Cleanup When

- **Learning the workflow** - Understanding each step helps build familiarity
- **Single branch cleanup** - You only need to update one or two specific branches
- **Custom requirements** - You need selective control over which branches to update
- **Troubleshooting** - Investigating issues requires step-by-step execution

### Use Automated Cleanup When

- **Multiple outdated branches** - You have many branches that need updating
- **Regular maintenance** - Performing routine cleanup after PR merges
- **Consistent workflow** - You want a repeatable, error-free process
- **Time efficiency** - You want to complete cleanup quickly

**Recommendation:** Start with manual cleanup to understand the process, then switch to the automated script for routine maintenance.

## Manual Cleanup Steps

### Step 1: Update Your Base Branch

First, ensure your base branch has the latest changes:

```bash
git checkout dev
git pull origin dev
```

### Step 2: Identify Outdated Branches

Find branches that are behind the base branch:

```bash
# List all local branches except dev and main
for branch in $(git for-each-ref --format='%(refname:short)' refs/heads | grep -vE '^(dev|main)$'); do
  # Count how many commits the branch is behind
  BEHIND=$(git rev-list --count "$branch..dev" 2>/dev/null || echo "0")
  
  if [ "$BEHIND" -gt 0 ]; then
    echo "Branch $branch is behind by $BEHIND commit(s)"
  fi
done
```

### Step 3: Clean Up Outdated Branches

For each outdated branch, decide whether to:

**Option A: Delete only (when work is merged or abandoned)**

```bash
# Delete local branch
git branch -d <branch-name>    # Safe delete (fails if unmerged)
# OR
git branch -D <branch-name>    # Force delete

# Delete remote branch (if exists)
git push origin --delete <branch-name>
```

**Option B: Delete and recreate (when continuing work on the branch)**

```bash
# Delete the outdated branch
git branch -D <branch-name>
git push origin --delete <branch-name>

# Recreate from updated base
git checkout -b <branch-name> dev
git push --set-upstream origin <branch-name>
```

### Step 4: Return to Your Working Branch

```bash
git checkout <your-current-branch>
```

## Automated Cleanup with cleanup_after_pr.sh

The `cleanup_after_pr.sh` script automates the entire cleanup process.

### Quick Start

**Default behavior (recreate branches):**

```bash
cd scripts/versioning/file_versioning/git
./cleanup_after_pr.sh
```

This will:

1. Update your base branch (default: `dev`)
2. Detect all local branches behind the base branch
3. Delete outdated branches (local and remote)
4. Recreate them from the updated base branch
5. Return to your original branch

**Delete-only mode (don't recreate):**

```bash
./cleanup_after_pr.sh --delete-only
```

Use this when you want to clean up old branches without recreating them.

### Configuration

Control the script behavior with environment variables:

```bash
# Use a different remote
REMOTE=upstream ./cleanup_after_pr.sh

# Use a different base branch
BASE_BRANCH=main ./cleanup_after_pr.sh

# Combine options
REMOTE=upstream BASE_BRANCH=main ./cleanup_after_pr.sh --delete-only
```

### What the Script Does

1. **Updates base branch:** Checks out and pulls latest changes from base branch
2. **Detects outdated branches:** Compares each local branch against base branch
3. **Protected branches:** Automatically skips `dev` and `main` branches
4. **Deletes branches:**
   - Attempts safe delete (`git branch -d`)
   - Falls back to force delete if needed (`git branch -D`)
   - Removes remote branches if they exist
   - ⚠️ **Warning:** Force delete (`git branch -D`) will delete branches with unmerged commits. Ensure branches are fully merged or abandoned before running the script.
5. **Recreates branches (default):** Creates new branches from updated base
6. **Restores context:** Returns to your original branch when complete

### Script Options

```bash
./cleanup_after_pr.sh [OPTIONS]

Options:
  --delete-only    Delete outdated branches without recreating them
  --help, -h       Show help message

Environment Variables:
  REMOTE           Git remote name (default: origin)
  BASE_BRANCH      Base branch to compare against (default: dev)
```

### Example Workflows

**Standard cleanup after PR merge:**

```bash
# After your PR merges into dev
cd scripts/versioning/file_versioning/git
./cleanup_after_pr.sh
```

**Cleanup with custom configuration:**

```bash
# Using main as base branch with upstream remote
BASE_BRANCH=main REMOTE=upstream ./cleanup_after_pr.sh
```

**Remove old feature branches:**

```bash
# Delete without recreating (cleanup abandoned work)
./cleanup_after_pr.sh --delete-only
```

## Safety Features

Both manual and automated approaches include safety features:

### Manual Cleanup Safety

- **Safe delete** (`git branch -d`) prevents accidental deletion of unmerged work
- **Manual review** lets you verify each branch before deletion
- **Step-by-step** execution allows stopping at any point

### Automated Script Safety

- **Protected branches** - Never touches `dev` or `main` branches
- **Remote validation** - Checks if remote branches exist before deletion
- **Safe delete first** - Tries safe delete before force delete
- **Branch restoration** - Returns to original branch after completion
- **Error handling** - Uses `set -euo pipefail` for strict error checking

⚠️ **Important:** The script will force-delete (`git branch -D`) branches when safe delete fails. This can remove branches with unmerged local commits. Before running the script, verify that target branches are fully merged or no longer needed. For selective/safer cleanup, use the manual workflow instead.

## Troubleshooting

### "Failed to delete branch" Error

If you see this during manual cleanup:

```bash
error: The branch '<branch-name>' is not fully merged.
```

This means the branch has commits not in the base branch. Options:

- Verify the commits are merged (check PR status)
- Use force delete: `git branch -D <branch-name>` (only if you're certain)

### "Remote branch not deleted" Warning

The script may show:

```
ℹ Remote branch not deleted (permissions/protection?).
```

This is usually due to:

- Branch protection rules on the remote
- Insufficient permissions
- The remote branch was already deleted

This is safe to ignore if the local branch cleanup succeeded.

### Detached HEAD State

If you're in detached HEAD state after script execution:

```bash
# The script attempts to restore your original branch
# If it fails, manually checkout a branch:
git checkout dev
```

## Best Practices

1. **Run cleanup after each PR merge** - Keeps your local repository organized
2. **Review outdated branches** - Consider if work needs to continue before recreating
3. **Use delete-only for abandoned work** - No need to recreate branches you won't use
4. **Start manual, then automate** - Understand the process before automating it
5. **Communicate with team** - Ensure remote branches can be safely deleted

## Related Documentation

- [Git Scripts README](README.md) - Overview of all git utility scripts
- [Create Branch Script](create_branch.sh) - Creating branches with validation
- [Delete Branch Script](delete_branch.sh) - Deleting branches safely
- [Orchestrators Documentation](../orchestrators/README.md) - Workflow orchestration

## Summary

| Aspect | Manual Cleanup | Automated Script |
|--------|---------------|------------------|
| **Best for** | Learning, single branches | Multiple branches, routine maintenance |
| **Speed** | Slower, step-by-step | Fast, automated |
| **Control** | High - review each step | Moderate - batch processing |
| **Error prevention** | Manual verification | Built-in safety checks |
| **Repeatability** | Variable | Consistent |
| **Recommended use** | Initial learning, custom needs | Regular maintenance |

**Get Started:** Try the manual process once to understand the workflow, then use `cleanup_after_pr.sh` for routine maintenance after PR merges.
