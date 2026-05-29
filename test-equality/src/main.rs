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
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    // this benchmark verifies halleys comet correctness over 25 thousand steps
    // cpp::run("halleys-comet", 0.0014, 25000);
    // rust::run("halleys-comet", 0.0014, 25000);
    // test::run("halleys-comet", 25000);
    cpp::bench("halleys-comet", 0.0014, 25000000);
    rust::bench("halleys-comet", 0.0014, 25000000);
    println!("{}", "-".repeat(50));

    // this benchmark does nothing useful
    cpp::run("two-colliding-particles", 0.0014, 100);
    rust::run("two-colliding-particles", 0.0014, 100);
    test::run("two-colliding-particles", 100);
    println!("{}", "-".repeat(50));

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001", 0.0007, 1);
    rust::run("two-bodies-collision-0001", 0.0007, 1);
    test::run("two-bodies-collision-0001", 1);
    println!("{}", "-".repeat(50));

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    cpp::bench("two-bodies-collision-0001", 0.0007, 20);
    rust::bench("two-bodies-collision-0001", 0.0007, 20);
    // test::run("two-bodies-collision-0001", 20);
    println!("{}", "-".repeat(50));

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    cpp::bench("two-bodies-collision-0001", 0.0007, 50);
    rust::bench("two-bodies-collision-0001", 0.0007, 50);
    // test::run("two-bodies-collision-0001", 50);
    println!("{}", "-".repeat(50));

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001-linked-cells", 0.0007, 1);
    rust::run("two-bodies-collision-0001-linked-cells", 0.0007, 1);
    test::run("two-bodies-collision-0001-linked-cells", 1);
    println!("{}", "-".repeat(50));

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    cpp::bench("two-bodies-collision-0001-linked-cells", 0.0007, 20);
    rust::bench("two-bodies-collision-0001-linked-cells", 0.0007, 20);
    // test::run("two-bodies-collision-0001", 20);
    println!("{}", "-".repeat(50));

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    cpp::bench("two-bodies-collision-0001-linked-cells", 0.0007, 50);
    rust::bench("two-bodies-collision-0001-linked-cells", 0.0007, 50);
    // test::run("two-bodies-collision-0001", 50);
    println!("{}", "-".repeat(50));
}
