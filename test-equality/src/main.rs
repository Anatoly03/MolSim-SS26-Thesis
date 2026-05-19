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
    rust::build();
    // println!("=== Generate. Outputs ===");
    cpp::run("halleys-comet", 0.0014, 250);
    rust::run("halleys-comet", 0.0014, 250);
    test::run("halleys-comet", 250);

    cpp::run("two-colliding-particles", 0.0014, 100);
    rust::run("two-colliding-particles", 0.0014, 100);
    test::run("two-colliding-particles", 100);
}
