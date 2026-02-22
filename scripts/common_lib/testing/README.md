# Shell Test Helpers

Shared helper functions for shell-based regression tests.

## Files

- `shell_test_helpers.sh`
  - `shell_test_mktemp_dir`: cross-platform temporary directory creation
  - `shell_test_write_passthrough_jq_mock`: writes a simple `jq` passthrough mock

## Usage

```bash
# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"
```
