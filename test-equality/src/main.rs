//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod cpp;
mod log;
mod rust;
mod test;

use std::{format, fs::File, io::Write};

pub use log::Logger;
use lscpu::Cpu;

/// Amount of times (repetitions) to run a program for benching. It has been set
/// to `5` previously for Github CI and is set to `20` on the `full` feature for
/// better averaged data.
#[cfg(not(feature = "full"))]
pub const REPETITIONS: usize = 5;

/// Amount of times (repetitions) to run a program for benching. It has been set
/// to `5` previously for Github CI and is set to `20` on the `full` feature for
/// better averaged data.
#[cfg(feature = "full")]
pub const REPETITIONS: usize = 20;

/// Maximal amount of ticks (repetitions) to run a program for benching. It steps
/// every `10` timesteps and was previously set to `50` for CI. With the `full`
/// feature it is extended to make sure the Two Bodies Collision occurs for linked
/// cells.
#[cfg(not(feature = "full"))]
#[allow(non_snake_case)]
pub fn TIME_STEPS() -> Vec<usize> {
    return vec![1, 20, 50];
}

/// Maximal amount of ticks (repetitions) to run a program for benching. It steps
/// every `10` timesteps and was previously set to `50` for CI. With the `full`
/// feature it is extended to make sure the Two Bodies Collision occurs for linked
/// cells.
#[cfg(feature = "full")]
#[allow(non_snake_case)]
pub fn TIME_STEPS() -> Vec<usize> {
    use std::ops::Range;

    Range {
        start: 10,
        end: 500,
    }
    .into_iter()
    .filter(|i| i % 10 == 0)
    .collect::<Vec<usize>>()
}

/// Optionally retrieve the filename of how to save the current logs. If set to none,
/// no file name should be created.
#[cfg(not(feature = "print"))]
#[allow(non_snake_case)]
pub fn LOG_FILE_NAME() -> Option<String> {
    None
}

/// Optionally retrieve the filename of how to save the current logs. If set to none,
/// no file name should be created.
#[cfg(feature = "print")]
#[allow(non_snake_case)]
pub fn LOG_FILE_NAME() -> Option<String> {
    use chrono::Utc;
    let now = Utc::now();

    // there is a bug where "1th and 2th" are generated. the fix is to not run this program
    // on any day of the month which ends with 1 or 2.

    Some(now.format("%dth %B, %H:%M.txt").to_string())
}

/// Optionally creates a log file in which everything that is printed to the console is
/// also saved.
#[allow(non_snake_case)]
pub fn LOG_FILE() -> Option<File> {
    match LOG_FILE_NAME()
        .map(|f| format!("test-equality/results/{f}"))
        .map(|path| File::create(path))
    {
        Some(Ok(f)) => Some(f),
        _ => None,
    }
}

/// Sets the number of threads for the parallel implementations, which is the environment
/// variable `RAYON_NUM_THREADS` for Rust and `OMP_NUM_THREADS` for C++.
pub fn set_thread_count(log: &mut Logger, count: usize) {
    log.info("Threads :=", &count.to_string());
    
    unsafe {
        std::env::set_var("RAYON_NUM_THREADS", count.to_string());
        std::env::set_var("OMP_NUM_THREADS", count.to_string());
    }
}

