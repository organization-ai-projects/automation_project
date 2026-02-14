#!/usr/bin/env bash

set -u

# Exit codes (stable contract for automation)
E_USAGE=2
E_DEPENDENCY=3
E_GIT=4
E_NO_DATA=5
E_PARTIAL=6

SCRIPT_PATH="./scripts/versioning/file_versioning/github/generate_pr_description.sh"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/classification.sh"
source "${SCRIPT_DIR}/lib/rendering.sh"

print_usage() {
  cat <<EOF
Usage: ${SCRIPT_PATH} [--keep-artifacts] [--debug] [--duplicate-mode MODE] [--auto-edit PR_NUMBER] MAIN_PR_NUMBER [OUTPUT_FILE]
       ${SCRIPT_PATH} --dry-run [--base BRANCH] [--head BRANCH] [--create-pr] [--allow-partial-create] [--duplicate-mode MODE] [--debug] [--auto-edit PR_NUMBER] [--yes] [OUTPUT_FILE]
       ${SCRIPT_PATH} --auto [--base BRANCH] [--head BRANCH] [--debug] [--yes]
EOF
}

print_help() {
  print_usage
  cat <<EOF

Notes:
  --dry-run       Extract PRs from local git history (base..head).
  --create-pr     In dry-run mode, attempts GitHub enrichment before creating the PR.
  --auto-edit     Generate body in memory and update an existing PR directly.
  --duplicate-mode  Duplicate handling mode: safe | auto-close.
  --debug         Print extraction/classification trace to stderr.
  --auto          RAM-first mode: dry-run + create-pr, body kept in memory.
EOF
}

usage_error() {
  local message="$1"
  echo "Erreur: ${message}" >&2
  print_usage >&2
  exit "$E_USAGE"
}

require_option_value() {
  local option_name="$1"
  local option_value="${2:-}"
  if [[ -z "$option_value" || "$option_value" == --* ]]; then
    usage_error "${option_name} requiert une valeur."
  fi
}

main_pr_number=""
output_file="pr_description.md"
keep_artifacts="false"
dry_run="false"
base_ref=""
head_ref=""
create_pr="false"
allow_partial_create="false"
assume_yes="false"
auto_mode="false"
auto_edit_pr_number=""
debug_mode="false"
duplicate_mode=""

positionals=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --keep-artifacts)
      keep_artifacts="true"
      shift
      ;;
    --dry-run)
      dry_run="true"
      shift
      ;;
    --base)
      require_option_value "--base" "${2:-}"
      base_ref="${2:-}"
      shift 2
      ;;
    --head)
      require_option_value "--head" "${2:-}"
      head_ref="${2:-}"
      shift 2
      ;;
    --create-pr)
      create_pr="true"
      shift
      ;;
    --allow-partial-create)
      allow_partial_create="true"
      shift
      ;;
    --yes)
      assume_yes="true"
      shift
      ;;
    --auto)
      auto_mode="true"
      shift
      ;;
    --auto-edit)
      require_option_value "--auto-edit" "${2:-}"
      auto_edit_pr_number="${2:-}"
      shift 2
      ;;
    --duplicate-mode)
      require_option_value "--duplicate-mode" "${2:-}"
      duplicate_mode="${2:-}"
      shift 2
      ;;
    --debug)
      debug_mode="true"
      shift
      ;;
    -h|--help)
      print_help
      exit 0
      ;;
    *)
      positionals+=("$1")
      shift
      ;;
  esac
done

debug_log() {
  if [[ "$debug_mode" == "true" ]]; then
    echo "[debug] $*" >&2
  fi
}

