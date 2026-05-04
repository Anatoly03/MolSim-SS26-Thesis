//! TODO document

use crate::{DirectSum, ParticleContainer, SimulationArgs};
use crate::{Force, LennardJonesForce};
use crate::{Particle, simulation::Simulation};
use std::collections::HashMap;
use std::sync::Arc;

/// The [LinkedCells] simulation method is the a simple optimization of particle
/// management over the quadratic [DirectSum][crate::DirectSum] method.
///
/// The generic type is the cell type, which is treated as an internal simulation.
/// For example, if the subtype is [DirectSum][crate::DirectSum], then the particles
/// in the cell will be calculated using the direct sum method.
pub struct LinkedCells<Cell: ParticleContainer> {
    /// The particles are stored in a hash map, where the key is the cell coordinates
    /// and the value representing the cell. The value is for LinkedCells a direct sum
    /// sub-simulation cell.
    cells: HashMap<Vec<i32>, Cell>,
}

impl<Cell: ParticleContainer> ParticleContainer for LinkedCells<Cell> {
    fn system_name(&self) -> &str {
        "linked-cells"
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
        self.cells
            .values()
            .flat_map(|cell| cell.particles())
            .count()
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        todo!()
    }
}

impl<Cell: ParticleContainer> Default for LinkedCells<Cell> {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod equivalence_tests {
    use crate::{DirectSum, LennardJonesForce, LinkedCells, Particle, Simulation};
    use std::sync::Arc;

    #[test]
    #[ignore = "not implemented"]
    fn with_direct_sum() {
        let mut particles = vec![];
        let force = Arc::new(LennardJonesForce::default());

        // cut off radius is 3.0 (default), so a chunk with 5.0 (default) in linked cells
        // should be equivalent to direct sum

        for x in 0..100 {
            for y in 0..100 {
                particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
            }
        }

        let mut cells_simulation = Simulation::<LinkedCells<DirectSum>>::default();
        let mut sum_simulation = Simulation::<DirectSum>::default();

        cells_simulation.set_force(force.clone());
        sum_simulation.set_force(force);

        cells_simulation.add_particles(particles.clone());
        sum_simulation.add_particles(particles);

        for i in 0..10 {
            cells_simulation.step((i as f64) * 0.01);
            sum_simulation.step((i as f64) * 0.01);

            assert_eq!(cells_simulation.particles(), sum_simulation.particles());
        }
    }
}

#[cfg(all(test, nightly))]
mod benchmark {
    use crate::{
        CustomForce, LennardJonesForce, LinkedCells, NewtonForce, Particle, Simulation,
        SimulationArgs, Vec3,
    };
    use meval::Expr;
    use std::sync::Arc;
    use test::Bencher;

    #[bench]
    fn ten_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            particles.push(Particle::at(x as f64, 0.0, 0.0).with_mass(1.0));
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn ten_bodies_lennard_jones(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            particles.push(Particle::at(x as f64, 0.0, 0.0).with_mass(1.0));
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn hundred_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn hundred_bodies_lennard_jones(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn thousand_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
                }
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn thousand_bodies_lennard_jones(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
                }
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }
}
