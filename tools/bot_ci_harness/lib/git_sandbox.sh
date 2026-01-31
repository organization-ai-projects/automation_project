#!/usr/bin/env bash
set -euo pipefail

# Creates:
#   $SANDBOX/remote.git (bare)
#   $SANDBOX/work       (clone)
# With branches main + dev, and configurable divergence.

create_sandbox() {
  SANDBOX="$(mktemp -d)"
  export SANDBOX
  REMOTE_BARE="$SANDBOX/remote.git"
  WORKDIR="$SANDBOX/work"

  git init --bare --initial-branch=main "$REMOTE_BARE" >/dev/null
  git clone "$REMOTE_BARE" "$WORKDIR" >/dev/null 2>&1

  pushd "$WORKDIR" >/dev/null
  git config user.name "harness"
  git config user.email "harness@local"

  echo "base" > file.txt
  git add file.txt
  git commit -m "base commit" >/dev/null

  git branch -M main
  git push -u origin main >/dev/null

  git switch -c dev >/dev/null
  git push -u origin dev >/dev/null

  popd >/dev/null

  export REMOTE_BARE WORKDIR
}

cleanup_sandbox() {
  rm -rf "${SANDBOX:-}" || true
}

# Adds commits to main only => dev behind
main_add_commit() {
  local msg="${1:-main update}"
  pushd "$WORKDIR" >/dev/null
  git switch main >/dev/null
  echo "$msg $(date +%s)" >> file.txt
  git add file.txt
  git commit -m "$msg" >/dev/null
  git push origin main >/dev/null
  popd >/dev/null
}

# Adds commits to dev only => dev diverges
dev_add_commit() {
  local msg="${1:-dev update}"
  pushd "$WORKDIR" >/dev/null
  git switch dev >/dev/null
  echo "$msg $(date +%s)" >> dev.txt
  git add dev.txt
  git commit -m "$msg" >/dev/null
  git push origin dev >/dev/null
  popd >/dev/null
}

# Creates an intentional conflict:
# - main changes file.txt line 1
# - dev changes file.txt line 1 differently
create_merge_conflict() {
  pushd "$WORKDIR" >/dev/null

  git switch main >/dev/null
  echo "MAIN" > file.txt
  git add file.txt
  git commit -m "main conflicting change" >/dev/null
  git push origin main >/dev/null

  git switch dev >/dev/null
  echo "DEV" > file.txt
  git add file.txt
  git commit -m "dev conflicting change" >/dev/null
  git push origin dev >/dev/null

  popd >/dev/null
}

background_commit() {
  local branch="$1"
  local msg="${2:-background update}"
  local tmpdir="$SANDBOX/bg-$(date +%s%N)"

  git clone "$REMOTE_BARE" "$tmpdir" >/dev/null 2>&1
  pushd "$tmpdir" >/dev/null
  git config user.name "harness"
  git config user.email "harness@local"

  if git show-ref --verify --quiet "refs/heads/$branch"; then
    git switch "$branch" >/dev/null
  else
    git switch -c "$branch" "origin/$branch" >/dev/null
  fi

  local target_file="file.txt"
  if [[ "$branch" == "dev" ]]; then
    target_file="dev.txt"
  fi

  echo "$msg $(date +%s)" >> "$target_file"
  git add "$target_file"
  git commit -m "$msg" >/dev/null
  git push origin "$branch" >/dev/null
  popd >/dev/null
  rm -rf "$tmpdir"
}
