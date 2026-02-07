# Bot CI Harness Tests

This directory contains tests for the bot_ci_harness test framework itself.

## Unit Tests

Unit tests validate the individual library functions without requiring the full test harness setup.

### Running Unit Tests

```bash
./tools/bot_ci_harness/tests/unit_tests.sh
```

### What's Tested

- **assert.sh**: All assertion functions (assert_eq, assert_ne, assert_contains, etc.)
- **validation.sh**: Input validation functions (validate_numeric, validate_enum, etc.)
- **git_operations.sh**: Git utility functions (git_branches_identical, git_get_sha, etc.)

### Adding New Unit Tests

To add a new unit test:

1. Create a test function in `unit_tests.sh` following the naming convention `test_<module>_<function>()`
2. Use the `test_assert` helper to run the test
3. Return 0 for success, 1 for failure

Example:

```bash
test_my_new_function() {
  if my_function "input"; then
    return 0
  else
    return 1
  fi
}

# In main():
test_assert "my_function with valid input" test_my_new_function
```

## Integration Tests

Integration tests are the main scenarios in the parent directory. They test the complete workflow including:

- Git repository setup
- Mock GitHub CLI
- Script execution
- Assertion validation

See `../scenarios/` for test definitions.
