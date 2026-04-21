//! This module contains the [Particle] struct.

use serde::{Deserialize, Serialize};
use crate::Vec3;

/// A struct representing a particle record in the simulation.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Particle {
    /// Position of the particle in 3D space.
    position: Vec3<f64>,

    /// Velocity of the particle in 3D space.
    velocity: Vec3<f64>,

    /// Force effective on the particle in 3D space.
    force: Vec3<f64>,

    /// Force which was effective on the particle in the previous time step.
    old_force: Vec3<f64>,

    /// Mass of the particle.
    mass: f64,
}
