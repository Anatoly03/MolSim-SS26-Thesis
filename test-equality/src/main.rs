//! This runnable builds and tests the equality of the C++ and Rust implementations
//! for arithmetic equality.

mod cpp;
mod graph;
mod log;
mod runner;
mod rust;
mod test;

pub use log::Log;

fn main() {
    cpp::build();
    rust::build();
    std::fs::create_dir_all("output/rs").expect("");
    std::fs::create_dir_all("output/cpp").expect("");

    let mut graph = graph::ConsoleGraph::default();
    graph.console_width = 100;
    graph.console_height = 20;

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

    // this benchmark measures I/O performance
    let cpp_v = cpp::run("two-bodies-collision-0001", 0.0007, 1);
    let rust_v = rust::run("two-bodies-collision-0001", 0.0007, 1);
    test::run("two-bodies-collision-0001", 1);

    graph.add_point(1.0, cpp_v);
    graph.add_point(1.0, rust_v);

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    let cpp_v = cpp::bench("two-bodies-collision-0001", 0.0007, 20);
    let rust_v = rust::bench("two-bodies-collision-0001", 0.0007, 20);

    graph.add_point(20.0, cpp_v);
    graph.add_point(20.0, rust_v);

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    let cpp_v = cpp::bench("two-bodies-collision-0001", 0.0007, 50);
    let rust_v = rust::bench("two-bodies-collision-0001", 0.0007, 50);

    graph.add_point(50.0, cpp_v);
    graph.add_point(50.0, rust_v);

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001-linked-cells", 0.0007, 1);
    rust::run("two-bodies-collision-0001-linked-cells", 0.0007, 1);
    test::run("two-bodies-collision-0001-linked-cells", 1);

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    cpp::bench("two-bodies-collision-0001-linked-cells", 0.0007, 20);
    rust::bench("two-bodies-collision-0001-linked-cells", 0.0007, 20);

    // this benchmark additionally measures the accumulation of floating-point errors
    // over many steps
    cpp::bench("two-bodies-collision-0001-linked-cells", 0.0007, 50);
    rust::bench("two-bodies-collision-0001-linked-cells", 0.0007, 50);

    // this benchmark measures I/O performance
    cpp::run("two-bodies-collision-0001-parallel", 0.0007, 1);
    rust::run("two-bodies-collision-0001-parallel", 0.0007, 1);

    cpp::bench("two-bodies-collision-0001-parallel", 0.0007, 20);
    rust::bench("two-bodies-collision-0001-parallel", 0.0007, 20);

    cpp::bench("two-bodies-collision-0001-parallel", 0.0007, 50);
    rust::bench("two-bodies-collision-0001-parallel", 0.0007, 50);

    graph.plot();
}
