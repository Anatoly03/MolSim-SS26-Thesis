//! This module contains the [NewtonForce] struct, which implements the
//! [Force] trait according to Newton's law of universal gravitation.

use crate::{Particle, forces::Force};

/// A struct representing a Newton (or Coloumb-like) force, which implements
/// the [Force] trait.
pub struct NewtonForce {
    factor: f64,
}

#[cfg(test)]
impl NewtonForce {
    /// The constructor of [NewtonForce]. The parameters are set to make
    /// tests forward-compatible (not relying on the [Default] implementation).
    fn with_gravity_factor(factor: f64) -> Self {
        Self { factor }
    }
}

impl Force for NewtonForce {
    fn system_name(&self) -> &str {
        "newton"
    }

    /// Calculates the potential energy between two particles according to Newton's
    /// law of universal gravitation.
    /// 
    /// ```text
    /// potential = -G * M / r
    /// potential = -M / r          (assuming G = 1)
    /// ```
    fn potential(&self, particle: &Particle, other: &Particle) -> f64 {
        let distance = Particle::distance(particle, other);

        if distance == 0.0 {
            0.0
        } else {
            let mul_mass = Particle::mass_product(particle, other);
            -self.factor * mul_mass / distance
        }

        // TODO consider using the following code instead of above (after fixing paraview bugs)

        // let distance = Particle::distance(particle, other);

        // // match distance {
        // //     0.0 => 0.0,
        // //     _ => {
        // //         let mul_mass = Particle::mass_product(particle, other);
        // //         -self.factor * mul_mass / distance
        // //     }
        // // }

        // if distance == 0.0 {
        //     0.0
        // } else {
        //     let mul_mass = Particle::mass_product(particle, other);
        //     -self.factor * mul_mass / distance
        // }
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
    use crate::{Force, NewtonForce, Particle, Vec3};

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

    /// This test validates the correctness of the potential energy calculation.
    /// 
    /// In this test, the distance is strictly `1.0` and the masses are `1.0`.
    /// Therefore, the potential energy should be `-1.0`.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 1.0 / 1.0
    ///   = -1.0
    /// ```
    #[test]
    fn potential_energy_validation1() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(1.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 1.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let potential = newton.potential(&p1, &p2);

        assert_eq!(potential, -1.0, "potential energy should be -1.0");
    }

    /// This test validates the correctness of the potential energy calculation.
    /// 
    /// In this test, the distance is strictly `1.0` and the masses are respectively
    /// `1.0` and `4.0`. Since the product of masses is in the numerator, the potential
    /// energy should scale by the factor `4`, yielding the potential energy `-4.0`.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 4.0 / 1.0
    ///   = -4.0
    /// ```
    #[test]
    fn potential_energy_validation2() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(1.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 4.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let potential = newton.potential(&p1, &p2);

        assert_eq!(potential, -4.0, "potential energy should be -4.0");
    }

    /// This test validates the correctness of the potential energy calculation.
    /// 
    /// In this test, the distance is strictly `1.0` and the masses are `4.0`.
    /// Since the product of masses is in the numerator, the potential energy
    /// should scale by the factor squared `16`, yielding the potential energy
    /// `-16.0`.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 4.0 * 4.0 / 1.0
    ///   = -16.0
    /// ```
    #[test]
    fn potential_energy_validation3() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(1.0, 0.0, 0.0);

        let p1m = 4.0;
        let p2m = 4.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let potential = newton.potential(&p1, &p2);

        assert_eq!(potential, -16.0, "potential energy should be -16.0");
    }

    /// This test validates the correctness of the potential energy calculation. This
    /// test depends on [potential_energy_validation2] and [potential_energy_validation3],
    /// which validate the masses being in the numerator.
    /// 
    /// In this test, the distance is strictly `2.0` and the masses are `1.0`.
    /// Since the masses do not affect the equation as covered by other tests, the
    /// potential energy should scale by in the inverse factor, yielding `-0.5`.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 1.0 / 2.0
    ///   = -0.5
    /// ```
    /// 
    /// This test validates the correctness of the potential energy calculation.
    /// In this test, the distance is strictly 2.0 and the masses are strictly 1.0.
    /// Since the tests [potential_energy_validation2] and [potential_energy_validation3]
    /// validate the masses being in the numerator, we expect the numerator to be -1.0.
    /// 
    /// Since the distance is in the denominator, we expect the potential energy to be -0.5.
    #[test]
    fn potential_energy_validation4() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(2.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 1.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let potential = newton.potential(&p1, &p2);

        assert_eq!(potential, -0.5, "potential energy should be -0.5");
    }

