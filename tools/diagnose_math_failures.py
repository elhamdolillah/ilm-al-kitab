#!/usr/bin/env python3
import contextlib
import io
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path('/root/arabic_math_lang')
sys.path.insert(0, str(ROOT))
from math_complete import حلل_رموز, حلل_برنامج, compile_program

CASES = [
    ('t40_sqrt', '⎕ نص(جذر(144))', '12'),
    ('t40_sqrt2', '⎕ نص(جذر(100))', '10'),
    ('t40_sqrt3', '⎕ نص(جذر(3·3 + 4·4))', '5'),
    ('t40_floor', '⎕ نص(أرضية(37 ÷ 10))', '3'),
    ('t40_pow', '⎕ نص(قوة(2، 10))', '1024'),
    ('t40_pow2', '⎕ نص(قوة(3، 5))', '243'),
    ('t40_abs', '⎕ نص(مطلق(0 - 42))', '42'),
    ('t40_abs2', '⎕ نص(مطلق(42))', '42'),
    ('t41_basic', 'ق ≔ قناة()\nأرسل(ق، 42)\nر ≔ استقبل(ق)\n⎕ نص(ر)', '42'),
    ('t41_multi', 'ق ≔ قناة()\nأرسل(ق، 10)\nأرسل(ق، 20)\nأرسل(ق، 30)\nم ≔ استقبل(ق) + استقبل(ق) + استقبل(ق)\n⎕ نص(م)', '60'),
    ('t42_id_str', 'مطابقة ≡ λس. س\n⎕ مطابقة("مرحبا")', 'مرحبا'),
    ('t42_index', 'ق ≔ ⟨10، 20، 30، 40، 50⟩\n⎕ نص(ق[2])', '30'),
    ('t42_map', 'طبّق ≡ λ(د، ق). ﴿\n  ن ≔ ⟨⟩\n  ⋄ ∀ ع ∈ ق : ﴿ ن ≔ ألحق(ن، د(ع)) ﴾\n  ⋄ ن\n﴾\nمربع ≡ λس. س · س\nن ≔ طبّق(مربع، ⟨1، 2، 3، 4⟩)\n⎕ نص(مجموع_قائمة(ن))', '30'),
    ('t42_fold', 'اطوِ ≡ λ(د، بداية، ق). ﴿\n  ن ≔ بداية\n  ⋄ ∀ ع ∈ ق : ﴿ ن ≔ د(ن، ع) ﴾\n  ⋄ ن\n﴾\nجمع ≡ λ(أ، ب). أ + ب\nن ≔ اطوِ(جمع، 0، ⟨1، 2، 3، 4، 5⟩)\n⎕ نص(ن)', '15'),
]

def classify(exc: Exception) -> str:
    msg = str(exc)
    if 'رمز غير معروف' in msg or 'Unexpected' in msg or 'متوقع' in msg:
        return 'LEXER_OR_PARSER'
    if 'دالة غير معرفة' in msg:
        return 'COMPILER_BUILTIN_OR_RUNTIME'
    return 'COMPILE_OR_RUNTIME'

print('=== MATH_COMPLETE FAILURE DIAGNOSTIC ===')
print(f'cases={len(CASES)}')
counts = {}
for name, source, expected in CASES:
    print(f'\n--- {name} ---')
    print(f'expected={expected!r}')
    try:
        tokens = حلل_رموز(source)
        print(f'tokens=OK count={len(tokens)}')
        program = حلل_برنامج(tokens)
        print('program=OK')
        asm = compile_program(program)
        print(f'compile=OK asm_bytes={len(asm.encode("utf-8"))}')
        with tempfile.TemporaryDirectory(prefix='mal_diag_') as td:
            base = Path(td) / name
            asm_path = base.with_suffix('.asm')
            obj_path = base.with_suffix('.o')
            asm_path.write_text(asm, encoding='utf-8')
            nasm = subprocess.run(['nasm', '-f', 'elf64', str(asm_path), '-o', str(obj_path)], capture_output=True, text=True)
            print(f'nasm_exit={nasm.returncode}')
            if nasm.returncode:
                print('nasm_stderr=', nasm.stderr.strip()[:1000])
                raise RuntimeError('nasm failed')
            link = subprocess.run(['ld', str(obj_path), '-o', str(base)], capture_output=True, text=True)
            print(f'ld_exit={link.returncode}')
            if link.returncode:
                print('ld_stderr=', link.stderr.strip()[:1000])
                raise RuntimeError('ld failed')
            run = subprocess.run([str(base)], capture_output=True, text=True, timeout=10)
            print(f'run_exit={run.returncode}')
            print(f'run_stdout={run.stdout.strip()!r}')
            print(f'run_stderr={run.stderr.strip()!r}')
            print(f'classification={"PASS" if run.stdout.strip() == expected else "RUNTIME_OUTPUT_MISMATCH"}')
            counts['RUNTIME_OUTPUT_MISMATCH' if run.stdout.strip() != expected else 'PASS'] = counts.get('RUNTIME_OUTPUT_MISMATCH' if run.stdout.strip() != expected else 'PASS', 0) + 1
    except Exception as exc:
        category = classify(exc)
        counts[category] = counts.get(category, 0) + 1
        print(f'exception_type={type(exc).__name__}')
        print(f'exception={exc}')
        print(f'classification={category}')
print('\n=== SUMMARY ===')
for key in sorted(counts):
    print(f'{key}={counts[key]}')
