use crate::Log;
use std::process::{Command, Stdio};

/// Builds the C++ code using CMake and Make. Runs the following two commands.
///
/// ```bash
/// cmake . -B target/cpp
/// make -C target/cpp -j4 --no-print-directory
/// ```
pub fn build() {
    Log::Success.log("Compiling", "target-cpp");

    let cmake_status = Command::new("cmake")
        .args([".", "-B", "target/cpp", "--config", "Release"])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !cmake_status.success() {
        Log::Failure.log("Error", "`cmake` configuration failed");
        std::process::exit(1);
    }

    let make_status = Command::new("make")
        .args(["-C", "target/cpp", "-j4", "--no-print-directory"])
        // .stdout(Stdio::null())
        .status()
        .expect("Failed to execute make");

    if !make_status.success() {
        Log::Failure.log("Error", "`make` compilation failed");
        std::process::exit(1);
    }
}

/// Runs C++
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
        &format!("output/cpp/{name}.xyz"),
    ];

    // current time
    let current_time = std::time::SystemTime::now();

    let cmd = format!("`./target/cpp/MolSim {}`", args.join(" "));
    Log::Success.log("Running", &cmd);
    let cpp_molsim_status = Command::new("./target/cpp/MolSim")
        .args(args)
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    // log elapsed time
    if let Ok(elapsed) = current_time.elapsed() {
        let elapsed_nano = elapsed.as_nanos();
        Log::Info.log("Bench", &format!("{} ms", elapsed_nano as f64 / 1e6));
    }

    if !cpp_molsim_status.success() {
        Log::Failure.log("Error", "failed to run `target/cpp/MolSim`");
        std::process::exit(1);
    }
}
