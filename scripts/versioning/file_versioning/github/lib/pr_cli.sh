#!/usr/bin/env bash

# Compatibility entrypoint; delegated to lib/pr/pr_cli.sh.

source "$(cd "${BASH_SOURCE[0]%/*}/pr" && pwd)/pr_cli.sh"
