#!/usr/bin/env bash
set -euo pipefail
# TASK-PREP: Add tempfile dev-dependency for unit and integration tests
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: Cargo.toml
python3 "$(dirname "$0")/TASK-PREP.py"
