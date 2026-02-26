#!/usr/bin/env bash

resolve_repo_name_with_owner() {
  if [[ -n "${GH_REPO:-}" ]]; then
    printf '%s\n' "$GH_REPO"
    return 0
  fi

  local gh_repo
  gh_repo="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
  if [[ -n "$gh_repo" ]]; then
    printf '%s\n' "$gh_repo"
    return 0
  fi

  # Fallback: derive owner/name from origin remote URL when gh context is unavailable.
  local remote_url
  remote_url="$(git config --get remote.origin.url 2>/dev/null || true)"
  if [[ -z "$remote_url" ]]; then
    return 0
  fi

  # Supports:
  # - https://github.com/owner/repo.git
  # - git@github.com:owner/repo.git
  # - ssh://git@github.com/owner/repo.git
  printf '%s\n' "$remote_url" \
    | sed -E 's#^(https?://[^/]+/|ssh://[^/]+/|git@[^:]+:)##; s#\.git$##'
}

normalize_parent_value() {
  local raw="$1"
  raw="$(printf '%s' "$raw" | tr '[:upper:]' '[:lower:]')"
  raw="$(printf '%s' "$raw" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')"
  raw="${raw#(}"
  raw="${raw%)}"
  printf '%s\n' "$raw"
}

extract_issue_refs_from_text() {
  local text="$1"
  echo "$text" | awk '
    {
      line = $0
      lower = tolower($0)
      while (match(lower, /(closes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#[0-9]+/)) {
        matched = substr(line, RSTART, RLENGTH)
        keyword = tolower(matched)
        gsub(/[[:space:]]+#[0-9]+$/, "", keyword)
        issue = matched
        sub(/^.*#/, "", issue)
        print keyword "|" issue
        line = substr(line, RSTART + RLENGTH)
        lower = substr(lower, RSTART + RLENGTH)
      }
    }
  ' | sort -u
}

issue_has_children() {
  local issue_number="$1"
  local repo="$2"

  local child_count
  child_count="$(gh issue list -R "$repo" --state all --search "\"Parent: #${issue_number}\" in:body" --limit 1 --json number --jq 'length' 2>/dev/null || echo "0")"
  [[ "$child_count" != "0" ]]
}

issue_parent_value() {
  local issue_number="$1"
  local repo="$2"
  local body
  local parent_line
  local parent_value

  body="$(gh issue view "$issue_number" -R "$repo" --json body -q '.body // ""' 2>/dev/null || true)"
  parent_line="$(printf '%s\n' "$body" | grep -iE '^[[:space:]]*Parent:[[:space:]]*(#?[0-9]+|none|\(none\))[[:space:]]*$' | tail -n1 || true)"
  if [[ -z "$parent_line" ]]; then
    printf 'none\n'
    return 0
  fi

  parent_value="$(printf '%s\n' "$parent_line" | sed -E 's/^[[:space:]]*Parent:[[:space:]]*//I')"
  normalize_parent_value "$parent_value"
}

issue_is_root_parent() {
  local issue_number="$1"
  local repo="$2"
  local parent_value

  if ! issue_has_children "$issue_number" "$repo"; then
    return 1
  fi

  parent_value="$(issue_parent_value "$issue_number" "$repo")"
  [[ "$parent_value" == "none" || -z "$parent_value" ]]
}
