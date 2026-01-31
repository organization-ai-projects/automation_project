#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "❌ ASSERT FAIL: $*" >&2
  exit 1
}

assert_eq() {
  local got="$1"
  local expected="$2"
  local msg="${3:-}"
  [[ "$got" == "$expected" ]] || fail "${msg} (got='$got', expected='$expected')"
}

assert_contains() {
  local hay="$1"
  local needle="$2"
  local msg="${3:-}"
  echo "$hay" | grep -Fq "$needle" || fail "${msg} (missing='$needle')"
}

assert_file_exists() {
  local path="$1"
  [[ -f "$path" ]] || fail "file does not exist: $path"
}

assert_cmd_success() {
  "$@" >/dev/null 2>&1 || fail "command failed: $*"
}

assert_cmd_fail() {
  if "$@" >/dev/null 2>&1; then
    fail "command should have failed but succeeded: $*"
  fi
}

info() {
  echo "ℹ️  $*"
}
