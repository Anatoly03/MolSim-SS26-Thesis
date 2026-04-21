//! This module contains the [Particle] struct.

use serde::{Deserialize, Serialize};
use crate::Vec3;

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

    /// Calculate the updated position of the particle given a delta time step.
    /// This functionality is constant across different simulation algorithms,
    /// so it is implemented here.
    pub fn update_position(&mut self, delta_time: f64) {
        self.position += self.velocity * delta_time + self.force * (delta_time.powi(2) / (2.0 * self.mass));
    }

    /// Calculate the updated velocity of the particle given a delta time step.
    /// This functionality is constant across different simulation algorithms,
    /// so it is implemented here.
    pub fn update_velocity(&mut self, delta_time: f64) {
        self.velocity += (self.force + self.old_force) * (delta_time / (2.0 * self.mass));
    }
}
