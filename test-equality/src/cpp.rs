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
        .args([".", "-B", "target/cpp"])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !cmake_status.success() {
        Log::Failure.log("Error", "`cmake` configuration failed");
        std::process::exit(1);
    }

    let make_status = Command::new("make")
        .args(["-C", "target/cpp", "-j4", "--no-print-directory"])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute make");

    if !make_status.success() {
        Log::Failure.log("Error", "`make` compilation failed");
        std::process::exit(1);
    }
}

/// Runs C++
pub fn run(name: &str) {
    Log::Success.log("Running", "`target/cpp/MolSim`");

    let cpp_molsim_status = Command::new("./target/cpp/MolSim")
        .args([
            &format!("input/{name}.yaml"),
            "-t",
            "0.1",
            "-d",
            "0.0014",
            "-s",
            "1",
            "-o",
            &format!("output/cpp/{name}.xyz"),
        ])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !cpp_molsim_status.success() {
        Log::Failure.log("Error", "failed to run `target/cpp/MolSim`");
        std::process::exit(1);
    }
}
