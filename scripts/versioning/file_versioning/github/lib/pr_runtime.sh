#!/usr/bin/env bash

# Compatibility entrypoint; delegated to github/pr/pr_runtime.sh.

source "$(cd "${BASH_SOURCE[0]%/*}/../pr" && pwd)/pr_runtime.sh"
