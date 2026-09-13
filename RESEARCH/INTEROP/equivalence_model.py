"""MAL-INTEROP-C2 Equivalence Model"""
import sys
PASS = 0
FAIL = 0
def check(name, ok, detail=""):
    global PASS, FAIL
    if ok:
        print(f"  [EQUIVALENT] {name}")
        PASS += 1
    else:
        print(f"  [NOT_EQUIVALENT] {name} — {detail}")
        FAIL += 1
def old_atbaa(x):
    if not isinstance(x, (int, float)):
        raise TypeError("unsupported")
    return str(x)
def old_jamaa(a, b):
    if not isinstance(a, (int, float)) or not isinstance(b, (int, float)):
        raise TypeError("unsupported")
    return a + b
def old_iqraa(s=None):
    return s if s else ""
def new_print(x):
    if not isinstance(x, (int, float)):
        raise TypeError("unsupported")
    return str(x)
def new_plus(a, b):
    if not isinstance(a, (int, float)) or not isinstance(b, (int, float)):
        raise TypeError("unsupported")
    return a + b
def new_read(s=None):
    return s if s else ""
print("=== MAL-INTEROP-C2 RUN ===")
print()
check("INTEROP-001: print(5)", old_atbaa(5) == new_print(5), "")
check("INTEROP-002: print(0)", old_atbaa(0) == new_print(0), "")
check("INTEROP-003: print(-1)", old_atbaa(-1) == new_print(-1), "")
check("INTEROP-004: plus(3,4)", old_jamaa(3,4) == new_plus(3,4), "")
check("INTEROP-005: plus(-5,10)", old_jamaa(-5,10) == new_plus(-5,10), "")
check("INTEROP-006: plus(0,0)", old_jamaa(0,0) == new_plus(0,0), "")
check("INTEROP-007: read()", old_iqraa("t") == new_read("t"), "")
oe = ne = None
try: old_atbaa()
except TypeError: oe = True
try: new_print()
except TypeError: ne = True
check("INTEROP-008: print() no args", oe and ne, "")
oe = ne = None
try: old_jamaa(1)
except TypeError: oe = True
try: new_plus(1)
except TypeError: ne = True
check("INTEROP-009: plus(1) wrong args", oe and ne, "")
oe = ne = None
try: old_jamaa("a","b")
except TypeError: oe = True
try: new_plus("a","b")
except TypeError: ne = True
check("INTEROP-010: strings unsupported", oe and ne, "")
ox = old_atbaa(old_jamaa(3,4))
nx = new_print(new_plus(3,4))
check("INTEROP-011: full program", ox == nx, "")
print()
print("=== SUMMARY ===")
print(f"EQUIVALENT: {PASS}/{PASS+FAIL}")
if FAIL > 0:
    print("STATUS=FAIL_CLOSED")
    sys.exit(1)
else:
    print("STATUS=EQUIVALENT")
    sys.exit(0)
