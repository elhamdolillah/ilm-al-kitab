
# ═══════════════════════════════════════════════════════════════
# Mathematical Semantics Layer — Formal Specifications
# ═══════════════════════════════════════════════════════════════
#
# State: Σ = (regs: R → ℤ, mem: Addr → Byte, ip: Addr, flags: F)
# where R = {rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp, ...}
#       F = {ZF, CF, SF, OF}
#
# Instructions:
#   mov: R × Operand → Σ   where Σ.regs[r] ← eval(operand)
#   add: R × Operand → Σ   where Σ.regs[r] ← Σ.regs[r] + eval(operand)
#   sub: R × Operand → Σ   where Σ.regs[r] ← Σ.regs[r] - eval(operand)
#   push: Operand → Σ      where Σ.mem[Σ.regs[rsp] - 8] ← eval(op)
#                          and Σ.regs[rsp] ← Σ.regs[rsp] - 8
#   pop: R → Σ             where Σ.regs[r] ← Σ.mem[Σ.regs[rsp]]
#                          and Σ.regs[rsp] ← Σ.regs[rsp] + 8
#   cmp: Operand × Operand → Σ   where flags ← compare(eval(a), eval(b))
#
# Jumps (∀ label L, address a = addr(L)):
#   jmp: L → Σ             where Σ.ip ← a
#   jz: L → Σ              where (ZF = 1) ⟹ Σ.ip ← a
#   jnz: L → Σ             where (ZF = 0) ⟹ Σ.ip ← a
#   jl: L → Σ              where (SF ≠ OF) ⟹ Σ.ip ← a
#
# Control Flow:
#   call: L → Σ            where Σ.mem[Σ.regs[rsp] - 8] ← Σ.ip + inst_len
#                          and Σ.regs[rsp] ← Σ.regs[rsp] - 8
#                          and Σ.ip ← a
#   ret: Σ                 where Σ.ip ← Σ.mem[Σ.regs[rsp]]
#                          and Σ.regs[rsp] ← Σ.regs[rsp] + 8
#
# Syscalls (n = Σ.regs[rax]):
#   syscall: Σ → Σ
#     n = 0  (sys_read):   (rdi=0) ⟹ Σ.mem[rsi..rsi+rdx] ← stdin
#                          and Σ.regs[rax] ← bytes_read
#     n = 1  (sys_write):  (rdi=1) ⟹ stdout ← Σ.mem[rsi..rsi+rdx]
#                          and Σ.regs[rax] ← bytes_written
#     n = 60 (sys_exit):   raise ProgramExit(Σ.regs[rdi])
#     else:                raise UnknownSyscall(n)
#
# Algebraic Properties:
#   add(a, b) ≡ add(b, a)          [commutativity]
#   add(add(a, b), c) ≡ add(a, add(b, c))   [associativity]
#   add(a, 0) ≡ a                  [identity]
#   sub(a, a) ≡ 0                  [inverse]
# ═══════════════════════════════════════════════════════════════
"""
ASM-SEM-001: Offline x86-64 semantics model
Status: RESEARCH — not part of MAL Parser or Compiler
Instructions: mov, add, sub, push, pop, cmp, test
Contract: Instruction × State ⇀ State + Error
"""
from dataclasses import dataclass, field
from typing import Dict, Optional, Tuple
# State space: Σ = Mem × Reg × Flags
MASK64 = (1 << 64) - 1
SIGN64 = 1 << 63
@dataclass
class State:
    """Complete machine state."""
    mem: Dict[int, int] = field(default_factory=dict)  # byte-addressable
    regs: Dict[str, int] = field(default_factory=lambda: {
        'rax': 0, 'rbx': 0, 'rcx': 0, 'rdx': 0,
        'rsi': 0, 'rdi': 0, 'rsp': 0x1000, 'rbp': 0,
        'r8': 0, 'r9': 0, 'r10': 0, 'r11': 0,
        'r12': 0, 'r13': 0, 'r14': 0, 'r15': 0,
    })
    flags: Dict[str, bool] = field(default_factory=lambda: {
        'CF': False, 'ZF': False, 'SF': False,
        'OF': False, 'PF': False, 'AF': False,
    })
    rip: int = 0  # instruction pointer
    labels: Dict[str, int] = field(default_factory=dict)  # label -> address
