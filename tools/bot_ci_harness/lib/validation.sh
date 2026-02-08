#!/usr/bin/env bash
# Validation and error handling utilities

# Validate that a command exists
require_command() {
  local cmd="$1"
  local package="${2:-$cmd}"
  
  if ! command -v "$cmd" &> /dev/null; then
    echo "Error: Required command '$cmd' not found." >&2
    echo "Please install '$package' before running this script." >&2
    return 1
  fi
}

# Validate that a file exists
require_file() {
  local file="$1"
  local description="${2:-file}"
  
  if [[ ! -f "$file" ]]; then
    echo "Error: Required $description not found: $file" >&2
    return 1
  fi
}

# Validate that a directory exists
require_directory() {
  local dir="$1"
  local description="${2:-directory}"
  
  if [[ ! -d "$dir" ]]; then
    echo "Error: Required $description not found: $dir" >&2
    return 1
  fi
}

# Validate that an environment variable is set
require_env() {
  local var_name="$1"
  local description="${2:-$var_name}"
  
  if [[ -z "${!var_name:-}" ]]; then
    echo "Error: Required environment variable $var_name ($description) is not set." >&2
    return 1
  fi
}

# Validate that a value is numeric
validate_numeric() {
  local value="$1"
  local var_name="${2:-value}"
  
  if ! [[ "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: $var_name must be numeric, got: $value" >&2
    return 1
  fi
}

# Validate that a value is one of allowed values
validate_enum() {
  local value="$1"
  local var_name="$2"
  shift 2
  local allowed=("$@")
  
  for option in "${allowed[@]}"; do
    if [[ "$value" == "$option" ]]; then
      return 0
    fi
  done
  
  echo "Error: $var_name must be one of: ${allowed[*]}, got: $value" >&2
  return 1
}

# Validate scenario configuration
validate_scenario() {
  local scenario_file="$1"
  
  # Check file exists
  require_file "$scenario_file" "scenario file" || return 1
  
  # Source the scenario file to load variables
  # shellcheck disable=SC1090
  source "$scenario_file"
  
  # Validate required variables
  if [[ -z "${SCENARIO_NAME:-}" ]]; then
    echo "Error: SCENARIO_NAME not defined in $scenario_file" >&2
    return 1
  fi
  
  if [[ -z "${SETUP:-}" ]]; then
    echo "Error: SETUP not defined in $scenario_file" >&2
    return 1
  fi
  
  if [[ -z "${EXPECT_EXIT:-}" ]]; then
    echo "Error: EXPECT_EXIT not defined in $scenario_file" >&2
    return 1
  fi
  
  # Validate SETUP value
  validate_enum "$SETUP" "SETUP" "noop" "main_ahead" "conflict" || return 1
  
  # Validate EXPECT_EXIT is numeric
  validate_numeric "$EXPECT_EXIT" "EXPECT_EXIT" || return 1
  
  return 0
}

# Safe exit that cleans up before exiting
safe_exit() {
  local exit_code="${1:-0}"
  local cleanup_func="${2:-}"
  
  if [[ -n "$cleanup_func" ]] && declare -F "$cleanup_func" >/dev/null; then
    "$cleanup_func"
  fi
  
  exit "$exit_code"
}

# Trap errors and call cleanup
setup_error_trap() {
  local cleanup_func="$1"
  trap "safe_exit 1 $cleanup_func" ERR
  trap "safe_exit 130 $cleanup_func" INT TERM
}

# Remove error trap
remove_error_trap() {
  trap - ERR INT TERM
}
