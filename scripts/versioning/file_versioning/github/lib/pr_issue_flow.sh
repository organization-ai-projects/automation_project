#!/usr/bin/env bash

# Compatibility entrypoint; delegated to github/pr/pr_issue_flow.sh.

source "$(cd "${BASH_SOURCE[0]%/*}/../pr" && pwd)/pr_issue_flow.sh"
