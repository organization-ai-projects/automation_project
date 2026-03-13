#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"

cd "${ROOT_DIR}"

echo "Running Rust regression suite for PR description generation"
cargo test -q -p versioning_automation pr::tests::generate_description
echo "PASS [generate-pr-description-regression]"
