"""C1.2 Set Theory Model"""
import sys
PASS = 0
FAIL = 0
def check(name, ok, detail=""):
    global PASS, FAIL
    if ok:
        print(f"  [PASS] {name}")
        PASS += 1
    else:
        print(f"  [FAIL] {name} — {detail}")
        FAIL += 1
def union(a, b):
    """A ∪ B = {x | x in A or x in B}"""
    return a | b
def intersection(a, b):
    """A ∩ B = {x | x in A and x in B}"""
    return a & b
def difference(a, b):
    """A \ B = {x | x in A and x not in B}"""
    return a - b
def symmetric_diff(a, b):
    """A Δ B = (A \ B) ∪ (B \ A)"""
    return (a - b) | (b - a)
def cartesian(a, b):
    """A × B = {(x,y) | x in A and y in B}"""
    return {(x, y) for x in a for y in b}
def subset(a, b):
    """A ⊆ B iff forall x in A, x in B"""
    return a <= b
def superset(a, b):
    """A ⊇ B iff forall x in B, x in A"""
    return a >= b
print("=== C1.2 SET THEORY MODEL RUN ===")
print()
check("SET-001: union", union({1,2}, {2,3}) == {1,2,3}, "")
check("SET-002: intersection", intersection({1,2}, {2,3}) == {2}, "")
check("SET-003: difference", difference({1,2,3}, {2}) == {1,3}, "")
check("SET-004: symmetric diff", symmetric_diff({1,2}, {2,3}) == {1,3}, "")
check("SET-005: cartesian", cartesian({1,2}, {3,4}) == {(1,3),(1,4),(2,3),(2,4)}, "")
check("SET-006: subset true", subset({1,2}, {1,2,3}) == True, "")
check("SET-007: superset true", superset({1,2}, {1}) == True, "")
check("SET-008: subset false", subset({1,2}, {1}) == False, "")
check("SET-009: union identity", union(set(), {1,2}) == {1,2}, "")
check("SET-010: intersection empty", intersection({1,2}, set()) == set(), "")
check("SET-011: diff with empty", difference({1,2}, set()) == {1,2}, "")
check("SET-012: reflexivity", subset({1,2}, {1,2}) == True, "")
print()
print("=== SUMMARY ===")
print(f"RESULTS: {PASS}/{PASS+FAIL} passed")
if FAIL > 0:
    print(f"FAILURES: {FAIL}")
    print("STATUS=FAIL_CLOSED")
    sys.exit(1)
else:
    print("STATUS=PASS")
    sys.exit(0)
