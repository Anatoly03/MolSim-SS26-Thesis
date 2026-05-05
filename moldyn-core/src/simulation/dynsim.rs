//! Dynamic simulation module.

pub use super::args::SimulationArgs;
pub use super::cells::LinkedCells;
pub use super::container::ParticleContainer;
pub use super::sum::DirectSum;
pub use crate::Simulation;
use crate::{Force, Particle};
use serde::{Deserialize, Serialize, de::Visitor};
use std::sync::Arc;

/// Object-safe wrapper trait for dynamic simulations.
///
/// Allows `Box<dyn SimulationTrait>` to be used when concrete container type is unknown.
pub trait SimulationTrait {
    /// # Returns
    ///
    /// Name of the simulation system, which is used for serialization and
    /// deserialization. The characters are expected to be in `lowercase`.
    fn system_name(&self) -> &str;

    /// Get the particles in the simulation, returns as a slice.
    ///
    /// # Usage
    ///
    /// ```
    /// use moldyn_core::{DirectSum, Particle, Simulation, Vec3};
    ///
    /// let mut simulation: Box<dyn Simulation> = Box::new(DirectSum::default());
    ///
    /// simulation.add_particles(vec![
    ///     Particle::from_data(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), 1.0),
    /// ]);
    ///
    /// for particle in simulation.particles() {
    ///     println!("Particle at position: {:?}", particle.get_position());
    /// }
    /// ```
    fn particles(&self) -> &[Particle];

    /// Get the particles in the simulation, returns as a mutable slice.
    ///
    /// # Usage
    ///
    /// ```
    /// use moldyn_core::{DirectSum, Particle, Simulation, Vec3};
    ///
    /// let mut simulation: Box<dyn Simulation> = Box::new(DirectSum::default());
    ///
    /// simulation.add_particles(vec![
    ///     Particle::from_data(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), 1.0),
    /// ]);
    ///
    /// for particle in simulation.particles_mut() {
    ///     particle.update_position(0.01);
    /// }
    /// ```
    fn particles_mut(&mut self) -> &mut [Particle];

    /// Invokes a lambda callback for each particle in the simulation.
    ///
    /// # Usage
    ///
    /// ```
    /// use moldyn_core::{DirectSum, Particle, Simulation, Vec3};
    ///
    /// let mut simulation: Box<dyn Simulation> = Box::new(DirectSum::default());
    ///
    /// simulation.add_particles(vec![
    ///     Particle::from_data(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), 1.0),
    /// ]);
    ///     
    /// simulation.for_each_particles(&mut |p| {
    ///     println!("Particle at position: {:?}", p.get_position());
    /// });
    /// ```
    fn for_each_particles(&self, f: &mut dyn FnMut(&Particle));

    /// Invokes a lambda callback for each particle (mutable) in the simulation.
    fn for_each_particles_mut(&mut self, f: &mut dyn FnMut(&mut Particle));

    /// The core method of the trait. Different implementations of [Simulation] vary
    /// in performance as this is the heaviest part of the simulation. Invokes a lambda
    /// callback for pair of particles in the simulation, with the following limitations:
    ///
    /// - An iterator over distinct pairs of particles, accounting for symmetry.
    /// - If you receive a pair `(a, b)` it is guaranteed that you will not receive `(b, a)`.
    /// - There is no guarantee you will receive all pairs.
    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle));

    /// The number of particles in the simulation.
    fn particle_count(&self) -> usize;

    /// Set the particles in the simulation.
    fn add_particles(&mut self, particles: Vec<Particle>);

    /// Get the force calculation method.
    fn get_force(&self) -> Arc<dyn Force>;

    /// Set the force calculation method.
    fn set_force(&mut self, force: Arc<dyn Force>);

    /// Get the simulation arguments.
    fn args(&self) -> SimulationArgs;

    /// Set the simulation arguments.
    fn set_args(&mut self, args: SimulationArgs);

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
        self.delay_force();
        self.update_force();
        // APPLY GRAVITY HERE
        // CALCULATE BORDER BEHAVIOUR
        self.update_velocity(delta_t);
        // TODO UPDATE CURRENT TIME += DELTA TIME
    }

    // TODO PLOT PARTICLES
}

impl<P> SimulationTrait for Simulation<P>
where
    P: ParticleContainer + 'static,
{
    fn system_name(&self) -> &str {
        Simulation::system_name(self)
    }

    fn particles(&self) -> &[Particle] {
        Simulation::particles(self)
    }

    fn particles_mut(&mut self) -> &mut [Particle] {
        Simulation::particles_mut(self)
    }

    fn for_each_particles(&self, f: &mut dyn FnMut(&Particle)) {
        Simulation::for_each_particles(self, f)
    }

    fn for_each_particles_mut(&mut self, f: &mut dyn FnMut(&mut Particle)) {
        Simulation::for_each_particles_mut(self, f)
    }

    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        Simulation::for_each_particle_pairs_mut(self, f)
    }

    fn particle_count(&self) -> usize {
        Simulation::particle_count(self)
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        Simulation::add_particles(self, particles)
    }

    fn get_force(&self) -> Arc<dyn Force> {
        Simulation::get_force(self)
    }

    fn set_force(&mut self, force: Arc<dyn Force>) {
        Simulation::set_force(self, force)
    }

    fn args(&self) -> SimulationArgs {
        Simulation::args(self)
    }

    fn set_args(&mut self, args: SimulationArgs) {
        Simulation::set_args(self, args)
    }

    fn update_position(&mut self, delta_t: f64) {
        Simulation::update_position(self, delta_t)
    }

    fn delay_force(&mut self) {
        Simulation::delay_force(self)
    }

    fn update_force(&mut self) {
        Simulation::update_force(self)
    }

    fn update_velocity(&mut self, delta_t: f64) {
        Simulation::update_velocity(self, delta_t)
    }

    fn step(&mut self, delta_t: f64) {
        Simulation::step(self, delta_t)
    }
}

impl Serialize for Box<dyn SimulationTrait> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.system_name())
    }
}

struct BoxSimVisitor;

impl<'de> Visitor<'de> for BoxSimVisitor {
    type Value = Box<dyn SimulationTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a simulation type name (direct-sum, ds, linked-cells, or lc)")
    }

    /// If the simulation is represented as a string, parse it as a known simulation type.
    /// Strings are case-insensitive.
    ///
    /// # Example
    ///
    /// ```yaml
    /// # Particle definition input file example
    /// name: halleys-comet
    /// algorithm: direct-sum
    /// ```
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.to_ascii_lowercase().as_str() {
            "direct-sum" | "ds" => Ok(Box::new(Simulation::<DirectSum>::default())),
            "linked-cells" | "lc" => Ok(Box::new(Simulation::<LinkedCells<DirectSum>>::default())),
            _ => Err(E::custom(format!("Unknown simulation type: {value}"))),
        }
    }
}

impl<'de> Deserialize<'de> for Box<dyn SimulationTrait> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(BoxSimVisitor)
    }
}

impl Default for Box<dyn SimulationTrait> {
    fn default() -> Self {
        Box::new(Simulation::<DirectSum>::default())
    }
}
