use moldyn_core::{DirectSum, Simulation, SimulationTrait};
use wasm_bindgen::prelude::*;

use crate::particle::ParticleWrapper;

/// A direct sum simulation instance.
#[wasm_bindgen(js_name = Simulation)]
pub struct SimulationWrapper {
    #[wasm_bindgen(skip)]
    sum: Box<dyn SimulationTrait>,
}

#[wasm_bindgen]
impl SimulationWrapper {
    /// Creates a new simulation with the default settings.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sum: Box::new(Simulation::<DirectSum>::default()),
        }
    }

    /// Adds a single particle to the simulation.
    #[wasm_bindgen(js_name = add)]
    pub fn add_single_particle(&mut self, particle: &ParticleWrapper) {
        let p = (*particle).into();
        self.sum.add_particles(vec![p]);
    }

    /// Adds multiple particles to the simulation.
    #[wasm_bindgen(js_name = add)]
    pub fn add_particles(&mut self, particles: Vec<ParticleWrapper>) {
        self.sum
            .add_particles(particles.iter().map(|p| (*p).into()).collect());
    }

    /// Advances the simulation by a given time step.
    #[wasm_bindgen]
    pub fn step(&mut self, delta_time: f64) {
        self.sum.step(delta_time);
    }

    /// Retrieves the current particles in the simulation as readonly instances.
    #[wasm_bindgen]
    pub fn particles(&self) -> Vec<ParticleWrapper> {
        self.sum.particles().map(|p| (*p).into()).collect()
    }
}
