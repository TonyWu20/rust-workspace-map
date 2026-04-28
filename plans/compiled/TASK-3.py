#!/usr/bin/env python3
"""TASK-3: Expand ErrorEntry struct with severity, kind, context, and cause fields"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-3"
STEPS = json.loads('[{"before_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKfQo=", "after_b64": "I1tkZXJpdmUoRGVidWcsIENsb25lLCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JFbnRyeSB7CiAgICBwdWIgZmlsZTogU3RyaW5nLAogICAgI1tidWlsZGVyKGRlZmF1bHQpXQogICAgcHViIGxpbmU6IHVzaXplLAogICAgcHViIG1lc3NhZ2U6IFN0cmluZywKICAgIHB1YiBzZXZlcml0eTogRXJyb3JTZXZlcml0eSwKICAgIHB1YiBraW5kOiBTdHJpbmcsCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY29udGV4dDogT3B0aW9uPEVycm9yQ29udGV4dD4sCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgY2F1c2U6IE9wdGlvbjxTdHJpbmc+LAp9Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')

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
