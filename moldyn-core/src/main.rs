//! The main module for the moldyn-core library.
#![crate_name = "moldyn_core"]

mod particle;
mod vec3;

pub use vec3::Vec3;
pub use particle::Particle;

/// The main entry point for the moldyn-core library.
fn main() {
    println!("Hello, world!");
}
