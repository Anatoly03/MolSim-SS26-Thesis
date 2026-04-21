//! TODO document

mod sum;

use crate::Particle;
use std::vec::IntoIter;
pub use sum::DirectSum;

/// An interface-level abstraction of a molecular dynamics simulation. A
/// `simulation` is a method of organizing the particles and forces in a way
/// that allows for efficient computation.
pub trait Simulation {
    /// TODO document
    fn particles(&mut self) -> IntoIter<&mut Particle>;

    /// TODO document
    fn particle_pairs(&mut self) -> IntoIter<(&mut Particle, &mut Particle)>;

    /// The number of particles in the simulation. This is a convenience method that
    /// can be used to ensure the particle count remains consistent (or wantedly
    /// decreased) during the simulation.
    fn particle_count(&self) -> usize {
        self.particles().len()
    }

    // /// TODO document
    // /// TODO do we need this method? it breaks dyn compatibility (generics)
    // /// TODO retink
    // fn for_each_particle(&mut self, mut f: impl FnMut(&mut Particle)) {
    //     for particle in self.particles() {
    //         f(particle);
    //     }
    // }

    /// TODO document
    fn step(&mut self) {}
}
