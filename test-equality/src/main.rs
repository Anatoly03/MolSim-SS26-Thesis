//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod log;

pub use log::Log;

fn main() {
    Log::Success.log("Success", "log example");
    Log::Warn.log("Warning", "log example");
    Log::Failure.log("Failure", "log example");
}
