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
use serde::{Deserialize, de::Visitor};

/// A deserialization-utility representing "particle-like" entries in the
/// input. Intuitively, a particle-like is either a single particle or a
/// generator yielding a set of particles.
pub enum ParticleLike {
    Single(Particle),
    Cuboid(Vec<Particle>),
}

struct ParticleLikeVisitor;

impl<'de> Visitor<'de> for ParticleLikeVisitor {
    type Value = ParticleLike;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a particle-like entry")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for ParticleLike {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ParticleLikeVisitor)
    }
}
