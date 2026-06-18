//! This crate defines the binary runnable for the Molecular Dynamics Rust project.
//! The codebase handles the argument parsing with [clap] and propagates the program
//! execution to the [moldyn_core] main library crate.
//!
//! The main function reads the input file, creates the output directories and starts
//! the simulation loop.
//!
//! # Help
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
//!
//! ### Output
//!
//! The help message will look something like this.
//!
//! ```text
//! Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics
//!
//! Usage: moldyn-cli [OPTIONS] <INPUT>
//!
//! Arguments:
//!   <INPUT>  The input file for the simulation. The parser will be selected from the file extension. Supported formats are: YAML
//!
//! Options:
//!   -o, --output <OUTPUT>              The output file for the simulation results. If a deep path is provided, the directories along the path will be created if they do not exist. The output format will be selected from the file extension. Supported formats are: YAML [default: output/out.vtk]
//!   -d, --delta-time <DELTA_TIME>      The time step for the simulation [default: 0.0014]
//!   -t, --total-time <TOTAL_TIME>      The total time for the simulation to run [default: 1000]
//!   -s, --frame-period <FRAME_PERIOD>  The period (in frames) for writing the simulation output. This defines the frequency of output writes [default: 250]
//!   -h, --help                         Print help
//!   -V, --version                      Print version
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

    /// The time step for the simulation. If provided, this will override the
    /// value from the input file.
    #[arg(short, long)]
    delta_time: Option<f64>,

    /// The total time for the simulation to run. If provided, this will override
    /// the value from the input file.
    #[arg(short, long)]
    total_time: Option<f64>,

    /// The period (in frames) for writing the simulation output. This defines the
    /// frequency of output writes.  If set to zero, disables output writing (used
    /// for benchmarking).
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
            eprintln!("Error reading input file: {e}");
            std::process::exit(1);
        }
    };

    if let Some(name) = &input.name {
        println!("simulation name: `{name}`");
        println!("algorithm: `{}`", input.algorithm.system_name());
        println!("force: `{}`", input.force.system_name());
    }

    // create output directory
    if args.frame_period > 0 {
        match args.output.parent().map(fs::create_dir_all) {
            Some(Ok(())) | None => (),
            Some(Err(e)) => {
                eprintln!("Error creating output directory: {e}");
                std::process::exit(1);
            }
        };
    }

    // generate simulation
    let mut simulation: Box<dyn SimulationTrait> = input.into();

    // set up output writer
    let output_extension = args.output.extension().unwrap_or(std::ffi::OsStr::new(""));

    let mut output_writer =
        match <dyn OutputWriter>::from_extension(output_extension.to_str().unwrap_or("")) {
            Ok(writer) => writer,
            Err(e) => {
                eprintln!("Error creating output writer: {e}");
                std::process::exit(1);
            }
        };

    #[cfg(debug_assertions)]
    let start_time = std::time::Instant::now();

    let mut current_time = simulation.args().time_start.unwrap_or(0.0);
    // Prefer CLI values when present, otherwise fall back to the input file,
    // and finally to the hard-coded defaults.
    let delta_time = args
        .delta_time
        .or(simulation.args().time_step)
        .unwrap_or(0.0014);
    let end_time = args
        .total_time
        .or(simulation.args().time_end)
        .unwrap_or(1000.0);

    let mut frame = 0;
    while current_time < end_time {
        let print_frame = args.frame_period > 0 && frame % args.frame_period == 0;

        // println!(
        //     "Step {frame: >8} [{current_time:.4} / {end_time:.4}] {}",
        //     if print_frame { "WRITE" } else { "" }
        // );

        simulation.step(delta_time);

        if print_frame {
            output_writer
                .write(&args.output, &*simulation)
                .expect("error occured during simulation write");
        }

        current_time += delta_time;
        frame += 1;
    }

    #[cfg(debug_assertions)]
    {
        let end_time = std::time::Instant::now();
        eprintln!("Simulation time: {:?}", end_time - start_time);
    }
}
