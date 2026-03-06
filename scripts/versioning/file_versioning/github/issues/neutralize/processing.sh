#!/usr/bin/env bash

neutralize_upsert_pr_comment() {
  local repo_name="$1"
  local pr_number="$2"
  local marker="$3"
  local body="$4"
  local comment_id

  comment_id="$({
    gh api "repos/${repo_name}/issues/${pr_number}/comments" --paginate
  } | jq -r --arg marker "$marker" '
      map(select((.body // "") | contains($marker)))
      | sort_by(.updated_at)
      | last
      | .id // empty
    ' 2>/dev/null || true)"

  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${repo_name}/issues/comments/${comment_id}" \
      -f body="$body" >/dev/null
  else
    gh api "repos/${repo_name}/issues/${pr_number}/comments" \
      -f body="$body" >/dev/null
  fi
}

neutralize_issue_non_compliance_reason() {
  local issue_number="$1"
  local repo_name="$2"
  issue_fetch_non_compliance_reason "$issue_number" "$repo_name"
}

neutralize_run() {
  local pr_number=""
  local repo_name="${GH_REPO:-}"

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --pr)
      pr_number="${2:-}"
      shift 2
      ;;
    --repo)
      repo_name="${2:-}"
      shift 2
      ;;
    -h | --help)
      neutralize_usage
      exit 0
      ;;
    *)
      echo "Error: unknown option: $1" >&2
      neutralize_usage >&2
      exit 2
      ;;
    esac
  done

  [[ -n "$pr_number" ]] || {
    echo "Error: --pr is required." >&2
    neutralize_usage >&2
    exit 2
  }
  neutralize_require_number "--pr" "$pr_number"
  neutralize_require_deps

  if [[ -z "$repo_name" ]]; then
    repo_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
  fi
  [[ -n "$repo_name" ]] || {
    echo "Error: unable to determine repository." >&2
    exit 3
  }

  local marker="<!-- closure-neutralizer:${pr_number} -->"

  local pr_json
  pr_json="$(gh pr view "$pr_number" -R "$repo_name" --json body,url,number 2>/dev/null || true)"
  if [[ -z "$pr_json" ]]; then
    echo "Error: unable to read PR #${pr_number}." >&2
    exit 4
  fi

  local original_body updated_body
  original_body="$(echo "$pr_json" | jq -r '.body // ""')"
  updated_body="$original_body"

  declare -A seen_ref
  declare -A neutralized_reason
  declare -A neutralized_action
  local neutralized_count=0
  local action issue_key issue_number dedupe_key reason keyword_pattern escaped_issue_key

  while IFS='|' read -r action issue_key; do
    issue_key="$(neutralize_trim "$issue_key")"
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    issue_number="${issue_key//#/}"
    dedupe_key="${action}|${issue_key}"
    if [[ -n "${seen_ref[$dedupe_key]:-}" ]]; then
      continue
    fi
    seen_ref["$dedupe_key"]=1

    reason="$(neutralize_issue_non_compliance_reason "$issue_number" "$repo_name")"
    [[ -n "$reason" ]] || continue

    keyword_pattern="$(neutralize_keyword_pattern_from_action "$action")"
    [[ -n "$keyword_pattern" ]] || continue

    escaped_issue_key="$(printf '%s' "$issue_key" | sed 's/[^^]/[&]/g; s/\^/\\^/g')"
    updated_body="$(
      perl -0777 -pe "s/\\b((?:${keyword_pattern}))\\b(\\s+)(?!rejected\\b)([^\\s]*${escaped_issue_key})\\b/\$1\$2rejected \$3/ig" \
        <<<"$updated_body"
    )"

    neutralized_reason["$issue_key"]="$reason"
    neutralized_action["$issue_key"]="$action"
    neutralized_count=$((neutralized_count + 1))
  done < <(parse_closing_issue_refs_from_text "$original_body")

  while IFS='|' read -r action issue_key; do
    issue_key="$(neutralize_trim "$issue_key")"
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    issue_number="${issue_key//#/}"
    dedupe_key="${action}|${issue_key}"
    if [[ -n "${seen_ref[$dedupe_key]:-}" ]]; then
      continue
    fi
    seen_ref["$dedupe_key"]=1

    reason="$(neutralize_issue_non_compliance_reason "$issue_number" "$repo_name")"

    keyword_pattern="$(neutralize_keyword_pattern_from_action "$action")"
    [[ -n "$keyword_pattern" ]] || continue

    escaped_issue_key="$(printf '%s' "$issue_key" | sed 's/[^^]/[&]/g; s/\^/\\^/g')"

    if [[ -n "$reason" ]]; then
      neutralized_reason["$issue_key"]="$reason"
      neutralized_action["$issue_key"]="$action"
      neutralized_count=$((neutralized_count + 1))
    else
      updated_body="$(
        perl -0777 -pe "s/\\b((?:${keyword_pattern}))\\b(\\s+)rejected\\s+([^\\s]*${escaped_issue_key})\\b/\$1\$2\$3/ig" \
          <<<"$updated_body"
      )"
    fi
  done < <(parse_neutralized_closing_issue_refs_from_text "$original_body")

  if [[ "$updated_body" != "$original_body" ]]; then
    gh pr edit "$pr_number" -R "$repo_name" --body "$updated_body" >/dev/null
  fi

  local comment_body
  if [[ "$neutralized_count" -gt 0 ]]; then
    comment_body="$marker
### Closure Neutralization Status

⚠️ Non-compliant issue references were neutralized to prevent incorrect auto-close.

"
    for issue_key in "${!neutralized_reason[@]}"; do
      comment_body+="- ${neutralized_action[$issue_key]} rejected ${issue_key}: ${neutralized_reason[$issue_key]}"$'\n'
    done
    comment_body+=$'\n'"How to restore standard auto-close:"$'\n'
    comment_body+="- Fix issue required fields/title contract (if applicable)."$'\n'
    comment_body+="- Remove or adjust \`Reopen #...\` for issues that should close now."$'\n'
    comment_body+="- Remove \`rejected\` from closure lines in PR body."
  else
    comment_body="$marker
### Closure Neutralization Status

✅ No non-compliant closure refs detected. No neutralization applied."
  fi

  neutralize_upsert_pr_comment "$repo_name" "$pr_number" "$marker" "$comment_body"

  echo "Closure neutralization evaluated for PR #${pr_number}."
}
