#!/usr/bin/env python3
"""TASK-1: Create Cargo.toml with all dependencies"""
import base64, json, sys
from pathlib import Path

TASK_ID = "TASK-1"
STEPS = json.loads('[{"content_b64": "W3BhY2thZ2VdCm5hbWUgPSAicnVzdC13b3Jrc3BhY2UtbWFwIgp2ZXJzaW9uID0gIjAuMS4wIgplZGl0aW9uID0gIjIwMjQiCgpbZGVwZW5kZW5jaWVzXQpzZXJkZSA9IHsgdmVyc2lvbiA9ICIxIiwgZmVhdHVyZXMgPSBbImRlcml2ZSJdIH0Kc2VyZGVfanNvbiA9ICIxIgpzeW4gPSB7IHZlcnNpb24gPSAiMiIsIGZlYXR1cmVzID0gWyJmdWxsIiwgImV4dHJhLXRyYWl0cyJdIH0KdG9tbCA9ICIwLjgiCnJheW9uID0gIjEiCmFueWhvdyA9ICIxIgpjbGFwID0geyB2ZXJzaW9uID0gIjQiLCBmZWF0dXJlcyA9IFsiZGVyaXZlIl0gfQpib24gPSAiMyIKdGhpc2Vycm9yID0gIjIiCmdsb2IgPSAiMC4zIgo=", "target": "Cargo.toml", "index": 0}]')

for step in STEPS:
    content = base64.b64decode(step["content_b64"]).decode()
    target = step["target"]
    idx = step["index"]

    target_path = Path(target)
    target_path.parent.mkdir(parents=True, exist_ok=True)
    target_path.write_text(content)

    if not target_path.exists():
        print(f"FAILED {TASK_ID} change {idx}: file not created at {target}", file=sys.stderr)
        sys.exit(1)

    print(f"OK {TASK_ID} change {idx}: created {target}")

print(f"OK {TASK_ID}: all files created")
