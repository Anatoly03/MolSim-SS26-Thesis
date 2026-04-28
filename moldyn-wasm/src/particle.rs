//! The module which wraps the core library [Vec3] struct into a WebAssembly
//! compatible binding.

use moldyn_core::Particle;
use wasm_bindgen::prelude::*;

use crate::Vec3Wrapper;

/// A particle in three-dimensional space.
#[wasm_bindgen(inspectable, js_name = Particle)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParticleWrapper {
    /// Position of the particle in 3D space.
    #[wasm_bindgen(readonly)]
    pub position: Vec3Wrapper,

    /// Velocity of the particle in 3D space.
    #[wasm_bindgen(readonly)]
    pub velocity: Vec3Wrapper,

    /// Force effective on the particle in 3D space.
    #[wasm_bindgen(readonly)]
    pub force: Vec3Wrapper,

    /// Mass of the particle.
    #[wasm_bindgen(readonly)]
    pub mass: f64,
}

#[wasm_bindgen]
impl ParticleWrapper {
    /// Creates a new particle with unset values.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Particle } from "moldyn-wasm";
    /// const particle = new Particle();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            position: Vec3Wrapper::new_zero(),
            velocity: Vec3Wrapper::new_zero(),
            force: Vec3Wrapper::new_zero(),
            mass: 0.0,
        }
    }

    /// Creates a new particle given a position, initial velocity and
    /// mass.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Particle, Vec } from "moldyn-wasm";
    ///
    /// const particle = new Particle(
    ///     new Vec(2, 2, 0), // position
    ///     new Vec(1, 0, 0), // velocity
    ///     1.0               // mass
    /// );
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn from_data(position: Vec3Wrapper, velocity: Vec3Wrapper, mass: f64) -> Self {
        Self {
            position,
            velocity,
            force: Vec3Wrapper::new_zero(),
            mass,
        }
    }
}

impl From<Particle> for ParticleWrapper {
    fn from(part: Particle) -> Self {
        Self {
            position: part.get_position().into(),
            velocity: part.get_velocity().into(),
            force: part.get_force().into(),
            mass: part.get_mass(),
        }
    }
}

impl Into<Particle> for ParticleWrapper {
    fn into(self) -> Particle {
        let mut part = Particle::from_data(self.position.into(), self.velocity.into(), self.mass);
        part.apply_force(self.force.into());
        part
    }
}