class Error(Exception):
    """Explicit failure — no silent success."""
    pass
class UnknownRegister(Error):
    pass
class InvalidMemory(Error):
    pass
def read_reg(s: State, name: str) -> int:
    if name not in s.regs:
        raise UnknownRegister(f"unknown register: {name}")
    return s.regs[name]
def write_reg(s: State, name: str, val: int) -> None:
    if name not in s.regs:
        raise UnknownRegister(f"unknown register: {name}")
    s.regs[name] = val & MASK64
def read_mem(s: State, addr: int, width: int = 8) -> int:
    val = 0
    for i in range(width):
        a = (addr + i) & MASK64
        if a not in s.mem:
            raise InvalidMemory(f"invalid memory address: 0x{a:x}")
        val |= s.mem[a] << (8 * i)
    return val
def write_mem(s: State, addr: int, val: int, width: int = 8) -> None:
    for i in range(width):
        a = (addr + i) & MASK64
        s.mem[a] = (val >> (8 * i)) & 0xFF
def parity(v: int) -> bool:
    """PF=1 if low byte has even number of set bits."""
    low = v & 0xFF
    bits = bin(low).count('1')
    return bits % 2 == 0
def set_flags_arith(s: State, a: int, b: int, r: int, is_add: bool) -> None:
    """Set flags for ADD/SUB."""
    s.flags['ZF'] = (r == 0)
    s.flags['SF'] = (r & SIGN64) != 0
    s.flags['PF'] = parity(r)
    s.flags['CF'] = (r < a) if is_add else (a < b)
    # OF: overflow if signs of inputs same but result differs
    sign_a = (a & SIGN64) != 0
    sign_b = (b & SIGN64) != 0
    sign_r = (r & SIGN64) != 0
    if is_add:
        s.flags['OF'] = (sign_a == sign_b) and (sign_r != sign_a)
    else:
        s.flags['OF'] = (sign_a != sign_b) and (sign_r != sign_a)
def set_flags_logic(s: State, r: int) -> None:
    """Set flags for AND/OR/XOR/TEST."""
    s.flags['ZF'] = (r == 0)
    s.flags['SF'] = (r & SIGN64) != 0
    s.flags['PF'] = parity(r)
    s.flags['CF'] = False
    s.flags['OF'] = False
# ─── Instructions ───────────────────────────────────────────
def mov(s: State, dst: str, src) -> None:
    """mov dst, src — dst can be register, src can be register or immediate."""
    val = read_reg(s, src) if isinstance(src, str) else src
    write_reg(s, dst, val)
def add(s: State, dst: str, src) -> None:
    """add: R × Operand → Σ where Σ.regs[r] ← Σ.regs[r] + eval(op)"""
    a = read_reg(s, dst)
    b = read_reg(s, src) if isinstance(src, str) else src
    r = (a + b) & MASK64
    write_reg(s, dst, r)
    set_flags_arith(s, a, b, r, is_add=True)
def sub(s: State, dst: str, src) -> None:
    """sub: R × Operand → Σ where Σ.regs[r] ← Σ.regs[r] - eval(op)"""
    a = read_reg(s, dst)
    b = read_reg(s, src) if isinstance(src, str) else src
    r = (a - b) & MASK64
    write_reg(s, dst, r)
    set_flags_arith(s, a, b, r, is_add=False)
def push(s: State, src) -> None:
    """push: Operand → Σ where Σ.mem[rsp-8] ← op and rsp ← rsp-8"""
    val = read_reg(s, src) if isinstance(src, str) else src
    new_rsp = (s.regs['rsp'] - 8) & MASK64
    write_reg(s, 'rsp', new_rsp)
    write_mem(s, new_rsp, val, 8)
