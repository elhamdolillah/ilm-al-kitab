"""
ASM-SEM-003: Syscall corpus runner
Tests: sys_read, sys_write, sys_exit, syscall dispatch
"""
import sys
sys.path.insert(0, '.')
from model_x86 import State, mov, write_mem, syscall, Error, UnknownSyscall, InvalidFD, InvalidMemory, ProgramExit
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
print("=== ASM-SEM-003 CORPUS RUN ===")
print()
# SYS-001: sys_write to stdout
print("--- SYS-001: write 5 bytes to stdout ---")
s = State()
s.regs['rax'] = 1  # sys_write
s.regs['rdi'] = 1  # stdout
s.regs['rsi'] = 0x100  # buffer address
s.regs['rdx'] = 5  # count
for i, c in enumerate(b'Hello'):
    s.mem[0x100 + i] = c  # write bytes in correct order
syscall(s)
check("rax=5 (bytes written)", s.regs['rax'] == 5, f"got {s.regs['rax']}")
check("stdout_data correct", hasattr(s, 'stdout_data') and bytes(s.stdout_data) == b'Hello', f"got {bytes(s.stdout_data) if hasattr(s, 'stdout_data') else 'none'}")
# SYS-002: sys_read from stdin
print("--- SYS-002: read 10 bytes from stdin ---")
s = State()
s.stdin_data = bytearray(b'World12345')
s.regs['rax'] = 0  # sys_read
s.regs['rdi'] = 0  # stdin
s.regs['rsi'] = 0x200  # buffer address
s.regs['rdx'] = 10  # count
syscall(s)
check("rax=10 (bytes read)", s.regs['rax'] == 10, f"got {s.regs['rax']}")
check("memory contains World12345", all(s.mem.get(0x200 + i) == b'World12345'[i] for i in range(10)), "memory mismatch")
check("stdin_data consumed", len(s.stdin_data) == 0, f"got {len(s.stdin_data)} bytes remaining")
# SYS-003: sys_exit with code 0
print("--- SYS-003: exit with code 0 ---")
s = State()
s.regs['rax'] = 60  # sys_exit
s.regs['rdi'] = 0  # exit code
expect_error("program exits", ProgramExit, lambda: syscall(s))
# SYS-004: sys_exit with code 42
print("--- SYS-004: exit with code 42 ---")
s = State()
s.regs['rax'] = 60  # sys_exit
s.regs['rdi'] = 42  # exit code
def check_exit_code():
    try:
        syscall(s)
        return None
    except ProgramExit as e:
        return e.code
code = check_exit_code()
check("exit code=42", code == 42, f"got {code}")
# SYS-005: unknown syscall
print("--- SYS-005: unknown syscall (n=999) ---")
s = State()
s.regs['rax'] = 999
expect_error("unknown syscall", UnknownSyscall, lambda: syscall(s))
# SYS-006: invalid FD for write
print("--- SYS-006: invalid FD for write (fd=99) ---")
s = State()
s.regs['rax'] = 1  # sys_write
s.regs['rdi'] = 99  # invalid fd
s.regs['rsi'] = 0x100
s.regs['rdx'] = 5
write_mem(s, 0x100, 0x48656C6C6F, 8)
expect_error("invalid FD", InvalidFD, lambda: syscall(s))
# SYS-007: invalid FD for read
print("--- SYS-007: invalid FD for read (fd=5) ---")
s = State()
s.regs['rax'] = 0  # sys_read
s.regs['rdi'] = 5  # invalid fd
s.regs['rsi'] = 0x100
s.regs['rdx'] = 10
expect_error("invalid FD for read", InvalidFD, lambda: syscall(s))
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
