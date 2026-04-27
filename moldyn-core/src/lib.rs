//! The main crate of the molecular dynamics simulation. This library exposes the
//! main components of the simulation.
//!
//! # Features
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

pub use forces::{CustomForce, Force, LennardJonesForce, NewtonForce};
pub use particle::Particle;
pub use simulation::{DirectSum, Simulation, SimulationArgs};
pub use vec3::Vec3;
