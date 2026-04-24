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

use crate::Particle;
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
            ParticleLike::Cuboid(_) => todo!("cuboid support"),
        }
    }
}
