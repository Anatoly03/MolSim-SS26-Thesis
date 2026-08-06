//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

#![allow(dead_code, unused)]

mod cpp;
mod log;
mod rust;
mod test;

use std::{format, fs::File, process::{Command, Output, Stdio}};

pub use log::Logger;
use lscpu::Cpu;

/// Amount of times (repetitions) to run a program for benching. It has been set
/// to `5` previously for Github CI and is set to `20` on the `full` feature for
/// better averaged data.
#[cfg(feature = "quick")]
pub const REPETITIONS: usize = 5;

/// Amount of times (repetitions) to run a program for benching. It has been set
/// to `5` previously for Github CI and is set to `20` on the `full` feature for
/// better averaged data.
#[cfg(not(feature = "quick"))]
pub const REPETITIONS: usize = 20;

// warn simple errors
#[cfg(all(feature = "quick", feature = "full"))]
compile_error!("the features `quick` and `full` are mutually exclusive [hint: `extended` enables `full`]");

// #[cfg(not(any(feature = "rust", feature = "cpp")))]
// compile_error!("at least either of features `rust` or `cpp` have to be set");

/// Maximal amount of ticks (repetitions) to run a program for benching. It steps
/// every `10` timesteps and was previously set to `50` for CI. With the `full`
/// feature it is extended to use the argument and generate a custom range which
/// steps every 10 units.
#[cfg(feature = "quick")]
#[allow(non_snake_case)]
pub fn TIME_STEPS(_limit: usize, _step: usize) -> Vec<usize> {
    return vec![1, 20, 50, 250];
}

