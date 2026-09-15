#!/usr/bin/env python3
import sys

PASS = FAIL = 0

def check(name, ok):
    global PASS, FAIL
    print(("PASS " if ok else "FAIL ") + name)
    PASS += ok
    FAIL += not ok

def transfer(env, src, dst):
    if src not in env or env[src] != "owned":
        raise RuntimeError("use-after-move")
    env[src] = "moved"
    env[dst] = "owned"

check("single transfer", (lambda e: (transfer(e, "س", "ص"), e["س"] == "moved" and e["ص"] == "owned"))( {"س":"owned"} )[1])
e = {"س":"owned"}; transfer(e, "س", "ص")
try: transfer(e, "س", "ع"); ok = False
except RuntimeError: ok = True
check("second transfer rejected", ok)
e = {"س":"moved"}
try: transfer(e, "س", "ص"); ok = False
except RuntimeError: ok = True
check("already moved rejected", ok)
e = {"س":"owned"}; transfer(e, "س", "ص"); check("destination owns", e["ص"] == "owned")
check("source not owned", e["س"] == "moved")
print(f"SUMMARY: {PASS}/5 PASS")
sys.exit(1 if FAIL else 0)
