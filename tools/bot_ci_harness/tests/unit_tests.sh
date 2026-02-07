#!/usr/bin/env bash
# Unit tests for bot_ci_harness libraries

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_DIR="$SCRIPT_DIR/../lib"

# Source libraries
source "$LIB_DIR/assert.sh"
source "$LIB_DIR/validation.sh"
source "$LIB_DIR/git_operations.sh"

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper functions
test_assert() {
  local test_name="$1"
  shift
  
  ((TESTS_RUN++)) || true
  echo -n "Testing: $test_name... "
  
  if "$@"; then
    ((TESTS_PASSED++)) || true
    echo "✓"
  else
    ((TESTS_FAILED++)) || true
    echo "✗"
  fi
}

# Reset TEST_FAILED flag before each test
reset_test_state() {
  TEST_FAILED=0
}

# ===== Tests for assert.sh =====

test_assert_eq() {
  reset_test_state
  assert_eq "foo" "foo" "should be equal"
  return $?
}

test_assert_eq_fail() {
  reset_test_state
  if assert_eq "foo" "bar" "should fail" 2>/dev/null; then
    return 1  # Test should have failed
  else
    return 0  # Expected failure
  fi
}

test_assert_ne() {
  reset_test_state
  assert_ne "foo" "bar" "should not be equal"
  return $?
}

test_assert_ne_fail() {
  reset_test_state
  if assert_ne "foo" "foo" "should fail" 2>/dev/null; then
    return 1  # Test should have failed
  else
    return 0  # Expected failure
  fi
}

test_assert_contains() {
  reset_test_state
  assert_contains "hello world" "world" "should contain"
  return $?
}

test_assert_contains_fail() {
  reset_test_state
  if assert_contains "hello world" "foo" "should fail" 2>/dev/null; then
    return 1  # Test should have failed
  else
    return 0  # Expected failure
  fi
}

# ===== Tests for validation.sh =====

test_validate_numeric() {
  if validate_numeric "123" "test" 2>/dev/null; then
    return 0
  else
    return 1
  fi
}

test_validate_numeric_fail() {
  if validate_numeric "abc" "test" 2>/dev/null; then
    return 1  # Should have failed
  else
    return 0  # Expected failure
  fi
}

test_validate_enum() {
  if validate_enum "foo" "test" "foo" "bar" "baz" 2>/dev/null; then
    return 0
  else
    return 1
  fi
}

test_validate_enum_fail() {
  if validate_enum "qux" "test" "foo" "bar" "baz" 2>/dev/null; then
    return 1  # Should have failed
  else
    return 0  # Expected failure
  fi
}

# ===== Tests for git_operations.sh =====

test_git_branches_identical() {
  # Create temporary git repo
  local tmpdir
  tmpdir=$(mktemp -d)
  pushd "$tmpdir" >/dev/null
  
  git init >/dev/null 2>&1
  git config user.name "test" >/dev/null 2>&1
  git config user.email "test@test" >/dev/null 2>&1
  echo "test" > file.txt
  git add file.txt >/dev/null 2>&1
  git commit -m "test" >/dev/null 2>&1
  
  # Test: same branch should be identical to itself
  local result
  if git_branches_identical HEAD HEAD; then
    result=0
  else
    result=1
  fi
  
  popd >/dev/null
  rm -rf "$tmpdir"
  
  return $result
}

test_git_get_sha() {
  # Create temporary git repo
  local tmpdir
  tmpdir=$(mktemp -d)
  pushd "$tmpdir" >/dev/null
  
  git init >/dev/null 2>&1
  git config user.name "test" >/dev/null 2>&1
  git config user.email "test@test" >/dev/null 2>&1
  echo "test" > file.txt
  git add file.txt >/dev/null 2>&1
  git commit -m "test" >/dev/null 2>&1
  
  # Test: get_sha should return a SHA
  local sha
  sha=$(git_get_sha HEAD)
  local result
  if [[ -n "$sha" ]] && [[ ${#sha} -eq 40 ]]; then
    result=0
  else
    result=1
  fi
  
  popd >/dev/null
  rm -rf "$tmpdir"
  
  return $result
}

# ===== Run all tests =====

main() {
  echo "Running unit tests for bot_ci_harness libraries..."
  echo ""
  
  # Assert.sh tests
  echo "Testing assert.sh..."
  test_assert "assert_eq with equal values" test_assert_eq
  test_assert "assert_eq with unequal values (should fail)" test_assert_eq_fail
  test_assert "assert_ne with unequal values" test_assert_ne
  test_assert "assert_ne with equal values (should fail)" test_assert_ne_fail
  test_assert "assert_contains with matching text" test_assert_contains
  test_assert "assert_contains with non-matching text (should fail)" test_assert_contains_fail
  echo ""
  
  # Validation.sh tests
  echo "Testing validation.sh..."
  test_assert "validate_numeric with number" test_validate_numeric
  test_assert "validate_numeric with text (should fail)" test_validate_numeric_fail
  test_assert "validate_enum with valid value" test_validate_enum
  test_assert "validate_enum with invalid value (should fail)" test_validate_enum_fail
  echo ""
  
  # Git operations tests
  echo "Testing git_operations.sh..."
  test_assert "git_branches_identical" test_git_branches_identical
  test_assert "git_get_sha" test_git_get_sha
  echo ""
  
  # Summary
  echo "═══════════════════════════════════════════"
  echo "Tests run: $TESTS_RUN"
  echo "Tests passed: $TESTS_PASSED"
  echo "Tests failed: $TESTS_FAILED"
  echo ""
  
  if [[ $TESTS_FAILED -eq 0 ]]; then
    echo "🎉 All tests passed!"
    exit 0
  else
    echo "❌ Some tests failed"
    exit 1
  fi
}

main
