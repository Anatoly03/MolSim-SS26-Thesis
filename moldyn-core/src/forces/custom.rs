//! This module contains the [LennardJonesForce] struct, which implements
//! the [Force] trait according to the Lennard-Jones potential.

use crate::{Force, Particle};

pub struct CustomForce {
    func: Box<dyn Fn(&Particle, &Particle) -> f64>,
}

impl Force for CustomForce {
    fn system_name(&self) -> &str {
        "custom-potential"
    }

    fn potential(&self, particle: &Particle, other: &Particle) -> f64 {
        (self.func)(particle, other)
    }
}

impl CustomForce {
    pub fn new(func: Box<dyn Fn(&Particle, &Particle) -> f64>) -> Self {
        Self { func }
    }
}
