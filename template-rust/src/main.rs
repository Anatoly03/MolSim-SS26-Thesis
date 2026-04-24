//! Welcome to the Molecular Dynamics teaching code (Rust Edition). This template
//! code provides a basic structure for a simple simulation program.
//!
//! If you are rather new to Rust, you might want to check the following
//! resources:
//!
//! - [Install Rust](https://rust-lang.org/tools/install/)
//! - [The Rust Book](https://doc.rust-lang.org/book/)
//! - [The Rust Reference](https://doc.rust-lang.org/reference/)
//! - [What is RustDoc?](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
//!
//! The following Rust commands are useful for development:
//! 
//! - [cargo build](https://doc.rust-lang.org/nightly/cargo/commands/cargo-build.html)
//! - [cargo clippy](https://doc.rust-lang.org/nightly/cargo/commands/cargo-clippy.html)
//! - [cargo fmt](https://doc.rust-lang.org/nightly/cargo/commands/cargo-fmt.html)
//! - [cargo run](https://doc.rust-lang.org/nightly/cargo/commands/cargo-run.html)

mod file_reader;
mod output_writer;
mod particle;

use crate::file_reader::FileReader;
#[cfg(feature = "vtk")]
use crate::output_writer::VTKWriter;
use crate::output_writer::{OutputWriter, XYZWriter};
use crate::particle::Particle;

const START_TIME: f64 = 0.0;
const END_TIME: f64 = 1000.0;
const DELTA_T: f64 = 0.014;

// This is the entry point of the Rust application.
pub fn main() -> std::io::Result<()> {
    println!("Hello from MolSim for PSE!");

    if std::env::args().len() != 2 {
        println!("Usage: cargo run -- <filename>");
        std::process::exit(1);
    }

    // TODO: what data structure to pick?
    let mut particles: Vec<Particle> = Vec::new();

    FileReader::read_file(&mut particles, std::env::args().nth(1).unwrap().as_str())?;

    let mut current_time = START_TIME;
    let mut iteration = 0;

    while current_time < END_TIME {
        // calculate new x
        calculate_x(&mut particles);
        // calculate new f
        calculate_f(&mut particles);
        // calculate new v
        calculate_v(&mut particles);

        iteration += 1;
        if iteration % 10 == 0 {
            plot_particles(&mut particles, iteration);
        }
        println!("Iteration {iteration} finished.");

        current_time += DELTA_T;
    }

    println!("output written. Terminating...");
    Ok(())
}

/// calculate the force for all particles
fn calculate_f(particles: &mut Vec<Particle>) {
    for i1 in 0..particles.len() {
        for i2 in 0..particles.len() {
            let p1 = &particles[i1];
            let p2 = &particles[i2];

            // TODO: insert calculation of forces here!
        }
    }
}

/// calculate the position for all particles
fn calculate_x(particles: &mut Vec<Particle>) {
    for p in particles {
        // TODO: insert calculation of position updates here!
    }
}

/// calculate the velocity for all particles
fn calculate_v(particles: &mut Vec<Particle>) {
    for p in particles {
        // TODO: insert calculation of velocity updates here!
    }
}

fn plot_particles(particles: &mut Vec<Particle>, iteration: usize) {
    let out_name = String::from("MD_vtk");

    #[cfg(feature = "vtk")]
    let writer = VTKWriter;
    #[cfg(not(feature = "vtk"))]
    let writer = XYZWriter;

    writer.plot_particles(particles, &out_name, iteration);

    // // Assuming outputWriter::XYZWriter is defined elsewhere
    // let writer = outputWriter::XYZWriter::new();
    // writer.plotParticles(&particles, &out_name, iteration);
}
