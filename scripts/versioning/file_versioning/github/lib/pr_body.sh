#!/usr/bin/env bash

# Compatibility entrypoint; delegated to github/pr/body.sh.

source "$(cd "${BASH_SOURCE[0]%/*}/../pr" && pwd)/body.sh"
