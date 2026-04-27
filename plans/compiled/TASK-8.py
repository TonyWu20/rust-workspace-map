#!/usr/bin/env python3
"""TASK-8: Create src/render.rs with render_json and render_to_writer"""
import base64, json, sys
from pathlib import Path

TASK_ID = "TASK-8"
STEPS = json.loads('[{"content_b64": "dXNlIGNyYXRlOjpzY2hlbWE6OldvcmtzcGFjZU1hcDsKdXNlIHN0ZDo6aW86OldyaXRlOwoKLy8vIFNlcmlhbGl6ZSB0aGUgd29ya3NwYWNlIG1hcCB0byBhIEpTT04gc3RyaW5nIHdpdGggMi1zcGFjZSBpbmRlbnRhdGlvbi4KcHViIGZuIHJlbmRlcl9qc29uKG1hcDogJldvcmtzcGFjZU1hcCkgLT4gc2VyZGVfanNvbjo6UmVzdWx0PFN0cmluZz4gewogICAgc2VyZGVfanNvbjo6dG9fc3RyaW5nX3ByZXR0eShtYXApCn0KCi8vLyBTZXJpYWxpemUgdGhlIHdvcmtzcGFjZSBtYXAgdG8gdGhlIGdpdmVuIHdyaXRlci4KcHViIGZuIHJlbmRlcl90b193cml0ZXIobWFwOiAmV29ya3NwYWNlTWFwLCB3cml0ZXI6IGltcGwgV3JpdGUpIC0+IHNlcmRlX2pzb246OlJlc3VsdDwoKT4gewogICAgc2VyZGVfanNvbjo6dG9fd3JpdGVyX3ByZXR0eSh3cml0ZXIsIG1hcCkKfQo=", "target": "src/render.rs", "index": 0}]')

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
