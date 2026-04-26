//! This module contains the [Particle] struct.

use crate::Vec3;
use serde::{Deserialize, Serialize};

/// A struct representing a particle record in the simulation.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Particle {
    /// Position of the particle in 3D space.
    #[serde(default)]
    position: Vec3,

    /// Velocity of the particle in 3D space.
    #[serde(default)]
    velocity: Vec3,

    /// Force effective on the particle in 3D space.
    #[serde(default, skip_deserializing)]
    force: Vec3,

    /// Force which was effective on the particle in the previous time step.
    #[serde(default, skip_deserializing)]
    old_force: Vec3,

    /// Mass of the particle.
    #[serde(default)]
    mass: f64,
}

impl Particle {
    /// [Particle] constructor from position, initial velocity and mass.
    pub fn from_data(position: Vec3, velocity: Vec3, mass: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
            ..Default::default()
        }
    }

    /// Returns the current force of the particle.
    pub fn get_force(&self) -> Vec3 {
        self.force
    }

    /// Propagates the current force to the old force. This has to be called
    /// every time step before invoking [Particle::apply_force] to apply new
    /// forces.
    pub fn delay_force(&mut self) {
        self.old_force = self.force;
        self.force = Vec3::zero();
    }

    /// Applies the given force to the particle (addition). It assumes that the
    /// force was reset with [Particle::delay_force] in a timestep.
    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    /// Returns the current position of the particle.
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    /// Calculate the updated position of the particle given a delta time step.
    /// This functionality is constant across different simulation algorithms,
    /// so it is implemented here.
    pub fn update_position(&mut self, delta_time: f64) {
        self.position +=
            self.velocity * delta_time + self.force * (delta_time.powi(2) / (2.0 * self.mass));
    }

    /// Returns the current velocity of the particle.
    pub fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    /// Calculate the updated velocity of the particle given a delta time step.
    /// This functionality is constant across different simulation algorithms,
    /// so it is implemented here.
    pub fn update_velocity(&mut self, delta_time: f64) {
        self.velocity += (self.force + self.old_force) * (delta_time / (2.0 * self.mass));
    }

    /// Returns the constant mass of the particle.
    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    /// Calculate the vector difference between two particles' positions. Note
    /// that the order of the particles affects the sign.
    /// 
    /// - `direction(a, b) == -direction(b, a)`.
    pub fn position_difference(particle1: &Particle, particle2: &Particle) -> Vec3 {
        particle1.position - particle2.position
    }

    /// Calculate the normalized vector difference between two particles' positions.
    /// Note that the order of the particles affects the sign.
    /// 
    /// - If result is `Some`: `direction(a, b) == -direction(b, a)`.
    /// - If result is `None`: `direction(a, b) == direction(b, a) == None`.
    pub fn direction(particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
        Particle::position_difference(particle1, particle2).normal()
    }

    /// Calculate the distance between two particles' positions. This function is
    /// symmetric:
    /// 
    /// - `distance(a, b) == distance(b, a)`.
    pub fn distance(particle1: &Particle, particle2: &Particle) -> f64 {
        Particle::position_difference(particle1, particle2).length()
    }

    /// Calculate the product of the masses of two particles.
    pub fn mass_product(particle1: &Particle, particle2: &Particle) -> f64 {
        particle1.mass * particle2.mass
    }
}

/// These methods are used for creating particles in tests.
#[cfg(test)]
impl Particle {
    /// Creates a particle at the given position with zero velocity.
    pub fn at(x: f64, y: f64, z: f64) -> Self {
        Self {
            position: Vec3::new(x, y, z),
            ..Default::default()
        }
    }

    /// Builder method for testing. Manually sets the velocity of the particle
    /// to the given value.
    pub fn with_velocity(mut self, x: f64, y: f64, z: f64) -> Self {
        self.velocity = Vec3::new(x, y, z);
        self
    }

    /// Builder method for testing. Manually sets the force of the particle
    /// to the given value.
    pub fn with_force(mut self, x: f64, y: f64, z: f64) -> Self {
        self.force = Vec3::new(x, y, z);
        self
    }

    /// Builder method for testing. Manually sets the mass of the particle
    /// to the given value.
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }
}

// TODO tests for [update_velocity], [update_position], and [delay_force]
