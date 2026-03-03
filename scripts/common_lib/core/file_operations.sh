#!/bin/bash

# File operation utilities

# Check if a file exists
file_exists() {
    local file="$1"
    [[ -f "$file" ]]
}

# Check if a directory exists
dir_exists() {
    local dir="$1"
    [[ -d "$dir" ]]
}

# Backup a file with timestamp
backup_file() {
    local file="$1"
    if [[ ! -f "$file" ]]; then
        warn "File does not exist: $file"
        return 1
    fi

    local backup="${file}.bak.$(date +%Y%m%d-%H%M%S)"
    cp "$file" "$backup"
    info "Backed up $file to $backup"
}

# Create directory if it doesn't exist
ensure_dir() {
    local dir="$1"
    if [[ ! -d "$dir" ]]; then
        mkdir -p "$dir"
        info "Created directory: $dir"
    fi
}
