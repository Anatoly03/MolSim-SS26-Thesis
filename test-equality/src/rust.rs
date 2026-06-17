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

pub fn internal(name: &str, delta: f64, frames: usize, write_output: bool, program_runs: usize) {
    let frame_period = if write_output { "1" } else { "0" };
    let args = [
        &format!("input/{name}.yaml"),
        "-t",
        &(delta * (frames as f64)).to_string(),
        "-d",
        &delta.to_string(),
        "-s",
        frame_period,
        "-o",
        &format!("output/rs/{name}.xyz"),
    ];

    let cmd = format!("`./target/release/moldyn-cli {}`", args.join(" "));
    Log::Success.log("Running", &cmd);

    let mut run_durations = vec![];
    for run_index in 0..program_runs {
        // current time
        let current_time = std::time::Instant::now();

        let rs_moldyn_status = Command::new("./target/release/moldyn-cli")
            .args(args)
            .stdout(Stdio::null())
            .status()
            .expect("Failed to execute cmake");

        // log elapsed time
        let elapsed_nano = current_time.elapsed().as_nanos();
        Log::Info.log(
            "Bench",
            &format!("{} ms [run {}]", elapsed_nano as f64 / 1e6, run_index + 1),
        );
        run_durations.push(elapsed_nano);

        if !rs_moldyn_status.success() {
            Log::Failure.log("Error", "failed to run `target/release/moldyn-cli`");
        }
    }

    if run_durations.len() > 1 {
        // i do not know how the math works, ask supervisor for meaningful benchmark data
        let avg = run_durations.iter().sum::<u128>() as f64 / run_durations.len() as f64;
        let min = run_durations.iter().min().unwrap_or(&0);
        let max = run_durations.iter().max().unwrap_or(&0);
        let threshold = (max - min) / 2;

        // rust prints benchmarks like this: 32,118.43 ns/iter (+/- 565.76)
        Log::Info.log(
            "Bench",
            &format!("{} +/- {} ms", avg as f64 / 1e6, threshold as f64 / 1e6),
        );
    }
}

/// Runs C++
pub fn run(name: &str, delta: f64, frames: usize) {
    Log::header(format!("{name} (rust)"));
    internal(name, delta, frames, true, 1);
}

/// Runs C++
pub fn bench(name: &str, delta: f64, frames: usize) {
    Log::header(format!("{name} (rust)"));
    internal(name, delta, frames, false, 5);
}
