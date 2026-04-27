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

// https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html
// build.rs is specifically to allow this type of benchmarks
#[cfg(all(test, nightly))]
mod benchmark {
    use crate::{
        CustomForce, DirectSum, LennardJonesForce, NewtonForce, Particle, Simulation,
        SimulationArgs, Vec3,
    };
    use meval::Expr;
    use std::sync::Arc;
    use test::Bencher;

    #[bench]
    fn halleys_comet(b: &mut Bencher) {
        let particles = vec![
            // sun
            Particle::from_data(Vec3::zero(), Vec3::zero(), 1.0),
            // earth
            Particle::from_data(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 3.0e-6),
            // jupiter
            Particle::from_data(
                Vec3::new(0.0, 5.36, 0.0),
                Vec3::new(-0.425, 0.0, 0.0),
                9.55e-4,
            ),
            // halleys comet
            Particle::from_data(
                Vec3::new(34.75, 0.0, 0.0),
                Vec3::new(0.0, 0.0296, 0.0),
                1.0e-14,
            ),
        ];

        let mut simulation = DirectSum::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn halleys_comet_lennard_jones(b: &mut Bencher) {
        let particles = vec![
            // sun
            Particle::from_data(Vec3::zero(), Vec3::zero(), 1.0),
            // earth
            Particle::from_data(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 3.0e-6),
            // jupiter
            Particle::from_data(
                Vec3::new(0.0, 5.36, 0.0),
                Vec3::new(-0.425, 0.0, 0.0),
                9.55e-4,
            ),
            // halleys comet
            Particle::from_data(
                Vec3::new(34.75, 0.0, 0.0),
                Vec3::new(0.0, 0.0296, 0.0),
                1.0e-14,
            ),
        ];

        let mut simulation = DirectSum::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn halleys_comet_custom_newton_force(b: &mut Bencher) {
        let particles = vec![
            // sun
            Particle::from_data(Vec3::zero(), Vec3::zero(), 1.0),
            // earth
            Particle::from_data(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 3.0e-6),
            // jupiter
            Particle::from_data(
                Vec3::new(0.0, 5.36, 0.0),
                Vec3::new(-0.425, 0.0, 0.0),
                9.55e-4,
            ),
            // halleys comet
            Particle::from_data(
                Vec3::new(34.75, 0.0, 0.0),
                Vec3::new(0.0, 0.0296, 0.0),
                1.0e-14,
            ),
        ];

        let mut simulation = DirectSum::default();
        let force_expr = CustomForce::from_expr("M / r");
        let force = simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn halleys_comet_custom_lennard_jones(b: &mut Bencher) {
        let particles = vec![
            // sun
            Particle::from_data(Vec3::zero(), Vec3::zero(), 1.0),
            // earth
            Particle::from_data(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 3.0e-6),
            // jupiter
            Particle::from_data(
                Vec3::new(0.0, 5.36, 0.0),
                Vec3::new(-0.425, 0.0, 0.0),
                9.55e-4,
            ),
            // halleys comet
            Particle::from_data(
                Vec3::new(34.75, 0.0, 0.0),
                Vec3::new(0.0, 0.0296, 0.0),
                1.0e-14,
            ),
        ];

        // let frac = self.sigma / distance;
        //     let frac6 = frac.powi(6);
        //     let frac12 = frac6.powi(2);

        //     4.0 * self.epsilon * (frac12 - frac6)

        let mut simulation = DirectSum::default();
        let force_expr = CustomForce::from_expr("20.0 * ((1 / r)^12 - (1 / r)^6)");
        let force = simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn halleys_comet_custom_heavy(b: &mut Bencher) {
        let particles = vec![
            // sun
            Particle::from_data(Vec3::zero(), Vec3::zero(), 1.0),
            // earth
            Particle::from_data(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), 3.0e-6),
            // jupiter
            Particle::from_data(
                Vec3::new(0.0, 5.36, 0.0),
                Vec3::new(-0.425, 0.0, 0.0),
                9.55e-4,
            ),
            // halleys comet
            Particle::from_data(
                Vec3::new(34.75, 0.0, 0.0),
                Vec3::new(0.0, 0.0296, 0.0),
                1.0e-14,
            ),
        ];

        let mut simulation = DirectSum::default();
        let force_expr = CustomForce::from_expr("(1 / r)^12 - (1 / r)^11 + (1 / r)^10 - (1 / r)^9 + (1 / r)^6 - (1 / r)^5");
        let force = simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn ten_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            particles.push(Particle::at(x as f64, 0.0, 0.0).with_mass(1.0));
        }

        let mut simulation = DirectSum::default();
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

        let mut simulation = DirectSum::default();
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

        let mut simulation = DirectSum::default();
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

        let mut simulation = DirectSum::default();
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

        let mut simulation = DirectSum::default();
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

        let mut simulation = DirectSum::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }
}
