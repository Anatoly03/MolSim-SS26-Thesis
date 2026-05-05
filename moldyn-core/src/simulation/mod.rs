//! TODO document

mod args;
mod cells;
mod container;
mod sum;
mod dynsim;

use crate::{Force, LennardJonesForce, Particle};
pub use args::SimulationArgs;
pub use cells::LinkedCells;
pub use container::ParticleContainer;
use serde::{Serialize};
use std::sync::Arc;
pub use sum::DirectSum;
pub use dynsim::SimulationTrait;

/// An interface-level abstraction of a molecular dynamics simulation. A
/// [Simulation] is a method of organizing the particles and forces in a way
/// that allows for efficient computation.
pub struct Simulation<Container: ParticleContainer> {
    // TODO explain in slides why Arc works and Box does not
    force: Arc<dyn Force>,
    container: Container,
    args: SimulationArgs,
}

impl<Container: ParticleContainer> Simulation<Container> {
    /// # Returns
    ///
    /// Name of the simulation system, which is used for serialization and
    /// deserialization. The characters are expected to be in `lowercase`.
    fn system_name(&self) -> &str {
        self.container.system_name()
    }

    /// Get the particles in the simulation, returns as a slice.
    ///
    /// # Usage
    ///
    /// ```rust,no_test,no_run
    /// for particle in simulation.particles() {
    ///     println!("Particle at position: {:?}", particle.get_position());
    /// }
    /// ```
    fn particles(&self) -> Box<dyn Iterator<Item = &Particle> + '_> {
        self.container.particles()
    }

    /// Get the particles in the simulation, returns as a mutable slice.
    ///
    /// # Usage
    ///
    /// ```rust,no_test,no_run
    /// for particle in simulation.particles_mut() {
    ///     particle.update_position(0.01);
    /// }
    /// ```
    fn particles_mut(&mut self) -> Box<dyn Iterator<Item = &mut Particle> + '_> {
        self.container.particles_mut()
    }

    /// Invokes a lambda callback for each particle in the simulation.
    ///
    /// # Usage
    ///
    /// ```rust,no_test,no_run
    /// simulation.for_each_particles(&mut |p| {
    ///     println!("Particle at position: {:?}", p.get_position());
    /// });
    /// ```
    fn for_each_particles(&self, f: &mut dyn FnMut(&Particle)) {
        self.container.for_each_particles(f);
    }

    /// Invokes a lambda callback for each particle (mutable) in the simulation.
    fn for_each_particles_mut(&mut self, f: &mut dyn FnMut(&mut Particle)) {
        self.container.for_each_particles_mut(f);
    }

    /// The core method of the trait. Different implementations of [Simulation] vary
    /// in performance as this is the heaviest part of the simulation. Invokes a lambda
    /// callback for pair of particles in the simulation, with the following limitations:
    ///
    /// - An iterator over distinct pairs of particles, accounting for symmetry.
    /// - If you receive a pair `(a, b)` it is guaranteed that you will not receive `(b, a)`.
    /// - There is no guarantee you will receive all pairs.
    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        self.container.for_each_particle_pairs_mut(f);
    }

    /// The number of particles in the simulation.
    fn particle_count(&self) -> usize {
        self.container.particle_count()
    }

    /// Set the particles in the simulation.
    fn add_particles(&mut self, particles: Vec<Particle>) {
        self.container.add_particles(particles);
    }

    /// Get the force calculation method.
    fn get_force(&self) -> Arc<dyn Force> {
        self.force.clone()
    }

    /// Set the force calculation method.
    fn set_force(&mut self, force: Arc<dyn Force>) {
        self.force = force;
    }

    /// Get the simulation arguments.
    fn args(&self) -> SimulationArgs {
        self.args.clone()
    }

    /// Set the simulation arguments.
    fn set_args(&mut self, args: SimulationArgs) {
        self.args = args;
    }

    /// Updates the position of all particles.
    fn update_position(&mut self, delta_t: f64) {
        self.for_each_particles_mut(&mut |p| p.update_position(delta_t));
    }

    /// Delays the force.
    fn delay_force(&mut self) {
        self.for_each_particles_mut(&mut |p| p.delay_force());
    }

    /// Updates the velocity of all particles.
    fn update_force(&mut self) {
        // cannot borrow `*self` as mutable because it is also borrowed as immutable
        // mutable borrow occurs hererustcClick for full compiler diagnostic
        // mod.rs(50, 21): immutable borrow occurs here
        // mod.rs(52, 14): immutable borrow later used by call
        let force: Arc<dyn Force> = self.get_force();

        self.for_each_particle_pairs_mut(&mut |p1, p2| {
            force.apply_force(p1, p2);
        });
    }

    /// Updates the velocity of all particles.
    fn update_velocity(&mut self, delta_t: f64) {
        self.for_each_particles_mut(&mut |p| p.update_velocity(delta_t));
    }

    /// TODO document
    fn step(&mut self, delta_t: f64) {
        self.update_position(delta_t);
        self.container.on_after_position_update();
        self.delay_force();
        self.update_force();
        // APPLY GRAVITY HERE
        self.container.on_after_force_update();
        // TODO CALCULATE BORDER BEHAVIOUR in `on_after_force_update`
        self.update_velocity(delta_t);
        self.container.on_after_velocity_update();
        // TODO UPDATE CURRENT TIME += DELTA TIME
    }

    // TODO PLOT PARTICLES
}

impl<P> Serialize for Simulation<P>
where
    P: ParticleContainer + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.system_name())
    }
}

impl<P> Default for Simulation<P>
where
    P: ParticleContainer + Default,
{
    /// The default simulation system for this project is the direct sum.
    fn default() -> Self {
        Self {
            force: Arc::new(LennardJonesForce::default()),
            container: P::default(),
            args: SimulationArgs::default(),
        }
    }
}
