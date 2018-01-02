extern crate logoc;

use std::fs::{self, File};
use std::path::Path;

use logoc::cargo;

fn main() {
    //TODO: Get these from command line args
    // The directory to place all build artifacts in
    let build_dir = Path::new("build");
    // The filename (without extension) of the output executable
    let output = "foo";
    // The path of the intermediate Rust file that will be generated
    let output_path = format!("{}.rs", output);

    fs::create_dir_all(build_dir)
        .expect("Failed to create directory for build output");

    let output_cargo_cfg = build_dir.join("Cargo.toml");
    let output_cargo_cfg = File::create(output_cargo_cfg).unwrap();
    cargo::write_cargo_toml(output_cargo_cfg, output, &output_path);
}
