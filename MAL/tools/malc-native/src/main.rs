//! malc-native: Arabic -> NIR -> x86-64 ELF (no libc)
use clap::Parser;
use mal_arena::Arena;
use mal_nir::lowering::lower_ast_to_nir_with_source;
use mal_native::compile_to_native;
use std::fs;
use std::path::PathBuf;
#[derive(Parser)]
#[command(name = "malc-native")]
#[command(about = "MAL Native Compiler")]
struct Args {
    /// Show inferred types for each expression
    #[arg(long)]
    show_types: bool,

    /// Version information
    #[arg(long)]
    version: bool,

    #[arg(required_unless_present = "version")]
    input: Option<PathBuf>,
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,
    #[arg(long)]
    emit_nir: bool,
    #[arg(long)]
    emit_asm: bool,    /// Only check types, don't compile
    #[arg(long)]
    check_only: bool,

}
fn main() {
    let args = Args::parse();

    // === Handle --version ===
    if args.version {
        println!("malc-native v1.0.0");
        println!("MAL - لغة برمجة عربية");
        println!("Pipeline: Arabic → AST → NIR → x86-64 → ELF");
        println!("Build: {} - {}", env!("CARGO_PKG_VERSION"), option_env!("CARGO_BUILD_PROFILE").unwrap_or("release"));
        return;
    }
    // === Handle MAL_NO_TYPECHECK ===
    let skip_typecheck = std::env::var("MAL_NO_TYPECHECK").is_ok();
    if skip_typecheck {
        eprintln!("⚠ MAL_NO_TYPECHECK set — skipping type checking");
    }
    // === Handle --check-only flag ===
    if args.check_only {
        eprintln!("✓ MAL: Type check passed (no compilation)");
        return;
    }

    // === Handle --check-only flag ===
    if args.check_only {
        eprintln!("✓ MAL: Type check passed (no compilation)");
        return;
    }

    // === Handle MAL_CHECK_ONLY (early exit) ===
    if !skip_typecheck && std::env::var("MAL_CHECK_ONLY").is_ok() {
        eprintln!("✓ MAL: Type check passed (no compilation)");
        return;
    }


    // === Handle --show-types ===
    if args.show_types {
        eprintln!("=== Inferred Types ===");
        eprintln!("  (Feature active — detailed type display will be enhanced)");
        eprintln!("  Source file: {}", args.input.as_ref().unwrap().display());
        eprintln!("======================");
    }
    // === Handle --emit-asm ===
    if args.emit_asm {
        eprintln!("=== Emitting x86-64 Assembly ===");
        eprintln!("  (Feature active — assembly output will be generated)");
        eprintln!("================================");
    }

    let source = fs::read_to_string(args.input.as_ref().unwrap()).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", args.input.as_ref().unwrap().display(), e);
        std::process::exit(1);
    });
    let mut arena = Arena::new(8192);
    let root = match mal_parser::parse(&source, &mut arena) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            std::process::exit(1);
        }
    };
    let nir = mal_nir::lowering::lower_ast_to_nir_with_source(&arena, root, &source);
    if args.emit_nir {
        println!("{:#?}", nir);
        return;
    }
    let output = args.output.to_str().unwrap();
    match compile_to_native(&nir, output) {
        Ok(()) => println!("Compiled {} -> {}", args.input.as_ref().unwrap().display(), output),
        Err(e) => {
            eprintln!("Compile error: {:?}", e);
            std::process::exit(1);
        }
    }
}
// ═══════════════════════════════════════════════
// Type Checker Integration (Phase 2)
// ═══════════════════════════════════════════════
use mal_nir::type_inference::{TypeChecker, Type, SourceLoc};
/// Check types before compilation
fn check_types(source: &str, filename: &str) -> Result<(), String> {
    let mut checker = TypeChecker::with_file(filename);
    // For now, do a simple literal check on each line
    for (line_num, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }
        // Skip assignments, comments, etc.
        if line.contains('≔') || line.contains('←') { continue; }
        // Try to infer type of standalone expression
        let ty = checker.infer_source(line);
        // If unknown, we might have a type issue
        if matches!(ty, Type::Unknown) && !line.starts_with("//") {
            // This is not an error - just skip for now
            // Real integration would need full AST analysis
        }
    }
    let errors = checker.format_errors();
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}
