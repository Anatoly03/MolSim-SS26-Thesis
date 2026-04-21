//! This module contains the [Particle] struct.

use serde::{Deserialize, Serialize};
use crate::Vec3;

/// A struct representing a particle record in the simulation.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Particle {
    /// Position of the particle in 3D space.
    pub position: Vec3,

    /// Velocity of the particle in 3D space.
    pub velocity: Vec3,

    /// Force effective on the particle in 3D space.
    pub force: Vec3,

    /// Force which was effective on the particle in the previous time step.
    old_force: Vec3,

    /// Mass of the particle.
    pub mass: f64,
}

impl Particle {
    /// Propagates the current force to the old force. This is used in velocity
    /// calculations which depend on the previous time step's force.
    pub fn push_force(&mut self, force: Vec3) {
        self.old_force = self.force;
        self.force = force;
    }
}
