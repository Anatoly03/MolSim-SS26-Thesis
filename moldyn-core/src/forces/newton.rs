//! This module contains the [NewtonForce] struct, which implements the
//! [Force] trait according to Newton's law of universal gravitation.

use crate::{Particle, Vec3, forces::Force};

/// A struct representing a Newton (or Coloumb-like) force, which implements
/// the [Force] trait.
#[derive(Default)]
pub struct NewtonForce;

impl Force for NewtonForce {
    fn force(&self, particle: &Particle, other: &Particle) -> Vec3 {
        todo!()
    }
}