/// Maximal amount of ticks (repetitions) to run a program for benching. It steps
/// every `10` timesteps and was previously set to `50` for CI. With the `full`
/// feature it is extended to make sure the Two Bodies Collision occurs for linked
/// cells.
#[cfg(not(feature = "quick"))]
#[allow(non_snake_case)]
pub fn TIME_STEPS(limit: usize, step: usize) -> Vec<usize> {
    use std::ops::Range;

    Range {
        start: 10,
        end: limit + 1, // +1 so the limit is inclusive
    }
    .into_iter()
    .filter(|i| i % step == 0)
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
/// 
/// # Links
/// 
/// - https://www.openmp.org/spec-html/5.0/openmpse50.html#x289-20540006.2
/// - https://github.com/rayon-rs/rayon/blob/main/FAQ.md
pub fn set_thread_count(log: &mut Logger, count: usize) {
    log.info("Threads :=", &count.to_string());
    
    unsafe {
        std::env::set_var("RAYON_NUM_THREADS", count.to_string());
        std::env::set_var("OMP_NUM_THREADS", count.to_string());
    }
}

/// Sets the environment variable `OMP_PLACES`. Possible values: `cores`, `threads` or `sockets`.
/// 
/// See https://www.openmp.org/spec-html/5.0/openmpse53.html
#[cfg(feature = "direct-sum-parallel")]
pub fn set_omp_places(log: &mut Logger, place: &str) {
    log.info("OMP Places :=", place);

    unsafe {
        std::env::set_var("OMP_PLACES", place);
    }
}

/// Sets the environment variable `OMP_PROC_BIND`. Possible values: `CLOSE` or `SPREAD`.
/// 
/// See https://www.openmp.org/spec-html/5.0/openmpse52.html
#[cfg(feature = "direct-sum-parallel")]
pub fn set_omp_proc_bind(log: &mut Logger, proc_bind: &str) {
    log.info("OMP Proc Bind :=", proc_bind);

    unsafe {
        std::env::set_var("OMP_PROC_BIND", proc_bind);
    }
}

fn main() {
    let log_file = LOG_FILE();
    let mut log: Logger = log_file.into();

    #[cfg(all(feature = "extended", feature = "quick"))]
    panic!("Features `extended` and `quick` not compatible.");

    // Print commit id and runner command.
    let header_line = {
        let git_hash = option_env!("GIT_COMMIT_HASH")
            .map(|commit| format!("on commit {commit}"))
            .unwrap_or("".into());
        let features = env!("ENABLED_FEATURES");

        format!("{git_hash}:\nFeatures: {features}\n\n")
    };
    log.file_only(header_line);

    cpp::build(&mut log);
    rust::build(&mut log);
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    // https://stackoverflow.com/questions/22155130/determine-number-of-cores-using-rust
    #[allow(non_snake_case)]
    let THREAD_COUNT = num_cpus::get();

    // lscpu
    log.header(format!("lscpu [console]"));
    match Command::new("lscpu")
        .stdout(Stdio::piped())
        .output() {
            Ok(Output { stdout, .. }) => {
                let lscpu_stdout = String::from_utf8(stdout).unwrap();
                let lscpu_lines = lscpu_stdout.lines().for_each(|line| {
                    let mut iter = line.split_inclusive(':');
                    let title = iter.next().unwrap_or("");
                    let body = iter.collect::<Vec<&str>>().join("");
                    log.info(title, body.trim());
                });
            },
            Err(e) => log.warn("failed to run lscpu:", &e.to_string()),
        };

    let cpu = Cpu::new();
    log.header(format!("lscpu [rust crate]"));
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

    #[cfg(all(feature = "extended", feature = "halleys-comet"))]
    {
        // this benchmark verifies halleys comet correctness over 25 thousand steps
        // cpp::run("halleys-comet", 0.0014, 25000);
        // rust::run("halleys-comet", 0.0014, 25000);
        // test::run("halleys-comet", 25000);
        cpp::bench(&mut log, "halleys-comet", "halleys-comet", 0.0014, 25000000);
        rust::bench(&mut log, "halleys-comet", "halleys-comet", 0.0014, 25000000);

        // test two colliding particles (direct sum, newton force)
        cpp::run(&mut log, "two-colliding-particles", "two-colliding-particles", 0.0014, 100);
        rust::run(&mut log, "two-colliding-particles", "two-colliding-particles", 0.0014, 100);
        test::run(&mut log, "two-colliding-particles", 100);

        // test two-bodies-collision (direct sum, lennard jones)
        cpp::run(&mut log, "two-bodies-collision [direct-sum]", "two-bodies-collision-0001", 0.0014, 100);
        rust::run(&mut log, "two-bodies-collision [direct-sum]", "two-bodies-collision-0001", 0.0014, 100);
        test::run(&mut log, "two-bodies-collision-0001", 100);

        // test two-bodies-collision (direct sum, parallel, lennard jones)
        cpp::run(&mut log, "two-bodies-collision [direct-sum, parallel]", "two-bodies-collision-0001-parallel", 0.0014, 100);
        rust::run(&mut log, "two-bodies-collision [direct-sum, parallel]", "two-bodies-collision-0001-parallel", 0.0014, 100);
        test::run(&mut log, "two-bodies-collision-0001-parallel", 100);
    }

    #[cfg(feature = "direct-sum")]
    {
        #[cfg(not(feature = "quick"))]
        {
            // this benchmark measures I/O performance
            cpp::run(&mut log, "two-bodies-collision-0001 [IO]", "two-bodies-collision-0001", 0.0007, 1);
            rust::run(&mut log, "two-bodies-collision-0001 [IO]", "two-bodies-collision-0001", 0.0007, 1);
            test::run(&mut log, "two-bodies-collision-0001", 1);
        }
    
        // This benchmark measures DirectSum (Sequential).
        for frames in TIME_STEPS(500, 10) {
            cpp::bench(&mut log, &format!("two-bodies-collision [direct-sum]"), "two-bodies-collision-0001", 0.0007, frames);
            rust::bench(&mut log, &format!("two-bodies-collision [direct-sum]"), "two-bodies-collision-0001", 0.0007, frames);
        }
    }

    #[cfg(feature = "linked-cells")]
    {
        #[cfg(not(feature = "quick"))]
        {
            // this benchmark measures I/O performance
            cpp::run(&mut log, "two-bodies-collision-0001-linked-cells [IO]", "two-bodies-collision-0001-linked-cells", 0.007, 1);
            rust::run(&mut log, "two-bodies-collision-0001-linked-cells [IO]", "two-bodies-collision-0001-linked-cells", 0.007, 1);
            test::run(&mut log, "two-bodies-collision-0001-linked-cells", 1);
        }
    
        // This benchmark measures LinkedCells (Sequential).
        for frames in TIME_STEPS(500, 10) {
            cpp::bench(&mut log, &format!("two-bodies-collision [linked-cells]"), "two-bodies-collision-0001-linked-cells", 0.007, frames);
            rust::bench(&mut log, &format!("two-bodies-collision [linked-cells]"), "two-bodies-collision-0001-linked-cells", 0.007, frames);
        }
    }

    #[cfg(feature = "linked-cells-collision-point")]
    {
        // With manual CLI tests I found a good combination of values:
        // ./target/release/moldyn-cli input/two-bodies-collision-0001-linked-cells.yaml -d 0.0014 -t 10 -s10
        // Collision starts at time step 700.

        for frames in TIME_STEPS(2000, 20) {
            cpp::bench(&mut log, &format!("two-bodies-collision [linked-cells; collision at=700]"), "two-bodies-collision-0001-linked-cells", 0.0014, frames);
            rust::bench(&mut log, &format!("two-bodies-collision [linked-cells; collision at=700]"), "two-bodies-collision-0001-linked-cells", 0.0014, frames);
        }
    }

    #[cfg(feature = "direct-sum-parallel")]
    {
        // this doesn't work
        // https://www.openmp.org/spec-html/5.0/openmpse61.html
        unsafe {
            std::env::set_var("OMG_DISPLAY_AFFINITY", "TRUE");
        }

        #[cfg(not(feature = "quick"))]
        {
            // this benchmark measures I/O performance
            cpp::run(&mut log, "two-bodies-collision-0001-parallel [IO]", "two-bodies-collision-0001-parallel", 0.0007, 1);
            rust::run(&mut log, "two-bodies-collision-0001-parallel [IO]", "two-bodies-collision-0001-parallel", 0.0007, 1);
        }
    
        // This benchmark measures DirectSum (Parallel).
        #[cfg(feature = "twice-thread-cap")]
        let thread_count_cap = (THREAD_COUNT * 2) + 1;

        #[cfg(not(feature = "twice-thread-cap"))]
        let thread_count_cap = THREAD_COUNT + 1;

        #[cfg(any(feature = "full", feature = "extended"))]
        for thread_count in (1 .. thread_count_cap).rev() {
            set_thread_count(&mut log, thread_count);

            #[cfg(feature = "extended")]
            for frames in TIME_STEPS(500, 25) {
                for omp_place in ["cores", "threads", "sockets"].iter() {
                    set_omp_places(&mut log, omp_place);

                    for omp_proc_bind in ["CLOSE", "SPREAD"].iter() {
                        set_omp_proc_bind(&mut log, omp_proc_bind);

                        cpp::bench(
                            &mut log,
                            &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}, omp_place={omp_place}, omp_proc_bind={omp_proc_bind}]"),
                            "two-bodies-collision-0001-parallel", 0.0007, frames);
                    }
                }
                rust::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}]"), "two-bodies-collision-0001-parallel", 0.0007, frames);
            }

            #[cfg(not(feature = "extended"))]
            {
                // if not feature "extended", compute different thread counts for 200 ticks and 500

                // 200
                for omp_place in ["cores", "threads", "sockets"].iter() {
                    set_omp_places(&mut log, omp_place);

                    for omp_proc_bind in ["CLOSE", "SPREAD"].iter() {
                        set_omp_proc_bind(&mut log, omp_proc_bind);

                        cpp::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}, omp_place={omp_place}, omp_proc_bind={omp_proc_bind}]"), "two-bodies-collision-0001-parallel", 0.0007, 200);
                    }
                }

                rust::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}]"), "two-bodies-collision-0001-parallel", 0.0007, 200);

                // 500
                for omp_place in ["cores", "threads", "sockets"].iter() {
                    set_omp_places(&mut log, omp_place);

                    for omp_proc_bind in ["CLOSE", "SPREAD"].iter() {
                        set_omp_proc_bind(&mut log, omp_proc_bind);

                        cpp::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}, omp_place={omp_place}, omp_proc_bind={omp_proc_bind}]"), "two-bodies-collision-0001-parallel", 0.0007, 500);
                    }
                }

                rust::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={thread_count}]"), "two-bodies-collision-0001-parallel", 0.0007, 500);
            }
        }

        #[cfg(not(any(feature = "full", feature = "extended")))]
        {
            set_thread_count(&mut log, THREAD_COUNT);

            // This benchmark measures DirectSum (Sequential).
            for frames in TIME_STEPS(500, 10) {
                cpp::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={THREAD_COUNT}]"), "two-bodies-collision-0001-parallel", 0.0007, frames);
                rust::bench(&mut log, &format!("two-bodies-collision [direct-sum, parallel, threads={THREAD_COUNT}]"), "two-bodies-collision-0001-parallel", 0.0007, frames);
            }
        }
    }
}
