#!/usr/bin/env bash
set -euo pipefail
# TASK-1: Wrap module_tree errors with kind=module_tree_error to align with plan spec
# Source: /Users/tony/programming/rust-workspace-map/notes/pr-reviews/phase-0.2/fix-plan.toml
# Type: replace
# File: src/lib.rs
python3 "$(dirname "$0")/TASK-1.py"
