//! This module contains the [LennardJonesForce] struct, which implements
//! the [Force] trait according to the Lennard-Jones potential.

use crate::{Force, Particle};
use meval::Expr;

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

    /// Quickly creates a force system from a string-like, panicking on any
    /// problem. Useful in test environments and for benchmarks.
    #[allow(dead_code)]
    #[cfg(test)]
    pub fn from_expr(expr: &str) -> Self {
        expr.parse::<Expr>()
            .expect("failed to parse expression")
            .try_into()
            .expect("failed to convert expression to custom force")
    }
}

impl TryFrom<Expr> for CustomForce {
    type Error = meval::Error;

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        let func = value.bind2("r", "M")?;

        let wrap = move |p1: &Particle, p2: &Particle| {
            let distance = Particle::distance(p1, p2);
            let mul_mass = Particle::mass_product(p1, p2);

            if distance == 0.0 {
                0.0
            } else {
                -func(distance, mul_mass)
            }
        };

        Ok(CustomForce::new(Box::new(wrap)))
    }
}
