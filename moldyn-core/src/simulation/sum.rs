//! TODO document

use std::slice::IterMut;

use crate::{Particle, simulation::Simulation};

/// The [DirectSum] simulation method is the most intuitive way to process
/// a molecular dynamics simulation. It bases the computation on the
/// [Direct Sum](https://en.wikipedia.org/wiki/Direct_sum) method.
///
/// **Newton Pair Optimization**: The only optimization [DirectSum] performs
/// is avoiding computing the same pair of particles twice.
pub struct DirectSum {
    particles: Vec<Particle>,
}

impl Simulation for DirectSum {
    fn particles<'a>(&'a self) -> std::slice::Iter<'a, Particle> {
        self.particles.iter()
    }

    fn particles_mut<'a>(&'a mut self) -> IterMut<'a, Particle> {
        self.particles.iter_mut()
    }

    fn particle_pairs_mut<'a>(&'a mut self) -> IterMut<'a, (Particle, Particle)> {
        todo!()
    }
}
