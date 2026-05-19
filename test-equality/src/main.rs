//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod cpp;
mod log;
mod rust;
mod test;

pub use log::Log;

fn main() {
    // println!("=== Build Binaries ===");
    cpp::build();
    cpp::run();
    rust::build();
    rust::run();
    // println!("=== Test Evaluations ===");
    test::run();
}
