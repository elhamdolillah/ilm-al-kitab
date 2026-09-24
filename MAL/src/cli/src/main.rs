//! # malc — MAL Compiler (Modern CLI)
//!
//! Usage:
//!   malc check <file>          Check syntax and types
//!   malc compile <file> -o <out> Compile to native binary
//!   malc run <file>            Compile and run immediately
//!   malc --version             Show version
use clap::{Parser, Subcommand};
use mal_backend::{compile_c_to_binary, transpile_to_c, CompileOptions};
use std::fs;
use std::path::Path;
use std::process::{self, Command};
#[derive(Parser)]
#[command(name = "malc")]
#[command(author = "MAL Team")]
#[command(version = "1.1.0")]
#[command(about = "MAL Compiler — Mathematical Arabic Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Check syntax and types without generating code
    Check {
        /// Input MAL source file
        file: String,
    },
    /// Compile MAL source to a native binary
    Compile {
        /// Input MAL source file
        file: String,
        /// Output binary path (default: a.out)
        #[arg(short, long, default_value = "a.out")]
        output: String,
        /// Emit C source only, do not invoke GCC
        #[arg(long)]
        emit_c: bool,
        /// Disable optimizations
        #[arg(long)]
        no_opt: bool,
    },
    /// Compile and run the MAL program immediately
    Run {
        /// Input MAL source file
        file: String,
        /// Arguments to pass to the compiled program
        #[arg(last = true)]
        args: Vec<String>,
    },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Check { file } => {
            if !Path::new(&file).exists() {
                eprintln!("❌ Error: Input file '{}' not found", file);
                process::exit(1);
            }
            let source = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ Error reading '{}': {}", file, e);
                    process::exit(1);
                }
            };
            println!("🔍 Checking {}...", file);
            // Note: Full type checking integration will be added here
            match transpile_to_c(&source) {
                Ok(_) => println!("✅ {} is syntactically valid and transpiles successfully.", file),
                Err(e) => {
                    eprintln!("❌ Transpilation/Check error: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Compile { file, output, emit_c, no_opt } => {
            if !Path::new(&file).exists() {
                eprintln!("❌ Error: Input file '{}' not found", file);
                process::exit(1);
            }
            let source = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ Error reading '{}': {}", file, e);
                    process::exit(1);
                }
            };
            println!("⚙️ Compiling {}...", file);
            let tu = match transpile_to_c(&source) {
                Ok(tu) => tu,
                Err(e) => {
                    eprintln!("❌ Transpilation error: {}", e);
                    process::exit(1);
                }
            };
            let c_source = tu.render();
            if emit_c {
                let c_file = format!("{}.c", output.trim_end_matches(".out").trim_end_matches(".exe"));
                match fs::write(&c_file, &c_source) {
                    Ok(_) => {
                        println!("✅ Wrote C source to {}", c_file);
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("❌ Error writing '{}': {}", c_file, e);
                        process::exit(1);
                    }
                }
            }
            let options = CompileOptions {
                optimize: !no_opt,
                output_name: output.clone(),
                keep_c_source: false,
            };
            let result = compile_c_to_binary(&c_source, &options);
            if result.success {
                if let Some(ref bin_path) = result.binary_path {
                    if bin_path != &output {
                        let _ = fs::rename(bin_path, &output);
                    }
                }
                println!("✅ Compiled successfully → {}", output);
                if !result.warnings.is_empty() {
                    println!("⚠️  {} warning(s) from GCC", result.warnings.len());
                }
            } else {
                eprintln!("❌ Compilation failed");
                for err in &result.errors {
                    eprintln!("  {}", err);
                }
                process::exit(1);
            }
        }
        Commands::Run { file, args } => {
            if !Path::new(&file).exists() {
                eprintln!("❌ Error: Input file '{}' not found", file);
                process::exit(1);
            }
            let source = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ Error reading '{}': {}", file, e);
                    process::exit(1);
                }
            };
            println!("⚙️ Compiling and running {}...", file);
            let tu = match transpile_to_c(&source) {
                Ok(tu) => tu,
                Err(e) => {
                    eprintln!("❌ Transpilation error: {}", e);
                    process::exit(1);
                }
            };
            let c_source = tu.render();
            let temp_bin = "mal_temp_run".to_string();
            let options = CompileOptions {
                optimize: true,
                output_name: temp_bin.clone(),
                keep_c_source: false,
            };
            let result = compile_c_to_binary(&c_source, &options);
            if result.success {
                println!("▶️ Running...");
                let mut cmd = Command::new(format!("./{}", temp_bin));
                cmd.args(&args);
                let status = cmd.status().expect("Failed to execute compiled binary");
                // Cleanup
                let _ = fs::remove_file(&temp_bin);
                if !status.success() {
                    process::exit(status.code().unwrap_or(1));
                }
            } else {
                eprintln!("❌ Compilation failed");
                for err in &result.errors {
                    eprintln!("  {}", err);
                }
                process::exit(1);
            }
        }
    }
}
