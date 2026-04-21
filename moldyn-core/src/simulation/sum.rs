//! TODO document

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
    fn particles(&mut self) -> std::vec::IntoIter<&mut Particle> {
        self.particles.into_iter()
    }

    fn particle_pairs(&mut self) -> std::vec::IntoIter<(&mut Particle, &mut Particle)> {
        todo!()
    }
}