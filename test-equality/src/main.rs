//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod cpp;
mod log;
mod rust;
mod test;

pub use log::Log;
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
pub fn TIME_STEPS() -> Vec<usize> {
    use std::ops::Range;

    Range { start: 10, end: 500 }
        .into_iter()
        .filter(|i| i % 10 == 0)
        .collect::<Vec<usize>>()
}

fn main() {
    cpp::build();
    rust::build();
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    Log::header(format!("lscpu"));
    let cpu = Cpu::new();
    Log::Info.log("Architecture\t", &cpu.architecture);
    Log::Info.log("CPU op modes\t", &cpu.cpu_op_modes);
    Log::Info.log("Address sizes\t", &cpu.address_sizes);
    Log::Info.log("Byte order\t", &cpu.byte_order);
    Log::Info.log("CPU count\t", &cpu.cpu_count.to_string());
    Log::Info.log("On-line CPU\t", &cpu.on_line_cpu.to_string());
    Log::Info.log("Vendor ID\t", &cpu.vendor_id);
    Log::Info.log("Model name\t", &cpu.model_name);
    Log::Info.log("CPU family\t", &cpu.cpu_family.to_string());
    Log::Info.log("CPU model\t", &cpu.cpu_model.to_string());
    Log::Info.log("Is hybrid\t", &cpu.is_hybrid);
    Log::Info.log("Threads per core", &cpu.threads_per_core.to_string());
    Log::Info.log("Cores per socket", &cpu.cores_per_socket.to_string());
    Log::Info.log("Sockets\t", &cpu.sockets.to_string());
    Log::Info.log("Stepping\t", &cpu.stepping.to_string());
    Log::Info.log("Boost enabled\t", &cpu.boost_enabled);

    #[cfg(feature = "extended")]
    {
        // this benchmark verifies halleys comet correctness over 25 thousand steps
        // cpp::run("halleys-comet", 0.0014, 25000);
        // rust::run("halleys-comet", 0.0014, 25000);
        // test::run("halleys-comet", 25000);
        cpp::bench("halleys-comet", 0.0014, 25000000);
        rust::bench("halleys-comet", 0.0014, 25000000);

        // this benchmark does nothing useful
        cpp::run("two-colliding-particles", 0.0014, 100);
        rust::run("two-colliding-particles", 0.0014, 100);
        test::run("two-colliding-particles", 100);
    }

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001", 0.0007, 1);
    rust::run("two-bodies-collision-0001", 0.0007, 1);
    test::run("two-bodies-collision-0001", 1);

    for frames in TIME_STEPS() {
        cpp::bench("two-bodies-collision-0001", 0.0007, frames);
        rust::bench("two-bodies-collision-0001", 0.0007, frames);
    }

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001-linked-cells", 0.0007, 1);
    rust::run("two-bodies-collision-0001-linked-cells", 0.0007, 1);
    test::run("two-bodies-collision-0001-linked-cells", 1);

    for frames in TIME_STEPS() {
        cpp::bench("two-bodies-collision-0001-linked-cells", 0.0007, frames);
        rust::bench("two-bodies-collision-0001-linked-cells", 0.0007, frames);
    }

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001-parallel", 0.0007, 1);
    rust::run("two-bodies-collision-0001-parallel", 0.0007, 1);

    for frames in TIME_STEPS() {
        cpp::bench("two-bodies-collision-0001-parallel", 0.0007, frames);
        rust::bench("two-bodies-collision-0001-parallel", 0.0007, frames);
    }
}
