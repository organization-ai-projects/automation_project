#!/usr/bin/env bash
set -euo pipefail

# Correct the HARNESS_DIR path resolution
HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCENARIOS_DIR="$HARNESS_DIR/scenarios"

# Ensure the scenarios directory exists
mkdir -p "$SCENARIOS_DIR"

# Color helpers
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

prompt() {
  local var_name="$1"
  local description="$2"
  local default="${3:-}"
  local -n result=$var_name

  if [[ -n "$default" ]]; then
    echo -ne "${YELLOW}$description${NC} [${GREEN}$default${NC}]: "
  else
    echo -ne "${YELLOW}$description${NC}: "
  fi

  read -r result
  if [[ -z "$result" ]] && [[ -n "$default" ]]; then
    result="$default"
  fi
}

# Update prompt_yn to loop until valid input
prompt_yn() {
  local var_name="$1"
  local description="$2"
  local default="${3:-n}"
  local -n result=$var_name

  while true; do
    if [[ "$default" == "y" ]]; then
      echo -ne "${YELLOW}$description${NC} [${GREEN}Y${NC}/n]: "
    else
      echo -ne "${YELLOW}$description${NC} [y/${GREEN}N${NC}]: "
    fi

    read -r ans
    case "${ans:-$default}" in
      y|Y) result="true"; break ;;
      n|N) result="false"; break ;;
      *) echo "Invalid input. Please enter 'y' or 'n'." ;;
    esac
  done
}

prompt_choice() {
  local var_name="$1"
  local description="$2"
  local -n options=$3
  local default="${4:-}"
  local -n result=$var_name

  echo -e "${YELLOW}$description${NC}:"
  local i=0
  for opt in "${options[@]}"; do
    echo "  $((i+1)). $opt"
    ((i++))
  done

  if [[ -n "$default" ]]; then
    echo -ne "${YELLOW}Choose [${GREEN}$default${NC}]: "
  else
    echo -ne "${YELLOW}Choose: "
  fi

  read -r choice
  choice="${choice:-$default}"
  if [[ -n "$choice" ]] && [[ "$choice" =~ ^[0-9]+$ ]] && [[ $choice -le ${#options[@]} ]]; then
    result="${options[$((choice-1))]}"
  else
    result="$default"
  fi
}

# Ensure nullglob is enabled to avoid errors when no .env files exist
shopt -s nullglob

main() {
  echo -e "${GREEN}=== Bot CI Harness: New Scenario Generator ===${NC}\n"

  # Find next scenario number
  local next_num=1
  if [[ -d "$SCENARIOS_DIR" ]]; then
    for f in "$SCENARIOS_DIR"/*.env; do
      if [[ -f "$f" ]]; then
        local num
        num=$(basename "$f" .env | sed 's/^0*//' | cut -d_ -f1)
        if [[ -n "$num" ]] && [[ "$num" =~ ^[0-9]+$ ]]; then
          if [[ $num -ge $next_num ]]; then
            next_num=$((num + 1))
          fi
        fi
      fi
    done
  fi

  # Collect inputs
  local scenario_name
  prompt scenario_name "Scenario name (short description)" "my_test_case"

  # Sanitize scenario name to replace invalid characters with '_'
  scenario_name=$(echo "$scenario_name" | sed 's/[^a-zA-Z0-9_-]/_/g')

  # Validate that the sanitized name is not empty
  if [[ -z "$scenario_name" ]]; then
    echo "Error: Scenario name cannot be empty after sanitization." >&2
    exit 1
  fi

  local setups=("noop" "main_ahead" "conflict")
  local setup
  prompt_choice setup "Git setup type" setups "main_ahead"

  local exit_code
  prompt exit_code "Expected exit code" "0"

  local pr_exists
  prompt_yn pr_exists "PR already exists in mock?" "n"

  local mock_unstable
  if prompt_yn mock_unstable "Simulate unstable PR status?" "n" && [[ "$mock_unstable" == "true" ]]; then
    local unstable_count
    prompt unstable_count "Number of UNKNOWN responses before stable" "3"
  else
    local unstable_count="0"
  fi

  local automerge_fail
  prompt_yn automerge_fail "Auto-merge should fail?" "n"

  local bg_commit
  if prompt_yn bg_commit "Background commit on main during run?" "n" && [[ "$bg_commit" == "true" ]]; then
    local bg_delay
    prompt bg_delay "Delay before commit (seconds)" "1"
    local bg_msg
    prompt bg_msg "Commit message" "main advances during test"
  fi

  # Build the .env file
  mkdir -p "$SCENARIOS_DIR"

  local scenario_num
  scenario_num=$(printf "%02d" "$next_num")
  local filename="$SCENARIOS_DIR/${scenario_num}_${scenario_name}.env"

  cat > "$filename" << EOF
# Auto-generated scenario
SCENARIO_NAME="$scenario_name"
SETUP="$setup"
EXPECT_EXIT="$exit_code"

# Mock GitHub state
MOCK_PR_EXISTS="$pr_exists"
MOCK_UNSTABLE_CALLS="$unstable_count"
MOCK_ENABLE_AUTOMERGE_FAIL="$automerge_fail"

EOF

  if [[ -n "${bg_delay:-}" ]]; then
    cat >> "$filename" << EOF
# Background commit timing
BACKGROUND_MAIN_COMMIT_DELAY="$bg_delay"
BACKGROUND_MAIN_COMMIT_MSG="$bg_msg"

EOF
  fi

  echo -e "\n${GREEN}✅ Created scenario: $filename${NC}\n"
  cat "$filename"

  echo -e "\n${GREEN}Run it with:${NC}"
  echo "  ./run_all.sh --scenario $next_num"
}

main
