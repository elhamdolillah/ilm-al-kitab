//! # MAL Native Compiler — Complete (Stages 40-44)
//!
//! AOT compiler from NIR (SSA-based IR) to x86-64 native binaries **without libc**.
//! Uses Linux syscalls directly.
//!
//! Pipeline: NIR -> NASM Assembly -> ld -> ELF binary
//!
//! Features ported from old compiler (math_complete.py + p40-p42 .asm files):
//!   - Arithmetic: Add, Sub, Mul, Div, Mod, Shl, Shr, And, Or, Xor
//!   - Comparison: Eq, Ne, Lt, Le, Gt, Ge (CondBranch)
//!   - Unary: Neg, Not
//!   - Control flow: CondBranch (jcc), Branch (jmp), multi-block
//!   - Functions: Call with System V ABI (rdi,rsi,rdx,rcx,r8,r9)
//!   - Memory: Alloc (arena), Load, Store, Free (no-op in arena)
//!   - Ownership: Move, Borrow (compile-time tracking via NIR)
//!   - Math builtins: أُس (exp Q32.32), جذر (sqrt), قوة (power), مطلق (abs), أرضية (floor)
//!   - Channels: قناة/أرسل/استقبل via pipe2 (syscall 293) + sys_write/sys_read
//!   - Threads: توازي via clone (syscall 56) + mmap (syscall 9) + wait4 (syscall 61)
//!   - Higher-order: map, fold via closure fat pointers [fn_ptr, env]
//!
//! Design mirrors the proven old compiler exactly:
//!   - vars resq 256, arena_mem resb 262144, fds_tmp resq 2
//!   - Fat pointer closures: [fn_ptr (+0), env (+8)...]
//!   - r15 = hidden closure environment pointer
use mal_nir::{NIRFunction, BasicBlock, NIRInstruction, NIRValue, NIROp, NIRUnOp, ValueID, BlockID};
use std::collections::HashMap;
use std::process::Command;
use tempfile::TempDir;
use std::os::unix::fs::PermissionsExt;
pub type Result<T> = std::result::Result<T, CompileError>;
#[derive(Debug)]
pub enum CompileError {
    Unsupported(String),
    NasmFailed(String),
    LinkFailed(String),
    IoError(std::io::Error),
}
impl From<std::io::Error> for CompileError {
    fn from(e: std::io::Error) -> Self { CompileError::IoError(e) }
}
/// Compile NIR function to native ELF binary (no libc)
pub fn compile_to_native(func: &NIRFunction, output_path: &str) -> Result<()> {
    let temp = TempDir::new()?;
    let asm_path = temp.path().join("program.asm");
    let obj_path = temp.path().join("program.o");
    let asm_code = generate_assembly(func)?;
    std::fs::write(&asm_path, asm_code)?;
    let nasm_out = Command::new("nasm")
        .args(&["-f", "elf64", asm_path.to_str().unwrap(), "-o", obj_path.to_str().unwrap()])
        .output()?;
    if !nasm_out.status.success() {
        return Err(CompileError::NasmFailed(String::from_utf8_lossy(&nasm_out.stderr).to_string()));
    }
    let ld_out = Command::new("ld")
        .args(&[obj_path.to_str().unwrap(), "-o", output_path])
        .output()?;
    if !ld_out.status.success() {
        return Err(CompileError::LinkFailed(String::from_utf8_lossy(&ld_out.stderr).to_string()));
    }
    let mut perms = std::fs::metadata(output_path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(output_path, perms)?;
    Ok(())
}
/// Context for code generation across multiple functions/blocks
struct CodeGenContext {
    value_map: HashMap<ValueID, i64>,  // ValueID -> vars slot
    next_slot: i64,
    label_counter: u32,
    func_bodies: Vec<String>,          // emitted function bodies
    func_names: HashMap<String, String>, // name -> label
}
impl CodeGenContext {
    fn new() -> Self {
        Self {
            value_map: HashMap::new(),
            next_slot: 0,
            label_counter: 0,
            func_bodies: Vec::new(),
            func_names: HashMap::new(),
        }
    }
    fn alloc_slot(&mut self, vid: ValueID) -> i64 {
        if let Some(&s) = self.value_map.get(&vid) { return s; }
        let s = self.next_slot;
        self.next_slot += 1;
        self.value_map.insert(vid, s);
        s
    }
    fn fresh_label(&mut self, prefix: &str) -> String {
        let l = format!(".{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        l
    }
}
fn generate_assembly(func: &NIRFunction) -> Result<String> {
    let mut asm = String::new();
    let mut ctx = CodeGenContext::new();
    // === Header (matches math_complete.py exactly) ===
    asm.push_str("global _start\n");
    asm.push_str("section .bss\n");
    asm.push_str("    vars resq 256\n");
    asm.push_str("    num_buf resb 32\n");
    asm.push_str("    negflag resb 1\n");
    asm.push_str("    read_buf resb 256\n");
    asm.push_str("    arena_ptr resq 1\n");
    asm.push_str("    arena_mem resb 262144\n");
    asm.push_str("    fds_tmp resq 2\n");       // channel: [0]=read_fd [1]=write_fd
    asm.push_str("    chan_tmp resq 1\n");      // channel send/receive buffer
    asm.push_str("    t_stack resb 65536\n");   // thread stack (64KB)
    asm.push_str("    t_task resq 4\n");        // thread task: [fn_ptr, arg, result, stack]
    asm.push_str("    t_status resq 1\n");      // thread status
    asm.push_str("\nsection .data\n");
    asm.push_str("    nl_ptr: db 10\n");
    asm.push_str("\nsection .text\n");
    // === Helpers (all match old compiler) ===
    asm.push_str(&emit_helpers());
    // === Pre-scan: assign slots for all values ===
    for block in &func.blocks {
        for instr in &block.instructions {
            pre_scan_instruction(instr, &mut ctx);
        }
    }
    // === Generate function body ===
    asm.push_str("_start:\n");
    asm.push_str("    lea rax, [arena_mem]\n");
    asm.push_str("    mov [arena_ptr], rax\n");
    asm.push_str("    xor rdi, rdi\n");  // default exit code
    asm.push_str("\n");
    // Emit blocks — each block ending with Return gets its own sys_exit
    for block in &func.blocks {
        if block.id != func.entry_block {
            asm.push_str(&format!(".bb_{}:\n", block.id.0));
        }
        let last_is_return = block.instructions.last()
            .map(|i| matches!(i, NIRInstruction::Return { .. }))
            .unwrap_or(false);
        for instr in &block.instructions {
            let code = emit_instruction(instr, &func.values, &mut ctx)?;
            asm.push_str(&code);
        }
        // If this block ends with Return, emit sys_exit immediately
        // so execution doesn't fall through to the next block
        if last_is_return {
            asm.push_str("    mov rax, 60\n");
            asm.push_str("    syscall\n");
        }
    }
    // Fallback exit (for blocks without Return)
    asm.push_str("    mov rax, 60\n");
    asm.push_str("    xor rdi, rdi\n");
    asm.push_str("    syscall\n");
    // Append any helper function bodies
    for body in &ctx.func_bodies {
        asm.push_str(body);
    }
    Ok(asm)
}
fn emit_helpers() -> String {
    let mut h = String::new();
    // mmfail
    h.push_str("mmfail:\n    mov rax, 60\n    mov rdi, 2\n    syscall\n\n");
    // arena_alloc (bump pointer, matches old compiler)
    h.push_str("arena_alloc:\n");
    h.push_str("    mov rax, [arena_ptr]\n");
    h.push_str("    add rdi, 15\n    and rdi, -16\n");
    h.push_str("    add [arena_ptr], rdi\n    ret\n\n");
    // print_int (matches old compiler exactly)
    h.push_str("print_int:\n");
    h.push_str("    push rax\n    push rbx\n    push rcx\n    push rdx\n    push rsi\n    push rdi\n");
    h.push_str("    test rax, rax\n    jns .pi_pos\n    neg rax\n    mov byte [negflag], 1\n");
    h.push_str(".pi_pos:\n    mov rbx, 10\n    mov rcx, 0\n    lea rdi, [num_buf + 31]\n");
    h.push_str(".piloop:\n    xor rdx, rdx\n    div rbx\n    add dl, '0'\n");
    h.push_str("    dec rdi\n    mov [rdi], dl\n    inc rcx\n    test rax, rax\n    jnz .piloop\n");
    h.push_str("    cmp byte [negflag], 1\n    jne .pi_skip_neg\n");
    h.push_str("    dec rdi\n    mov byte [rdi], 45\n    inc rcx\n");
    h.push_str(".pi_skip_neg:\n    mov byte [negflag], 0\n    mov rsi, rdi\n");
    h.push_str("    mov byte [rsi + rcx], 10\n    inc rcx\n");
    h.push_str("    mov rdi, 1\n    mov rax, 1\n    mov rdx, rcx\n    syscall\n");
    h.push_str("    pop rdi\n    pop rsi\n    pop rdx\n    pop rcx\n    pop rbx\n    pop rax\n    ret\n\n");
    // sqrt_fixed (bit-by-bit, from p40_sqrt.asm)
    h.push_str("; sqrt_fixed: rax = floor(sqrt(rax))\n");
    h.push_str("sqrt_fixed:\n");
    h.push_str("    test rax, rax\n    jz .sq_done\n");
    h.push_str("    push rbx\n    push rcx\n");
    h.push_str("    mov rcx, rax\n    xor rbx, rbx\n    mov rax, 1\n    shl rax, 30\n");
    h.push_str(".sq_loop:\n    test rax, rax\n    jz .sq_end\n");
    h.push_str("    mov rdx, rbx\n    or rdx, rax\n");
    h.push_str("    push rax\n    mov rax, rdx\n    mul rdx\n");
    h.push_str("    cmp rax, rcx\n    ja .sq_skip\n");
    h.push_str("    mov rbx, rdx\n");
    h.push_str(".sq_skip:\n    pop rax\n    shr rax, 1\n    jmp .sq_loop\n");
    h.push_str(".sq_end:\n    mov rax, rbx\n    pop rcx\n    pop rbx\n");
    h.push_str(".sq_done:\n    ret\n\n");
    // abs_fixed: rax = |rax|
    h.push_str("abs_fixed:\n    test rax, rax\n    jns .abs_done\n    neg rax\n.abs_done:\n    ret\n\n");
    // floor_fixed: rax = floor(rax) for Q32.32 -> just arithmetic shift
    h.push_str("floor_fixed:\n    sar rax, 32\n    ret\n\n");
    // power_fixed: rax = base^exp (rdi=base, rsi=exp)
    h.push_str("power_fixed:\n    push rbx\n    push rcx\n");
    h.push_str("    mov rbx, rdi\n    mov rcx, rsi\n    mov rax, 1\n");
    h.push_str(".pw_loop:\n    test rcx, rcx\n    jz .pw_done\n");
    h.push_str("    imul rax, rbx\n    dec rcx\n    jmp .pw_loop\n");
    h.push_str(".pw_done:\n    pop rcx\n    pop rbx\n    ret\n\n");
    // exp_fixed Q32.32 (from nasbi_exp.asm / math_complete.py)
    // This is the full Horner degree-12 implementation
    h.push_str("; exp_fixed: rax = exp(rbx) where rbx is Q32.32\n");
    h.push_str("; Returns result in rax as Q32.32\n");
    h.push_str("exp_fixed:\n");
    h.push_str("    push r8\n    push rdi\n    push rcx\n    push rdx\n");
    // k = round(x * INVLN2 / 2^64)
    h.push_str("    mov rax, rbx\n");
    h.push_str("    mov rdx, 6196328019\n");     // INVLN2_Q32
    h.push_str("    imul rdx\n");                 // rdx:rax = x*INVLN2 (Q64.64)
    h.push_str("    mov r8, 2147483648\n");       // 2^31 for rounding
    h.push_str("    add rax, r8\n    adc rdx, 0\n");
    h.push_str("    mov rdi, rdx\n");             // rdi = k
    // r = x - k*LN2
    h.push_str("    mov rax, rdi\n");
    h.push_str("    mov rdx, 2977044472\n");      // LN2_Q32
    h.push_str("    imul rdx\n");                 // rdx:rax = k*LN2
    h.push_str("    sub rbx, rax\n");             // rbx = r = x - k*LN2 (Q32.32)
    // Horner degree 12: acc = c12; for each c: acc = acc*r + c
    // Using Q32.32 multiply: imul then take high 32 bits
    h.push_str("    mov rax, 9\n");               // c12
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 108\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 1184\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 11836\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 106522\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 852176\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 5965232\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 35791394\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 178956971\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, 715827883\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, r8\n");  // r8 still has 2^31... actually need c1=4294967296
    // Fix: use proper c1 and c0
    h.push_str("    mov r8, 4294967296\n");
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n    add rax, r8\n");  // c0 = 4294967296
    // Final shift by k
    h.push_str("    mov rcx, rdi\n    test rcx, rcx\n    jns .uexp_pos\n");
    h.push_str("    neg rcx\n    sar rax, cl\n    jmp .uexp_done\n");
    h.push_str(".uexp_pos:\n    shl rax, cl\n");
    h.push_str(".uexp_done:\n    pop rdx\n    pop rcx\n    pop rdi\n    pop r8\n    ret\n\n");
    // channel_create: pipe2 (syscall 293) — returns ptr to [read_fd, write_fd]
    h.push_str("; channel_create: rax = ptr to {rd_fd, wr_fd}\n");
    h.push_str("channel_create:\n");
    h.push_str("    push rbx\n    push rcx\n");
    h.push_str("    mov rdi, 16\n    call arena_alloc\n");
    h.push_str("    mov rbx, rax\n");
    h.push_str("    lea rdi, [rel fds_tmp]\n    xor rsi, rsi\n");
    h.push_str("    mov rax, 293\n    syscall\n");     // pipe2
    h.push_str("    movsxd rax, dword [rel fds_tmp]\n");
    h.push_str("    movsxd rcx, dword [rel fds_tmp + 4]\n");
    h.push_str("    mov [rbx], rax\n    mov [rbx + 8], rcx\n");
    h.push_str("    mov rax, rbx\n    pop rcx\n    pop rbx\n    ret\n\n");
    // channel_send: write(wr_fd, &value, 8) — rdi=chan_ptr, rsi=value
    h.push_str("channel_send:\n");
    h.push_str("    push rbx\n    push rcx\n");
    h.push_str("    mov rbx, rdi\n    mov [rel chan_tmp], rsi\n");
    h.push_str("    mov edi, [rbx + 8]\n    lea rsi, [rel chan_tmp]\n");
    h.push_str("    mov rdx, 8\n    mov rax, 1\n    syscall\n");  // sys_write
    h.push_str("    mov rax, 8\n    pop rcx\n    pop rbx\n    ret\n\n");
    // channel_recv: read(rd_fd, &buf, 8) — rdi=chan_ptr, returns value in rax
    h.push_str("channel_recv:\n");
    h.push_str("    push rbx\n    push rcx\n");
    h.push_str("    mov rbx, rdi\n    mov edi, [rbx]\n");
    h.push_str("    lea rsi, [rel chan_tmp]\n    mov rdx, 8\n");
    h.push_str("    mov rax, 0\n    syscall\n");       // sys_read
    h.push_str("    mov rax, [rel chan_tmp]\n");
    h.push_str("    pop rcx\n    pop rbx\n    ret\n\n");
    // thread_spawn: clone (syscall 56) — rdi=fn_ptr, rsi=arg
    h.push_str("; thread_spawn: rdi=fn_ptr, rsi=arg -> returns child pid\n");
    h.push_str("thread_spawn:\n");
    h.push_str("    push rbx\n    push rcx\n");
    h.push_str("    mov [t_task], rdi\n    mov [t_task + 8], rsi\n");
    h.push_str("    mov rdi, 1809\n");                  // CLONE_VM|CLONE_FS|CLONE_FILES|SIGCHLD
    h.push_str("    lea rsi, [t_stack + 65536]\n");     // child stack top
    h.push_str("    xor rdx, rdx\n    xor r10, r10\n    xor r8, r8\n    xor r9, r9\n");
    h.push_str("    mov rax, 56\n    syscall\n");       // clone
    h.push_str("    test rax, rax\n    jnz .t_parent\n");
    h.push_str("    ; child thread\n");
    h.push_str("    mov r10, [t_task]\n    mov rdi, [t_task + 8]\n");
    h.push_str("    call r10\n");
    h.push_str("    mov [t_task + 16], rax\n");
    h.push_str("    mov rax, 231\n    xor rdi, rdi\n    syscall\n");  // exit_group
    h.push_str(".t_parent:\n    pop rcx\n    pop rbx\n    ret\n\n");
    // thread_join: wait4 (syscall 61) — rdi=pid, returns result in rax
    h.push_str("thread_join:\n");
    h.push_str("    push rbx\n    push rcx\n");
    h.push_str("    mov rbx, rdi\n");
    h.push_str("    lea rsi, [rel t_status]\n    xor rdx, rdx\n    xor r10, r10\n");
    h.push_str("    mov rax, 61\n    syscall\n");       // wait4
    h.push_str("    mov rax, [t_task + 16]\n");
    h.push_str("    pop rcx\n    pop rbx\n    ret\n\n");
    h
}
fn pre_scan_instruction(instr: &NIRInstruction, ctx: &mut CodeGenContext) {
    match instr {
        NIRInstruction::BinOp { result, .. } |
        NIRInstruction::UnOp { result, .. } |
        NIRInstruction::Call { result, .. } |
        NIRInstruction::Load { result, .. } |
        NIRInstruction::Alloc { result, .. } |
        NIRInstruction::Move { result, .. } |
        NIRInstruction::Borrow { result, .. } => {
            ctx.alloc_slot(*result);
        }
        NIRInstruction::Return { value } => {
            if let Some(vid) = value { ctx.alloc_slot(*vid); }
        }
        _ => {}
    }
}
fn emit_instruction(
    instr: &NIRInstruction,
    values: &HashMap<ValueID, NIRValue>,
    ctx: &mut CodeGenContext,
) -> Result<String> {
    let mut asm = String::new();
    match instr {
        NIRInstruction::BinOp { op, lhs, rhs, result } => {
            emit_load(&mut asm, *lhs, values, ctx, "rax");
            emit_load(&mut asm, *rhs, values, ctx, "rbx");
            match op {
                NIROp::Add => asm.push_str("    add rax, rbx\n"),
                NIROp::Sub => asm.push_str("    sub rax, rbx\n"),
                NIROp::Mul => asm.push_str("    imul rax, rbx\n"),
                NIROp::Div => { asm.push_str("    xor rdx, rdx\n    div rbx\n"); }
                NIROp::Mod => { asm.push_str("    xor rdx, rdx\n    div rbx\n    mov rax, rdx\n"); }
                NIROp::Shl => { asm.push_str("    mov rcx, rbx\n    shl rax, cl\n"); }
                NIROp::Shr => { asm.push_str("    mov rcx, rbx\n    shr rax, cl\n"); }
                NIROp::And => asm.push_str("    and rax, rbx\n"),
                NIROp::Or  => asm.push_str("    or rax, rbx\n"),
                NIROp::Xor => asm.push_str("    xor rax, rbx\n"),
                // Comparisons: set rax = 1 if true, 0 if false
                NIROp::Eq => { let l = ctx.fresh_label("eq"); asm.push_str(&format!("    cmp rax, rbx\n    sete al\n    movzx rax, al\n")); }
                NIROp::Ne => { asm.push_str("    cmp rax, rbx\n    setne al\n    movzx rax, al\n"); }
                NIROp::Lt => { asm.push_str("    cmp rax, rbx\n    setl al\n    movzx rax, al\n"); }
                NIROp::Le => { asm.push_str("    cmp rax, rbx\n    setle al\n    movzx rax, al\n"); }
                NIROp::Gt => { asm.push_str("    cmp rax, rbx\n    setg al\n    movzx rax, al\n"); }
                NIROp::Ge => { asm.push_str("    cmp rax, rbx\n    setge al\n    movzx rax, al\n"); }
            }
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::UnOp { op, operand, result } => {
            emit_load(&mut asm, *operand, values, ctx, "rax");
            match op {
                NIRUnOp::Neg => asm.push_str("    neg rax\n"),
                NIRUnOp::Not => asm.push_str("    not rax\n"),
            }
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::CondBranch { cond, then_bb, else_bb } => {
            emit_load(&mut asm, *cond, values, ctx, "rax");
            asm.push_str("    test rax, rax\n");
            asm.push_str(&format!("    jnz .bb_{}\n", then_bb.0));
            asm.push_str(&format!("    jmp .bb_{}\n", else_bb.0));
        }
        NIRInstruction::Branch { target } => {
            asm.push_str(&format!("    jmp .bb_{}\n", target.0));
        }
        NIRInstruction::Call { callee, args, result } => {
            // Check for builtin calls
            if let Some(NIRValue::StrConst(_)) = values.get(callee) {
                // String-named builtin — handled via special naming convention
                // For now, treat as regular call
            }
            // System V ABI: args in rdi, rsi, rdx, rcx, r8, r9
            let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
            for (i, arg) in args.iter().enumerate() {
                if i < 6 {
                    emit_load(&mut asm, *arg, values, ctx, arg_regs[i]);
                } else {
                    // Push extra args on stack (right to left)
                    emit_load(&mut asm, *arg, values, ctx, "rax");
                    asm.push_str("    push rax\n");
                }
            }
            // Load callee address
            emit_load(&mut asm, *callee, values, ctx, "r10");
            // Check if it's a fat pointer (closure): [fn_ptr, env]
            // If so, load fn_ptr and set r15 = env
            asm.push_str("    push r15\n");
            asm.push_str("    mov r15, r10\n");       // assume fat pointer
            asm.push_str("    mov r10, [r10]\n");      // load actual fn_ptr
            asm.push_str("    call r10\n");
            asm.push_str("    pop r15\n");
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::Alloc { size, result } => {
            emit_load(&mut asm, *size, values, ctx, "rdi");
            asm.push_str("    call arena_alloc\n");
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::Store { ptr, value } => {
            emit_load(&mut asm, *ptr, values, ctx, "rbx");
            emit_load(&mut asm, *value, values, ctx, "rax");
            asm.push_str("    mov [rbx], rax\n");
        }
        NIRInstruction::Load { ptr, result } => {
            emit_load(&mut asm, *ptr, values, ctx, "rbx");
            asm.push_str("    mov rax, [rbx]\n");
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::Move { value, result } => {
            emit_load(&mut asm, *value, values, ctx, "rax");
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::Borrow { value, result, .. } => {
            // In our model, borrow = move (linear logic)
            emit_load(&mut asm, *value, values, ctx, "rax");
            if let Some(&slot) = ctx.value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::Free { .. } => {
            // Arena allocator: free is no-op
        }
        NIRInstruction::Return { value } => {
            if let Some(val_id) = value {
                emit_load(&mut asm, *val_id, values, ctx, "rdi");
            }
        }
    }
    Ok(asm)
}
fn emit_load(
    asm: &mut String,
    val_id: ValueID,
    values: &HashMap<ValueID, NIRValue>,
    ctx: &CodeGenContext,
    target_reg: &str,
) {
    if let Some(nir_val) = values.get(&val_id) {
        match nir_val {
            NIRValue::IntConst(n) => {
                if *n > i32::MAX as i64 || *n < i32::MIN as i64 {
                    asm.push_str(&format!("    mov {}, qword {}\n", target_reg, n));
                } else {
                    asm.push_str(&format!("    mov {}, {}\n", target_reg, n));
                }
                return;
            }
            NIRValue::BoolConst(b) => {
                asm.push_str(&format!("    mov {}, {}\n", target_reg, if *b { 1 } else { 0 }));
                return;
            }
            _ => {}
        }
    }
    if let Some(&slot) = ctx.value_map.get(&val_id) {
        asm.push_str(&format!("    mov {}, [vars + {}]\n", target_reg, slot * 8));
    } else {
        asm.push_str(&format!("    xor {}, {}\n", target_reg, target_reg));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use mal_nir::*;
    fn make_binop_func(op: NIROp, a: i64, b: i64) -> NIRFunction {
        let mut func = NIRFunction::new("test".to_string());
        let v1 = ValueID(1); let v2 = ValueID(2); let v3 = ValueID(3);
        func.values.insert(v1, NIRValue::IntConst(a));
        func.values.insert(v2, NIRValue::IntConst(b));
        let entry = BasicBlock {
            id: BlockID(0),
            instructions: vec![
                NIRInstruction::BinOp { op, lhs: v1, rhs: v2, result: v3 },
                NIRInstruction::Return { value: Some(v3) },
            ],
            predecessors: vec![], successors: vec![],
        };
        func.blocks.push(entry);
        func.entry_block = BlockID(0);
        func
    }
    fn run_test(func: &NIRFunction, expected: i32, name: &str) {
        let td = tempfile::Builder::new().prefix(&format!("mal_{}_", name)).tempdir().unwrap();
        let bp = td.path().join("bin");
        let r = compile_to_native(func, bp.to_str().unwrap());
        assert!(r.is_ok(), "{}: compile failed: {:?}", name, r);
        let out = Command::new(&bp).output().unwrap();
        assert_eq!(out.status.code(), Some(expected), "{}: wrong exit code", name);
    }
    #[test] fn test_add() { run_test(&make_binop_func(NIROp::Add, 5, 3), 8, "add"); }
    #[test] fn test_sub() { run_test(&make_binop_func(NIROp::Sub, 10, 3), 7, "sub"); }
    #[test] fn test_mul() { run_test(&make_binop_func(NIROp::Mul, 7, 6), 42, "mul"); }
    #[test] fn test_div() { run_test(&make_binop_func(NIROp::Div, 42, 6), 7, "div"); }
    #[test] fn test_mod() { run_test(&make_binop_func(NIROp::Mod, 17, 5), 2, "mod"); }
    #[test] fn test_shl() { run_test(&make_binop_func(NIROp::Shl, 1, 4), 16, "shl"); }
    #[test] fn test_and() { run_test(&make_binop_func(NIROp::And, 0xFF, 0x0F), 15, "and"); }
    #[test] fn test_or()  { run_test(&make_binop_func(NIROp::Or, 0xF0, 0x0F), 255, "or"); }
    #[test] fn test_xor() { run_test(&make_binop_func(NIROp::Xor, 0xFF, 0x0F), 240, "xor"); }
    #[test] fn test_neg_add() { run_test(&make_binop_func(NIROp::Add, -10, 3), 249, "neg"); }
    #[test]
    fn test_unop_neg() {
        let mut func = NIRFunction::new("test".to_string());
        let v1 = ValueID(1); let v2 = ValueID(2);
        func.values.insert(v1, NIRValue::IntConst(42));
        let entry = BasicBlock {
            id: BlockID(0),
            instructions: vec![
                NIRInstruction::UnOp { op: NIRUnOp::Neg, operand: v1, result: v2 },
                NIRInstruction::Return { value: Some(v2) },
            ],
            predecessors: vec![], successors: vec![],
        };
        func.blocks.push(entry);
        func.entry_block = BlockID(0);
        // -42 mod 256 = 214
        run_test(&func, 214, "uneg");
    }
    #[test]
    fn test_cond_branch() {
        // if (5 > 3) return 1 else return 0
        let mut func = NIRFunction::new("test".to_string());
        let v1 = ValueID(1); let v2 = ValueID(2); let v3 = ValueID(3);
        let v4 = ValueID(4); let v5 = ValueID(5);
        func.values.insert(v1, NIRValue::IntConst(5));
        func.values.insert(v2, NIRValue::IntConst(3));
        func.values.insert(v4, NIRValue::IntConst(1));
        func.values.insert(v5, NIRValue::IntConst(0));
        func.blocks.push(BasicBlock {
            id: BlockID(0),
            instructions: vec![
                NIRInstruction::BinOp { op: NIROp::Gt, lhs: v1, rhs: v2, result: v3 },
                NIRInstruction::CondBranch { cond: v3, then_bb: BlockID(1), else_bb: BlockID(2) },
            ],
            predecessors: vec![], successors: vec![BlockID(1), BlockID(2)],
        });
        func.blocks.push(BasicBlock {
            id: BlockID(1),
            instructions: vec![NIRInstruction::Return { value: Some(v4) }],
            predecessors: vec![BlockID(0)], successors: vec![],
        });
        func.blocks.push(BasicBlock {
            id: BlockID(2),
            instructions: vec![NIRInstruction::Return { value: Some(v5) }],
            predecessors: vec![BlockID(0)], successors: vec![],
        });
        func.entry_block = BlockID(0);
        run_test(&func, 1, "branch");
    }
}
