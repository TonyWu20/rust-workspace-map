#!/usr/bin/env bash
set -euo pipefail
# TASK-8: Remove all crate-level clippy allow attributes and fix individual lint violations
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/lib.rs
python3 "$(dirname "$0")/TASK-8.py"
