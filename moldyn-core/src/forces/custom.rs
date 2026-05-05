//! This module contains the [LennardJonesForce] struct, which implements
//! the [Force] trait according to the Lennard-Jones potential.

use crate::{Force, Particle};
use meval::Expr;

/// A type alias for a callback function that takes two particles and returns an output of type `Output`.
pub type ParticlePairCallback<Output> = Box<dyn Fn(&Particle, &Particle) -> Output>;

pub struct CustomForce {
    func: ParticlePairCallback<f64>,
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
    pub fn new(func: ParticlePairCallback<f64>) -> Self {
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

#[cfg(test)]
mod test {
    use crate::{CustomForce, Particle, Force};

    /// This test validates that the variable `M` in custom force expression is
    /// the product of masses.
    #[test]
    fn custom_m_is_mass_product() {
        let custom = CustomForce::from_expr("M");

        let p1 = Particle::at(2.0, 5.0, 10.0).with_mass(7.0);
        let p2 = Particle::at(1.0, 2.0, 4.0).with_mass(3.0);

        let potential = custom.potential(&p1, &p2);
        assert_eq!(potential, -21.0, "potential should be negative the product of masses");
    }

    /// This test validates that the variable `r` in custom force expression is
    /// the distance between particles.
    #[test]
    fn custom_r_is_distance() {
        let custom = CustomForce::from_expr("r");

        let p1 = Particle::at(1.0, 1.0, 10.0).with_mass(7.0);
        let p2 = Particle::at(1.0, 1.0, 4.0).with_mass(3.0);

        let potential = custom.potential(&p1, &p2);
        assert_eq!(potential, -6.0, "potential should be negative the distance between particles");
    }

    /// This test validates that newton is attractive. ( ͡° ͜ʖ ͡°)
    #[test]
    fn newton_is_attractive() {
        let newton = CustomForce::from_expr("M / r^2");

        let p1 = Particle::default().with_mass(1.0);
        let p2 = Particle::at(1.0, 0.0, 0.0).with_mass(1.0);

        let force_on_p1 = newton.force(&p1, &p2);
        let force_on_p2 = newton.force(&p2, &p1);

        assert!(force_on_p1.x > 0.0, "force on p1 should be attractive");
        assert!(force_on_p2.x < 0.0, "force on p2 should be attractive");
    }
}
