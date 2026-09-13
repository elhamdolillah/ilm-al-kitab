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
    a = read_reg(s, dst)
    b = read_reg(s, src) if isinstance(src, str) else src
    r = (a + b) & MASK64
    write_reg(s, dst, r)
    set_flags_arith(s, a, b, r, is_add=True)
def sub(s: State, dst: str, src) -> None:
    a = read_reg(s, dst)
    b = read_reg(s, src) if isinstance(src, str) else src
    r = (a - b) & MASK64
    write_reg(s, dst, r)
    set_flags_arith(s, a, b, r, is_add=False)
def push(s: State, src) -> None:
    val = read_reg(s, src) if isinstance(src, str) else src
    new_rsp = (s.regs['rsp'] - 8) & MASK64
    write_reg(s, 'rsp', new_rsp)
    write_mem(s, new_rsp, val, 8)
def pop(s: State, dst: str) -> None:
    val = read_mem(s, s.regs['rsp'], 8)
    write_reg(s, dst, val)
    s.regs['rsp'] = (s.regs['rsp'] + 8) & MASK64
def cmp(s: State, a_op, b_op) -> None:
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
