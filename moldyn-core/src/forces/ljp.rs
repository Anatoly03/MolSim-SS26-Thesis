//! This module contains the [LennardJonesForce] struct, which implements
//! the [Force] trait according to the Lennard-Jones potential.

use crate::{Particle, Vec3, forces::Force};

/// A struct representing a Lennard-Jones force, which implements the
/// [Force] trait.
#[derive(Default)]
pub struct LennardJonesForce {
    // TODO cutoff_radius
    // TODO lennard jones variables (epsilon, sigma)
}

impl Force for LennardJonesForce {
    fn system_name(&self) -> &str {
        "lennard-jones"
    }
    
    fn force(&self, particle: &Particle, other: &Particle) -> Vec3 {
        todo!("lennard jones force not implemented")
    }
}
