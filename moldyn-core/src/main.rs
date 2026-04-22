//! The main module for the moldyn-core library.
#![crate_name = "moldyn_core"]

mod forces;
mod particle;
mod reader;
mod simulation;
mod vec3;
mod writer;

use clap::Parser;
pub use forces::{Force, LennardJonesForce, NewtonForce};
pub use particle::Particle;
pub use simulation::Simulation;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
pub use vec3::Vec3;
pub use writer::OutputWriter;

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
    // TODO: make this output.yaml by default
    #[arg(short, long, default_value = "output/output.xyz")]
    output: PathBuf,

    /// The time step for the simulation.
    #[arg(short, long, default_value_t = 0.0014)]
    delta_time: f64,

    /// The total time for the simulation to run.
    #[arg(short, long, default_value_t = 100.0)]
    total_time: f64,
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

    // create output directory
    let write_directory = match args.output.parent().map(|path| {
        fs::create_dir_all(path)?;
        Ok::<PathBuf, std::io::Error>(path.to_owned())
    }) {
        // if parent path was specified and created successfully, use it
        Some(Ok(path)) => path.to_owned(),
        // if no parent path specified, all is good (大丈夫)
        None => PathBuf::new(),
        // if parent path was specified but not created, exit non-zero
        // we land here if OS error occured (e.g. path was a file)
        Some(Err(e)) => {
            eprintln!("Error creating output directory: {}", e);
            std::process::exit(1);
        }
    };

    // generate simulation
    let mut simulation: Box<dyn Simulation> = input.into();

    // set up output writer
    let output_extension = args.output.extension().unwrap_or(std::ffi::OsStr::new(""));

    let mut output_writer =
        match <dyn OutputWriter>::from_extension(output_extension.to_str().unwrap_or("")) {
            Ok(writer) => writer,
            Err(e) => {
                eprintln!("Error creating output writer: {}", e);
                std::process::exit(1);
            }
        };

    let mut current_time = 0.0;
    let mut frame = 0;
    while current_time < args.total_time {
        println!("Step {frame}");

        simulation.step(args.delta_time);

        if frame % 100 == 0 {
            output_writer
                .write(&args.output, &simulation)
                .expect("error occured during simulation write");
        }

        current_time += args.delta_time;
        frame += 1;
    }
}
