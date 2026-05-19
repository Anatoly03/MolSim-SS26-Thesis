//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod cpp;
mod log;
mod rust;
mod test;

pub use log::Log;

fn main() {
    cpp::build();
    rust::build();

    // make directories: `output/rs`, `output/cpp`
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    // run tests
    cpp::run("halleys-comet", 0.0014, 25000);
    rust::run("halleys-comet", 0.0014, 25000);
    test::run("halleys-comet", 25000);

    cpp::run("two-colliding-particles", 0.0014, 100);
    rust::run("two-colliding-particles", 0.0014, 100);
    test::run("two-colliding-particles", 100);
}
