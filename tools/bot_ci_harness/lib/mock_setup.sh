#!/usr/bin/env bash
# Mock setup utilities for bot_ci_harness

# Setup mock GitHub CLI environment
setup_mock_gh() {
  local sandbox="$1"
  local harness_dir="$2"
  
  local mockbin="$sandbox/mockbin"
  mkdir -p "$mockbin"
  
  # Copy and make executable
  cp "$harness_dir/mocks/gh" "$mockbin/gh"
  chmod +x "$mockbin/gh"
  
  # Setup mock environment variables
  export GH_MOCK_LOG="$sandbox/gh_calls.log"
  export GH_MOCK_STATE_DIR="$sandbox/gh_state"
  export GH_MOCK_TRACE_JSON="$sandbox/gh_trace.jsonl"
  
  # Create log files
  touch "$GH_MOCK_LOG"
  touch "$GH_MOCK_TRACE_JSON"
  
  # Add mock bin to PATH
  export PATH="$mockbin:$PATH"
}

# Export scenario mock variables with defaults
export_mock_vars() {
  export MOCK_PR_EXISTS="${MOCK_PR_EXISTS:-false}"
  export MOCK_PR_NUMBER="${MOCK_PR_NUMBER:-123}"
  export MOCK_PR_URL="${MOCK_PR_URL:-https://mock/pr/123}"
  export MOCK_UNSTABLE_CALLS="${MOCK_UNSTABLE_CALLS:-0}"
  export MOCK_CHECKS_STATUS="${MOCK_CHECKS_STATUS:-SUCCESS}"
  export MOCK_ENABLE_AUTOMERGE_FAIL="${MOCK_ENABLE_AUTOMERGE_FAIL:-false}"
  export MOCK_MERGE_AFTER_POLLS="${MOCK_MERGE_AFTER_POLLS:-1}"
  export MOCK_MERGEABLE_OK="${MOCK_MERGEABLE_OK:-MERGEABLE}"
}

# Unset all mock variables
unset_mock_vars() {
  unset MOCK_PR_EXISTS MOCK_PR_NUMBER MOCK_PR_URL
  unset MOCK_UNSTABLE_CALLS MOCK_CHECKS_STATUS MOCK_ENABLE_AUTOMERGE_FAIL
  unset MOCK_MERGE_AFTER_POLLS MOCK_MERGEABLE_OK
}

# Setup CI environment variables
setup_ci_env() {
  export CI="true"
  export REMOTE="${REMOTE:-origin}"
  export MAIN="${MAIN:-main}"
  export DEV="${DEV:-dev}"
  export STABLE_TIMEOUT_SECS="${STABLE_TIMEOUT_SECS:-120}"
  export GH_TOKEN="${GH_TOKEN:-mock_token}"
  export APP_GH_TOKEN="${APP_GH_TOKEN:-mock_token}"
}

# Unset CI environment variables
unset_ci_env() {
  unset CI REMOTE MAIN DEV STABLE_TIMEOUT_SECS
  # Don't unset GH_TOKEN and APP_GH_TOKEN as they might be real tokens
}
