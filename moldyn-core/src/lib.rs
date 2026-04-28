//! The main crate of the molecular dynamics simulation. This library exposes the
//! main components of the simulation.
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
