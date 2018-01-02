extern crate logoc;

use std::io::Write;
use std::fs::{self, File};
use std::path::Path;

use logoc::ast::{Program, Instruction, Expr};
use logoc::codegen::generate_rust;
use logoc::cargo;

fn main() {
    //TODO: Get these from command line args
    // The directory to place all build artifacts in
    let build_dir = Path::new("build");
    // The filename (without extension) of the output executable
    let output = "foo";
    // The path of the intermediate Rust file that will be generated
    let output_path = format!("{}.rs", output);

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
    let code = generate_rust(ast).unwrap();

    fs::create_dir_all(build_dir)
        .expect("Failed to create directory for build output");

    let output_code = build_dir.join(&output_path);
    let mut output_code = File::create(output_code).unwrap();
    writeln!(output_code, "{}", code.as_str()).unwrap();

    let output_cargo_cfg = build_dir.join("Cargo.toml");
    let output_cargo_cfg = File::create(output_cargo_cfg).unwrap();
    cargo::write_cargo_toml(output_cargo_cfg, output, &output_path);
}
