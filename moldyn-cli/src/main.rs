//! This crate defines the binary runnable for the Molecular Dynamics Rust project.
//! The codebase handles the argument parsing with [clap] and propagates the program
//! execution to the [moldyn_core] main library crate.
//!
//! The main function reads the input file, creates the output directories and starts
//! the simulation loop.
//!
//! Below are two examples how to build the project and print the help message.
//!
//! ```sh
//! cargo build --release
//! ./target/release/moldyn-cli --help
//! ```
//!
//! ```sh
//! cargo run --release -- --help
//! ```
#![crate_name = "moldyn_cli"]

use clap::Parser;
use moldyn_core::*;
use moldyn_io::*;
use std::fs;
use std::path::PathBuf;

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
    #[arg(short, long, default_value = "output/out.vtk")]
    output: PathBuf,

    /// The time step for the simulation.
    #[arg(short, long, default_value_t = 0.0014)]
    delta_time: f64,

    /// The total time for the simulation to run.
    #[arg(short, long, default_value_t = 1000.0)]
    total_time: f64,

    /// The period (in frames) for writing the simulation output. This defines the
    /// frequency of output writes.
    #[arg(short = 's', long, default_value_t = 250)]
    frame_period: usize,
}

/// The main entry point for the [moldyn_core] library. This function handles the
/// argument parsing, file reading, invoking of the simulation loop and writing
/// the output.
pub fn main() {
    let args = Args::parse();

    // read the input file and parse it into [FileDefinition]
    let input = match FileDefinition::try_from(args.input) {
        Ok(def) => def,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(name) = &input.name {
        println!("simulation name: `{name}`");
    }

    // create output directory
    match args.output.parent().map(fs::create_dir_all) {
        Some(Ok(())) | None => (),
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

    let mut current_time = simulation.args().time_start.unwrap_or(0.0);
    let delta_time = simulation.args().time_step.unwrap_or(args.delta_time);
    let end_time = simulation.args().time_end.unwrap_or(args.total_time);

    let mut frame = 0;
    while current_time < end_time {
        let print_frame = frame % args.frame_period == 0;

        println!(
            "Step {frame: >8} [{current_time:.4} / {end_time:.4}] {}",
            if print_frame { "WRITE" } else { "" }
        );

        simulation.step(delta_time);

        if print_frame {
            output_writer
                .write(&args.output, &*simulation)
                .expect("error occured during simulation write");
        }

        current_time += delta_time;
        frame += 1;
    }
}
