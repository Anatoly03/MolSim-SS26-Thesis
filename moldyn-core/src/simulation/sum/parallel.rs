//! TODO document

use crate::{Particle, ParticleContainer};
use rayon::prelude::*;

/// The [DirectSumParallel] simulation method is the most intuitive way to process
/// a molecular dynamics simulation. It bases the computation on the
/// [Direct Sum](https://en.wikipedia.org/wiki/Direct_sum) method.
///
/// **Newton Pair Optimization**: The only optimization [DirectSumParallel] performs
/// is avoiding computing the same pair of particles twice.
#[derive(Default)]
pub struct DirectSumParallel {
    particles: Vec<Particle>,
}

impl ParticleContainer for DirectSumParallel {
    fn system_name(&self) -> &str {
        "direct-sum"
    }

    fn particles(&self) -> Box<dyn Iterator<Item = &Particle> + '_> {
        Box::new(self.particles.iter())
    }

    fn particles_mut(&mut self) -> Box<dyn Iterator<Item = &mut Particle> + '_> {
        Box::new(self.particles.iter_mut())
    }

    // index-based approach because two mutable iterators were problematic
    fn for_each_particle_pairs_mut(&mut self, _f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        todo!("rust parallel direct sum not implemented");
    }

    fn particle_count(&self) -> usize {
        self.particles.len()
    }

    fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        self.particles.extend(particles);
    }
}
