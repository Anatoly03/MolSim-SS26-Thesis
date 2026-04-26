//! TODO document

use crate::SimulationArgs;
use crate::{Force, LennardJonesForce};
use crate::{Particle, simulation::Simulation};
use std::sync::Arc;

/// The [DirectSum] simulation method is the most intuitive way to process
/// a molecular dynamics simulation. It bases the computation on the
/// [Direct Sum](https://en.wikipedia.org/wiki/Direct_sum) method.
///
/// **Newton Pair Optimization**: The only optimization [DirectSum] performs
/// is avoiding computing the same pair of particles twice.
pub struct DirectSum {
    // TODO explain in slides why Arc works and Box does not
    force: Arc<dyn Force>,
    particles: Vec<Particle>,
    args: SimulationArgs,
}

impl Simulation for DirectSum {
    fn system_name(&self) -> &str {
        "direct-sum"
    }

    fn particles(&self) -> &[Particle] {
        &self.particles
    }

    fn particles_mut(&mut self) -> &mut [Particle] {
        &mut self.particles
    }

    // index-based approach because two mutable iterators were problematic
    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
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

                f(&mut left[i], &mut right[0]);
            }
        }
    }

    fn particle_count(&self) -> usize {
        self.particles.len()
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        self.particles.extend(particles);
    }

    fn get_force(&self) -> Arc<dyn Force> {
        self.force.clone()
    }

    fn set_force(&mut self, force: Arc<dyn Force>) {
        self.force = force;
    }

    fn args(&self) -> SimulationArgs {
        self.args.clone()
    }

    fn set_args(&mut self, args: SimulationArgs) {
        self.args = args;
    }
}

impl Default for DirectSum {
    fn default() -> Self {
        Self {
            force: Arc::new(LennardJonesForce::default()),
            particles: Vec::new(),
            args: SimulationArgs::default(),
        }
    }
}
