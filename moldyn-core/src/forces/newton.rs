//! This module contains the [NewtonForce] struct, which implements the
//! [Force] trait according to Newton's law of universal gravitation.

use crate::{Particle, forces::Force};

/// A struct representing a Newton (or Coloumb-like) force, which implements
/// the [Force] trait.
pub struct NewtonForce {
    factor: f64,
}

impl Force for NewtonForce {
    fn system_name(&self) -> &str {
        "newton"
    }

    fn potential(&self, particle: &Particle, other: &Particle) -> f64 {
        let diff = Particle::position_difference(particle, other);
        let distance = diff.length();

        if distance == 0.0 {
            0.0
        } else {
            let mul_mass = Particle::mass_product(particle, other);
            -self.factor * mul_mass / distance
        }
    }
}

impl Default for NewtonForce {
    /// The default instance of [NewtonForce]. The parameters are set
    /// to the following.
    ///
    /// | Parameter | Value |
    /// | --- | --- |
    /// | `factor` | `1.0` |
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

#[cfg(test)]
mod test {
    use crate::{Force, NewtonForce, Particle};

    /// This test validates that newton is attractive. ( ͡° ͜ʖ ͡°)
    #[test]
    fn newton_is_attractive() {
        let p1 = Particle::default().with_mass(1.0);
        let p2 = Particle::at(1.0, 0.0, 0.0).with_mass(1.0);

        let force_on_p1 = NewtonForce::default().force(&p1, &p2);
        let force_on_p2 = NewtonForce::default().force(&p2, &p1);

        assert!(force_on_p1.x > 0.0, "force on p1 should be attractive");
        assert!(force_on_p2.x < 0.0, "force on p2 should be attractive");
    }

    /// This test validates that newton's third law holds: actio = reactio.
    #[test]
    fn newtons_third_law() {
        let p1 = Particle::default().with_mass(1.0);
        let p2 = Particle::at(1.0, 0.0, 0.0).with_mass(1.0);

        let force_on_p1 = NewtonForce::default().force(&p1, &p2);
        let force_on_p2 = NewtonForce::default().force(&p2, &p1);

        assert_eq!(force_on_p1, -force_on_p2, "newton's third law should hold");
    }

    /// This test validates that forces occur on a single line (dimensional
    /// correctness)
    #[test]
    fn may_the_force_be_one_dimensional() {
        let p1 = Particle::default().with_mass(1.0);
        let p2 = Particle::at(1.0, 0.0, 0.0).with_mass(1.0);

        let force = NewtonForce::default().force(&p1, &p2);

        assert_eq!(force.y, 0.0, "force on p1 should be zero along y-axis only");
        assert_eq!(force.z, 0.0, "force on p1 should be zero along z-axis only");
    }
}
