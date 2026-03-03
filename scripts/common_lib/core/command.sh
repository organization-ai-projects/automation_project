#!/bin/bash

# Command utility functions

# Check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Retry a command multiple times with a delay
retry_command() {
    local retries="$1"
    local delay="$2"
    shift 2
    local count=0

    until "$@"; do
        count=$((count + 1))
        if [ "$count" -ge "$retries" ]; then
            return 1
        fi
        sleep "$delay"
    done
}

# Check if a command exists and exit if not
require_cmd() {
    local cmd="$1"
    command -v "$cmd" >/dev/null 2>&1 || die "'$cmd' not found. Please install it and try again."
}