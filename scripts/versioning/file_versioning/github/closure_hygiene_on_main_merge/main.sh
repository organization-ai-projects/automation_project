#!/usr/bin/env bash
# shellcheck shell=bash

closure_hygiene_main() {
  closure_hygiene_bootstrap
  closure_hygiene_scan_open_parents
  closure_hygiene_close_completed_milestones
  echo "Closure hygiene completed."
}
