//! The main crateof the molecular dynamics simulation. This library exposes the
//! main components of the simulation.
//! 
//! # Features
//! 
//! - `vtk`: Integrates VTK-support for reading and writing `.vtu` files. This
//!   feature is enabled by default.
//! - `yaml`: Integrates YAML-support for reading and writing `.yaml` files. This
//!   feature is enabled by default.
#![crate_name = "moldyn_core"]

mod forces;
mod particle;
mod reader;
mod simulation;
mod vec3;
mod writer;

pub use forces::{Force, LennardJonesForce, NewtonForce};
pub use particle::Particle;
pub use reader::{FileDefinition, SimulationArgs};
pub use simulation::Simulation;
pub use vec3::Vec3;
pub use writer::OutputWriter;