def pop(s: State, dst: str) -> None:
    """pop: R → Σ where Σ.regs[r] ← Σ.mem[rsp] and rsp ← rsp+8"""
    val = read_mem(s, s.regs['rsp'], 8)
    write_reg(s, dst, val)
    s.regs['rsp'] = (s.regs['rsp'] + 8) & MASK64
def cmp(s: State, a_op, b_op) -> None:
    """cmp: Op × Op → Σ where flags ← compare(a, b)"""
    a = read_reg(s, a_op) if isinstance(a_op, str) else a_op
    b = read_reg(s, b_op) if isinstance(b_op, str) else b_op
    r = (a - b) & MASK64
    set_flags_arith(s, a, b, r, is_add=False)
    # Note: result is NOT stored, only flags are set
def test(s: State, a_op, b_op) -> None:
    a = read_reg(s, a_op) if isinstance(a_op, str) else a_op
    b = read_reg(s, b_op) if isinstance(b_op, str) else b_op
    r = a & b
    set_flags_logic(s, r)
    # Note: result is NOT stored, only flags are set

# ─── Control Flow Instructions (ASM-SEM-002) ───────────────
class InvalidJumpTarget(Error):

    """cmp: Op × Op → Σ where flags ← compare(a, b)"""
    """pop: R → Σ where Σ.regs[r] ← Σ.mem[rsp] and rsp ← rsp+8"""
    """push: Operand → Σ where Σ.mem[rsp-8] ← op and rsp ← rsp-8"""
    """sub: R × Operand → Σ where Σ.regs[r] ← Σ.regs[r] - eval(op)"""
    """add: R × Operand → Σ where Σ.regs[r] ← Σ.regs[r] + eval(op)"""
    """mov: R × Operand → Σ where Σ.regs[r] ← eval(operand)"""    pass
def jmp(s: State, target: str) -> None:
    """jmp target — unconditional jump."""
    if target not in s.labels:
        raise InvalidJumpTarget(f"invalid jump target: {target}")
    s.rip = s.labels[target]
def jcc(s: State, cond: str, target: str) -> None:
    """jcc cond, target — conditional jump.
    cond is one of: je, jne, jl, jge, jle, jg, jb, jae, jbe, ja, js, jns, jo, jno, jp, jnp
    """
    if target not in s.labels:
        raise InvalidJumpTarget(f"invalid jump target: {target}")
    f = s.flags
    conditions = {
        'je': f['ZF'],
        'jz': f['ZF'],
        'jne': not f['ZF'],
        'jnz': not f['ZF'],
        'jl': f['SF'] != f['OF'],
        'jnge': f['SF'] != f['OF'],
        'jge': f['SF'] == f['OF'],
        'jnl': f['SF'] == f['OF'],
        'jle': (f['SF'] != f['OF']) or f['ZF'],
        'jng': (f['SF'] != f['OF']) or f['ZF'],
        'jg': (f['SF'] == f['OF']) and not f['ZF'],
        'jnle': (f['SF'] == f['OF']) and not f['ZF'],
        'jb': f['CF'],
        'jc': f['CF'],
        'jae': not f['CF'],
        'jnc': not f['CF'],
        'jbe': f['CF'] or f['ZF'],
        'jna': f['CF'] or f['ZF'],
        'ja': not f['CF'] and not f['ZF'],
        'jnbe': not f['CF'] and not f['ZF'],
        'js': f['SF'],
        'jns': not f['SF'],
        'jo': f['OF'],
        'jno': not f['OF'],
        'jp': f['PF'],
        'jpe': f['PF'],
        'jnp': not f['PF'],
        'jpo': not f['PF'],
    }
    if cond not in conditions:
        raise Error(f"unknown condition: {cond}")
    if conditions[cond]:
        s.rip = s.labels[target]
    else:
        s.rip += 1
def call(s: State, target: str, return_addr: int = None) -> None:
    """call target — push return address and jump to target."""
    if target not in s.labels:
        raise InvalidJumpTarget(f"invalid call target: {target}")
    if return_addr is None:
        return_addr = s.rip + 1
    new_rsp = (s.regs['rsp'] - 8) & MASK64
    write_reg(s, 'rsp', new_rsp)
    write_mem(s, new_rsp, return_addr, 8)
    s.rip = s.labels[target]
