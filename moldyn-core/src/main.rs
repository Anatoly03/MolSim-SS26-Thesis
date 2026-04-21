//! The main module for the moldyn-core library.
#![crate_name = "moldyn_core"]

mod forces;
mod particle;
mod reader;
mod simulation;
mod vec3;

use clap::Parser;
pub use forces::{Force, LennardJonesForce, NewtonForce};
pub use particle::Particle;
use std::path::PathBuf;
pub use vec3::Vec3;

use crate::reader::FileDefinition;

/// Molecular Dynamics Thesis Code. This library implements a simple
/// engine to simulate molecular dynamics.
#[derive(Parser)]
#[command(version, about, long_about = None)]
// see pathbuf: https://stackoverflow.com/q/76341332
struct Args {
    /// The input file for the simulation. The parser will be selected from the
    /// file extension. Supported formats are: YAML.
    input: PathBuf,

    /// The output file for the simulation results. If a deep path is provided,
    /// the directories along the path will be created if they do not exist.
    /// The output format will be selected from the file extension. Supported formats
    /// are: YAML.
    #[arg(short, long, default_value = "output/output.yaml")]
    output: PathBuf,
}

/// The main entry point for the moldyn-core library.
fn main() {
    let args = Args::parse();

    // read the input file and parse it into [FileDefinition]
    let input = match FileDefinition::try_from(args.input) {
        Ok(def) => def,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    println!("simulation name: `{}`", input.name);

    // this is supposed to panic and confirm that force deserialization is working
    let mut p1 = Particle::default();
    let mut p2 = Particle::default();
    input.force.apply_force(&mut p1, &mut p2);
}
