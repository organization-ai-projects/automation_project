#!/usr/bin/env bash

# Shared helpers for shell-based regression tests.

shell_test_mktemp_dir() {
  local prefix="${1:-shell_test_tmp}"
  mktemp -d 2>/dev/null || mktemp -d -t "$prefix"
}

shell_test_write_passthrough_jq_mock() {
  local mock_dir="${1:-}"
  cat > "${mock_dir}/jq" <<'MOCKJQ'
#!/usr/bin/env bash
set -euo pipefail
cat
MOCKJQ
  chmod +x "${mock_dir}/jq"
}