fn main() {
    let log_file = LOG_FILE();
    let mut log: Logger = log_file.into();

    cpp::build(&mut log);
    rust::build(&mut log);
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    #[allow(non_snake_case)]
    let THREAD_COUNT = num_cpus::get();

    // lscpu
    let cpu = Cpu::new();
    log.header(format!("lscpu"));
    log.info("Architecture\t", &cpu.architecture);
    log.info("CPU op modes\t", &cpu.cpu_op_modes);
    log.info("Address sizes\t", &cpu.address_sizes);
    log.info("Byte order\t", &cpu.byte_order);
    log.info("CPU count\t", &cpu.cpu_count.to_string());
    log.info("On-line CPU\t", &cpu.on_line_cpu.to_string());
    log.info("Vendor ID\t", &cpu.vendor_id);
    log.info("Model name\t", &cpu.model_name);
    log.info("CPU family\t", &cpu.cpu_family.to_string());
    log.info("CPU model\t", &cpu.cpu_model.to_string());
    log.info("Is hybrid\t", &cpu.is_hybrid);
    log.info("Threads per core", &cpu.threads_per_core.to_string());
    log.info("Cores per socket", &cpu.cores_per_socket.to_string());
    log.info("Sockets\t", &cpu.sockets.to_string());
    log.info("Stepping\t", &cpu.stepping.to_string());
    log.info("Boost enabled\t", &cpu.boost_enabled);

    // even more metrics
    log.header(format!("extended cpu metrics"));
    log.info("Thread Count", &THREAD_COUNT.to_string());

    #[cfg(feature = "extended")]
    {
        // this benchmark verifies halleys comet correctness over 25 thousand steps
        // cpp::run("halleys-comet", 0.0014, 25000);
        // rust::run("halleys-comet", 0.0014, 25000);
        // test::run("halleys-comet", 25000);
        cpp::bench(&mut log, "halleys-comet", "halleys-comet", 0.0014, 25000000);
        rust::bench(&mut log, "halleys-comet", "halleys-comet", 0.0014, 25000000);

        // this benchmark does nothing useful
        cpp::run(&mut log, "two-colliding-particles", "two-colliding-particles", 0.0014, 100);
        rust::run(&mut log, "two-colliding-particles", "two-colliding-particles", 0.0014, 100);
        test::run(&mut log, "two-colliding-particles", "two-colliding-particles", 100);
    }

    // this benchmark measures I/O performance
    cpp::run(&mut log, "two-bodies-collision-0001 [IO]", "two-bodies-collision-0001", 0.0007, 1);
    rust::run(&mut log, "two-bodies-collision-0001 [IO]", "two-bodies-collision-0001", 0.0007, 1);
    test::run(&mut log, "two-bodies-collision-0001", 1);

    // This benchmark measures DirectSum (Sequential).
    for frames in TIME_STEPS() {
        cpp::bench(&mut log, &format!("two-bodies-collision [direct-sum]"), "two-bodies-collision-0001", 0.0007, frames);
        rust::bench(&mut log, &format!("two-bodies-collision [direct-sum]"), "two-bodies-collision-0001", 0.0007, frames);
    }

    // this benchmark measures I/O performance
    cpp::run(&mut log, "two-bodies-collision-0001-linked-cells [IO]", "two-bodies-collision-0001-linked-cells", 0.0007, 1);
    rust::run(&mut log, "two-bodies-collision-0001-linked-cells [IO]", "two-bodies-collision-0001-linked-cells", 0.0007, 1);
    test::run(&mut log, "two-bodies-collision-0001-linked-cells", 1);

    // This benchmark measures LinkedCells (Sequential).
    for frames in TIME_STEPS() {
        cpp::bench(&mut log, &format!("two-bodies-collision [linked-cells]"), "two-bodies-collision-0001-linked-cells", 0.0007, frames);
        rust::bench(&mut log, &format!("two-bodies-collision [linked-cells]"), "two-bodies-collision-0001-linked-cells", 0.0007, frames);
    }

    // this benchmark measures I/O performance
    cpp::run(&mut log, "two-bodies-collision-0001-parallel [IO]", "two-bodies-collision-0001-parallel", 0.0007, 1);
    rust::run(&mut log, "two-bodies-collision-0001-parallel [IO]", "two-bodies-collision-0001-parallel", 0.0007, 1);

    // This benchmark measures DirectSum (Parallel).
    for thread_count in 1 .. THREAD_COUNT {
        set_thread_count(&mut log, thread_count);

        for frames in TIME_STEPS() {
            cpp::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}]"), "two-bodies-collision-0001-parallel", 0.0007, frames);
            rust::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}]"), "two-bodies-collision-0001-parallel", 0.0007, frames);
        }
    }
}
