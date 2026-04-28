#!/usr/bin/env python3
"""TASK-6: Update workspace.rs enumerate_members to return MissingWorkspaceSection error"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-6"
STEPS = json.loads('[{"before_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IHBhcnNlZAogICAgICAgIC5nZXQoIndvcmtzcGFjZSIpCiAgICAgICAgLmFuZF90aGVuKHx3fCB3LmdldCgibWVtYmVycyIpKQogICAgICAgIC5hbmRfdGhlbih8bXwgbS5hc19hcnJheSgpKQogICAgICAgIC5tYXAofGFycnwgewogICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAuZmlsdGVyX21hcCh8dnwgdi5hc19zdHIoKS5tYXAoU3RyaW5nOjpmcm9tKSkKICAgICAgICAgICAgICAgIC5jb2xsZWN0KCkKICAgICAgICB9KQogICAgICAgIC51bndyYXBfb3JfZGVmYXVsdCgpOw==", "after_b64": "ICAgIGxldCBtZW1iZXJzOiBWZWM8U3RyaW5nPiA9IG1hdGNoIHBhcnNlZC5nZXQoIndvcmtzcGFjZSIpIHsKICAgICAgICBOb25lID0+IHJldHVybiBFcnIoRXJyb3I6Ok1pc3NpbmdXb3Jrc3BhY2VTZWN0aW9uKSwKICAgICAgICBTb21lKHdvcmtzcGFjZSkgPT4gd29ya3NwYWNlCiAgICAgICAgICAgIC5nZXQoIm1lbWJlcnMiKQogICAgICAgICAgICAuYW5kX3RoZW4ofG18IG0uYXNfYXJyYXkoKSkKICAgICAgICAgICAgLm1hcCh8YXJyfCB7CiAgICAgICAgICAgICAgICBhcnIuaXRlcigpCiAgICAgICAgICAgICAgICAgICAgLmZpbHRlcl9tYXAofHZ8IHYuYXNfc3RyKCkubWFwKFN0cmluZzo6ZnJvbSkpCiAgICAgICAgICAgICAgICAgICAgLmNvbGxlY3Q6OjxWZWM8Xz4+KCkKICAgICAgICAgICAgfSkKICAgICAgICAgICAgLnVud3JhcF9vcl9kZWZhdWx0KCksCiAgICB9Ow==", "target": "src/workspace.rs", "index": 0, "is_create": false}, {"before_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgpwdWIgZm4gZmluZF93b3Jrc3BhY2Vfcm9vdChzdGFydF9wYXRoOiAmUGF0aCkgLT4gUmVzdWx0PFBhdGhCdWY+IHs=", "after_b64": "Ly8vIFdhbGsgdXAgdGhlIGRpcmVjdG9yeSB0cmVlIGZyb20gYHN0YXJ0X3BhdGhgIHRvIGZpbmQgYSBgQ2FyZ28udG9tbGAKLy8vIGNvbnRhaW5pbmcgYSBgW3dvcmtzcGFjZV1gIHNlY3Rpb24uIFJldHVybnMgdGhlIGRpcmVjdG9yeSBjb250YWluaW5nIGl0LgovLy8KLy8vICMgRXJyb3JzCi8vLwovLy8gUmV0dXJucyBgRXJyb3I6OldvcmtzcGFjZVJvb3ROb3RGb3VuZGAgaWYgbm8gYENhcmdvLnRvbWxgIHdpdGggYQovLy8gYFt3b3Jrc3BhY2VdYCBzZWN0aW9uIGlzIGZvdW5kIGluIGFueSBhbmNlc3RvciBkaXJlY3RvcnkuCnB1YiBmbiBmaW5kX3dvcmtzcGFjZV9yb290KHN0YXJ0X3BhdGg6ICZQYXRoKSAtPiBSZXN1bHQ8UGF0aEJ1Zj4gew==", "target": "src/workspace.rs", "index": 1, "is_create": false}, {"before_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KcHViIGZuIGVudW1lcmF0ZV9tZW1iZXJzKHJvb3Q6ICZQYXRoKSAtPiBSZXN1bHQ8VmVjPFBhdGhCdWY+PiB7", "after_b64": "Ly8vIFBhcnNlIHRoZSB3b3Jrc3BhY2UgYENhcmdvLnRvbWxgLCByZXNvbHZlIG1lbWJlciBwYXRocyAoaW5jbHVkaW5nIGdsb2IKLy8vIHBhdHRlcm5zKSwgYXBwbHkgYGV4Y2x1ZGVgIGxpc3QsIGFuZCByZXR1cm4gYWJzb2x1dGUgcGF0aHMgdG8gZWFjaCBtZW1iZXIKLy8vIGNyYXRlIGRpcmVjdG9yeS4KLy8vCi8vLyAjIEVycm9ycwovLy8KLy8vIFJldHVybnMgYEVycm9yOjpNaXNzaW5nV29ya3NwYWNlU2VjdGlvbmAgaWYgdGhlIGBDYXJnby50b21sYCBsYWNrcyBhCi8vLyBgW3dvcmtzcGFjZV1gIHNlY3Rpb24gZW50aXJlbHkuCnB1YiBmbiBlbnVtZXJhdGVfbWVtYmVycyhyb290OiAmUGF0aCkgLT4gUmVzdWx0PFZlYzxQYXRoQnVmPj4gew==", "target": "src/workspace.rs", "index": 2, "is_create": false}]')

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
