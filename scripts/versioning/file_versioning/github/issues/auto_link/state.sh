#!/usr/bin/env bash

auto_link_add_label() {
  local repo_name="$1"
  local issue_number="$2"
  local label="$3"
  gh api "repos/${repo_name}/issues/${issue_number}/labels" \
    -f labels[]="$label" >/dev/null 2>&1 || true
}

auto_link_remove_label() {
  local repo_name="$1"
  local issue_number="$2"
  local label="$3"
  gh api -X DELETE "repos/${repo_name}/issues/${issue_number}/labels/${label}" >/dev/null 2>&1 || true
}

auto_link_set_validation_error_state() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local required_missing_label="$4"
  local automation_failed_label="$5"
  local message="$6"
  local help_text="$7"
  local body
  body="$marker
### Parent Field Autolink Status

❌ $message

$help_text
"
  auto_link_add_label "$repo_name" "$issue_number" "$required_missing_label"
  auto_link_remove_label "$repo_name" "$issue_number" "$automation_failed_label"
  github_issue_upsert_marker_comment "$repo_name" "$issue_number" "$marker" "$body"
}

auto_link_set_runtime_error_state() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local automation_failed_label="$4"
  local message="$5"
  local help_text="$6"
  local body
  body="$marker
### Parent Field Autolink Status

⚠️ $message

$help_text
"
  auto_link_add_label "$repo_name" "$issue_number" "$automation_failed_label"
  github_issue_upsert_marker_comment "$repo_name" "$issue_number" "$marker" "$body"
}

auto_link_set_success_state() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local required_missing_label="$4"
  local automation_failed_label="$5"
  local message="$6"
  local body
  body="$marker
### Parent Field Autolink Status

✅ $message
"
  auto_link_remove_label "$repo_name" "$issue_number" "$required_missing_label"
  auto_link_remove_label "$repo_name" "$issue_number" "$automation_failed_label"
  github_issue_upsert_marker_comment "$repo_name" "$issue_number" "$marker" "$body"
}