def ret(s: State) -> None:
    """ret — pop return address and jump to it."""
    addr = read_mem(s, s.regs['rsp'], 8)
    s.regs['rsp'] = (s.regs['rsp'] + 8) & MASK64
    s.rip = addr
def loop(s: State, target: str) -> None:
    """loop target — decrement rcx, jump if rcx != 0."""
    if target not in s.labels:
        raise InvalidJumpTarget(f"invalid loop target: {target}")
    s.regs['rcx'] = (s.regs['rcx'] - 1) & MASK64
    if s.regs['rcx'] != 0:
        s.rip = s.labels[target]
    else:
        s.rip += 1

# ─── Syscall Instructions (ASM-SEM-003) ───────────────
class UnknownSyscall(Error):

    """ret: Σ where ip ← pop()"""
    """call: L → Σ where push(ip+len) and ip ← addr(L)"""
    """jmp: L → Σ where Σ.ip ← addr(L)"""    pass
class InvalidFD(Error):
    pass
class ProgramExit(Error):
    """Raised when sys_exit is called. Carries exit code."""
    def __init__(self, code):
        super().__init__(f'program exited with code {code}')
        self.code = code
def sys_read(s: State, fd: int, buf_addr: int, count: int) -> int:
    """sys_read (n=0): read count bytes from fd into memory at buf_addr."""
    if fd != 0:
        raise InvalidFD(f'invalid fd for read: {fd} (only stdin=0 supported)')
    if not hasattr(s, 'stdin_data') or not s.stdin_data:
        return 0
    to_read = min(count, len(s.stdin_data))
    data = s.stdin_data[:to_read]
    s.stdin_data = s.stdin_data[to_read:]
    for i, b in enumerate(data):

    """sys_read: fd × addr × count → Σ where mem[rsi..] ← stdin"""        s.mem[(buf_addr + i) & MASK64] = b if isinstance(b, int) else ord(b)
    return to_read
def sys_write(s: State, fd: int, buf_addr: int, count: int) -> int:
    """sys_write (n=1): write count bytes from memory at buf_addr to fd."""
    if fd not in (1, 2):

    """sys_write: fd × addr × count → Σ where stdout ← mem[rsi..]"""        raise InvalidFD(f'invalid fd for write: {fd} (only stdout=1, stderr=2 supported)')
    data = []
    for i in range(count):
        addr = (buf_addr + i) & MASK64
        if addr not in s.mem:
            raise InvalidMemory(f'invalid memory address: 0x{addr:x}')
        data.append(s.mem[addr])
    if not hasattr(s, 'stdout_data'):
        s.stdout_data = bytearray()
    if fd == 1:
        s.stdout_data.extend(data)
    else:
        if not hasattr(s, 'stderr_data'):
            s.stderr_data = bytearray()
        s.stderr_data.extend(data)
    return count
def sys_exit(s: State, code: int) -> None:
    """sys_exit (n=60): terminate program with exit code."""
    raise ProgramExit(code & MASK64)
def syscall(s: State) -> None:
    """syscall: dispatch based on rax value.
    - n=0: sys_read(fd=rdi, buf=rsi, count=rdx) -> rax=bytes_read
    - n=1: sys_write(fd=rdi, buf=rsi, count=rdx) -> rax=bytes_written
    - n=60: sys_exit(code=rdi) -> raises ProgramExit
    """
    n = s.regs['rax']
    if n == 0:
        result = sys_read(s, s.regs['rdi'], s.regs['rsi'], s.regs['rdx'])
        s.regs['rax'] = result
    elif n == 1:
        result = sys_write(s, s.regs['rdi'], s.regs['rsi'], s.regs['rdx'])
        s.regs['rax'] = result
    elif n == 60:
        sys_exit(s, s.regs['rdi'])
    else:
        raise UnknownSyscall(f'unknown syscall number: {n}')
