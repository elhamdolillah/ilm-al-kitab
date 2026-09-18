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
    input: PathBuf,
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,
    #[arg(long)]
    emit_nir: bool,
    #[arg(long)]
    emit_asm: bool,
}
fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(&args.input).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", args.input.display(), e);
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
        Ok(()) => println!("Compiled {} -> {}", args.input.display(), output),
        Err(e) => {
            eprintln!("Compile error: {:?}", e);
            std::process::exit(1);
        }
    }
}
