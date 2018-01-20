extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate logoc;

use std::io::Write;
use std::fs::{self, File};
use std::process::{self, Command};
use std::path::PathBuf;

use structopt::StructOpt;

use logoc::ast::{Program, Instruction, Expr};
use logoc::codegen;
use logoc::cargo;

#[derive(Debug, StructOpt)]
#[structopt(name = "logoc")]
struct CompilerOptions {
    /// The directory to place all build artifacts in
    #[structopt(long = "build-dir", default_value = "build", parse(from_os_str),
        help = "The directory to place build artifacts and the generated executable")]
    build_dir: PathBuf,
    /// The path of the LOGO program to compile
    #[structopt(parse(from_os_str), help = "The path of the LOGO program to compile")]
    program: PathBuf,
    /// The filename (without extension) of the output executable
    #[structopt(short = "o", long = "output-file",
        help = "The filename (without extension) of the output executable")]
    output: Option<String>,
    /// Whether to format the generated code
    #[structopt(long = "fmt", help = "Whether to format the generated Rust code")]
    format: bool,
}

fn main() {
    let CompilerOptions {
        build_dir,
        program,
        output,
        format,
    } = CompilerOptions::from_args();
    let output = &output.unwrap_or_else(|| program.file_stem()
        .expect("Please provide a valid filename to a LOGO program").to_str()
        .expect("LOGO program filename was not valid unicode").to_owned());

    //TODO: Read from input file that is provided by command line args
    let ast: Program = vec![
        Instruction::Forward(Expr::Number(100.0)),
        Instruction::Right(Expr::Number(90.0)),
        Instruction::Forward(Expr::Number(100.0)),
        Instruction::Right(Expr::Number(90.0)),
        Instruction::Forward(Expr::Number(100.0)),
        Instruction::Right(Expr::Number(90.0)),
        Instruction::Forward(Expr::Number(100.0)),
        Instruction::Right(Expr::Number(90.0)),
    ];
    let rust_code = codegen::to_tokens(ast).unwrap();

    fs::create_dir_all(&build_dir)
        .expect("Failed to create directory for build output");


    // The path of the intermediate Rust file that will be generated
    let rust_filename = &format!("{}.rs", output);
    let rust_file_path = &build_dir.join(rust_filename);
    let mut rust_code_file = File::create(rust_file_path).unwrap();
    writeln!(rust_code_file, "{}", rust_code.as_str()).unwrap();

    let output_cargo_cfg = build_dir.join("Cargo.toml");
    let output_cargo_cfg = File::create(output_cargo_cfg).unwrap();
    cargo::write_cargo_toml(output_cargo_cfg, output, rust_filename);

    if format {
        let status = Command::new("rustfmt")
            .arg(rust_filename)
            .current_dir(&build_dir)
            .status()
            .expect("Failed to execute rustfmt -- make sure you have rustfmt installed");
        if !status.success() {
            eprintln!("\nFailed to format generated Rust code. \
                This is an internal compiler error in logoc. \
                Please report this error with some details of what led to it.");
            process::exit(1);
        }
    }

    // Run the generated Rust code
    let status = Command::new("cargo")
        .arg("build")
        .current_dir(&build_dir)
        .status()
        .expect("Failed to execute Rust compiler -- make sure you have Rust and Cargo installed.");

    if !status.success() {
        eprintln!("\nCargo failed to build the generated Rust code. \
            This is an internal compiler error in logoc. \
            Please report this error with some details of what led to it.");
        process::exit(1);
    }
}
