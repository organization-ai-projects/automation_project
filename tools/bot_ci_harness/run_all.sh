#!/usr/bin/env bash
set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$HARNESS_DIR/../.." && pwd)"

export GIT_TERMINAL_PROMPT=0
export GIT_PAGER=cat

SCRIPT_UNDER_TEST="${SCRIPT_UNDER_TEST:-$REPO_ROOT/scripts/versioning/file_versioning/orchestrators/read/synch_main_dev_ci.sh}"

source "$HARNESS_DIR/lib/assert.sh"
source "$HARNESS_DIR/lib/git_sandbox.sh"

VERBOSE=${VERBOSE:-0}
SINGLE_SCENARIO=""
FILTER_PATTERN=""
FAIL_FAST=0

show_usage() {
  cat << 'EOF'
Usage: ./run_all.sh [OPTIONS]

Options:
  --scenario N           Run only scenario N (01_noop_... → N=1)
  --verbose              Show git state, gh trace, timings
  --filter PATTERN       Run only scenarios matching PATTERN (regex)
  --fail-fast            Stop after the first failure
  --help                 Show this help

Examples:
  ./run_all.sh --scenario 5                    # Run only scenario 05
  ./run_all.sh --filter "conflict|timeout"     # Run conflict & timeout tests
  ./run_all.sh --verbose                       # With debug output
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --verbose)
      VERBOSE=1
      export VERBOSE=1
      shift
      ;;
    --scenario)
      SINGLE_SCENARIO="$2"
      shift 2
      ;;
    --filter)
      FILTER_PATTERN="$2"
      shift 2
      ;;
    --fail-fast)
      FAIL_FAST=1
      shift
      ;;
    --help|-h)
      show_usage
      exit 0
      ;;
    *)
      error "Unknown option: $1"
      show_usage
      exit 1
      ;;
  esac
done

assert_file_exists "$SCRIPT_UNDER_TEST"

