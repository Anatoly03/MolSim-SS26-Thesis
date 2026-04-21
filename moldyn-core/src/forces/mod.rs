mod ljp;
mod newton;

use crate::{Particle, Vec3};
pub use ljp::LennardJonesForce;
pub use newton::NewtonForce;

pub trait Force {
    /// Calculates the force between two particles, not changing their state.
    fn force(&self, particle: &Particle, other: &Particle) -> Vec3;

    /// Applies the force to a particle pair. The force is added (or subtracted)
    /// to the `force` field of each particle forming an attraction or repulsion.
    fn apply_force(&self, particle: &mut Particle, other: &mut Particle) {
        let force = self.force(particle, other);
        particle.force += force;
        other.force -= force;
    }
}
