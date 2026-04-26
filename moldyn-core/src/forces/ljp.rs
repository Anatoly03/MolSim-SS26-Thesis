//! This module contains the [LennardJonesForce] struct, which implements
//! the [Force] trait according to the Lennard-Jones potential.

use crate::{Particle, forces::Force};

/// A struct representing a Lennard-Jones force, which implements the
/// [Force] trait.
pub struct LennardJonesForce {
    // TODO document
    cutoff_radius: f64,

    // TODO document
    epsilon: f64,

    // TODO document
    sigma: f64,
}

impl Force for LennardJonesForce {
    fn system_name(&self) -> &str {
        "lennard-jones"
    }

    fn potential(&self, particle: &Particle, other: &Particle) -> f64 {
        let distance = Particle::distance(particle, other);

        if distance == 0.0 || distance > self.cutoff_radius {
            0.0
        } else {
            let frac = self.sigma / distance;
            let frac6 = frac.powi(6);
            let frac12 = frac6.powi(2);

            4.0 * self.epsilon * (frac12 - frac6)
        }
    }
}

impl Default for LennardJonesForce {
    /// The default instance of [LennardJonesForce]. The parameters are set
    /// to the following.
    ///
    /// | Parameter | Value |
    /// | --- | --- |
    /// | `cutoff_radius` | `3.0` |
    /// | `epsilon` | `5.0` |
    /// | `sigma` | `1.0` |`
    fn default() -> Self {
        Self {
            // values are taken from 'assignment 3 task 2'
            cutoff_radius: 3.0,
            epsilon: 5.0,
            sigma: 1.0,
        }
    }
}