    /// This test validates the correctness of the potential energy calculation
    /// in a more complex scenario.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 5.0 * 2.0 / 10.0
    ///   = -1.0
    /// ```
    #[test]
    fn potential_energy_validation_complex() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(10.0, 0.0, 0.0);

        let p1m = 5.0;
        let p2m = 2.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let potential = newton.potential(&p1, &p2);

        assert_eq!(potential, -1.0, "potential energy should be -1.0");
    }

    /// This test validates the correctness of the force calculation. The
    /// formula is taken from the article [Wikipedia - Newton's law of universal gravitation](https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation)
    /// 
    /// In this test, the distance and the masses are strictly `1.0`.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 1.0 / 1.0
    ///   = -1.0
    /// 
    /// F =  -U / r
    ///   = 1.0 / 1.0
    /// ```
    #[test]
    fn force_validation1() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(1.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 1.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let force = newton.force(&p1, &p2);

        assert_eq!(force.length(), 1.0, "force length should be 1.0");
    }

    /// This test validates the correctness of the force calculation.
    /// 
    /// In this test, the distance is strictly `1.0` and one of the bodies has
    /// the mass `4.0`.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 4.0 / 1.0
    ///   = -4.0
    /// 
    /// F =  -U / r
    ///   = 4.0 / 1.0
    /// ```
    #[test]
    fn force_validation2() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(1.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 4.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let force = newton.force(&p1, &p2);

        assert_eq!(force.length(), 4.0, "force should be 4.0");
    }

    /// This test validates the correctness of the force calculation.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 4.0 * 4.0 / 1.0
    ///   = -16.0
    /// 
    /// F =   -U / r
    ///   = 16.0 / 1.0
    /// ```
    #[test]
    fn force_validation3() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(1.0, 0.0, 0.0);

        let p1m = 4.0;
        let p2m = 4.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let force = newton.force(&p1, &p2);

        assert_eq!(force.length(), 16.0, "force should be 16.0");
    }

    /// This test validates the correctness of the force calculation.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 1.0 / 2.0
    ///   = -0.5
    /// 
    /// F =   -U / r
    ///   =  0.5 / 2.0
    ///   =  0.25
    /// ```
    #[test]
    fn force_validation4() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(2.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 1.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let force = newton.force(&p1, &p2);

        assert_ne!(force.length(), 0.5, "newton force should not be 0.5. force is missing division of potential (correct) by distance (missing)");
        assert_eq!(force.length(), 0.25, "force should be 0.25");
    }

    /// This test validates the correctness of the force calculation.
    /// 
    /// # Equation
    /// 
    /// ```text
    /// U = -G *         M / r
    ///   = -1 * 1.0 * 2.0 / 2.0
    ///   = -1.0
    /// 
    /// F =   -U / r
    ///   = 1.0 / 2.0
    ///   = 0.5
    /// ```
    #[test]
    fn force_validation5() {
        let p1pos = Vec3::new(0.0, 0.0, 0.0);
        let p2pos = Vec3::new(2.0, 0.0, 0.0);

        let p1m = 1.0;
        let p2m = 2.0;

        let newton = NewtonForce::with_gravity_factor(1.0);

        let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
        let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

        let force = newton.force(&p1, &p2);

        assert_ne!(force.length(), 1.0, "newton force should not be 1.0. force is missing division of potential (correct) by distance (missing)");
        assert_eq!(force.length(), 0.5, "force should be 0.5");
    }

}


    // /// This test validates the correctness of the potential energy calculation
    // /// in a more complex scenario.
    // /// 
    // /// # Equation
    // /// 
    // /// ```text
    // /// U = -G *         M / r
    // ///   = -1 * 5.0 * 2.0 / 10.0
    // ///   = -1.0
    // /// ```
    // #[test]
    // fn potential_energy_validation_complex() {
    //     let p1pos = Vec3::new(0.0, 0.0, 0.0);
    //     let p2pos = Vec3::new(10.0, 0.0, 0.0);

    //     let p1m = 5.0;
    //     let p2m = 2.0;

    //     let newton = NewtonForce::with_gravity_factor(1.0);

    //     let p1 = Particle::from_data(p1pos, Vec3::zero(), p1m);
    //     let p2 = Particle::from_data(p2pos, Vec3::zero(), p2m);

    //     let potential = newton.potential(&p1, &p2);

    //     assert_eq!(potential, -1.0, "potential energy should be -1.0");
    // }
