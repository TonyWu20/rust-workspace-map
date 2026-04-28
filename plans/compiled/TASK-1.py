#!/usr/bin/env python3
"""TASK-1: Add ErrorSeverity enum and ErrorContext struct to schema.rs"""
import base64, json, subprocess, sys
from pathlib import Path

TASK_ID = "TASK-1"
STEPS = json.loads('[{"before_b64": "Ly8g4pSA4pSAIEludGVybmFsIHR5cGVzIOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIEludGVybmFsIGludGVybWVkaWF0ZSB0eXBlIGNvbnN1bWVkIGJ5IG1vZHVsZV90cmVlLgojW2Rlcml2ZShEZWJ1ZywgQ2xvbmUsIERlZmF1bHQpXQpwdWIgc3RydWN0IEZpbGVJbmZvIHsK", "after_b64": "Ly8g4pSA4pSAIEVycm9yIHNldmVyaXR5IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKI1tkZXJpdmUoRGVidWcsIENsb25lLCBDb3B5LCBQYXJ0aWFsRXEsIEVxLCBzZXJkZTo6U2VyaWFsaXplKV0KI1tzZXJkZShyZW5hbWVfYWxsID0gInNuYWtlX2Nhc2UiKV0KcHViIGVudW0gRXJyb3JTZXZlcml0eSB7CiAgICBFcnJvciwKICAgIFdhcm5pbmcsCn0KCi8vIOKUgOKUgCBFcnJvciBjb250ZXh0IOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAoKLy8vIE9wdGlvbmFsIGNvbnRleHQgYXR0YWNoZWQgdG8gYW4gZXJyb3IsIHByb3ZpZGluZyBhZGRpdGlvbmFsIGxvY2F0aW9uCi8vLyBhbmQgc291cmNlIGluZm9ybWF0aW9uIGZvciBkaWFnbm9zdGljcy4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0LCBzZXJkZTo6U2VyaWFsaXplLCBib246OkJ1aWxkZXIpXQojW3NlcmRlKHJlbmFtZV9hbGwgPSAiY2FtZWxDYXNlIildCnB1YiBzdHJ1Y3QgRXJyb3JDb250ZXh0IHsKICAgICNbc2VyZGUoc2tpcF9zZXJpYWxpemluZ19pZiA9ICJPcHRpb246OmlzX25vbmUiKV0KICAgIHB1YiBjcmF0ZV9uYW1lOiBPcHRpb248U3RyaW5nPiwKCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgbW9kdWxlX3BhdGg6IE9wdGlvbjxTdHJpbmc+LAoKICAgIC8vLyBMaW5lIG51bWJlciBpbiB0aGUgc291cmNlIGZpbGUgd2hlcmUgdGhlIGVycm9yIG9jY3VycmVkLgogICAgI1tzZXJkZShza2lwX3NlcmlhbGl6aW5nX2lmID0gIk9wdGlvbjo6aXNfbm9uZSIpXQogICAgcHViIGxpbmU6IE9wdGlvbjx1c2l6ZT4sCgogICAgLy8vIEEgc2hvcnQgc291cmNlIHNuaXBwZXQgbmVhciB0aGUgZXJyb3IgbG9jYXRpb24gKGlmIGF2YWlsYWJsZSkuCiAgICAjW3NlcmRlKHNraXBfc2VyaWFsaXppbmdfaWYgPSAiT3B0aW9uOjppc19ub25lIildCiAgICBwdWIgc25pcHBldDogT3B0aW9uPFN0cmluZz4sCn0KCi8vIOKUgOKUgCBJbnRlcm5hbCB0eXBlcyDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIDilIAKCi8vLyBJbnRlcm5hbCBpbnRlcm1lZGlhdGUgdHlwZSBjb25zdW1lZCBieSBtb2R1bGVfdHJlZS4KI1tkZXJpdmUoRGVidWcsIENsb25lLCBEZWZhdWx0KV0KcHViIHN0cnVjdCBGaWxlSW5mbyB7Cg==", "target": "src/schema.rs", "index": 0, "is_create": false}]')

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
