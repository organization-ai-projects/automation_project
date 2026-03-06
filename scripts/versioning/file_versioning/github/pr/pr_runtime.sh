#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Compatibility entrypoint; delegated to pr/runtime/module.sh.

source "$(cd "${BASH_SOURCE[0]%/*}/runtime" && pwd)/module.sh"
