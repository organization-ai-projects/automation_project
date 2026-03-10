#!/usr/bin/env bash
# shellcheck shell=bash

# Shared helper for invoking the Rust versioning automation CLI.

va_exec() {
  if command -v va >/dev/null 2>&1; then
    va "$@"
    return $?
  fi
  if command -v versioning_automation >/dev/null 2>&1; then
    versioning_automation "$@"
    return $?
  fi
  return 127
}
