#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Network utilities

# Check if a URL is reachable
url_reachable() {
    local url="$1"
    local timeout="${2:-5}"

    if command -v curl >/dev/null 2>&1; then
        curl --silent --head --fail --max-time "$timeout" "$url" >/dev/null 2>&1
    elif command -v wget >/dev/null 2>&1; then
        wget --spider --quiet --timeout="$timeout" "$url" >/dev/null 2>&1
    else
        warn "Neither curl nor wget found. Cannot check URL reachability."
        return 1
    fi
}

# Download a file
download_file() {
    local url="$1"
    local output="$2"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "$output" "$url"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "$output" "$url"
    else
        die "Neither curl nor wget found. Cannot download file."
    fi
}
