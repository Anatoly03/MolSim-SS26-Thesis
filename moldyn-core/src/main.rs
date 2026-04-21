//! The main module for the moldyn-core library.
#![crate_name = "moldyn_core"]

mod forces;
mod particle;
mod vec3;

pub use forces::{Force, LennardJonesForce, NewtonForce};
pub use particle::Particle;
pub use vec3::Vec3;

/// The main entry point for the moldyn-core library.
fn main() {
    println!("Hello, world!");
}
