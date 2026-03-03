#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Logging utility functions

# Log messages with timestamp
log_message() {
    local level="$1"
    local message="$2"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [$level] $message"
}

# Info logging
info() {
    log_message "INFO" "$*"
}

# Warning logging
warn() {
    log_message "WARN" "$*" >&2
}

# Error logging and exit
die() {
    log_message "ERROR" "$*" >&2
    exit 1
}
