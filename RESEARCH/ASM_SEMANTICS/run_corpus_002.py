"""
ASM-SEM-002: Control Flow corpus runner
Tests: jmp, jcc, call, ret, loop
"""
import sys
sys.path.insert(0, '.')
from model_x86 import State, mov, cmp, jmp, jcc, call, ret, loop, Error, InvalidJumpTarget
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
print("=== ASM-SEM-002 CORPUS RUN ===")
print()
# JMP-001
print("--- JMP-001: jmp target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
jmp(s, 'target')
check("rip=target", s.rip == 0x100, f"got 0x{s.rip:x}")
check("rax unchanged", s.regs['rax'] == 0, f"got {s.regs['rax']}")
# JE-001
print("--- JE-001: mov rax, 5; cmp rax, 5; je target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
mov(s, 'rax', 5)
cmp(s, 'rax', 5)
jcc(s, 'je', 'target')
check("rip=target", s.rip == 0x100, f"got 0x{s.rip:x}")
# JE-002
print("--- JE-002: mov rax, 5; cmp rax, 6; je target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
mov(s, 'rax', 5)
cmp(s, 'rax', 6)
jcc(s, 'je', 'target')
check("rip advances (no jump)", s.rip == 1, f"got 0x{s.rip:x}")
# JNE-001
print("--- JNE-001: mov rax, 5; cmp rax, 6; jne target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
mov(s, 'rax', 5)
cmp(s, 'rax', 6)
jcc(s, 'jne', 'target')
check("rip=target", s.rip == 0x100, f"got 0x{s.rip:x}")
# JL-001
print("--- JL-001: mov rax, -1; cmp rax, 0; jl target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
mov(s, 'rax', (1 << 64) - 1)  # -1 as unsigned
cmp(s, 'rax', 0)
jcc(s, 'jl', 'target')
check("rip=target", s.rip == 0x100, f"got 0x{s.rip:x}")
# CALL-001
print("--- CALL-001: call func ---")
s = State()
s.labels = {'func': 0x200}
s.rip = 0
call(s, 'func')
check("rip=func", s.rip == 0x200, f"got 0x{s.rip:x}")
check("rsp decremented", s.regs['rsp'] == 0xFF8, f"got 0x{s.regs['rsp']:x}")
check("return_addr pushed", s.mem.get(0xFF8) == 1 or s.mem.get(0xFF8) is not None, f"got {s.mem.get(0xFF8)}")
# CALL-002
print("--- CALL-002: mov rsp, 0x2000; call func ---")
s = State()
s.labels = {'func': 0x200}
s.rip = 0
s.regs['rsp'] = 0x2000
call(s, 'func')
check("rip=func", s.rip == 0x200, f"got 0x{s.rip:x}")
check("rsp=0x1FF8", s.regs['rsp'] == 0x1FF8, f"got 0x{s.regs['rsp']:x}")
# RET-001
print("--- RET-001: mov rsp, 0x1FF8; mov [rsp], 0x100; ret ---")
s = State()
s.regs['rsp'] = 0x1FF8
from model_x86 import write_mem
write_mem(s, 0x1FF8, 0x100, 8)
ret(s)
check("rip=0x100", s.rip == 0x100, f"got 0x{s.rip:x}")
check("rsp=0x2000", s.regs['rsp'] == 0x2000, f"got 0x{s.regs['rsp']:x}")
# LOOP-001
print("--- LOOP-001: mov rcx, 3; loop target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
mov(s, 'rcx', 3)
loop(s, 'target')
check("rcx=2", s.regs['rcx'] == 2, f"got {s.regs['rcx']}")
check("rip=target", s.rip == 0x100, f"got 0x{s.rip:x}")
# LOOP-002
print("--- LOOP-002: mov rcx, 1; loop target ---")
s = State()
s.labels = {'target': 0x100}
s.rip = 0
mov(s, 'rcx', 1)
loop(s, 'target')
check("rcx=0", s.regs['rcx'] == 0, f"got {s.regs['rcx']}")
check("rip advances (no jump)", s.rip == 1, f"got 0x{s.rip:x}")
# ERR-003
print("--- ERR-003: jmp invalid_addr ---")
s = State()
s.rip = 0
expect_error("invalid jump target", InvalidJumpTarget, lambda: jmp(s, 'invalid_addr'))
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
