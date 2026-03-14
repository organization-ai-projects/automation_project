#!/usr/bin/env bash
set -euo pipefail

if ! command -v versioning_automation >/dev/null 2>&1; then
	echo "Error: command 'versioning_automation' not found." >&2
	exit 127
fi

exec versioning_automation automation check-merge-conflicts "$@"
