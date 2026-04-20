//! Welcome to the Molecular Dynamics Rust Edition teaching code. If
//! you are rather new to Rust, you might want to check the following
//! resources:
//! 
//! - [The Rust Book](https://doc.rust-lang.org/book/)
//! - [The Rust Reference](https://doc.rust-lang.org/reference/)
//! - [What is RustDoc?](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
//! 

mod particle;

use particle::Particle;

const START_TIME: f64 = 0.0;
const END_TIME: f64 = 1000.0;
const DELTA_T: f64 = 0.014;

// This is the entry point of the Rust application.
pub fn main() {
    println!("Hello from MolSim for PSE!");

    if std::env::args().len() != 2 {
        println!("Usage: cargo run -- <filename>");
        std::process::exit(1);
    }

    // TODO: what data structure to pick?
    let mut particles: Vec<Particle> = Vec::new();

    // TODO implement file reading

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
            plot_particles(iteration);
        }
        println!("Iteration {iteration} finished.");

        current_time += DELTA_T;
    }

    println!("output written. Terminating...");
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

fn plot_particles(iteration: i32) {
    let out_name = String::from("MD_vtk");

    // // Assuming outputWriter::XYZWriter is defined elsewhere
    // let writer = outputWriter::XYZWriter::new();
    // writer.plotParticles(&particles, &out_name, iteration);
}

