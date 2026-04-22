//! TODO document

mod sum;

use crate::Particle;
use std::{slice::{Iter, IterMut}, vec::IntoIter};
pub use sum::DirectSum;

/// An interface-level abstraction of a molecular dynamics simulation. A
/// `simulation` is a method of organizing the particles and forces in a way
/// that allows for efficient computation.
pub trait Simulation {
    /// An iterator over the particles in the simulation, used in methods like
    /// [Simulation::particle_count]
    fn particles<'a>(&'a self) -> Iter<'a, Particle>;

    /// Mutable iterator over the particles in the simulation, used in methods
    /// like [Simulation::step].
    fn particles_mut<'a>(&'a mut self) -> IterMut<'a, Particle>;

    /// The core method of the trait. Different implementations of [Simulation] vary
    /// in performance as this is the heaviest part of the simulation.
    /// 
    /// # Returns
    /// 
    /// - An iterator over distinct pairs of particles, accounting for symmetry.
    /// - If you receive a pair `(a, b)` it is guaranteed that you will not receive `(b, a)`.
    /// - There is no guarantee you will receive all pairs.
    fn particle_pairs_mut<'a>(&'a mut self) -> IterMut<'a, (Particle, Particle)>;

    /// # Returns
    /// 
    /// The number of particles in the simulation.
    fn particle_count(&self) -> usize {
        self.particles().count()
    }

    /// TODO document
    fn step(&mut self) {}
}
