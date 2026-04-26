//! This module defines entries in the `particle`. section of the input file.
//! Intuitively, an entry is a definition of one or multiple particles. Sets
//! of particles can be defined with the appropriate `type` tag.
//!
//! The file format allows the following single-particle definition.
//!
//! ```yaml
//! particles:
//!   - position: [x, y, z]
//!   - velocity: [vx, vy, vz]
//!   - mass: m
//! ```
//!
//! The file format should also allow the following multiple-particle definitions,
//! supporting simple equations in the `position` and `velocity` tags containing
//! the foreach variables and supporting basic arithmetics.
//!
//! ```yaml
//! particles:
//!   - type: cuboid
//!   - foreach: [nx, ny, nz]
//!   - position: [x + n * dx, y + m * dy, z + l * dz]
//!   - velocity: [...]
//!   - mass: m
//!   - sigma: o
//!   - brownian_sigma: b
//! ```

use meval::Expr;
use moldyn_core::{Particle, Vec3};
use serde::{Deserialize, Serialize};

/// A deserialization-utility representing "particle-like" entries in the
/// input. Intuitively, a particle-like is either a single particle or a
/// generator yielding a set of particles.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParticleLike {
    Single(Particle),
    Cuboid(Cuboid),
}

/// A cuboid generator.
#[derive(Serialize, Deserialize)]
pub struct Cuboid {
    #[serde(rename = "type")]
    _type: CuboidTag,

    /// The width, height and depth of the cuboid in particle count.
    ///
    /// # Example
    ///
    /// ```yaml
    /// particles:
    ///   - type: cuboid
    ///     foreach: [100, 20]
    /// ```
    foreach: Vec3,

    /// The position of particles in the cuboid, evaluated as matthematical
    /// expressions.
    ///
    /// This is a required attribute, and not implement per default. This is
    /// by design because the iterating position of a cuboid should not be the
    /// constant `0`.
    ///
    /// # Example
    ///
    /// Vector array syntax supported.
    ///
    /// ```yaml
    /// particles:
    ///   - type: cuboid
    ///     foreach: [100, 20]
    ///     position: [
    ///       20.0 + 1.1225 * nx,
    ///       20.0 + 1.1225 * ny,
    ///     ]
    /// ```
    ///
    /// Vector map syntax is also supported.
    ///
    /// ```yaml
    /// particles:
    ///   - type: cuboid
    ///     foreach: [100, 20]
    ///     position:
    ///       x: 20.0 + 1.1225 * nx
    ///       y: 20.0 + 1.1225 * ny
    /// ```
    #[serde(skip_serializing)]
    position: Vec3<Option<Expr>>,

    /// The velocity of particles in the cuboid, evaluated as matthematical
    /// expressions.
    ///
    /// # Example
    ///
    /// Vector array syntax supported.
    ///
    /// ```yaml
    /// particles:
    ///   - type: cuboid
    ///     foreach: [100, 20]
    ///     velocity: [
    ///       0.00011225 * nx,
    ///       0.00011225 * ny,
    ///     ]
    /// ```
    ///
    /// Vector map syntax is also supported.
    ///
    /// ```yaml
    /// particles:
    ///   - type: cuboid
    ///     foreach: [100, 20]
    ///     velocity:
    ///       x: 0.00011225 * nx
    ///       y: 0.00011225 * ny
    /// ```
    #[serde(default, skip_serializing)]
    velocity: Vec3<Option<Expr>>,

    /// TODO document
    #[serde(default = "default_mass")]
    mass: f64,

    /// TODO document
    #[serde(default)]
    sigma: f64,

    ///TODO document
    #[serde(default)]
    brownian_sigma: f64,
}

/// A tag for cuboid generators, used to give [serde] a hint in deserialization.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum CuboidTag {
    Cuboid,
}

impl From<ParticleLike> for Vec<Particle> {
    fn from(value: ParticleLike) -> Self {
        match value {
            ParticleLike::Single(p) => vec![p],
            ParticleLike::Cuboid(c) => {
                let mut particles = Vec::new();

                let width = i32::max(c.foreach.x as i32, 1);
                let height = i32::max(c.foreach.y as i32, 1);
                let depth = i32::max(c.foreach.z as i32, 1);

                for nx in 0..width {
                    for ny in 0..height {
                        for nz in 0..depth {
                            let position = c.position.clone().map(|expr| {
                                expr.map(|e| {
                                    e.eval_with_context([
                                        ("nx", nx.into()),
                                        ("ny", ny.into()),
                                        ("nz", nz.into()),
                                    ])
                                    .unwrap()
                                })
                                .unwrap_or(0.0)
                            });
                            let velocity = c.velocity.clone().map(|expr| {
                                expr.map(|e| {
                                    e.eval_with_context([
                                        ("nx", nx.into()),
                                        ("ny", ny.into()),
                                        ("nz", nz.into()),
                                    ])
                                    .unwrap()
                                })
                                .unwrap_or(0.0)
                            });
                            let mass = c.mass;

                            particles.push(Particle::from_data(position, velocity, mass));
                        }
                    }
                }
                particles
            }
        }
    }
}

/// The default mass is not zero.
pub fn default_mass() -> f64 {
    1.0
}
