//! TODO document

use crate::{Particle, ParticleContainer};
use rayon::prelude::*;
use std::sync::Mutex;

/// The [DirectSumParallel] simulation method is the most intuitive way to process
/// a molecular dynamics simulation. It bases the computation on the
/// [Direct Sum](https://en.wikipedia.org/wiki/Direct_sum) method.
///
/// **Newton Pair Optimization**: The only optimization [DirectSumParallel] performs
/// is avoiding computing the same pair of particles twice.
#[derive(Default)]
pub struct DirectSumParallel {
    particles: Vec<Mutex<Particle>>,
}

impl ParticleContainer for DirectSumParallel {
    fn system_name(&self) -> &str {
        "direct-sum"
    }

    fn particles(&self) -> Box<dyn Iterator<Item = &Particle> + '_> {
        todo!()
    }

    fn particles_mut(&mut self) -> Box<dyn Iterator<Item = &mut Particle> + '_> {
        todo!()
    }

    fn for_each_particles(&self, f: &(dyn Fn(&Particle) + Send + Sync)) {
        self.particles.iter().for_each(|p| f(&p.lock().unwrap()));
        // self.particles.par_iter().for_each(|m| {
        //     let guard = m.lock().unwrap();
        //     f(&*guard);
        // });
    }

    fn for_each_particles_mut(&mut self, f: &(dyn Fn(&mut Particle) + Send + Sync)) {
        // keep synchronized
        self.particles.iter_mut().for_each(|p| f(&mut p.lock().unwrap()));
        // self.particles.par_iter_mut().for_each(|m| {
        //     let mut guard = m.lock().unwrap();
        //     f(&mut *guard);
        // });
    }

    fn for_each_particle_pairs_mut(&mut self, f: &(dyn Fn(&mut Particle, &mut Particle) + Send + Sync)) {
        let count = self.particle_count();

        // in c++ we parallelized outer loop too
        (0..count).into_par_iter().for_each(|i| {
            for j in (i + 1)..count {
                let left = &mut self.particles[i].lock().unwrap();
                let right = &mut self.particles[j].lock().unwrap();
                f(&mut *left, &mut *right);
            }
        });
    }

    fn particle_count(&self) -> usize {
        self.particles.len()
    }

    fn add_particle(&mut self, particle: Particle) {
        self.particles.push(Mutex::new(particle));
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        self.particles.extend(particles.into_iter().map(Mutex::new));
    }
}
