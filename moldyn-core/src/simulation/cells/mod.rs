//! TODO document

use crate::{DirectSum, ParticleContainer, SimulationArgs, Vec3};
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
    cells: HashMap<Vec3<i32>, Cell>,

    /// The size of the space division in this simulation. The cell size should be larger
    /// than the half the cut off radius.
    cell_size: Vec3<f64>,
}

impl<Cell> ParticleContainer for LinkedCells<Cell>
where
    Cell: ParticleContainer + Default,
{
    fn system_name(&self) -> &str {
        "linked-cells"
    }

    fn particles(&self) -> Box<dyn Iterator<Item = &Particle> + '_> {
        Box::new(self.cells.values().flat_map(|cell| cell.particles()))
    }

    fn particles_mut(&mut self) -> Box<dyn Iterator<Item = &mut Particle> + '_> {
        Box::new(
            self.cells
                .values_mut()
                .flat_map(|cell| cell.particles_mut()),
        )
    }

    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        let coords: Vec<Vec3<i32>> = self.cells.keys().cloned().collect();

        // Visit each neighboring cell pair only once: only the positive half-space offsets.
        for cell_coords in &coords {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        // Skip same-cell interactions here and avoid mirrored duplicates.
                        if dx < 0 || (dx == 0 && dy < 0) || (dx == 0 && dy == 0 && dz <= 0) {
                            continue;
                        }

                        let neighbour_coords =
                            Vec3::new(cell_coords.x + dx, cell_coords.y + dy, cell_coords.z + dz);

                        if !self.cells.contains_key(&neighbour_coords) {
                            continue;
                        }

                        let [cell, neighbour_cell] = self
                            .cells
                            .get_disjoint_mut([cell_coords, &neighbour_coords]);

                        if let (Some(cell), Some(neighbour_cell)) = (cell, neighbour_cell) {
                            for p1 in cell.particles_mut() {
                                for p2 in neighbour_cell.particles_mut() {
                                    f(p1, p2);
                                }
                            }
                        }
                    }
                }
            }
        }

        // For each cell, we invoke the subbsimulation particle pairs.
        for cell in self.cells.values_mut() {
            cell.for_each_particle_pairs_mut(f);
        }
    }

    fn particle_count(&self) -> usize {
        self.cells
            .values()
            .flat_map(|cell| cell.particles())
            .count()
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        for p in particles {
            let cx = (p.get_position().x / self.cell_size.x).floor() as i32;
            let cy = (p.get_position().y / self.cell_size.y).floor() as i32;
            let cz = (p.get_position().z / self.cell_size.z).floor() as i32;

            let cell_coords = Vec3::new(cx, cy, cz);
            let cell_ref = self.cells.entry(cell_coords).or_insert_with(Cell::default);

            cell_ref.add_particles(vec![p]);
        }
    }
}

impl<Cell: ParticleContainer> Default for LinkedCells<Cell> {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            cell_size: Vec3::new(5.0, 5.0, 5.0),
        }
    }
}

#[cfg(test)]
mod equivalence_tests {
    use crate::{DirectSum, LennardJonesForce, LinkedCells, Particle, Simulation};
    use std::sync::Arc;

    #[test]
    #[ignore = "test does not account for different particle ordering"]
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

            assert_eq!(
                cells_simulation.particles().collect::<Vec<_>>(),
                sum_simulation.particles().collect::<Vec<_>>()
            );
        }
    }
}

#[cfg(all(test, nightly))]
mod benchmark {
    use crate::{
        CustomForce, DirectSum, LennardJonesForce, LinkedCells, NewtonForce, Particle, Simulation,
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

        let mut simulation = Simulation::<LinkedCells<DirectSum>>::default();
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

        let mut simulation = Simulation::<LinkedCells<DirectSum>>::default();
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

        let mut simulation = Simulation::<LinkedCells<DirectSum>>::default();
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

        let mut simulation = Simulation::<LinkedCells<DirectSum>>::default();
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

        let mut simulation = Simulation::<LinkedCells<DirectSum>>::default();
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

        let mut simulation = Simulation::<LinkedCells<DirectSum>>::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }
}
