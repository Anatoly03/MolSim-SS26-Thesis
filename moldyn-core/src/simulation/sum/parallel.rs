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

// https://stackoverflow.com/questions/50258359/can-a-struct-containing-a-raw-pointer-implement-send-and-be-ffi-safe
pub struct PointerWrapper<T>(*mut T);
unsafe impl<T> Sync for PointerWrapper<T> {}
unsafe impl<T> Send for PointerWrapper<T> {}

impl<T> PointerWrapper<T> {
    unsafe fn at(&self, rhs: usize) -> *mut T {
        unsafe { self.0.add(rhs) }
    }
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

    fn for_each_particles(&self, f: &(dyn Fn(&Particle) + Send + Sync)) {
        self.particles.iter().for_each(|p| f(p));
        // self.particles.par_iter().for_each(|m| {
        //     let guard = m.lock().unwrap();
        //     f(&*guard);
        // });
    }

    fn for_each_particles_mut(&mut self, f: &(dyn Fn(&mut Particle) + Send + Sync)) {
        // keep synchronized
        self.particles.iter_mut().for_each(|p| f(p));
        // self.particles.par_iter_mut().for_each(|m| {
        //     let mut guard = m.lock().unwrap();
        //     f(&mut *guard);
        // });
    }

    fn for_each_particle_pairs_mut(
        &mut self,
        f: &(dyn Fn(&mut Particle, &Particle) + Send + Sync),
    ) {
        let count = self.particle_count();
        let particles = PointerWrapper(self.particles.as_mut_ptr());

        // in c++ we parallelized outer loop too
        (0..count).into_par_iter().for_each(|i| unsafe {
            let mut left = &mut *particles.at(i);

            for j in 0..count {
                if i == j {
                    continue;
                }

                let right = &*particles.at(j);
                f(&mut left, &right);
            }
        });
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