if [[ "$auto_mode" == "true" ]]; then
  dry_run="true"
  create_pr="true"
  if [[ ${#positionals[@]} -gt 0 ]]; then
    usage_error "--auto ne prend pas d'OUTPUT_FILE positional."
  fi
fi

if [[ -n "$auto_edit_pr_number" ]] && [[ ! "$auto_edit_pr_number" =~ ^[0-9]+$ ]]; then
  usage_error "--auto-edit requiert un PR_NUMBER numérique."
fi

if [[ -n "$duplicate_mode" ]] && [[ "$duplicate_mode" != "safe" && "$duplicate_mode" != "auto-close" ]]; then
  usage_error "--duplicate-mode doit être 'safe' ou 'auto-close'."
fi

if [[ "$create_pr" == "true" && "$dry_run" != "true" ]]; then
  usage_error "--create-pr est uniquement supporté avec --dry-run."
fi

if [[ "$allow_partial_create" == "true" && "$create_pr" != "true" ]]; then
  usage_error "--allow-partial-create nécessite --create-pr."
fi

if [[ -n "$auto_edit_pr_number" && "$create_pr" == "true" ]]; then
  usage_error "--auto-edit ne peut pas être combiné avec --create-pr/--auto."
fi

if [[ "$dry_run" == "false" ]]; then
  if [[ -n "$auto_edit_pr_number" && ${#positionals[@]} -gt 1 ]]; then
    usage_error "En mode --auto-edit (MAIN_PR_NUMBER), OUTPUT_FILE positional n'est pas autorisé."
  fi
  if [[ -z "$auto_edit_pr_number" && ${#positionals[@]} -gt 2 ]]; then
    usage_error "Trop d'arguments positionnels. Utilisation attendue: MAIN_PR_NUMBER [OUTPUT_FILE]."
  fi
  if [[ ${#positionals[@]} -ge 1 ]]; then
    main_pr_number="${positionals[0]}"
  fi
  if [[ -z "$auto_edit_pr_number" && ${#positionals[@]} -ge 2 ]]; then
    output_file="${positionals[1]}"
  fi
  if [[ -z "$main_pr_number" ]]; then
    usage_error "MAIN_PR_NUMBER est requis."
  fi
else
  if [[ -n "$auto_edit_pr_number" && "$auto_mode" != "true" && ${#positionals[@]} -gt 0 ]]; then
    usage_error "En mode --auto-edit (dry-run), OUTPUT_FILE positional n'est pas autorisé."
  fi
  if [[ -z "$auto_edit_pr_number" && "$auto_mode" != "true" && ${#positionals[@]} -gt 1 ]]; then
    usage_error "Trop d'arguments positionnels pour --dry-run. Seul OUTPUT_FILE est autorisé."
  fi
  if [[ -z "$auto_edit_pr_number" && "$auto_mode" != "true" && ${#positionals[@]} -ge 1 ]]; then
    output_file="${positionals[0]}"
  fi
fi

if [[ "$keep_artifacts" == "true" ]]; then
  extracted_prs_file="extracted_prs.txt"
  resolved_issues_file="resolved_issues.txt"
else
  extracted_prs_file="$(mktemp)"
  resolved_issues_file="$(mktemp)"
fi

features_tmp="$(mktemp)"
bugs_tmp="$(mktemp)"
refactors_tmp="$(mktemp)"
sync_tmp="$(mktemp)"
issues_tmp="$(mktemp)"
declare -A pr_title_hint
online_enrich="false"
pr_enrich_failed=0
breaking_detected=0
pr_created_successfully="false"

is_human_interactive_terminal() {
  [[ -t 0 && -t 1 && -z "${CI:-}" ]]
}

cleanup() {
  rm -f "$features_tmp" "$bugs_tmp" "$refactors_tmp" "$sync_tmp" "$issues_tmp"
  if [[ "$keep_artifacts" != "true" ]]; then
    rm -f "$extracted_prs_file" "$resolved_issues_file"
  fi
}
trap cleanup EXIT

has_gh="false"
if command -v gh >/dev/null 2>&1; then
  has_gh="true"
fi

need_gh="false"
if [[ "$dry_run" == "false" || "$create_pr" == "true" || -n "$auto_edit_pr_number" ]]; then
  need_gh="true"
fi
if [[ -n "$duplicate_mode" && "$dry_run" != "true" ]]; then
  need_gh="true"
fi
if [[ "$need_gh" == "true" && "$has_gh" != "true" ]]; then
  echo "Erreur: la commande 'gh' est introuvable." >&2
  exit "$E_DEPENDENCY"
fi

need_jq="false"
if [[ "$dry_run" == "false" || "$create_pr" == "true" ]]; then
  need_jq="true"
fi
if [[ "$need_jq" == "true" ]] && ! command -v jq >/dev/null 2>&1; then
  echo "Erreur: la commande 'jq' est introuvable." >&2
  exit "$E_DEPENDENCY"
fi

if [[ "$dry_run" == "true" ]]; then
  if ! command -v git >/dev/null 2>&1; then
    echo "Erreur: la commande 'git' est introuvable." >&2
    exit "$E_GIT"
  fi
  if [[ -z "$head_ref" ]]; then
    head_ref="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || true)"
  fi
  if [[ -z "$base_ref" ]]; then
    base_ref="dev"
  fi
  if [[ -z "$head_ref" ]]; then
    echo "Erreur: impossible de déterminer la branche head en mode --dry-run." >&2
    exit "$E_GIT"
  fi
else
  base_ref="$(gh pr view "$main_pr_number" --json baseRefName -q '.baseRefName' 2>/dev/null || echo "main")"
  head_ref="$(gh pr view "$main_pr_number" --json headRefName -q '.headRefName' 2>/dev/null || echo "dev")"
fi

if [[ "$dry_run" == "true" && "$create_pr" == "true" ]]; then
  online_enrich="true"
fi

if [[ "$has_gh" != "true" && "$dry_run" == "true" && "$create_pr" != "true" ]]; then
  debug_log "Running pure local dry-run without gh dependency."
fi

extract_child_prs() {
  local commit_headlines
  local main_pr_body
  local main_pr_comments
  local repo_owner_name
  local timeline_pr_refs

  repo_owner_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"

  # IMPORTANT: gh pr view --json commits is often capped (e.g. 100 items).
  # Use REST API with --paginate to collect all commit first lines.
  commit_headlines=""
  if [[ -n "$repo_owner_name" ]]; then
    commit_headlines="$(gh api "repos/${repo_owner_name}/pulls/${main_pr_number}/commits" --paginate \
      --jq '.[].commit.message | split("\n")[0]' 2>/dev/null || true)"
  fi

  main_pr_body="$(gh pr view "$main_pr_number" --json body -q '.body' 2>/dev/null || true)"
  main_pr_comments="$(gh pr view "$main_pr_number" --json comments -q '.comments[].body' 2>/dev/null || true)"
  timeline_pr_refs=""
  if [[ -n "$repo_owner_name" ]]; then
    timeline_pr_refs="$(gh api "repos/${repo_owner_name}/issues/${main_pr_number}/timeline" --paginate \
      --jq '.[] | select(.event=="cross-referenced") | select(.source.issue.pull_request.url != null) | .source.issue.number' 2>/dev/null \
      | sed -nE 's/^([0-9]+)$/#\1/p' || true)"
  fi

  if [[ -z "$commit_headlines" && -z "$main_pr_body" && -z "$main_pr_comments" && -z "$timeline_pr_refs" ]]; then
    return 1
  fi

  {
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
    # Capture PR references from main PR body/comments (e.g. /pull/306 or "PR #306")
    echo "$main_pr_body" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$main_pr_body" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$main_pr_body" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
    echo "$main_pr_comments" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$main_pr_comments" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$main_pr_comments" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
    echo "$timeline_pr_refs"
  } | grep -E '^#[0-9]+$' | sort -u | grep -v "^#${main_pr_number}$" > "$extracted_prs_file"
  debug_log "extract_child_prs(main=#${main_pr_number}) => $(tr '\n' ' ' < "$extracted_prs_file")"

  return 0
}

extract_child_prs_dry() {
  local commit_headlines
  local message
  commit_headlines="$(git log --oneline "${base_ref}..${head_ref}" 2>/dev/null || true)"
  if [[ -z "$commit_headlines" ]]; then
    debug_log "extract_child_prs_dry(${base_ref}..${head_ref}) => no commits found"
    return 1
  fi

  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    message="$(echo "$line" | cut -d' ' -f2-)"
    if [[ "$message" =~ Merge\ pull\ request\ \#([0-9]+) ]]; then
      pr_title_hint["#${BASH_REMATCH[1]}"]="$message"
    elif [[ "$message" =~ \(\#([0-9]+)\)[[:space:]]*$ ]]; then
      pr_title_hint["#${BASH_REMATCH[1]}"]="$message"
    fi
  done <<< "$commit_headlines"

  {
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
  } | sort -u > "$extracted_prs_file"
  debug_log "extract_child_prs_dry(${base_ref}..${head_ref}) => $(tr '\n' ' ' < "$extracted_prs_file")"

  return 0
}

issue_labels() {
  local issue_number="$1"
  local repo_name_with_owner

  if [[ "$has_gh" != "true" ]]; then
    debug_log "issue_labels(#${issue_number}): gh unavailable, fallback empty labels."
    echo ""
    return
  fi

  repo_name_with_owner="$(get_repo_name_with_owner)"

  if [[ -n "$repo_name_with_owner" ]]; then
    gh issue view "$issue_number" -R "$repo_name_with_owner" --json labels \
      -q '.labels | map(.name) | join("||")' 2>/dev/null || true
    return
  fi

  gh issue view "$issue_number" --json labels \
    -q '.labels | map(.name) | join("||")' 2>/dev/null || true
}

parse_issue_refs_from_body() {
  local body="$1"
  echo "$body" | awk '
    BEGIN { IGNORECASE = 1 }
    {
      line = $0
      lower = tolower($0)
      action = ""
      if (match(lower, /(closes?|closed|fixes?|fixed|resolves?|resolved)/)) {
        token = substr(lower, RSTART, RLENGTH)
        if (token ~ /^clos/) {
          action = "Closes"
        } else if (token ~ /^fix/) {
          action = "Fixes"
        } else {
          action = "Resolves"
        }

        while (match(line, /([[:alnum:]_.-]+\/)?#[0-9]+/)) {
          issue_ref = substr(line, RSTART, RLENGTH)
          sub(/^[[:alnum:]_.-]+\//, "", issue_ref)
          print action "|" issue_ref
          line = substr(line, RSTART + RLENGTH)
        }
      }
    }
  ' \
    | sort -u
}

parse_duplicate_refs_from_text() {
  local body="$1"
  echo "$body" | awk '
    BEGIN { IGNORECASE = 1 }
    {
      line = $0
      while (match(line, /#([0-9]+)[[:space:]]+duplicate[[:space:]]+of[[:space:]]+#([0-9]+)/)) {
        matched = substr(line, RSTART, RLENGTH)
        gsub(/[^0-9]+/, " ", matched)
        split(matched, nums, " ")
        if (nums[1] != "" && nums[2] != "") {
          print "#" nums[1] "|" "#" nums[2]
        }
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' | sort -u
}

text_indicates_breaking() {
  local text="${1:-}"
  local line
  local lower
  # Conventional commit breaking marker, generic type support:
  # type!: ... OR type(scope)!: ...
  local cc_breaking_re='^[[:space:]]*[a-z][a-z0-9_-]*(\([a-z0-9_./,-]+\))?!:[[:space:]]+'

  while IFS= read -r line; do
    lower="$(echo "$line" | tr '[:upper:]' '[:lower:]')"

    # Explicitly ignore "non-breaking change" phrasing.
    if [[ "$lower" =~ non[[:space:]-]?breaking[[:space:]_-]*change ]]; then
      continue
    fi

    # Explicit checklist signal in generated/template PR bodies.
    if [[ "$lower" =~ ^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*breaking[[:space:]_-]*change([[:space:]]|$) ]]; then
      return 0
    fi

    # Conventional BREAKING CHANGE footer signal.
    if [[ "$lower" =~ ^[[:space:]]*breaking[[:space:]_-]*change[[:space:]]*: ]]; then
      return 0
    fi

    # Conventional commit header with breaking marker.
    if [[ "$lower" =~ $cc_breaking_re ]]; then
      return 0
    fi
  done <<< "$text"

  return 1
}

normalize_issue_key() {
  local raw="${1:-}"
  local normalized

  normalized="$(echo "$raw" | sed -nE 's/.*#([0-9]+).*/#\1/p')"
  if [[ "$normalized" =~ ^#[0-9]+$ ]]; then
    echo "$normalized"
    return 0
  fi

  return 1
}

echo -n > "$extracted_prs_file"
echo -n > "$resolved_issues_file"

if [[ "$dry_run" == "true" ]]; then
  if ! extract_child_prs_dry; then
    echo "Avertissement: impossible d'extraire des PR depuis ${base_ref}..${head_ref}." >&2
  fi
else
  if ! extract_child_prs; then
    echo "Avertissement: impossible de récupérer les commits de la PR #${main_pr_number} (API indisponible ou PR introuvable)." >&2
  fi
fi

declare -A seen_issue
declare -A issue_category
declare -A issue_action
declare -A pr_ref_cache
declare -A duplicate_targets
repo_name_with_owner_cache=""
pr_count=0
issue_count=0

get_repo_name_with_owner() {
  if [[ "$has_gh" != "true" ]]; then
    echo ""
    return
  fi

  if [[ -n "$repo_name_with_owner_cache" ]]; then
    echo "$repo_name_with_owner_cache"
    return
  fi

  if [[ -n "${GH_REPO:-}" ]]; then
    repo_name_with_owner_cache="$GH_REPO"
    echo "$repo_name_with_owner_cache"
    return
  fi

  repo_name_with_owner_cache="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
  echo "$repo_name_with_owner_cache"
}

is_pull_request_ref() {
  local issue_number="$1"
  local cache_key="#${issue_number}"
  local repo_name_with_owner

  if [[ -n "${pr_ref_cache[$cache_key]:-}" ]]; then
    [[ "${pr_ref_cache[$cache_key]}" == "1" ]]
    return
  fi

  if [[ "$has_gh" != "true" ]]; then
    debug_log "is_pull_request_ref(#${issue_number}): gh unavailable, assume issue."
    pr_ref_cache["$cache_key"]="0"
    return 1
  fi

  repo_name_with_owner="$(get_repo_name_with_owner)"

  # Use REST pulls/{number}: reliable 404 for non-PR issue numbers.
  if [[ -n "$repo_name_with_owner" ]] \
    && gh api "repos/${repo_name_with_owner}/pulls/${issue_number}" >/dev/null 2>&1; then
    pr_ref_cache["$cache_key"]="1"
    return 0
  fi

  # Fallback for contexts where nameWithOwner cannot be resolved.
  if [[ -z "$repo_name_with_owner" ]] \
    && gh pr view "${issue_number}" >/dev/null 2>&1; then
    pr_ref_cache["$cache_key"]="1"
    return 0
  fi

  pr_ref_cache["$cache_key"]="0"
  return 1
}

add_issue_entry() {
  local action="$1"
  local issue_key="$2"
  local category="${3:-Unknown}"
  local normalized_issue_key
  local issue_number
  local labels_raw label_category
  local final_category
  local normalized_action

  if ! normalized_issue_key="$(normalize_issue_key "$issue_key")"; then
    return
  fi
  issue_key="$normalized_issue_key"
  issue_number="${issue_key//#/}"

  # Issues Resolved must only contain GitHub issues, not PR references.
  if is_pull_request_ref "$issue_number"; then
    debug_log "discard_issue_ref_as_pr: ${issue_key}"
    return
  fi

  if [[ -n "${seen_issue[$issue_key]:-}" ]]; then
    return
  fi
  seen_issue["$issue_key"]=1

  labels_raw="$(issue_labels "$issue_number")"
  label_category="$(issue_category_from_labels "$labels_raw")"
  if [[ "$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')" =~ (^|\|\|)breaking(\|\||$) ]]; then
    breaking_detected=1
  fi

  final_category="$label_category"
  if [[ "$final_category" == "Unknown" && "$category" != "Unknown" ]]; then
    final_category="$category"
  fi

  issue_category["$issue_key"]="$final_category"
  normalized_action="$(normalize_issue_action "$action" "$final_category")"
  issue_action["$issue_key"]="$normalized_action"
  debug_log "issue_entry: key=${issue_key} action=${normalized_action} category=${final_category}"
}

add_duplicate_entry() {
  local duplicate_issue_key_raw="$1"
  local canonical_issue_key_raw="$2"
  local duplicate_issue_key
  local canonical_issue_key

  if ! duplicate_issue_key="$(normalize_issue_key "$duplicate_issue_key_raw")"; then
    return
  fi
  if ! canonical_issue_key="$(normalize_issue_key "$canonical_issue_key_raw")"; then
    return
  fi
  if [[ "$duplicate_issue_key" == "$canonical_issue_key" ]]; then
    return
  fi

  duplicate_targets["$duplicate_issue_key"]="$canonical_issue_key"
  debug_log "duplicate_entry: ${duplicate_issue_key} -> ${canonical_issue_key}"
}

if [[ -s "$extracted_prs_file" ]]; then
  while read -r pr_ref; do
    [[ -z "$pr_ref" ]] && continue
    pr_number="${pr_ref//#/}"
    pr_view_json=""
    pr_labels_raw=""

    if [[ "$dry_run" == "true" && "$online_enrich" != "true" ]]; then
      pr_title="${pr_title_hint[$pr_ref]:-PR #${pr_number}}"
      pr_body=""
    else
      pr_view_json="$(gh pr view "$pr_number" --json title,body,labels 2>/dev/null || true)"
      if [[ -n "$pr_view_json" ]]; then
        pr_title="$(echo "$pr_view_json" | jq -r '.title // ""')"
        pr_body="$(echo "$pr_view_json" | jq -r '.body // ""')"
        pr_labels_raw="$(echo "$pr_view_json" | jq -r '.labels // [] | map(.name) | join("||")')"
        if [[ "$(echo "$pr_labels_raw" | tr '[:upper:]' '[:lower:]')" =~ (^|\|\|)breaking(\|\||$) ]]; then
          breaking_detected=1
        fi
      else
        pr_title=""
        pr_body=""
        if [[ "$online_enrich" == "true" ]]; then
          pr_enrich_failed=$((pr_enrich_failed + 1))
          debug_log "enrich_fallback: failed to read PR ${pr_ref} via gh pr view"
        fi
      fi
    fi

    if [[ -z "$pr_title" ]]; then
      pr_title="${pr_title_hint[$pr_ref]:-PR #${pr_number}}"
    fi
    if text_indicates_breaking "$pr_title"; then
      breaking_detected=1
    fi

    pr_category="$(classify_pr "$pr_ref" "$pr_title")"
    pr_count=$((pr_count + 1))

    if [[ -n "$pr_body" ]]; then
      if text_indicates_breaking "$pr_body"; then
        breaking_detected=1
      fi
      while IFS='|' read -r action issue_key; do
        debug_log "parsed_issue_ref(pr ${pr_ref}): ${action}|${issue_key}"
        add_issue_entry "$action" "$issue_key" "$pr_category"
      done < <(parse_issue_refs_from_body "$pr_body")
      while IFS='|' read -r duplicate_issue canonical_issue; do
        add_duplicate_entry "$duplicate_issue" "$canonical_issue"
      done < <(parse_duplicate_refs_from_text "$pr_body")
    fi

    :
  done < "$extracted_prs_file"
fi

if [[ "$dry_run" == "true" ]]; then
  # In branch dry-run mode, also parse issue refs from commit messages/footers
  # (e.g. "Closes #123") so issue detection works without child PR references.
  dry_commit_messages="$(git log --format=%B "${base_ref}..${head_ref}" 2>/dev/null || true)"
  if [[ -n "$dry_commit_messages" ]]; then
    if text_indicates_breaking "$dry_commit_messages"; then
      breaking_detected=1
    fi
    while IFS='|' read -r action issue_key; do
      debug_log "parsed_issue_ref(dry commits): ${action}|${issue_key}"
      add_issue_entry "$action" "$issue_key" "Mixed"
    done < <(parse_issue_refs_from_body "$dry_commit_messages")
    while IFS='|' read -r duplicate_issue canonical_issue; do
      add_duplicate_entry "$duplicate_issue" "$canonical_issue"
    done < <(parse_duplicate_refs_from_text "$dry_commit_messages")
  fi
fi

if [[ "$dry_run" == "false" ]]; then
  # Also include issues closed directly by the main PR itself.
  main_pr_body="$(gh pr view "$main_pr_number" --json body -q '.body' 2>/dev/null || true)"
  if [[ -n "$main_pr_body" ]]; then
    if text_indicates_breaking "$main_pr_body"; then
      breaking_detected=1
    fi
    while IFS='|' read -r action issue_key; do
      debug_log "parsed_issue_ref(main pr): ${action}|${issue_key}"
      add_issue_entry "$action" "$issue_key" "Mixed"
    done < <(parse_issue_refs_from_body "$main_pr_body")
    while IFS='|' read -r duplicate_issue canonical_issue; do
      add_duplicate_entry "$duplicate_issue" "$canonical_issue"
    done < <(parse_duplicate_refs_from_text "$main_pr_body")
  fi

fi

echo -n > "$issues_tmp"
for issue_key in "${!seen_issue[@]}"; do
  issue_number="${issue_key//#/}"
  echo "${issue_number}|${issue_category[$issue_key]}|${issue_action[$issue_key]}|${issue_key}" >> "$issues_tmp"
done

if [[ -s "$issues_tmp" ]]; then
  sort -t'|' -k1,1n "$issues_tmp" \
    | awk -F'|' '
      BEGIN {
        cats[1] = "Security"
        cats[2] = "Features"
        cats[3] = "Bug Fixes"
        cats[4] = "Refactoring"
        cats[5] = "Automation"
        cats[6] = "Testing"
        cats[7] = "Docs"
        cats[8] = "Mixed"
        cats[9] = "Unknown"
      }
      {
        lines[NR] = $0
      }
      END {
        for (c = 1; c <= 9; c++) {
          cat = cats[c]
          found = 0
          for (i = 1; i <= NR; i++) {
            split(lines[i], parts, "|")
            if (parts[2] == cat) {
              if (!found) {
                print "#### " cat
                found = 1
              }
              print "- " parts[3] " " parts[4]
            }
          }
          if (found) {
            print ""
          }
        }
      }
    ' > "$resolved_issues_file"
  issue_count="${#seen_issue[@]}"
fi

process_duplicate_mode() {
  local duplicate_issue_key
  local canonical_issue_key
  local duplicate_issue_number
  local repo_name_with_owner
  local comment_body

  if [[ -z "$duplicate_mode" ]]; then
    return
  fi

  local duplicate_count=0
  for duplicate_issue_key in "${!duplicate_targets[@]}"; do
    duplicate_count=$((duplicate_count + 1))
  done

  if [[ "$duplicate_count" -eq 0 ]]; then
    echo "Duplicate mode (${duplicate_mode}): no duplicate declarations detected."
    return
  fi

  echo "Duplicate mode (${duplicate_mode}): ${duplicate_count} duplicate declaration(s) detected."

  if [[ "$dry_run" == "true" ]]; then
    for duplicate_issue_key in "${!duplicate_targets[@]}"; do
      canonical_issue_key="${duplicate_targets[$duplicate_issue_key]}"
      echo "Duplicate mode (${duplicate_mode}) [dry-run]: ${duplicate_issue_key} -> ${canonical_issue_key}"
    done
    return
  fi

  if [[ "$has_gh" != "true" ]]; then
    echo "Erreur: --duplicate-mode requiert gh en mode non dry-run." >&2
    exit "$E_DEPENDENCY"
  fi

  repo_name_with_owner="$(get_repo_name_with_owner)"
  if [[ -z "$repo_name_with_owner" ]]; then
    echo "Erreur: impossible de déterminer le dépôt GitHub pour --duplicate-mode." >&2
    exit "$E_DEPENDENCY"
  fi

  for duplicate_issue_key in "${!duplicate_targets[@]}"; do
    canonical_issue_key="${duplicate_targets[$duplicate_issue_key]}"
    duplicate_issue_number="${duplicate_issue_key//#/}"

    if [[ "$duplicate_mode" == "safe" ]]; then
      comment_body="Potential duplicate detected by PR generation workflow: ${duplicate_issue_key} may duplicate ${canonical_issue_key}. Please review manually."
    else
      # GitHub recognizes this phrase for duplicate intent.
      comment_body="Duplicate of ${canonical_issue_key}"
    fi

    gh api "repos/${repo_name_with_owner}/issues/${duplicate_issue_number}/comments" \
      -f body="${comment_body}" >/dev/null
    echo "Duplicate mode (${duplicate_mode}): commented on ${duplicate_issue_key} (target ${canonical_issue_key})."

    if [[ "$duplicate_mode" == "auto-close" ]]; then
      gh api -X PATCH "repos/${repo_name_with_owner}/issues/${duplicate_issue_number}" \
        -f state="closed" -f state_reason="not_planned" >/dev/null
      echo "Duplicate mode (${duplicate_mode}): closed ${duplicate_issue_key}."
    fi
  done
}

body_content="$({
  echo "### Description"
  echo "This pull request merges the \`${head_ref}\` branch into \`${base_ref}\` and summarizes merged pull requests and resolved issues."
  echo ""
  echo "### Scope"
  echo "- Not explicitly provided."
  echo ""
  echo "### Compatibility"
  if [[ "$breaking_detected" -eq 1 ]]; then
    echo "- Breaking change."
  else
    echo "- Non-breaking change."
  fi
  echo ""
  echo "### Issues Resolved"
  echo "This PR resolves the following issues:"
  if [[ -s "$resolved_issues_file" ]]; then
    cat "$resolved_issues_file"
  else
    echo "- No resolved issues detected via GitHub references or PR body keywords."
  fi
  echo ""
  echo "### Key Changes"
  if [[ -s "$sync_tmp" ]]; then
    echo "#### Synchronization"
    write_section_from_file "$sync_tmp"
    echo ""
  fi
  echo "#### Features"
  write_section_from_file "$features_tmp"
  echo ""
  echo "#### Bug Fixes"
  write_section_from_file "$bugs_tmp"
  echo ""
  echo "#### Refactoring"
  write_section_from_file "$refactors_tmp"
  echo ""
  cat <<EOF
### Testing
- Ensure all project tests are executed before merge (for example: \`cargo test\`, script-specific checks, and CI workflow validation).
- Validate manually the automation workflows impacted by merged PRs.

### Validation Checklist
- [ ] Tests have been added or updated, and all tests pass.
- [ ] Documentation has been updated as needed.
- [ ] Breaking changes (if any) are clearly documented above.

### Additional Notes
- Documentation and PR summaries should be aligned with the resolved issues listed above.
- This generated description can be edited to add domain-specific details before submission.
EOF
})"

if [[ "$auto_mode" != "true" && -z "$auto_edit_pr_number" ]]; then
  printf "%s\n" "$body_content" > "$output_file"
  echo "Fichier généré: $output_file"
else
  if [[ "$auto_mode" == "true" ]]; then
    echo "Description PR générée en mémoire (mode --auto)."
  else
    echo "Description PR générée en mémoire (mode --auto-edit)."
  fi
fi
if [[ "$keep_artifacts" == "true" ]]; then
  echo "PR extraites: $extracted_prs_file"
  echo "Issues résolues: $resolved_issues_file"
fi

process_duplicate_mode

if [[ "$create_pr" == "true" ]]; then
  if [[ "$online_enrich" == "true" && "$pr_enrich_failed" -gt 0 && "$allow_partial_create" != "true" ]]; then
    echo "Erreur: enrichissement GitHub partiel (${pr_enrich_failed} PR non lues)." >&2
    echo "Le body peut être incomplet. Corrige le réseau/auth puis relance, ou utilise --allow-partial-create." >&2
    exit "$E_PARTIAL"
  fi

  default_title="$(build_dynamic_pr_title)"
  create_now="false"

  if [[ "$assume_yes" == "true" ]]; then
    create_now="true"
  elif [[ "$auto_mode" == "true" ]] && ! is_human_interactive_terminal; then
    # In non-interactive contexts (agent/CI), --auto should not block on prompt.
    create_now="true"
  else
    echo
    echo "Dry-run complete."
    echo "Base: ${base_ref}"
    echo "Head: ${head_ref}"
    if [[ "$auto_mode" != "true" ]]; then
      echo "Body file: ${output_file}"
    else
      echo "Body: in-memory"
    fi
    read -r -p "Create PR now with generated body? [y/N] " answer
    case "$answer" in
      y|Y|yes|YES)
        create_now="true"
        ;;
    esac
  fi

  if [[ "$create_now" == "true" ]]; then
    if [[ "$auto_mode" == "true" ]]; then
      pr_url="$(gh pr create --base "$base_ref" --head "$head_ref" --title "$default_title" --body "$body_content")"
    else
      pr_url="$(gh pr create --base "$base_ref" --head "$head_ref" --title "$default_title" --body-file "$output_file")"
    fi
    pr_created_successfully="true"
    echo "PR créée: $pr_url"
  else
    echo "PR creation skipped."
  fi
fi

if [[ -n "$auto_edit_pr_number" ]]; then
  update_now="false"

  if [[ "$assume_yes" == "true" ]]; then
    update_now="true"
  elif ! is_human_interactive_terminal; then
    usage_error "--auto-edit nécessite --yes en contexte non interactif."
  else
    echo
    echo "Body generated for update."
    read -r -p "Update PR #${auto_edit_pr_number} now with generated body? [y/N] " answer
    case "$answer" in
      y|Y|yes|YES)
        update_now="true"
        ;;
    esac
  fi

  if [[ "$update_now" == "true" ]]; then
    repo_name_with_owner="$(get_repo_name_with_owner)"
    if [[ -z "$repo_name_with_owner" ]]; then
      echo "Erreur: impossible de déterminer le dépôt GitHub pour --auto-edit." >&2
      exit "$E_DEPENDENCY"
    fi
    gh api -X PATCH "repos/${repo_name_with_owner}/pulls/${auto_edit_pr_number}" \
      --raw-field body="$body_content" >/dev/null
    echo "PR mise à jour: #${auto_edit_pr_number}"
  else
    echo "PR update skipped."
  fi
fi

# Non-fatal generation outcome for humans, but explicit signal for automation.
if [[ "$dry_run" == "true" && "$create_pr" == "true" && "$pr_created_successfully" != "true" && ! -s "$extracted_prs_file" ]]; then
  exit "$E_NO_DATA"
fi
