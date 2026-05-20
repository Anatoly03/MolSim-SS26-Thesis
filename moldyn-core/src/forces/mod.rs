mod custom;
mod ljp;
mod newton;

use crate::{Particle, Vec3};
pub use custom::CustomForce;
pub use ljp::LennardJonesForce;
use meval::Expr;
pub use newton::NewtonForce;
use serde::de::Error;
use serde::{Deserialize, Serialize, de::Visitor};

/// The trait for force systems. A force system is a mathematical model which
/// describes the [potential energy](https://en.wikipedia.org/wiki/Potential_energy)
/// between two particles.
///
/// This trait provides two methods [Force::potential] and [Force::force] for interacting
/// with the force system. The potential is the scalar value used as the factor to the
/// normalized force vector.
///
/// The method [Force::system_name] is used for serialization of the force system.
pub trait Force {
    /// # Returns
    ///
    /// Name of the force system, which is used for serialization and
    /// deserialization. The characters are expected to be in `dash-case`.
    fn system_name(&self) -> &str;

    /// Checks if the input is the name or alias of this force system. Used for
    /// deserialization of the force system from a string.
    fn matches_name(&self, name: &str) -> bool {
        self.system_name().eq_ignore_ascii_case(name)
    }

    /// Calculates the potential energy between two particles.
    fn potential(&self, particle: &Particle, other: &Particle) -> f64;

    /// Calculates the force between two particles. For directly applying the
    /// force, see [Force::apply_force].
    ///
    /// # Formula
    ///
    /// ```text
    /// F = -U / r    (simplified to scalar, actually a vector)
    /// ```
    ///
    /// The resulting force is a three dimensional vector pointing towards the other
    /// particle. The magnitude is the fraction of potential energy and distance.
    ///
    /// # Returns
    ///
    /// The force vector that should be applied to the first particle. According
    /// to the [Third Law](https://en.wikipedia.org/wiki/Newton%27s_laws_of_motion#Third_law)
    /// the second particle should receive the negated force.
    ///
    /// # Example
    ///
    /// ```rust
    /// use moldyn_core::{Particle, Vec3, LennardJonesForce, Force};
    ///
    /// let mut particle1 = Particle::from_data(Vec3::new(0.0, 0.0, 0.0), Vec3::zero(), 1.0);
    /// let mut particle2 = Particle::from_data(Vec3::new(1.0, 0.0, 0.0), Vec3::zero(), 1.0);
    ///
    /// let force = LennardJonesForce::default().force(&particle1, &particle2);
    ///
    /// particle1.apply_force(force);
    /// particle2.apply_force(-force);
    /// ```
    fn force(&self, particle: &Particle, other: &Particle) -> Vec3 {
        let potential = self.potential(particle, other);
        let diff = Particle::position_difference(other, particle);
        let distance = diff.length2();

        if distance == 0.0 {
            Vec3::zero()
        } else {
            -diff * (potential / distance)
        }

        // TODO consider using the following code instead of above (after fixing paraview bugs)

        // // If the direction is not calculatable (`particle.position == other.position`), we can skip the rest of the calculations.
        // let direction = match Particle::direction(particle, other) {
        //     Some(dir) => dir,
        //     None => return Vec3::zero(),
        // };

        // // The force is the potential multiplied by the normalized direction vector.
        // direction * self.potential(particle, other)
    }

    /// Applies the calculated force to a particle pair.
    ///
    /// # Example
    ///
    /// ```rust
    /// use moldyn_core::{Particle, Vec3, LennardJonesForce, Force};
    ///
    /// let mut particle1 = Particle::from_data(Vec3::new(0.0, 0.0, 0.0), Vec3::zero(), 1.0);
    /// let mut particle2 = Particle::from_data(Vec3::new(1.0, 0.0, 0.0), Vec3::zero(), 1.0);
    ///
    /// let lennard_jones = LennardJonesForce::default();
    /// let force = lennard_jones.apply_force(&mut particle1, &mut particle2);
    /// ```
    fn apply_force(&self, particle: &mut Particle, other: &mut Particle) {
        let force = self.force(particle, other);
        particle.apply_force(force);
        other.apply_force(-force);
    }
}

impl<'a> Serialize for dyn Force + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.system_name())
    }
}

struct ForceVisitor;

impl<'de> Visitor<'de> for ForceVisitor {
    type Value = Box<dyn Force>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a force")
    }

    /// If the force is represented as a string, we can parse it as a known force type,
    /// assuming default parameters. Strings are case-insensitive.
    ///
    /// # Example
    ///
    /// ```yaml
    /// # Particle definition input file example
    /// name: halleys-comet
    /// force: gravitational
    /// ```
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.to_ascii_lowercase().as_str() {
            "lennardjones" | "lennard-jones" | "lj" => Ok(Box::new(LennardJonesForce::default())),
            "newton" | "gravitational" => Ok(Box::new(NewtonForce::default())),
            _ => Err(E::custom(format!("Unknown force type: {value}"))),
        }
    }

    // TODO: implement deserialization with parameters.
    // idea: force: { type: ..., params... }
    // idea: force: lennard-jones: { epsilon: ..., sigma: ... }
    // idea: force: gravity: { factor: ... }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let force_type = map
            .next_key::<String>()?
            .ok_or_else(|| A::Error::custom("Expected a force type key"))?;

        match force_type.to_ascii_lowercase().as_str() {
            "custom-potential" | "potential" => {
                let params = map.next_value::<Expr>()?;
                let custom: CustomForce = params.try_into().map_err(|e| {
                    Error::custom(format!("could not bind custom potential functtuion: {e}"))
                })?;
                Ok(Box::new(custom))
            }
            _ => Err(A::Error::custom(format!(
                "Unknown force type: {force_type}"
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for Box<dyn Force> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ForceVisitor)
    }
}

impl Default for Box<dyn Force> {
    /// The default force calculation system for this project is the lennard
    /// jones potential. If not specified, the system will be initialized with
    /// default parameters.
    fn default() -> Self {
        Box::new(LennardJonesForce::default())
    }
}
