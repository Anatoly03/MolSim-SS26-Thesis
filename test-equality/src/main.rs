//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod log;

pub use log::Log;
use std::process::{Command, Stdio};

/// Builds the C++ code using CMake and Make. Runs the following two commands.
/// 
/// ```bash
/// cmake . -B target/cpp
/// make -C target/cpp -j4 --no-print-directory
/// ```
fn build_cpp() {
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
fn run_cpp() {
    Log::Success.log("Running", "`target/cpp/MolSim`");

    let cpp_molsim_status = Command::new("./target/cpp/MolSim")
        .args(["input/halleys-comet.yaml", "-t", "0.1", "-d", "0.0014", "-s", "1", "-o", "output/cpp/halleys-comet.xyz"])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !cpp_molsim_status.success() {
        Log::Failure.log("Error", "failed to run `target/cpp/MolSim`");
        std::process::exit(1);
    }
}

/// Builds Rust
fn build_rs() {
    Log::Success.log("Compiling", "target-rs");

    let _cargo_status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("Failed to execute cargo");
}

/// Runs Rust
fn run_rs() {
    Log::Success.log("Running", "`target/release/moldyn-cli`");

    // make directories: `output/rs`, `output/cpp`
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    let rs_moldyn_status = Command::new("./target/release/moldyn-cli")
        .args(["input/halleys-comet.yaml", "-t", "0.1", "-d", "0.0014", "-s", "1", "-o", "output/rs/halleys-comet.xyz"])
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute cmake");

    if !rs_moldyn_status.success() {
        Log::Failure.log("Error", "failed to run `target/release/moldyn-cli`");
        std::process::exit(1);
    }
}

fn main() {
    println!("=== Build Binaries ===");
    build_cpp();
    run_cpp();
    build_rs();
    run_rs();
    println!("=== Test Evaluations ===");
    println!("    Unimplemented.");
}
