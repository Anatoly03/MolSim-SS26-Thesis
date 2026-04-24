//! TODO document

mod sum;

use crate::{Force, Particle, reader::SimulationArgs};
use serde::{Deserialize, de::Visitor};
use std::sync::Arc;
pub use sum::DirectSum;

// to self: tried to keep dyn-compatibility. following approaches failed:
// - fn ...(... impl Fn) is technically generic
// - type PairIter = ... is also generic/ typed
// - returning `Iter` and `IterMut` works for `particles` (`particles_mut`)
//   but `particle_pairs` had implementation problems returning slide::IntoIter

/// An interface-level abstraction of a molecular dynamics simulation. A
/// [Simulation] is a method of organizing the particles and forces in a way
/// that allows for efficient computation.
pub trait Simulation {
    /// Invokes a lambda callback for each particle in the simulation.
    fn for_each_particles<'a>(&'a self, f: &mut dyn FnMut(&Particle));

    /// Invokes a lambda callback for each particle (mutable) in the simulation.
    fn for_each_particles_mut<'a>(&'a mut self, f: &mut dyn FnMut(&mut Particle));

    /// The core method of the trait. Different implementations of [Simulation] vary
    /// in performance as this is the heaviest part of the simulation. Invokes a lambda
    /// callback for pair of particles in the simulation, with the following limitations:
    ///
    /// - An iterator over distinct pairs of particles, accounting for symmetry.
    /// - If you receive a pair `(a, b)` it is guaranteed that you will not receive `(b, a)`.
    /// - There is no guarantee you will receive all pairs.
    fn for_each_particle_pairs_mut<'a>(
        &'a mut self,
        f: &mut dyn FnMut(&mut Particle, &mut Particle),
    );

    /// The number of particles in the simulation.
    fn particle_count(&self) -> usize;

    /// Set the particles in the simulation.
    fn add_particles(&mut self, particles: Vec<Particle>);

    /// Get the force calculation method.
    fn get_force(&self) -> Arc<dyn Force>;

    /// Set the force calculation method.
    fn set_force(&mut self, force: Arc<dyn Force>);

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

struct SimulationVisitor;

impl<'de> Visitor<'de> for SimulationVisitor {
    type Value = Box<dyn Simulation>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a simulation")
    }

    /// If the simulation is represented as a string, we can parse it as a known simulation
    /// type. Strings are case-insensitive.
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
            "direct-sum" | "ds" => Ok(Box::new(DirectSum::default())),
            // TODO linked-cells
            _ => Err(E::custom(format!("Unknown simulation type: {}", value))),
        }
    }
}

impl<'de> Deserialize<'de> for Box<dyn Simulation> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SimulationVisitor)
    }
}

impl Default for Box<dyn Simulation> {
    /// The default simulation system for this project is the direct sum.
    fn default() -> Self {
        Box::new(DirectSum::default())
    }
}
