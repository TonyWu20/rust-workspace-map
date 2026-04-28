#!/usr/bin/env python3
"""TASK-2: Add MissingWorkspaceSection variant to Error enum in schema.rs"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-2"
STEPS = json.loads('[{"before_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "after_b64": "ICAgICNbZXJyb3IoImdsb2IgcGF0dGVybiBlcnJvcjogezB9IildCiAgICBHbG9iUGF0dGVybihTdHJpbmcpLAoKICAgICNbZXJyb3IoIndvcmtzcGFjZSBDYXJnby50b21sIGlzIG1pc3NpbmcgdGhlIFt3b3Jrc3BhY2VdIHNlY3Rpb24iKV0KICAgIE1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uLAp9CgpwdWIgdHlwZSBSZXN1bHQ8VD4gPSBzdGQ6OnJlc3VsdDo6UmVzdWx0PFQsIEVycm9yPjs=", "target": "src/schema.rs", "index": 0, "is_create": false}]')

for step in STEPS:
    before = base64.b64decode(step["before_b64"]).decode()
    after = base64.b64decode(step["after_b64"]).decode()
    target = step["target"]
    idx = step["index"]
    is_create = step["is_create"]

    if is_create:
        target_path = Path(target)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        target_path.write_text(after)
        print(f"OK {TASK_ID} change {idx}: created {target}")
    else:
        target_path = Path(target)
        content = target_path.read_text()
        if before not in content:
            print(f"FAILED {TASK_ID} change {idx}: pattern not found in {target}", file=sys.stderr)
            print(f"Expected (first 200 chars): {repr(before[:200])}", file=sys.stderr)
            sys.exit(1)

        result = subprocess.run(
            ["sd", "-F", "-A", "-n", "1", "--", before, after, target],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            print(f"FAILED {TASK_ID} change {idx}: sd error: {result.stderr}", file=sys.stderr)
            sys.exit(result.returncode)

        new_content = target_path.read_text()
        if after and after not in new_content:
            print(f"FAILED {TASK_ID} change {idx}: replacement not found after apply", file=sys.stderr)
            sys.exit(1)

        print(f"OK {TASK_ID} change {idx}: applied to {target}")

print(f"OK {TASK_ID}: all changes applied")