run_one() {
  local scenario_file="$1"
  local start_time end_time duration_ms

  start_time=$(date +%s%N)

  unset SCENARIO_NAME SETUP EXPECT_EXIT
  unset MOCK_PR_EXISTS MOCK_PR_NUMBER MOCK_PR_URL
  unset MOCK_UNSTABLE_CALLS MOCK_CHECKS_STATUS MOCK_ENABLE_AUTOMERGE_FAIL
  unset MOCK_MERGE_AFTER_POLLS MOCK_MERGEABLE_OK
  unset BACKGROUND_MAIN_COMMIT_DELAY BACKGROUND_MAIN_COMMIT_MSG
  unset BACKGROUND_DEV_COMMIT_DELAY BACKGROUND_DEV_COMMIT_MSG

  # shellcheck disable=SC1090
  source "$scenario_file"

  # Check if scenario matches filter
  if [[ -n "$FILTER_PATTERN" ]] && ! echo "$SCENARIO_NAME" | grep -Eiq "$FILTER_PATTERN"; then
    [[ "$VERBOSE" == "1" ]] && info "⊘ Skipped: $SCENARIO_NAME"
    return 0
  fi

  info "▶ $SCENARIO_NAME"

  create_sandbox
  trap 'cleanup_sandbox' RETURN

  # Setup mock gh
  local mockbin="$SANDBOX/mockbin"
  mkdir -p "$mockbin"
  cp "$HARNESS_DIR/mocks/gh" "$mockbin/gh"
  chmod +x "$mockbin/gh"

  export GH_MOCK_LOG="$SANDBOX/gh_calls.log"
  export GH_MOCK_STATE_DIR="$SANDBOX/gh_state"
  export GH_MOCK_TRACE_JSON="$SANDBOX/gh_trace.jsonl"
  touch "$GH_MOCK_LOG"
  touch "$GH_MOCK_TRACE_JSON"

  # Export scenario mock vars
  export MOCK_PR_EXISTS="${MOCK_PR_EXISTS:-false}"
  export MOCK_PR_NUMBER="${MOCK_PR_NUMBER:-123}"
  export MOCK_PR_URL="${MOCK_PR_URL:-https://mock/pr/123}"
  export MOCK_UNSTABLE_CALLS="${MOCK_UNSTABLE_CALLS:-0}"
  export MOCK_CHECKS_STATUS="${MOCK_CHECKS_STATUS:-SUCCESS}"
  export MOCK_ENABLE_AUTOMERGE_FAIL="${MOCK_ENABLE_AUTOMERGE_FAIL:-false}"
  export MOCK_MERGE_AFTER_POLLS="${MOCK_MERGE_AFTER_POLLS:-1}"
  export MOCK_MERGEABLE_OK="${MOCK_MERGEABLE_OK:-MERGEABLE}"

  # Git setup
  case "${SETUP:-noop}" in
    noop) ;;
    main_ahead)
      main_add_commit "main ahead commit"
      ;;
    conflict)
      create_merge_conflict
      ;;
    *)
      fail "unknown SETUP: $SETUP"
      ;;
  esac

  # Background commits (optional)
  local bg_pids=()
  if [[ -n "${BACKGROUND_MAIN_COMMIT_DELAY:-}" ]]; then
    (
      sleep "$BACKGROUND_MAIN_COMMIT_DELAY"
      background_commit "main" "${BACKGROUND_MAIN_COMMIT_MSG:-main advances midrun}"
    ) &
    bg_pids+=($!)
  fi

  if [[ -n "${BACKGROUND_DEV_COMMIT_DELAY:-}" ]]; then
    (
      sleep "$BACKGROUND_DEV_COMMIT_DELAY"
      background_commit "dev" "${BACKGROUND_DEV_COMMIT_MSG:-dev advances midrun}"
    ) &
    bg_pids+=($!)
  fi

  # Execute script
  pushd "$WORKDIR" >/dev/null
  export PATH="$mockbin:$PATH"
  export CI="true"
  export REMOTE="origin"
  export MAIN="main"
  export DEV="dev"
  export STABLE_TIMEOUT_SECS="${STABLE_TIMEOUT_SECS:-120}"

  set +e
  bash "$SCRIPT_UNDER_TEST" 2>&1 | tee "$SANDBOX/script_output.log"
  local exit_code=$?
  set -e

  # Wait for background jobs
  for pid in "${bg_pids[@]}"; do
    wait "$pid" || true
  done

  popd >/dev/null

  # Assertions
  assert_file_exists "$GH_MOCK_LOG"

  # Validate EXPECT_EXIT values to ensure they are numeric
  if ! [[ "$EXPECT_EXIT" =~ ^[0-9]+$ ]]; then
    error "Invalid EXPECT_EXIT value: $EXPECT_EXIT. Must be numeric."
    exit 1
  fi

  # Ensure EXPECT_EXIT values are numeric and validate them
  if ! [[ "$EXPECT_EXIT" =~ ^[0-9]+$ ]]; then
    error "Invalid EXPECT_EXIT value: $EXPECT_EXIT. Must be numeric."
    exit 1
  fi

  # Compare exit_code to EXPECT_EXIT for success and failure cases
  if [[ "${EXPECT_EXIT:-0}" == "0" ]]; then
    assert_eq "$exit_code" "0" "script should succeed"
  else
    assert_ne "$exit_code" "0" "script should fail"
  fi

  local calls
  calls="$(cat "$GH_MOCK_LOG")"

  case "${SETUP:-noop}" in
    noop)
      if echo "$calls" | grep -Fq "pr create"; then
        fail "noop should not create PR"
      fi
      ;;
    main_ahead)
      assert_contains "$calls" "pr" "should call gh"
      assert_contains "$calls" "pr create" "should create PR"
      if [[ "${EXPECT_EXIT:-0}" == "0" ]]; then
        assert_contains "$calls" "pr merge" "should enable auto-merge"
      fi
      ;;
    conflict)
      if echo "$calls" | grep -Fq "pr create"; then
        fail "conflict should not create PR"
      fi
      ;;
  esac

  pushd "$WORKDIR" >/dev/null
  git fetch --prune origin >/dev/null 2>&1 || true
  case "${SETUP:-noop}" in
    main_ahead)
      assert_cmd_success git show-ref --verify --quiet refs/remotes/origin/sync/main-into-dev
      if [[ -z "${BACKGROUND_MAIN_COMMIT_DELAY:-}" ]]; then
        assert_cmd_success git merge-base --is-ancestor origin/main origin/sync/main-into-dev
      fi
      ;;
  esac
  popd >/dev/null

  # Verbose output
  if [[ "$VERBOSE" == "1" ]]; then
    info "  📊 Git state:"
    pushd "$WORKDIR" >/dev/null
    git branch -a | sed 's/^/    /'
    git log -n 2 --oneline --all 2>/dev/null | sed 's/^/    /' || true
    popd >/dev/null

    if [[ -f "$GH_MOCK_TRACE_JSON" ]] && [[ -s "$GH_MOCK_TRACE_JSON" ]]; then
      info "  🔍 GitHub CLI calls:"
      sed 's/^/    /' "$GH_MOCK_TRACE_JSON"
    fi
  fi

  end_time=$(date +%s%N)
  duration_ms=$(( (end_time - start_time) / 1000000 ))

  info "✅ $SCENARIO_NAME (${duration_ms}ms)"
  cleanup_sandbox
  trap - RETURN
}

