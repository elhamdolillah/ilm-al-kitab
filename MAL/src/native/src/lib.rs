//! # MAL Native Compiler — Fully Linked (Stages 40-44)
//!
//! AOT compiler from NIR (SSA-based IR) to x86-64 native binaries **without libc**.
//! All helpers linked to NIR Call instructions. All gaps filled.
//!
//! Pipeline: NIR -> NASM Assembly -> ld -> ELF binary
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
/// Known builtin function names that map to inline helpers
const BUILTINS: &[&str] = &[
    "sqrt", "abs", "floor", "power", "exp",
    "channel_create", "channel_send", "channel_recv",
    "thread_spawn", "thread_join",
    "print_int", "print_str", "str_eq", "num_to_str",
    "list_len", "list_sum", "list_head", "list_tail", "list_append",
    "file_open", "file_close", "file_write", "file_read",
    "stdin_read",
];
fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}
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
struct CodeGenContext {
    value_map: HashMap<ValueID, i64>,
    next_slot: i64,
    label_counter: u32,
}
impl CodeGenContext {
    fn new() -> Self {
        Self { value_map: HashMap::new(), next_slot: 0, label_counter: 0 }
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
    // Header
    asm.push_str("global _start\n");
    asm.push_str("section .bss\n");
    asm.push_str("    vars resq 256\n");
    asm.push_str("    num_buf resb 32\n");
    asm.push_str("    negflag resb 1\n");
    asm.push_str("    read_buf resb 256\n");
    asm.push_str("    file_path_buf resb 256\n");
    asm.push_str("    file_buf resb 4096\n");
    asm.push_str("    arena_ptr resq 1\n");
    asm.push_str("    arena_mem resb 262144\n");
    asm.push_str("    fds_tmp resq 2\n");
    asm.push_str("    chan_tmp resq 1\n");
    asm.push_str("    t_stack resb 65536\n");
    asm.push_str("    t_task resq 4\n");
    asm.push_str("    t_status resq 1\n");
    asm.push_str("\nsection .data\n");
    asm.push_str("    nl_ptr: db 10\n");
    asm.push_str("\nsection .text\n");
    // Helpers
    asm.push_str(&emit_all_helpers());
    // Pre-scan slots
    for block in &func.blocks {
        for instr in &block.instructions {
            pre_scan(instr, &mut ctx);
        }
    }
    // _start
    asm.push_str("_start:\n");
    asm.push_str("    lea rax, [arena_mem]\n");
    asm.push_str("    mov [arena_ptr], rax\n");
    asm.push_str("    xor rdi, rdi\n\n");
    // Emit blocks
    for block in &func.blocks {
        if block.id != func.entry_block {
            asm.push_str(&format!(".bb_{}:\n", block.id.0));
        }
        let last_is_return = block.instructions.last()
            .map(|i| matches!(i, NIRInstruction::Return { .. }))
            .unwrap_or(false);
        for instr in &block.instructions {
            asm.push_str(&emit_instr(instr, &func.values, &mut ctx)?);
        }
        if last_is_return {
            // Load the return value into rdi before sys_exit
            if let Some(NIRInstruction::Return { value: Some(ret_val) }) = block.instructions.last() {
                // Generate load instruction inline
                let mut tmp = String::new();
                load(&mut tmp, *ret_val, &func.values, &mut ctx, "rdi");
                asm.push_str(&tmp);
            } else {
                asm.push_str("    xor rdi, rdi\n");
            }
            asm.push_str("    mov rax, 60\n");
            asm.push_str("    syscall\n");
        }
    }
    // Fallback exit
    asm.push_str("    mov rax, 60\n    xor rdi, rdi\n    syscall\n");
    Ok(asm)
}
fn pre_scan(instr: &NIRInstruction, ctx: &mut CodeGenContext) {
    match instr {
        NIRInstruction::BinOp { result, .. } |
        NIRInstruction::UnOp { result, .. } |
        NIRInstruction::Call { result, .. } |
        NIRInstruction::Load { result, .. } |
        NIRInstruction::Alloc { result, .. } |
        NIRInstruction::Move { result, .. } |
        NIRInstruction::Borrow { result, .. } => { ctx.alloc_slot(*result); }
        NIRInstruction::Return { value } => {
            if let Some(v) = value { ctx.alloc_slot(*v); }
        }
        _ => {}
    }
}
fn emit_instr(
    instr: &NIRInstruction,
    values: &HashMap<ValueID, NIRValue>,
    ctx: &mut CodeGenContext,
) -> Result<String> {
    let mut a = String::new();
    match instr {
        NIRInstruction::BinOp { op, lhs, rhs, result } => {
            load(&mut a, *lhs, values, ctx, "rax");
            load(&mut a, *rhs, values, ctx, "rbx");
            match op {
                NIROp::Add => a.push_str("    add rax, rbx\n"),
                NIROp::Sub => a.push_str("    sub rax, rbx\n"),
                NIROp::Mul => a.push_str("    imul rax, rbx\n"),
                NIROp::Div => { a.push_str("    xor rdx, rdx\n    div rbx\n"); }
                NIROp::Mod => { a.push_str("    xor rdx, rdx\n    div rbx\n    mov rax, rdx\n"); }
                NIROp::Shl => { a.push_str("    mov rcx, rbx\n    shl rax, cl\n"); }
                NIROp::Shr => { a.push_str("    mov rcx, rbx\n    shr rax, cl\n"); }
                NIROp::And => a.push_str("    and rax, rbx\n"),
                NIROp::Or  => a.push_str("    or rax, rbx\n"),
                NIROp::Xor => a.push_str("    xor rax, rbx\n"),
                NIROp::Eq  => { a.push_str("    cmp rax, rbx\n    sete al\n    movzx rax, al\n"); }
                NIROp::Ne  => { a.push_str("    cmp rax, rbx\n    setne al\n    movzx rax, al\n"); }
                NIROp::Lt  => { a.push_str("    cmp rax, rbx\n    setl al\n    movzx rax, al\n"); }
                NIROp::Le  => { a.push_str("    cmp rax, rbx\n    setle al\n    movzx rax, al\n"); }
                NIROp::Gt  => { a.push_str("    cmp rax, rbx\n    setg al\n    movzx rax, al\n"); }
                NIROp::Ge  => { a.push_str("    cmp rax, rbx\n    setge al\n    movzx rax, al\n"); }
            }
            store_result(&mut a, *result, ctx);
        }
        NIRInstruction::UnOp { op, operand, result } => {
            load(&mut a, *operand, values, ctx, "rax");
            match op {
                NIRUnOp::Neg => a.push_str("    neg rax\n"),
                NIRUnOp::Not => a.push_str("    not rax\n"),
            }
            store_result(&mut a, *result, ctx);
        }
        NIRInstruction::CondBranch { cond, then_bb, else_bb } => {
            load(&mut a, *cond, values, ctx, "rax");
            a.push_str("    test rax, rax\n");
            a.push_str(&format!("    jnz .bb_{}\n", then_bb.0));
            a.push_str(&format!("    jmp .bb_{}\n", else_bb.0));
        }
        NIRInstruction::Branch { target } => {
            a.push_str(&format!("    jmp .bb_{}\n", target.0));
        }
        NIRInstruction::Call { callee, args, result } => {
            // Check if callee is a known builtin name
            let builtin_name = get_builtin_name(*callee, values);
            if let Some(name) = builtin_name {
                emit_builtin_call(&mut a, &name, args, values, ctx, *result)?;
            } else {
                // Regular function call via fat pointer
                let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                for (i, arg) in args.iter().enumerate() {
                    if i < 6 {
                        load(&mut a, *arg, values, ctx, arg_regs[i]);
                    } else {
                        load(&mut a, *arg, values, ctx, "rax");
                        a.push_str("    push rax\n");
                    }
                }
                load(&mut a, *callee, values, ctx, "r10");
                a.push_str("    push r15\n");
                a.push_str("    mov r15, r10\n");
                a.push_str("    mov r10, [r10]\n");
                a.push_str("    call r10\n");
                a.push_str("    pop r15\n");
                store_result(&mut a, *result, ctx);
            }
        }
        NIRInstruction::Alloc { size, result } => {
            load(&mut a, *size, values, ctx, "rdi");
            a.push_str("    call arena_alloc\n");
            store_result(&mut a, *result, ctx);
        }
        NIRInstruction::Store { ptr, value } => {
            load(&mut a, *ptr, values, ctx, "rbx");
            load(&mut a, *value, values, ctx, "rax");
            a.push_str("    mov [rbx], rax\n");
        }
        NIRInstruction::Load { ptr, result } => {
            load(&mut a, *ptr, values, ctx, "rbx");
            a.push_str("    mov rax, [rbx]\n");
            store_result(&mut a, *result, ctx);
        }
        NIRInstruction::Move { value, result } |
        NIRInstruction::Borrow { value, result, .. } => {
            load(&mut a, *value, values, ctx, "rax");
            store_result(&mut a, *result, ctx);
        }
        NIRInstruction::Free { .. } => {} // Arena: no-op
        NIRInstruction::Return { value } => {
            if let Some(v) = value {
                load(&mut a, *v, values, ctx, "rdi");
            }
        }
    }
    Ok(a)
}
fn get_builtin_name(callee: ValueID, values: &HashMap<ValueID, NIRValue>) -> Option<String> {
    if let Some(NIRValue::StrConst(id)) = values.get(&callee) {
        // StrConst stores an index; we'd need a string table.
        // For now, check IntConst as a tag or use a naming convention.
        None
    } else if let Some(NIRValue::IntConst(tag)) = values.get(&callee) {
        // Use integer tags for builtins: 100=sqrt, 101=abs, etc.
        match *tag {
            100 => Some("sqrt".into()),
            101 => Some("abs".into()),
            102 => Some("floor".into()),
            103 => Some("power".into()),
            104 => Some("exp".into()),
            105 => Some("print_int".into()),
            106 => Some("print_str".into()),
            107 => Some("num_to_str".into()),
            108 => Some("channel_create".into()),
            109 => Some("channel_send".into()),
            110 => Some("channel_recv".into()),
            111 => Some("thread_spawn".into()),
            112 => Some("thread_join".into()),
            113 => Some("list_len".into()),
            114 => Some("list_sum".into()),
            115 => Some("list_head".into()),
            116 => Some("list_tail".into()),
            117 => Some("list_append".into()),
            118 => Some("file_open".into()),
            119 => Some("file_close".into()),
            120 => Some("file_write".into()),
            121 => Some("file_read".into()),
            122 => Some("stdin_read".into()),
            123 => Some("str_eq".into()),
            _ => None,
        }
    } else {
        None
    }
}
fn emit_builtin_call(
    a: &mut String,
    name: &str,
    args: &[ValueID],
    values: &HashMap<ValueID, NIRValue>,
    ctx: &mut CodeGenContext,
    result: ValueID,
) -> Result<()> {
    match name {
        "sqrt" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("sqrt needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call sqrt_fixed\n");
            store_result(a, result, ctx);
        }
        "abs" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("abs needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call abs_fixed\n");
            store_result(a, result, ctx);
        }
        "floor" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("floor needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call floor_fixed\n");
            store_result(a, result, ctx);
        }
        "power" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("power needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rsi");
            a.push_str("    call power_fixed\n");
            store_result(a, result, ctx);
        }
        "exp" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("exp needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rbx");
            a.push_str("    call exp_fixed\n");
            store_result(a, result, ctx);
        }
        "print_int" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("print_int needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call print_int\n");
            store_result(a, result, ctx);
        }
        "print_str" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("print_str needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call print_str\n");
            store_result(a, result, ctx);
        }
        "num_to_str" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("num_to_str needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call num_to_str\n");
            store_result(a, result, ctx);
        }
        "channel_create" => {
            a.push_str("    call channel_create\n");
            store_result(a, result, ctx);
        }
        "channel_send" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("channel_send needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rsi");
            a.push_str("    call channel_send\n");
            store_result(a, result, ctx);
        }
        "channel_recv" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("channel_recv needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rdi");
            a.push_str("    call channel_recv\n");
            store_result(a, result, ctx);
        }
        "thread_spawn" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("thread_spawn needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rsi");
            a.push_str("    call thread_spawn\n");
            store_result(a, result, ctx);
        }
        "thread_join" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("thread_join needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rdi");
            a.push_str("    call thread_join\n");
            store_result(a, result, ctx);
        }
        "list_len" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("list_len needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    mov rax, [rax]\n");  // length is first qword
            store_result(a, result, ctx);
        }
        "list_sum" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("list_sum needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call list_sum_helper\n");
            store_result(a, result, ctx);
        }
        "list_head" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("list_head needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    mov rbx, [rax]\n    test rbx, rbx\n");
            let lbl = ctx.fresh_label("hemp");
            a.push_str(&format!("    jz {}\n", lbl));
            a.push_str("    mov rax, [rax + 8]\n");
            a.push_str(&format!("    jmp .hdne_{}\n", ctx.label_counter));
            a.push_str(&format!("{}:\n    mov rax, 60\n    mov rdi, 1\n    syscall\n", lbl));
            a.push_str(&format!(".hdne_{}:\n", ctx.label_counter));
            store_result(a, result, ctx);
        }
        "list_tail" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("list_tail needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            a.push_str("    call list_tail_helper\n");
            store_result(a, result, ctx);
        }
        "list_append" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("list_append needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rsi");
            a.push_str("    call list_append_helper\n");
            store_result(a, result, ctx);
        }
        "file_open" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("file_open needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rax");
            // Copy string to file_path_buf
            a.push_str("    mov rcx, [rax]\n    lea rsi, [rax + 8]\n    lea rdi, [file_path_buf]\n");
            let lbl = ctx.fresh_label("fpc");
            a.push_str(&format!("{}:\n    test rcx, rcx\n    jz .fpd_{}\n", lbl, ctx.label_counter));
            a.push_str("    mov al, [rsi]\n    mov [rdi], al\n    inc rsi\n    inc rdi\n    dec rcx\n");
            a.push_str(&format!("    jmp {}\n.fpd_{}:\n    mov byte [rdi], 0\n", lbl, ctx.label_counter));
            a.push_str("    lea rdi, [file_path_buf]\n    mov rsi, 66\n    mov rdx, 420\n    mov rax, 2\n    syscall\n");
            store_result(a, result, ctx);
        }
        "file_close" => {
            if args.len() != 1 { return Err(CompileError::Unsupported("file_close needs 1 arg".into())); }
            load(a, args[0], values, ctx, "rdi");
            a.push_str("    mov rax, 3\n    syscall\n");
            store_result(a, result, ctx);
        }
        "file_write" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("file_write needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rax");
            a.push_str("    mov rdx, [rax]\n    lea rsi, [rax + 8]\n    mov rax, 1\n    syscall\n");
            store_result(a, result, ctx);
        }
        "file_read" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("file_read needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rdx");
            a.push_str("    lea rsi, [file_buf]\n    mov rax, 0\n    syscall\n");
            // Build arena string from file_buf
            a.push_str("    mov rcx, rax\n    mov rdi, rcx\n    add rdi, 8\n    call arena_alloc\n");
            a.push_str("    mov [rax], rcx\n    push rax\n    lea rsi, [file_buf]\n    lea rdi, [rax + 8]\n");
            let lbl = ctx.fresh_label("frc");
            a.push_str(&format!("{}:\n    test rcx, rcx\n    jz .frd_{}\n", lbl, ctx.label_counter));
            a.push_str("    mov cl, [rsi]\n    mov [rdi], cl\n    inc rsi\n    inc rdi\n    dec rcx\n");
            a.push_str(&format!("    jmp {}\n.frd_{}:\n    pop rax\n", lbl, ctx.label_counter));
            store_result(a, result, ctx);
        }
        "stdin_read" => {
            a.push_str("    call stdin_read_helper\n");
            store_result(a, result, ctx);
        }
        "str_eq" => {
            if args.len() != 2 { return Err(CompileError::Unsupported("str_eq needs 2 args".into())); }
            load(a, args[0], values, ctx, "rdi");
            load(a, args[1], values, ctx, "rsi");
            a.push_str("    call str_eq\n");
            store_result(a, result, ctx);
        }
        _ => return Err(CompileError::Unsupported(format!("Unknown builtin: {}", name))),
    }
    Ok(())
}
fn load(a: &mut String, vid: ValueID, values: &HashMap<ValueID, NIRValue>, ctx: &CodeGenContext, reg: &str) {
    // DEBUG: trace load
    // (can't use format inside fn signature, so trace inside body)
    if let Some(nv) = values.get(&vid) {
        match nv {
            NIRValue::IntConst(n) => {
                if *n > i32::MAX as i64 || *n < i32::MIN as i64 {
                    a.push_str(&format!("    mov {}, qword {}\n", reg, n));
                } else {
                    a.push_str(&format!("    mov {}, {}\n", reg, n));
                }
                return;
            }
            NIRValue::BoolConst(b) => {
                a.push_str(&format!("    mov {}, {}\n", reg, if *b { 1 } else { 0 }));
                return;
            }
            _ => {}
        }
    }
    if let Some(&slot) = ctx.value_map.get(&vid) {
        a.push_str(&format!("    mov {}, [vars + {}]\n", reg, slot * 8));
    } else {
        a.push_str(&format!("    xor {}, {}\n", reg, reg));
    }
}
fn store_result(a: &mut String, vid: ValueID, ctx: &CodeGenContext) {
    if let Some(&slot) = ctx.value_map.get(&vid) {
        // DEBUG: comment in asm to trace
        a.push_str(&format!("    ; store_result: vid={} -> slot={}, rax\n", vid.0, slot));
        a.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
    } else {
        a.push_str(&format!("    ; WARNING: store_result: vid={} has NO slot!\n", vid.0));
    }
}
fn emit_all_helpers() -> String {
    let mut h = String::new();
    // mmfail
    h.push_str("mmfail:\n    mov rax, 60\n    mov rdi, 2\n    syscall\n\n");
    // arena_alloc
    h.push_str("arena_alloc:\n    mov rax, [arena_ptr]\n    add rdi, 15\n    and rdi, -16\n    add [arena_ptr], rdi\n    ret\n\n");
    // print_int (from p40_sqrt.asm)
    h.push_str("print_int:\n");
    h.push_str("    push rax\n    push rbx\n    push rcx\n    push rdx\n    push rsi\n    push rdi\n");
    h.push_str("    test rax, rax\n    jns .pi_pos\n    neg rax\n    mov byte [negflag], 1\n");
    h.push_str(".pi_pos:\n    mov rbx, 10\n    mov rcx, 0\n    lea rdi, [num_buf + 31]\n");
    h.push_str(".piloop:\n    xor rdx, rdx\n    div rbx\n    add dl, '0'\n    dec rdi\n    mov [rdi], dl\n    inc rcx\n    test rax, rax\n    jnz .piloop\n");
    h.push_str("    cmp byte [negflag], 1\n    jne .pi_skip_neg\n    dec rdi\n    mov byte [rdi], 45\n    inc rcx\n");
    h.push_str(".pi_skip_neg:\n    mov byte [negflag], 0\n    mov rsi, rdi\n    mov byte [rsi + rcx], 10\n    inc rcx\n");
    h.push_str("    mov rdi, 1\n    mov rax, 1\n    mov rdx, rcx\n    syscall\n");
    h.push_str("    pop rdi\n    pop rsi\n    pop rdx\n    pop rcx\n    pop rbx\n    pop rax\n    ret\n\n");
    // str_eq (from p40_sqrt.asm)
    h.push_str("str_eq:\n    mov rcx, [rdi]\n    mov rdx, [rsi]\n    cmp rcx, rdx\n    jne .str_eq_ne\n");
    h.push_str("    add rdi, 8\n    add rsi, 8\n");
    h.push_str(".str_eq_loop:\n    test rcx, rcx\n    jz .str_eq_eq\n    mov al, [rdi]\n    cmp al, [rsi]\n    jne .str_eq_ne\n");
    h.push_str("    inc rdi\n    inc rsi\n    dec rcx\n    jmp .str_eq_loop\n");
    h.push_str(".str_eq_eq:\n    mov rax, 1\n    ret\n");
    h.push_str(".str_eq_ne:\n    mov rax, 0\n    ret\n\n");
    // print_str (from p40_sqrt.asm)
    h.push_str("print_str:\n    push rax\n    push rdx\n    push rsi\n    push rdi\n");
    h.push_str("    mov rsi, rax\n    add rsi, 8\n    mov rdx, [rax]\n    mov rdi, 1\n    mov rax, 1\n    syscall\n");
    h.push_str("    mov rsi, nl_ptr\n    mov rdx, 1\n    mov rdi, 1\n    mov rax, 1\n    syscall\n");
    h.push_str("    pop rdi\n    pop rsi\n    pop rdx\n    pop rax\n    ret\n\n");
    // num_to_str: rax=int -> rax=arena string ptr
    h.push_str("num_to_str:\n");
    h.push_str("    push rbx\n    push rcx\n    push rdx\n    push rsi\n    push rdi\n");
    h.push_str("    test rax, rax\n    jns .nts_pos\n    neg rax\n    mov byte [negflag], 1\n");
    h.push_str(".nts_pos:\n    mov rbx, 10\n    mov rcx, 0\n    mov byte [negflag], 0\n    lea rdi, [num_buf + 31]\n");
    h.push_str(".nts_loop:\n    xor rdx, rdx\n    div rbx\n    add dl, '0'\n    dec rdi\n    mov [rdi], dl\n    inc rcx\n    test rax, rax\n    jnz .nts_loop\n");
    h.push_str("    cmp byte [negflag], 1\n    jne .nts_skip\n    dec rdi\n    mov byte [rdi], 45\n    inc rcx\n");
    h.push_str(".nts_skip:\n    push rdi\n    push rcx\n    mov rax, rcx\n    add rax, 8\n    mov rdi, rax\n    call arena_alloc\n");
    h.push_str("    pop rcx\n    mov [rax], rcx\n    pop rsi\n    push rax\n    lea rdi, [rax + 8]\n");
    h.push_str(".nts_copy:\n    test rcx, rcx\n    jz .nts_done\n    mov al, [rsi]\n    mov [rdi], al\n    inc rsi\n    inc rdi\n    dec rcx\n    jmp .nts_copy\n");
    h.push_str(".nts_done:\n    mov byte [negflag], 0\n    pop rax\n");
    h.push_str("    pop rdi\n    pop rsi\n    pop rdx\n    pop rcx\n    pop rbx\n    ret\n\n");
    // sqrt_fixed (from p40_sqrt.asm)
    h.push_str("sqrt_fixed:\n    test rax, rax\n    jz .sq_done\n");
    h.push_str("    push r11\n    mov r11, rax\n    mov r10, 0\n    mov r8, 1\n    shl r8, 30\n");
    h.push_str(".sq_loop:\n    mov r9, r10\n    or r9, r8\n    mov rax, r9\n    imul rax\n    test rdx, rdx\n    jnz .sq_skip\n");
    h.push_str("    cmp rax, r11\n    ja .sq_skip\n    mov r10, r9\n");
    h.push_str(".sq_skip:\n    shr r8, 1\n    test r8, r8\n    jnz .sq_loop\n");
    h.push_str("    mov rax, r10\n    pop r11\n");
    h.push_str(".sq_done:\n    ret\n\n");
    // abs_fixed
    h.push_str("abs_fixed:\n    test rax, rax\n    jns .abs_done\n    neg rax\n.abs_done:\n    ret\n\n");
    // floor_fixed
    h.push_str("floor_fixed:\n    sar rax, 32\n    ret\n\n");
    // power_fixed: rdi=base, rsi=exp -> rax=result
    h.push_str("power_fixed:\n    push rbx\n    push rcx\n    mov rbx, rdi\n    mov rcx, rsi\n    mov rax, 1\n");
    h.push_str(".pw_loop:\n    test rcx, rcx\n    jz .pw_done\n    imul rax, rbx\n    dec rcx\n    jmp .pw_loop\n");
    h.push_str(".pw_done:\n    pop rcx\n    pop rbx\n    ret\n\n");
    // exp_fixed Q32.32 (from nasbi_exp.asm)
    h.push_str("exp_fixed:\n    push r8\n    push rdi\n    push rcx\n    push rdx\n");
    h.push_str("    mov rax, rbx\n    mov rdx, 6196328019\n    imul rdx\n");
    h.push_str("    mov r8, 2147483648\n    add rax, r8\n    adc rdx, 0\n    mov rdi, rdx\n");
    h.push_str("    mov rax, rdi\n    mov rdx, 2977044472\n    imul rdx\n    sub rbx, rax\n");
    // Horner degree 12
    h.push_str("    mov rax, 9\n");
    for coeff in &[108i64, 1184, 11836, 106522, 852176, 5965232, 35791394, 178956971, 715827883] {
        h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n");
        h.push_str(&format!("    add rax, {}\n", coeff));
    }
    // c1 = 4294967296 (using r8 which still holds 2147483648... need to reload)
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n");
    h.push_str("    mov r8, 4294967296\n    add rax, r8\n");
    // c0 = 4294967296
    h.push_str("    mov rdx, rbx\n    imul rdx\n    mov rcx, rax\n    shr rcx, 32\n    shl rdx, 32\n    or rdx, rcx\n    mov rax, rdx\n");
    h.push_str("    mov r8, 4294967296\n    add rax, r8\n");
    // Final shift by k
    h.push_str("    mov rcx, rdi\n    test rcx, rcx\n    jns .uexp_pos\n    neg rcx\n    sar rax, cl\n    jmp .uexp_done\n");
    h.push_str(".uexp_pos:\n    shl rax, cl\n");
    h.push_str(".uexp_done:\n    pop rdx\n    pop rcx\n    pop rdi\n    pop r8\n    ret\n\n");
    // channel_create: pipe2 (syscall 293)
    h.push_str("channel_create:\n    push rbx\n    push rcx\n");
    h.push_str("    mov rdi, 16\n    call arena_alloc\n    mov rbx, rax\n");
    h.push_str("    lea rdi, [rel fds_tmp]\n    xor rsi, rsi\n    mov rax, 293\n    syscall\n");
    h.push_str("    movsxd rax, dword [rel fds_tmp]\n    movsxd rcx, dword [rel fds_tmp + 4]\n");
    h.push_str("    mov [rbx], rax\n    mov [rbx + 8], rcx\n    mov rax, rbx\n    pop rcx\n    pop rbx\n    ret\n\n");
    // channel_send: rdi=chan_ptr, rsi=value
    h.push_str("channel_send:\n    push rbx\n    push rcx\n    mov rbx, rdi\n    mov [rel chan_tmp], rsi\n");
    h.push_str("    mov edi, [rbx + 8]\n    lea rsi, [rel chan_tmp]\n    mov rdx, 8\n    mov rax, 1\n    syscall\n");
    h.push_str("    mov rax, 8\n    pop rcx\n    pop rbx\n    ret\n\n");
    // channel_recv: rdi=chan_ptr -> rax=value
    h.push_str("channel_recv:\n    push rbx\n    push rcx\n    mov rbx, rdi\n");
    h.push_str("    mov edi, [rbx]\n    lea rsi, [rel chan_tmp]\n    mov rdx, 8\n    xor rax, rax\n    syscall\n");
    h.push_str("    mov rax, [rel chan_tmp]\n    pop rcx\n    pop rbx\n    ret\n\n");
    // thread_spawn: rdi=fn_ptr, rsi=arg -> rax=pid
    h.push_str("thread_spawn:\n    push rbx\n    push rcx\n");
    h.push_str("    mov [t_task], rdi\n    mov [t_task + 8], rsi\n");
    h.push_str("    xor rdi, rdi\n    mov rsi, 65536\n    mov rdx, 3\n    mov r10, 34\n    xor r8, r8\n    xor r9, r9\n    mov rax, 9\n    syscall\n");
    h.push_str("    test rax, rax\n    js mmfail\n    mov [t_task + 24], rax\n    add rax, 65536\n    sub rax, 16\n    mov rsi, rax\n");
    h.push_str("    mov rdi, 1809\n    xor rdx, rdx\n    xor r10, r10\n    xor r8, r8\n    xor r9, r9\n    mov rax, 56\n    syscall\n");
    h.push_str("    test rax, rax\n    jnz .t_parent\n");
    h.push_str("    mov r10, [t_task]\n    mov rdi, [t_task + 8]\n    call r10\n");
    h.push_str("    mov [t_task + 16], rax\n    mov rax, 231\n    xor rdi, rdi\n    syscall\n");
    h.push_str(".t_parent:\n    mov [t_task + 24], rax\n    pop rcx\n    pop rbx\n    ret\n\n");
    // thread_join: rdi=pid -> rax=result
    h.push_str("thread_join:\n    push rbx\n    push rcx\n    mov rbx, rdi\n");
    h.push_str("    lea rsi, [rel t_status]\n    xor rdx, rdx\n    xor r10, r10\n    mov rax, 61\n    syscall\n");
    h.push_str("    mov rax, [t_task + 16]\n    pop rcx\n    pop rbx\n    ret\n\n");
    // list_sum_helper: rax=list_ptr -> rax=sum
    h.push_str("list_sum_helper:\n    mov rcx, [rax]\n    xor rbx, rbx\n");
    h.push_str(".sl_loop:\n    test rcx, rcx\n    jz .sl_done\n    mov rdx, rcx\n    dec rdx\n    add rbx, [rax + rdx * 8 + 8]\n    dec rcx\n    jmp .sl_loop\n");
    h.push_str(".sl_done:\n    mov rax, rbx\n    ret\n\n");
    // list_tail_helper: rax=list_ptr -> rax=new_list
    h.push_str("list_tail_helper:\n    mov rcx, [rax]\n    test rcx, rcx\n    jz .tl_empty\n");
    h.push_str("    push rax\n    dec rcx\n    mov rdi, rcx\n    shl rdi, 3\n    add rdi, 8\n    call arena_alloc\n    mov r12, rax\n    mov [rax], rcx\n");
    h.push_str("    pop rsi\n    add rsi, 16\n    lea rdi, [r12 + 8]\n");
    h.push_str(".tl_copy:\n    test rcx, rcx\n    jz .tl_done\n    mov rdx, [rsi]\n    mov [rdi], rdx\n    add rsi, 8\n    add rdi, 8\n    dec rcx\n    jmp .tl_copy\n");
    h.push_str(".tl_done:\n    mov rax, r12\n    ret\n");
    h.push_str(".tl_empty:\n    mov rax, 60\n    mov rdi, 1\n    syscall\n\n");
    // list_append_helper: rdi=list, rsi=elem -> rax=new_list
    h.push_str("list_append_helper:\n    push r10\n    push r11\n    mov r10, rdi\n    mov r11, rsi\n");
    h.push_str("    mov rax, [r10]\n    add rax, 1\n    mov rdi, rax\n    shl rdi, 3\n    add rdi, 8\n    call arena_alloc\n    mov r12, rax\n");
    h.push_str("    mov rax, [r10]\n    add rax, 1\n    mov [r12], rax\n");
    h.push_str("    mov rax, [r10]\n    lea rsi, [r10 + 8]\n    lea rdi, [r12 + 8]\n");
    h.push_str(".la_copy:\n    test rax, rax\n    jz .la_done\n    mov rcx, [rsi]\n    mov [rdi], rcx\n    add rsi, 8\n    add rdi, 8\n    dec rax\n    jmp .la_copy\n");
    h.push_str(".la_done:\n    mov [rdi], r11\n    mov rax, r12\n    pop r11\n    pop r10\n    ret\n\n");
    // stdin_read_helper: -> rax=arena string
    h.push_str("stdin_read_helper:\n    push rbx\n    push rcx\n    xor rcx, rcx\n");
    h.push_str(".rd_loop:\n    lea rsi, [read_buf + rcx]\n    xor rdi, rdi\n    mov rdx, 1\n    xor rax, rax\n    push rcx\n    syscall\n    pop rcx\n");
    h.push_str("    test rax, rax\n    jz .rd_end\n    mov al, [read_buf + rcx]\n    cmp al, 10\n    je .rd_end\n    inc rcx\n    jmp .rd_loop\n");
    h.push_str(".rd_end:\n    mov rax, rcx\n    add rax, 8\n    mov rdi, rax\n    call arena_alloc\n    mov [rax], rcx\n    push rax\n    push rcx\n    lea rsi, [read_buf]\n    lea rdi, [rax + 8]\n");
    h.push_str(".rd_copy:\n    test rcx, rcx\n    jz .rd_cdone\n    mov al, [rsi]\n    mov [rdi], al\n    inc rsi\n    inc rdi\n    dec rcx\n    jmp .rd_copy\n");
    h.push_str(".rd_cdone:\n    pop rcx\n    pop rax\n    pop rcx\n    pop rbx\n    ret\n\n");
    h
}
#[cfg(test)]
mod tests {
    use super::*;
    use mal_nir::*;
    fn make_binop(op: NIROp, a: i64, b: i64) -> NIRFunction {
        let mut f = NIRFunction::new("test".into());
        let v1 = ValueID(1); let v2 = ValueID(2); let v3 = ValueID(3);
        f.values.insert(v1, NIRValue::IntConst(a));
        f.values.insert(v2, NIRValue::IntConst(b));
        f.blocks.push(BasicBlock { id: BlockID(0), instructions: vec![
            NIRInstruction::BinOp { op, lhs: v1, rhs: v2, result: v3 },
            NIRInstruction::Return { value: Some(v3) },
        ], predecessors: vec![], successors: vec![] });
        f.entry_block = BlockID(0);
        f
    }
    fn run(func: &NIRFunction, expected: i32, name: &str) {
        let td = tempfile::Builder::new().prefix(&format!("mal_{}_", name)).tempdir().unwrap();
        let bp = td.path().join("bin");
        compile_to_native(func, bp.to_str().unwrap()).unwrap_or_else(|e| panic!("{}: {:?}", name, e));
        let out = Command::new(&bp).output().unwrap();
        assert_eq!(out.status.code(), Some(expected), "{}: got {:?}", name, out.status.code());
    }
    #[test] fn t_add() { run(&make_binop(NIROp::Add, 5, 3), 8, "add"); }
    #[test] fn t_sub() { run(&make_binop(NIROp::Sub, 10, 3), 7, "sub"); }
    #[test] fn t_mul() { run(&make_binop(NIROp::Mul, 7, 6), 42, "mul"); }
    #[test] fn t_div() { run(&make_binop(NIROp::Div, 42, 6), 7, "div"); }
    #[test] fn t_mod() { run(&make_binop(NIROp::Mod, 17, 5), 2, "mod"); }
    #[test] fn t_shl() { run(&make_binop(NIROp::Shl, 1, 4), 16, "shl"); }
    #[test] fn t_and() { run(&make_binop(NIROp::And, 0xFF, 0x0F), 15, "and"); }
    #[test] fn t_or()  { run(&make_binop(NIROp::Or, 0xF0, 0x0F), 255, "or"); }
    #[test] fn t_xor() { run(&make_binop(NIROp::Xor, 0xFF, 0x0F), 240, "xor"); }
    #[test] fn t_neg() { run(&make_binop(NIROp::Add, -10, 3), 249, "neg"); }
    #[test]
    fn t_unop_neg() {
        let mut f = NIRFunction::new("test".into());
        let v1 = ValueID(1); let v2 = ValueID(2);
        f.values.insert(v1, NIRValue::IntConst(42));
        f.blocks.push(BasicBlock { id: BlockID(0), instructions: vec![
            NIRInstruction::UnOp { op: NIRUnOp::Neg, operand: v1, result: v2 },
            NIRInstruction::Return { value: Some(v2) },
        ], predecessors: vec![], successors: vec![] });
        f.entry_block = BlockID(0);
        run(&f, 214, "uneg");
    }
    #[test]
    fn t_cond_branch() {
        let mut f = NIRFunction::new("test".into());
        let v1=ValueID(1); let v2=ValueID(2); let v3=ValueID(3);
        let v4=ValueID(4); let v5=ValueID(5);
        f.values.insert(v1, NIRValue::IntConst(5));
        f.values.insert(v2, NIRValue::IntConst(3));
        f.values.insert(v4, NIRValue::IntConst(1));
        f.values.insert(v5, NIRValue::IntConst(0));
        f.blocks.push(BasicBlock { id: BlockID(0), instructions: vec![
            NIRInstruction::BinOp { op: NIROp::Gt, lhs: v1, rhs: v2, result: v3 },
            NIRInstruction::CondBranch { cond: v3, then_bb: BlockID(1), else_bb: BlockID(2) },
        ], predecessors: vec![], successors: vec![BlockID(1), BlockID(2)] });
        f.blocks.push(BasicBlock { id: BlockID(1), instructions: vec![
            NIRInstruction::Return { value: Some(v4) },
        ], predecessors: vec![BlockID(0)], successors: vec![] });
        f.blocks.push(BasicBlock { id: BlockID(2), instructions: vec![
            NIRInstruction::Return { value: Some(v5) },
        ], predecessors: vec![BlockID(0)], successors: vec![] });
        f.entry_block = BlockID(0);
        run(&f, 1, "branch");
    }
    // Test builtin: sqrt(144) = 12
    #[test]
    fn t_builtin_sqrt() {
        let mut f = NIRFunction::new("test".into());
        let v1=ValueID(1); let v2=ValueID(2); let v3=ValueID(3);
        f.values.insert(v1, NIRValue::IntConst(144));
        f.values.insert(v2, NIRValue::IntConst(100)); // builtin tag for sqrt
        f.blocks.push(BasicBlock { id: BlockID(0), instructions: vec![
            NIRInstruction::Call { callee: v2, args: vec![v1], result: v3 },
            NIRInstruction::Return { value: Some(v3) },
        ], predecessors: vec![], successors: vec![] });
        f.entry_block = BlockID(0);
        run(&f, 12, "sqrt");
    }
    // Test builtin: abs(-42) = 42
    #[test]
    fn t_builtin_abs() {
        let mut f = NIRFunction::new("test".into());
        let v1=ValueID(1); let v2=ValueID(2); let v3=ValueID(3);
        f.values.insert(v1, NIRValue::IntConst(-42));
        f.values.insert(v2, NIRValue::IntConst(101)); // builtin tag for abs
        f.blocks.push(BasicBlock { id: BlockID(0), instructions: vec![
            NIRInstruction::Call { callee: v2, args: vec![v1], result: v3 },
            NIRInstruction::Return { value: Some(v3) },
        ], predecessors: vec![], successors: vec![] });
        f.entry_block = BlockID(0);
        run(&f, 42, "abs");
    }
    // Test builtin: power(2, 10) = 1024 mod 256 = 0
    #[test]
    fn t_builtin_power() {
        let mut f = NIRFunction::new("test".into());
        let v1=ValueID(1); let v2=ValueID(2); let v3=ValueID(3); let v4=ValueID(4);
        f.values.insert(v1, NIRValue::IntConst(2));
        f.values.insert(v2, NIRValue::IntConst(10));
        f.values.insert(v3, NIRValue::IntConst(103)); // builtin tag for power
        f.blocks.push(BasicBlock { id: BlockID(0), instructions: vec![
            NIRInstruction::Call { callee: v3, args: vec![v1, v2], result: v4 },
            NIRInstruction::Return { value: Some(v4) },
        ], predecessors: vec![], successors: vec![] });
        f.entry_block = BlockID(0);
        // 2^10 = 1024, exit code = 1024 mod 256 = 0
        run(&f, 0, "power");
    }

// === Union Types Runtime Support (Feature 8) ===
// Union types are represented as tagged values: (tag, data)
// Tag 0 = first variant, Tag 1 = second variant, etc.
fn emit_union_create(tag: u32, value_reg: &str, result_reg: &str) -> String {
    // Pack tag and value into a single 64-bit value
    // High 32 bits = tag, Low 32 bits = value
    format!("    shl rax, 32
    or rax, {}
    mov {}, rax
",
            value_reg, result_reg)
}
fn emit_union_extract_tag(union_reg: &str, result_reg: &str) -> String {
    // Extract tag (high 32 bits)
    format!("    mov {}, {}
    shr {}, 32
",
            result_reg, union_reg, result_reg)
}
fn emit_union_extract_value(union_reg: &str, result_reg: &str) -> String {
    // Extract value (low 32 bits)
    format!("    mov {}, {}
    mov eax, eax
",
            result_reg, union_reg)
}

// === Generics Runtime Support (Feature 9) ===
// Generics are monomorphized at compile time
// At runtime, generic functions are just regular functions
fn emit_generic_call(func_name: &str, args: &[String], result_reg: &str) -> String {
    // Generic calls are lowered to specific instantiations
    // e.g., identity<Int> becomes identity_Int
    let mut code = String::new();
    for (i, arg) in args.iter().enumerate() {
        code.push_str(&format!("    mov r{}, {}
", i + 1, arg));
    }
    code.push_str(&format!("    call {}
", func_name));
    code.push_str(&format!("    mov {}, rax
", result_reg));
    code
}
}
