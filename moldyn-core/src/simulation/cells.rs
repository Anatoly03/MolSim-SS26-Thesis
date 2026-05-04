//! TODO document

use crate::{Force, LennardJonesForce};
use crate::{Particle, simulation::Simulation};
use std::sync::Arc;

/// The [LinkedCells] simulation method is the a simple optimization of particle
/// management over the quadratic [DirectSum][crate::DirectSum] method.
pub struct LinkedCells {
    // TODO explain in slides why Arc works and Box does not
    force: Arc<dyn Force>,
    particles: Vec<Particle>,
}

impl Simulation for LinkedCells {
    fn system_name(&self) -> &str {
        todo!()
    }

    fn particles(&self) -> &[Particle] {
        todo!()
    }

    fn particles_mut(&mut self) -> &mut [Particle] {
        todo!()
    }

    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        todo!()
    }

    fn particle_count(&self) -> usize {
        todo!()
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        todo!()
    }

    fn get_force(&self) -> Arc<dyn Force> {
        todo!()
    }

    fn set_force(&mut self, force: Arc<dyn Force>) {
        todo!()
    }

    fn args(&self) -> super::SimulationArgs {
        todo!()
    }

    fn set_args(&mut self, args: super::SimulationArgs) {
        todo!()
    }
}

impl Default for LinkedCells {
    fn default() -> Self {
        Self {
            force: Arc::new(LennardJonesForce::default()),
            particles: Vec::new(),
        }
    }
}
