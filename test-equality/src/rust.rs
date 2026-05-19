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
    let args = [
        &format!("input/{name}.yaml"),
        "-t",
        &(delta * (frames as f64)).to_string(),
        "-d",
        &delta.to_string(),
        "-s",
        "1",
        "-o",
        &format!("output/rs/{name}.xyz"),
    ];

    // current time
    let current_time = std::time::SystemTime::now();

    let cmd = format!("`./target/release/moldyn-cli {}`", args.join(" "));
    Log::Success.log("Running", &cmd);
    let rs_moldyn_status = Command::new("./target/release/moldyn-cli")
        .args(args)
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    // log elapsed time
    if let Ok(elapsed) = current_time.elapsed() {
        let elapsed_nano = elapsed.as_nanos();
        Log::Info.log("Bench", &format!("{} ms", elapsed_nano as f64 / 1e6));
    }

    if !rs_moldyn_status.success() {
        Log::Failure.log("Error", "failed to run `target/release/moldyn-cli`");
        std::process::exit(1);
    }
}
