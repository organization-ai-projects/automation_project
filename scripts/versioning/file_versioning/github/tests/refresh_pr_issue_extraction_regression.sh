#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"

cd "${ROOT_DIR}"

echo "Running Rust regression suite for refresh-pr issue extraction"
cargo test -q -p versioning_automation pr::tests::refresh_validation
echo "PASS [refresh-pr-issue-extraction-regression]"
