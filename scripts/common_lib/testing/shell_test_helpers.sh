#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

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
