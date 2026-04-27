//! TODO document

use serde::{Deserialize, Serialize};

/// The struct representing the global simulation settings, which includes the
/// time range and simulation delta time.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SimulationArgs {
    /// TODO document
    #[serde(default)]
    pub time_start: Option<f64>,

    /// TODO document
    #[serde(default)]
    pub time_end: Option<f64>,

    /// TODO document
    #[serde(default)]
    pub time_step: Option<f64>,
}

#[cfg(test)]
impl SimulationArgs {
    /// Creates a new [SimulationArgs] with the given time range and time step.
    pub fn new(time: f64, time_step: f64) -> Self {
        Self {
            time_start: Some(0.0),
            time_end: Some(time),
            time_step: Some(time_step),
        }
    }

    /// Amount of simulation ticks to run with a fixed time step of `0.01`.
    /// Used for benchmarking tests to run for a fixed amount of time.
    pub fn ticks(ticks: usize) -> Self {
        const TIME_STEP: f64 = 0.01;

        Self {
            time_start: Some(0.0),
            time_end: Some(ticks as f64 * TIME_STEP),
            time_step: Some(TIME_STEP),
        }
    }
}
