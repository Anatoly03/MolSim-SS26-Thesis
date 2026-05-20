//! The main crate of the molecular dynamics simulation. This library exposes the
//! main components of the simulation.
//!
//! # Usage
//!
//! Here is a simple example of how to use the library to run a simulation. This will
//! create a direct sum simulation instance with a single particle and run it for 1000
//! time steps, printing the position of the particle at each step.
//!
//! ```
//! use moldyn_core::{DirectSum, Particle, Simulation, SimulationTrait, Vec3};
//!
//! let mut simulation = Simulation::<DirectSum>::default();
//!
//! simulation.add_particles(vec![
//!     Particle::from_data(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), 1.0),
//! ]);
//!
//! let mut current_time = 0.0;
//! let delta_time = 0.01;
//! let end_time = 10.0;
//!
//! while current_time < end_time {
//!     simulation.step(delta_time);
//!     current_time += delta_time;
//!
//!     for particle in simulation.particles() {
//!         println!("Particle at position: {:?}", particle.get_position());
//!     }
//! }
//! ```
//!
//! # Features
// //!
// //! ### `serde` (default)
// //!
// //! Integrates [serde](https://serde.rs/) support for serializing and deserializing
// //! simulation data. This feature is enabled by default but can be disabled to
// //! reduce the binary size of the resulting program.
//!
//! ### `nightly`
//!
//! Enables features that require a nightly Rust compiler. This feature is
//! disabled by default, and enabling it may cause the program to be unstable.
#![crate_name = "moldyn_core"]
#![cfg_attr(all(test, nightly), feature(test))]

#[cfg(all(test, nightly))]
extern crate test;

mod forces;
mod particle;
mod simulation;
mod vec3;

pub use forces::*;
pub use particle::Particle;
pub use simulation::*;
pub use vec3::Vec3;