# Ajout d'une variable pour suivre les scénarios échoués
FAILED_SCENARIOS=()

main() {
  local -a scenarios=()
  local passed=0
  local failed=0

  # Collect scenarios
  for f in "$HARNESS_DIR/scenarios/"*.env; do
    local name
    name=$(basename "$f" .env)

    if [[ -n "$FILTER_PATTERN" ]] && ! echo "$name" | grep -Eiq "$FILTER_PATTERN"; then
      continue
    fi

    if [[ -n "$SINGLE_SCENARIO" ]]; then
      local num
      num=$(echo "$name" | sed 's/^0*//' | cut -d_ -f1)
      [[ "$num" == "$SINGLE_SCENARIO" ]] || continue
    fi

    scenarios+=("$f")
  done

  if [[ ${#scenarios[@]} -eq 0 ]]; then
    info "No scenarios found"
    return 1
  fi

  info "Running ${#scenarios[@]} scenarios..."
  info ""

  for f in "${scenarios[@]}"; do
    if run_one "$f"; then
      ((passed++))
    else
      ((failed++))
      FAILED_SCENARIOS+=("$SCENARIO_NAME")
      if [[ $FAIL_FAST -eq 1 ]]; then
        info "❌ Stopping early due to --fail-fast"
        break
      fi
    fi
  done

  info ""
  info "═══════════════════════════════════════════"
  if [[ $failed -eq 0 ]]; then
    info "🎉 All $passed scenarios passed!"
  else
    info "❌ $failed failed, $passed passed"
    exit 1
  fi

  # Ajout d'une option pour arrêter au premier échec
  if [[ $FAIL_FAST -eq 1 && ${#FAILED_SCENARIOS[@]} -gt 0 ]]; then
    echo "\n[FAIL-FAST] Arrêt après le premier échec."
    exit 1
  fi

  # Ajout d'un résumé des scénarios échoués
  if [[ ${#FAILED_SCENARIOS[@]} -gt 0 ]]; then
    echo "\nRésumé des scénarios échoués :"
    for scenario in "${FAILED_SCENARIOS[@]}"; do
      echo "- $scenario"
    done
    exit 1
  else
    echo "\n🎉 Tous les scénarios ont réussi !"
    exit 0
  fi
}

main

# Define a helper function for error handling
error() {
  echo "[ERROR] $1" >&2
}
