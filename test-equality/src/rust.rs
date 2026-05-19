use crate::Log;
use std::process::{Command, Stdio};

/// Builds Rust
pub fn build() {
    Log::Success.log("Compiling", "target-rs");

    let _cargo_status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("Failed to execute cargo");
}

/// Runs Rust
pub fn run() {
    Log::Success.log("Running", "`target/release/moldyn-cli`");

    // make directories: `output/rs`, `output/cpp`
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    let rs_moldyn_status = Command::new("./target/release/moldyn-cli")
        .args([
            "input/halleys-comet.yaml",
            "-t",
            "0.1",
            "-d",
            "0.0014",
            "-s",
            "1",
            "-o",
            "output/rs/halleys-comet.xyz",
        ])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !rs_moldyn_status.success() {
        Log::Failure.log("Error", "failed to run `target/release/moldyn-cli`");
        std::process::exit(1);
    }
}
