//! TODO document

use crate::Particle;

// to self: tried to keep dyn-compatibility. following approaches failed:
// - fn ...(... impl Fn) is technically generic
// - type PairIter = ... is also generic/ typed
// - returning `Iter` and `IterMut` works for `particles` (`particles_mut`)
//   but `particle_pairs` had implementation problems returning slide::IntoIter

/// An interface-level abstraction of a molecular dynamics simulation. A
/// [Simulation] is a method of organizing the particles and forces in a way
/// that allows for efficient computation.
pub trait ParticleContainer {
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
    fn particles(&self) -> Box<dyn Iterator<Item = &Particle> + '_>;

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
    fn particles_mut(&mut self) -> Box<dyn Iterator<Item = &mut Particle> + '_>;

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
    fn for_each_particles(&self, f: &mut dyn FnMut(&Particle)) {
        for part in self.particles() {
            f(part);
        }
    }

    /// Invokes a lambda callback for each particle (mutable) in the simulation.
    fn for_each_particles_mut(&mut self, f: &mut dyn FnMut(&mut Particle)) {
        for part in self.particles_mut() {
            f(part);
        }
    }

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
}
