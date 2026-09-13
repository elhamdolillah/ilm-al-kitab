"""
ASM-SEM-001: Corpus runner
Runs 9 test cases against the model, outputs results with SHA-256.
"""
import sys
import hashlib
sys.path.insert(0, '.')
from model_x86 import State, mov, add, sub, push, pop, cmp, test, Error, UnknownRegister, InvalidMemory
PASS = 0
FAIL = 0
ERRORS = 0
def check(name, condition, detail=""):
    global PASS, FAIL
    if condition:
        print(f"  [PASS] {name}")
        PASS += 1
    else:
        print(f"  [FAIL] {name} — {detail}")
        FAIL += 1
def expect_error(name, exc_type, fn):
    global PASS, FAIL, ERRORS
    try:
        fn()
        print(f"  [FAIL] {name} — expected {exc_type.__name__} but succeeded")
        FAIL += 1
    except exc_type as e:
        print(f"  [PASS] {name} — {exc_type.__name__}: {e}")
        PASS += 1
        ERRORS += 1
    except Exception as e:
        print(f"  [FAIL] {name} — expected {exc_type.__name__}, got {type(e).__name__}: {e}")
        FAIL += 1
print("=== ASM-SEM-001 CORPUS RUN ===")
print()
# MOV-001
print("--- MOV-001: mov rax, 7 ---")
s = State()
mov(s, 'rax', 7)
check("rax=7", s.regs['rax'] == 7, f"got {s.regs['rax']}")
check("rbx unchanged", s.regs['rbx'] == 0, f"got {s.regs['rbx']}")
check("rsp unchanged", s.regs['rsp'] == 0x1000, f"got 0x{s.regs['rsp']:x}")
# ADD-001
print("--- ADD-001: mov rax, 7; add rax, 5 ---")
s = State()
mov(s, 'rax', 7)
add(s, 'rax', 5)
check("rax=12", s.regs['rax'] == 12, f"got {s.regs['rax']}")
check("ZF=0", s.flags['ZF'] == False, f"got {s.flags['ZF']}")
check("SF=0", s.flags['SF'] == False, f"got {s.flags['SF']}")
# SUB-001
print("--- SUB-001: mov rax, 7; sub rax, 5 ---")
s = State()
mov(s, 'rax', 7)
sub(s, 'rax', 5)
check("rax=2", s.regs['rax'] == 2, f"got {s.regs['rax']}")
check("ZF=0", s.flags['ZF'] == False, f"got {s.flags['ZF']}")
check("SF=0", s.flags['SF'] == False, f"got {s.flags['SF']}")
# STACK-001
print("--- STACK-001: rsp=0x1000; push rax; pop rbx ---")
s = State()
s.regs['rsp'] = 0x1000
mov(s, 'rax', 42)
push(s, 'rax')
pop(s, 'rbx')
check("rbx=42", s.regs['rbx'] == 42, f"got {s.regs['rbx']}")
check("rsp restored", s.regs['rsp'] == 0x1000, f"got 0x{s.regs['rsp']:x}")
# CMP-001
print("--- CMP-001: mov rax, 4; cmp rax, 4 ---")
s = State()
mov(s, 'rax', 4)
cmp(s, 'rax', 4)
check("ZF=1", s.flags['ZF'] == True, f"got {s.flags['ZF']}")
check("rax unchanged", s.regs['rax'] == 4, f"got {s.regs['rax']}")
# TEST-001
print("--- TEST-001: mov rax, 0; test rax, rax ---")
s = State()
mov(s, 'rax', 0)
test(s, 'rax', 'rax')
check("ZF=1", s.flags['ZF'] == True, f"got {s.flags['ZF']}")
check("rax unchanged", s.regs['rax'] == 0, f"got {s.regs['rax']}")
# ERR-001
print("--- ERR-001: pop rax with invalid memory ---")
s = State()
s.mem = {}  # empty memory
expect_error("pop invalid mem", InvalidMemory, lambda: pop(s, 'rax'))
# ERR-002
print("--- ERR-002: mov unknown, 1 ---")
s = State()
expect_error("unknown register", UnknownRegister, lambda: mov(s, 'unknown', 1))
# BOUND-001
print("--- BOUND-001: 2^64-1 + 1 ---")
s = State()
s.regs['rax'] = (1 << 64) - 1
add(s, 'rax', 1)
check("rax=0 (modular)", s.regs['rax'] == 0, f"got {s.regs['rax']}")
check("CF=1 (carry)", s.flags['CF'] == True, f"got {s.flags['CF']}")
check("OF=0", s.flags['OF'] == False, f"got {s.flags['OF']}")
# Summary
print()
print("=== SUMMARY ===")
print(f"RESULTS: {PASS}/{PASS+FAIL} passed")
print(f"ERRORS (expected failures caught): {ERRORS}")
if FAIL > 0:
    print(f"FAILURES: {FAIL}")
    print("STATUS=FAIL_CLOSED")
    sys.exit(1)
else:
    print("STATUS=PASS")
    sys.exit(0)
