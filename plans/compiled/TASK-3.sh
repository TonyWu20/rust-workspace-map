#!/usr/bin/env bash
set -euo pipefail
# TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields
# Source: /Users/tony/programming/rust-workspace-map/plans/phase-0.2.toml
# Type: replace
# File: src/schema.rs
python3 "$(dirname "$0")/TASK-3.py"
