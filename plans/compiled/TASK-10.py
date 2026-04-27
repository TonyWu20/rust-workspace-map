#!/usr/bin/env python3
"""TASK-10: Create src/main.rs with clap CLI and main() entry point"""
import base64, json, sys
from pathlib import Path

TASK_ID = "TASK-10"
STEPS = json.loads('[{"content_b64": "dXNlIGNsYXA6OlBhcnNlcjsKdXNlIHN0ZDo6cGF0aDo6UGF0aEJ1ZjsKCiNbZGVyaXZlKFBhcnNlcildCiNbY29tbWFuZCgKICAgIG5hbWUgPSAicnVzdC13b3Jrc3BhY2UtbWFwIiwKICAgIHZlcnNpb24sCiAgICBhYm91dCA9ICJHZW5lcmF0ZSBhIEpTT04gbWFwIG9mIGEgUnVzdCB3b3Jrc3BhY2UncyBwdWJsaWMgQVBJIHN1cmZhY2UiCildCnN0cnVjdCBDbGkgewogICAgLy8vIFBhdGggdG8gdGhlIHdvcmtzcGFjZSByb290IG9yIGFueSBkaXJlY3Rvcnkgd2l0aGluIGl0CiAgICAjW2FyZyh2YWx1ZV9uYW1lID0gIlBBVEgiKV0KICAgIHBhdGg6IFBhdGhCdWYsCgogICAgLy8vIFdyaXRlIEpTT04gb3V0cHV0IHRvIGZpbGUgaW5zdGVhZCBvZiBzdGRvdXQKICAgICNbYXJnKHNob3J0ID0gJ28nLCBsb25nID0gIm91dHB1dCIsIHZhbHVlX25hbWUgPSAiRklMRSIpXQogICAgb3V0cHV0OiBPcHRpb248UGF0aEJ1Zj4sCn0KCmZuIG1haW4oKSAtPiBhbnlob3c6OlJlc3VsdDwoKT4gewogICAgbGV0IGNsaSA9IENsaTo6cGFyc2UoKTsKCiAgICBsZXQgd29ya3NwYWNlX3BhdGggPSBzdGQ6OnBhdGg6OmFic29sdXRlKCZjbGkucGF0aCkKICAgICAgICAubWFwX2Vycih8ZXwgYW55aG93Ojphbnlob3chKCJpbnZhbGlkIHBhdGgge306IHt9IiwgY2xpLnBhdGguZGlzcGxheSgpLCBlKSk/OwoKICAgIGxldCBjb25maWcgPSBydXN0X3dvcmtzcGFjZV9tYXA6OkNvbmZpZzo6YnVpbGRlcigpCiAgICAgICAgLndvcmtzcGFjZV9wYXRoKHdvcmtzcGFjZV9wYXRoKQogICAgICAgIC5vdXRwdXRfcGF0aChjbGkub3V0cHV0KQogICAgICAgIC5idWlsZCgpOwoKICAgIHJ1c3Rfd29ya3NwYWNlX21hcDo6cnVuKGNvbmZpZykKfQo=", "target": "src/main.rs", "index": 0}]')

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
