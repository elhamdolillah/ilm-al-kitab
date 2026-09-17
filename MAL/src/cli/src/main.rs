//! # malc — MAL Compiler (Phase 63)
//!
//! Usage:
//!   malc <input.mal> -o <output>      Compile MAL source to binary
//!   malc <input.mal> --emit-c         Emit C source only (no GCC)
//!   malc --help                       Show help
//!
//! Pipeline:
//!   .mal source → NIR → C99 → GCC → ELF binary
//!
//! Constitutional Compliance:
//!   - Principle 5 (البيان): Honest error messages
//!   - Principle 7 (التفكر): 5 tests verify pipeline
use mal_backend::{compile_c_to_binary, transpile_to_c, CompileOptions};
use std::env;
use std::fs;
use std::path::Path;
use std::process;
fn print_help() {
    println!("malc — MAL Compiler v1.0.0");
    println!();
    println!("Usage:");
    println!("  malc <input.mal> -o <output>    Compile to native binary");
    println!("  malc <input.mal> --emit-c       Emit C source only");
    println!("  malc --help                     Show this help");
    println!();
    println!("Examples:");
    println!("  malc hello.mal -o hello");
    println!("  malc fib.mal -o fib && ./fib");
    println!();
    println!("Pipeline: .mal → NIR → C99 → GCC → native binary");
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        process::exit(1);
    }
    if args[1] == "--help" || args[1] == "-h" {
        print_help();
        process::exit(0);
    }
    // Parse arguments
    let input_path = &args[1];
    let mut output_path = String::from("a.out");
    let mut emit_c_only = false;
    let mut optimize = true;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_path = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: -o requires a filename argument");
                    process::exit(1);
                }
            }
            "--emit-c" => emit_c_only = true,
            "--no-opt" => optimize = false,
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }
    // Read input file
    if !Path::new(input_path).exists() {
        eprintln!("Error: input file '{}' not found", input_path);
        process::exit(1);
    }
    let source = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading '{}': {}", input_path, e);
            process::exit(1);
        }
    };
    println!("malc: compiling {}", input_path);
    // Stage 1: Transpile MAL → C
    let tu = match transpile_to_c(&source) {
        Ok(tu) => tu,
        Err(e) => {
            eprintln!("Transpilation error: {}", e);
            process::exit(1);
        }
    };
    let c_source = tu.render();
    // Emit C source if requested
    if emit_c_only {
        let c_file = format!("{}.c", output_path.trim_end_matches(".out"));
        match fs::write(&c_file, &c_source) {
            Ok(_) => {
                println!("malc: wrote C source to {}", c_file);
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Error writing '{}': {}", c_file, e);
                process::exit(1);
            }
        }
    }
    // Stage 2: Compile C → binary
    let options = CompileOptions {
        optimize,
        output_name: output_path.clone(),
        keep_c_source: false,
    };
    let result = compile_c_to_binary(&c_source, &options);
    if result.success {
        // Move binary to requested output path if different
        if let Some(ref bin_path) = result.binary_path {
            if bin_path != &output_path {
                let _ = fs::rename(bin_path, &output_path);
            }
        }
        println!("malc: compiled successfully → {}", output_path);
        if !result.warnings.is_empty() {
            println!("malc: {} warning(s) from GCC", result.warnings.len());
        }
        process::exit(0);
    } else {
        eprintln!("malc: compilation failed");
        for err in &result.errors {
            eprintln!("{}", err);
        }
        process::exit(1);
    }
}
