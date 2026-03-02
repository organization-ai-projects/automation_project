#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# String manipulation utilities

# Convert string to uppercase
string_to_upper() {
    local str="$1"
    echo "$str" | tr '[:lower:]' '[:upper:]'
}

# Convert string to lowercase
string_to_lower() {
    local str="$1"
    echo "$str" | tr '[:upper:]' '[:lower:]'
}

# Trim leading/trailing whitespace
string_trim() {
    local str="$1"
    # Remove leading whitespace
    str="${str#"${str%%[![:space:]]*}"}"
    # Remove trailing whitespace
    str="${str%"${str##*[![:space:]]}"}"
    echo "$str"
}

# Check if string contains substring
string_contains() {
    local haystack="$1"
    local needle="$2"
    [[ "$haystack" == *"$needle"* ]]
}
