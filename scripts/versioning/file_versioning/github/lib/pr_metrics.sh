#!/usr/bin/env bash

# Compatibility entrypoint; delegated to github/pr/pr_metrics.sh.

source "$(cd "${BASH_SOURCE[0]%/*}/../pr" && pwd)/pr_metrics.sh"
