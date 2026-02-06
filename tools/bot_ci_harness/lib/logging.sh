#!/usr/bin/env bash
# Structured logging library for bot_ci_harness

# Log levels
LOG_LEVEL_DEBUG=0
LOG_LEVEL_INFO=1
LOG_LEVEL_WARN=2
LOG_LEVEL_ERROR=3

# Default log level (can be overridden with LOG_LEVEL env var)
CURRENT_LOG_LEVEL=${LOG_LEVEL:-$LOG_LEVEL_INFO}

# Color codes for different log levels
COLOR_DEBUG='\033[0;36m'  # Cyan
COLOR_INFO='\033[0;32m'   # Green
COLOR_WARN='\033[1;33m'   # Yellow
COLOR_ERROR='\033[0;31m'  # Red
COLOR_RESET='\033[0m'     # Reset

# Enable/disable colors (default: enabled)
USE_COLORS=${USE_COLORS:-true}

# Get timestamp in ISO 8601 format
# Note: Uses GNU date format. For portability, falls back to second precision on non-GNU systems.
_log_timestamp() {
  # Try GNU date format first (Linux)
  if date -u +"%Y-%m-%dT%H:%M:%S.%3NZ" 2>/dev/null | grep -q "\."; then
    date -u +"%Y-%m-%dT%H:%M:%S.%3NZ"
  else
    # Fallback for macOS/BSD (no millisecond support)
    date -u +"%Y-%m-%dT%H:%M:%SZ"
  fi
}

# Internal log function
_log() {
  local level="$1"
  local level_num="$2"
  local color="$3"
  shift 3
  local message="$*"
  
  # Check if we should log this level
  if [[ $level_num -lt $CURRENT_LOG_LEVEL ]]; then
    return 0
  fi
  
  local timestamp
  timestamp=$(_log_timestamp)
  
  local prefix="[$timestamp] [$level]"
  
  if [[ "$USE_COLORS" == "true" ]]; then
    echo -e "${color}${prefix}${COLOR_RESET} ${message}" >&2
  else
    echo "${prefix} ${message}" >&2
  fi
}

# Public logging functions
log_debug() {
  _log "DEBUG" "$LOG_LEVEL_DEBUG" "$COLOR_DEBUG" "$@"
}

log_info() {
  _log "INFO" "$LOG_LEVEL_INFO" "$COLOR_INFO" "$@"
}

log_warn() {
  _log "WARN" "$LOG_LEVEL_WARN" "$COLOR_WARN" "$@"
}

log_error() {
  _log "ERROR" "$LOG_LEVEL_ERROR" "$COLOR_ERROR" "$@"
}

# Helper for logging command execution
log_command() {
  local cmd="$1"
  log_debug "Executing: $cmd"
}

# Helper for logging test events
log_test_start() {
  local test_name="$1"
  log_info "▶ Starting test: $test_name"
}

log_test_pass() {
  local test_name="$1"
  local duration="${2:-}"
  if [[ -n "$duration" ]]; then
    log_info "✅ Test passed: $test_name ($duration)"
  else
    log_info "✅ Test passed: $test_name"
  fi
}

log_test_fail() {
  local test_name="$1"
  local reason="${2:-unknown}"
  log_error "❌ Test failed: $test_name - $reason"
}

# Helper for logging scenario events
log_scenario() {
  local action="$1"
  shift
  case "$action" in
    start)
      log_info "📋 Scenario: $*"
      ;;
    setup)
      log_debug "🔧 Setup: $*"
      ;;
    cleanup)
      log_debug "🧹 Cleanup: $*"
      ;;
    *)
      log_info "$action: $*"
      ;;
  esac
}
