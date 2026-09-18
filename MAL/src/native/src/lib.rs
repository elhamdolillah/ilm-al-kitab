//! # MAL Native Compiler
//!
//! AOT compiler from NIR (SSA-based IR) to x86-64 native binaries **without libc**.
//! Uses Linux syscalls directly.
//!
//! Pipeline: NIR -> NASM Assembly -> ld -> ELF binary
//!
//! Design: mirrors the proven old compiler (math_complete.py / p42_fold.asm):
//!   - vars resq 256  -- BSS global storage, 256 slots x 8 bytes
//!   - arena_mem resb 262144 -- 256KB bump-pointer heap
//!   - Each NIR value gets a unique slot in vars, referenced by [vars + offset]
use mal_nir::{NIRFunction, BasicBlock, NIRInstruction, NIRValue, NIROp, ValueID, BlockID};
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
    ExecutionFailed(String),
}
impl From<std::io::Error> for CompileError {
    fn from(e: std::io::Error) -> Self {
        CompileError::IoError(e)
    }
}
/// Compile NIR function to native ELF binary (no libc)
pub fn compile_to_native(func: &NIRFunction, output_path: &str) -> Result<()> {
    let temp = TempDir::new()?;
    let asm_path = temp.path().join("program.asm");
    let obj_path = temp.path().join("program.o");
    let asm_code = generate_assembly(func)?;
    std::fs::write(&asm_path, asm_code)?;
    let nasm_output = Command::new("nasm")
        .args(&["-f", "elf64", asm_path.to_str().unwrap(), "-o", obj_path.to_str().unwrap()])
        .output()?;
    if !nasm_output.status.success() {
        return Err(CompileError::NasmFailed(
            String::from_utf8_lossy(&nasm_output.stderr).to_string()
        ));
    }
    let ld_output = Command::new("ld")
        .args(&[obj_path.to_str().unwrap(), "-o", output_path])
        .output()?;
    if !ld_output.status.success() {
        return Err(CompileError::LinkFailed(
            String::from_utf8_lossy(&ld_output.stderr).to_string()
        ));
    }
    // Ensure the binary is executable
    let mut perms = std::fs::metadata(output_path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(output_path, perms)?;
    Ok(())
}
fn generate_assembly(func: &NIRFunction) -> Result<String> {
    let mut asm = String::new();
    // === Header ===
    asm.push_str("global _start\n");
    asm.push_str("section .bss\n");
    asm.push_str("    vars resq 256\n");
    asm.push_str("    num_buf resb 32\n");
    asm.push_str("    negflag resb 1\n");
    asm.push_str("    arena_ptr resq 1\n");
    asm.push_str("    arena_mem resb 262144\n");
    asm.push_str("\nsection .data\n");
    asm.push_str("    nl_ptr: db 10\n");
    asm.push_str("\nsection .text\n");
    // === mmfail helper ===
    asm.push_str("mmfail:\n");
    asm.push_str("    mov rax, 60\n");
    asm.push_str("    mov rdi, 2\n");
    asm.push_str("    syscall\n\n");
    // === arena_alloc ===
    asm.push_str("arena_alloc:\n");
    asm.push_str("    mov rax, [arena_ptr]\n");
    asm.push_str("    add rdi, 15\n");
    asm.push_str("    and rdi, -16\n");
    asm.push_str("    add [arena_ptr], rdi\n");
    asm.push_str("    ret\n\n");
    // === print_int (matches old compiler) ===
    asm.push_str("print_int:\n");
    asm.push_str("    push rax\n    push rbx\n    push rcx\n");
    asm.push_str("    push rdx\n    push rsi\n    push rdi\n");
    asm.push_str("    test rax, rax\n");
    asm.push_str("    jns .pi_pos\n");
    asm.push_str("    neg rax\n");
    asm.push_str("    mov byte [negflag], 1\n");
    asm.push_str(".pi_pos:\n");
    asm.push_str("    mov rbx, 10\n");
    asm.push_str("    mov rcx, 0\n");
    asm.push_str("    lea rdi, [num_buf + 31]\n");
    asm.push_str(".piloop:\n");
    asm.push_str("    xor rdx, rdx\n");
    asm.push_str("    div rbx\n");
    asm.push_str("    add dl, '0'\n");
    asm.push_str("    dec rdi\n");
    asm.push_str("    mov [rdi], dl\n");
    asm.push_str("    inc rcx\n");
    asm.push_str("    test rax, rax\n");
    asm.push_str("    jnz .piloop\n");
    asm.push_str("    cmp byte [negflag], 1\n");
    asm.push_str("    jne .pi_skip_neg\n");
    asm.push_str("    dec rdi\n");
    asm.push_str("    mov byte [rdi], 45\n");
    asm.push_str("    inc rcx\n");
    asm.push_str(".pi_skip_neg:\n");
    asm.push_str("    mov byte [negflag], 0\n");
    asm.push_str("    mov rsi, rdi\n");
    asm.push_str("    mov byte [rsi + rcx], 10\n");
    asm.push_str("    inc rcx\n");
    asm.push_str("    mov rdi, 1\n");
    asm.push_str("    mov rax, 1\n");
    asm.push_str("    mov rdx, rcx\n");
    asm.push_str("    syscall\n");
    asm.push_str("    pop rdi\n    pop rsi\n    pop rdx\n");
    asm.push_str("    pop rcx\n    pop rbx\n    pop rax\n");
    asm.push_str("    ret\n\n");
    // === Function body ===
    asm.push_str(&generate_function_body(func)?);
    Ok(asm)
}
fn generate_function_body(func: &NIRFunction) -> Result<String> {
    let mut asm = String::new();
    // _start entry point
    asm.push_str("_start:\n");
    asm.push_str("    lea rax, [arena_mem]\n");
    asm.push_str("    mov [arena_ptr], rax\n");
    asm.push_str("    xor rdi, rdi\n");  // ★ default exit code = 0 (Return will overwrite)
    asm.push_str("\n");
    // Value -> vars slot mapping (numeric offset)
    let mut value_map: HashMap<ValueID, i64> = HashMap::new();
    let mut next_slot: i64 = 0;
    // Pre-scan: assign a slot for every ValueID in the entry block
    if let Some(entry_block) = func.blocks.iter().find(|b| b.id == func.entry_block) {
        for instr in &entry_block.instructions {
            match instr {
                NIRInstruction::BinOp { result, .. } => {
                    if !value_map.contains_key(result) {
                        value_map.insert(*result, next_slot);
                        next_slot += 1;
                    }
                }
                NIRInstruction::Return { value } => {
                    if let Some(vid) = value {
                        if !value_map.contains_key(vid) {
                            value_map.insert(*vid, next_slot);
                            next_slot += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    // Emit instructions
    if let Some(entry_block) = func.blocks.iter().find(|b| b.id == func.entry_block) {
        for instr in &entry_block.instructions {
            let code = generate_instruction(instr, &func.values, &value_map)?;
            asm.push_str(&code);
        }
    }
    // ★ Exit — do NOT clear rdi here (Return already set it)
    asm.push_str("    mov rax, 60\n");  // sys_exit, rdi is the exit code
    asm.push_str("    syscall\n");
    Ok(asm)
}
fn generate_instruction(
    instr: &NIRInstruction,
    values: &HashMap<ValueID, NIRValue>,
    value_map: &HashMap<ValueID, i64>,
) -> Result<String> {
    let mut asm = String::new();
    match instr {
        NIRInstruction::BinOp { op, lhs, rhs, result } => {
            // Load lhs into rbx
            emit_load_value(&mut asm, *lhs, values, value_map, "rbx");
            // Load rhs into rax
            emit_load_value(&mut asm, *rhs, values, value_map, "rax");
            match op {
                NIROp::Add => asm.push_str("    add rax, rbx\n"),
                NIROp::Sub => asm.push_str("    sub rax, rbx\n"),
                NIROp::Mul => asm.push_str("    imul rax, rbx\n"),
                NIROp::Div => {
                    asm.push_str("    push rax\n");
                    asm.push_str("    push rbx\n");
                    asm.push_str("    pop rax\n");
                    asm.push_str("    xor rdx, rdx\n");
                    asm.push_str("    pop rbx\n");
                    asm.push_str("    div rbx\n");
                }
                _ => return Err(CompileError::Unsupported(format!("Op {:?}", op))),
            }
            // Store result to vars slot
            if let Some(&slot) = value_map.get(result) {
                asm.push_str(&format!("    mov [vars + {}], rax\n", slot * 8));
            }
        }
        NIRInstruction::Return { value } => {
            if let Some(val_id) = value {
                // ★ Load value into rdi (sys_exit takes exit code in rdi)
                emit_load_value(&mut asm, *val_id, values, value_map, "rdi");
            }
            // If no value, rdi stays 0 (set at _start)
        }
        _ => return Err(CompileError::Unsupported(format!("Instruction {:?}", instr))),
    }
    Ok(asm)
}
/// Emit assembly to load a NIR value into a register
fn emit_load_value(
    asm: &mut String,
    val_id: ValueID,
    values: &HashMap<ValueID, NIRValue>,
    value_map: &HashMap<ValueID, i64>,
    target_reg: &str,
) {
    // Check literal constant
    if let Some(nir_val) = values.get(&val_id) {
        if let NIRValue::IntConst(n) = nir_val {
            if *n > i32::MAX as i64 || *n < i32::MIN as i64 {
                asm.push_str(&format!("    mov {}, qword {}\n", target_reg, n));
            } else {
                asm.push_str(&format!("    mov {}, {}\n", target_reg, n));
            }
            return;
        }
    }
    // Load from vars slot
    if let Some(&slot) = value_map.get(&val_id) {
        asm.push_str(&format!("    mov {}, [vars + {}]\n", target_reg, slot * 8));
    } else {
        asm.push_str(&format!("    xor {}, {}\n", target_reg, target_reg));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use mal_nir::*;
    fn make_add_func(a: i64, b: i64) -> NIRFunction {
        let mut func = NIRFunction::new("test".to_string());
        let v1 = ValueID(1);
        let v2 = ValueID(2);
        let v3 = ValueID(3);
        func.values.insert(v1, NIRValue::IntConst(a));
        func.values.insert(v2, NIRValue::IntConst(b));
        let entry_block = BasicBlock {
            id: BlockID(0),
            instructions: vec![
                NIRInstruction::BinOp {
                    op: NIROp::Add,
                    lhs: v1,
                    rhs: v2,
                    result: v3,
                },
                NIRInstruction::Return { value: Some(v3) },
            ],
            predecessors: vec![],
            successors: vec![],
        };
        func.blocks.push(entry_block);
        func.entry_block = BlockID(0);
        func
    }
    fn make_mul_func(a: i64, b: i64) -> NIRFunction {
        let mut func = NIRFunction::new("test".to_string());
        let v1 = ValueID(1);
        let v2 = ValueID(2);
        let v3 = ValueID(3);
        func.values.insert(v1, NIRValue::IntConst(a));
        func.values.insert(v2, NIRValue::IntConst(b));
        let entry_block = BasicBlock {
            id: BlockID(0),
            instructions: vec![
                NIRInstruction::BinOp {
                    op: NIROp::Mul,
                    lhs: v1,
                    rhs: v2,
                    result: v3,
                },
                NIRInstruction::Return { value: Some(v3) },
            ],
            predecessors: vec![],
            successors: vec![],
        };
        func.blocks.push(entry_block);
        func.entry_block = BlockID(0);
        func
    }
    #[test]
    fn test_simple_add() {
        let func = make_add_func(5, 3);
        let temp_dir = tempfile::Builder::new()
            .prefix("mal_native_add_")
            .tempdir()
            .unwrap();
        let binary_path = temp_dir.path().join("test_add_bin");
        let result = compile_to_native(&func, binary_path.to_str().unwrap());
        assert!(result.is_ok(), "Compilation failed: {:?}", result);
        let output = Command::new(&binary_path).output();
        assert!(output.is_ok(), "Execution failed: {:?}", output.err());
        let output = output.unwrap();
        let code = output.status.code();
        assert_eq!(code, Some(8), "Expected exit 8 (5+3), got {:?}", code);
    }
    #[test]
    fn test_simple_mul() {
        let func = make_mul_func(7, 6);
        let temp_dir = tempfile::Builder::new()
            .prefix("mal_native_mul_")
            .tempdir()
            .unwrap();
        let binary_path = temp_dir.path().join("test_mul_bin");
        let result = compile_to_native(&func, binary_path.to_str().unwrap());
        assert!(result.is_ok(), "Compilation failed: {:?}", result);
        let output = Command::new(&binary_path).output();
        assert!(output.is_ok(), "Execution failed: {:?}", output.err());
        let output = output.unwrap();
        assert_eq!(output.status.code(), Some(42), "Expected 42 (7x6)");
    }
    #[test]
    fn test_negative_add() {
        let func = make_add_func(-10, 3);
        let temp_dir = tempfile::Builder::new()
            .prefix("mal_native_neg_")
            .tempdir()
            .unwrap();
        let binary_path = temp_dir.path().join("test_neg_bin");
        let result = compile_to_native(&func, binary_path.to_str().unwrap());
        assert!(result.is_ok(), "Compilation failed: {:?}", result);
        let output = Command::new(&binary_path).output();
        assert!(output.is_ok(), "Execution failed: {:?}", output.err());
        // -10 + 3 = -7 -> exit code mod 256 = 249
        assert_eq!(output.unwrap().status.code(), Some(249));
    }
}
