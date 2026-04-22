//! TODO document

use crate::{Particle, simulation::Simulation};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

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
    fn for_each_particles<'a>(&'a self, f: &mut dyn FnMut(&Particle)) {
        for part in &self.particles {
            f(part);
        }
    }

    fn for_each_particles_mut<'a>(&'a mut self, f: &mut dyn FnMut(&mut Particle)) {
        for part in &mut self.particles {
            f(part);
        }
    }

    // index-based approach because two mutable iterators were problematic
    fn for_each_particle_pairs_mut<'a>(&'a mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        let count = self.particle_count();

        for i in 0..count {
            // newtons third law: skip same pairs
            for j in (i + 1)..count {
                // TODO maybe move this line one above, check efficiency of split_at_mut
                // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_at_mut
                let (left, right) = self.particles.split_at_mut(j);

                // conceptually:
                //
                // [p1,  p2,  p3,  p4,  p5]
                //        i         j
                // [p1,  p2,  p3],[p4,  p5]
                //       ^^        ^^ avoid borrow issue with split_at_mut
                
                // TODO document in slides

                f(&mut left[i], &mut right[0]);
            }
        }
    }

    fn particle_count(&self) -> usize {
        self.particles.len()
    }
}
