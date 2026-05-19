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
pub fn run(name: &str, delta: f64, frames: usize) {
    Log::Success.log("Running", "`target/release/moldyn-cli`");

    // make directories: `output/rs`, `output/cpp`
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    let rs_moldyn_status = Command::new("./target/release/moldyn-cli")
        .args([
            &format!("input/{name}.yaml"),
            "-t",
            &(delta * (frames as f64)).to_string(),
            "-d",
            &delta.to_string(),
            "-s",
            "1",
            "-o",
            &format!("output/rs/{name}.xyz"),
        ])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !rs_moldyn_status.success() {
        Log::Failure.log("Error", "failed to run `target/release/moldyn-cli`");
        std::process::exit(1);
    }
}
