//! TODO document

use serde::{Deserialize, Serialize};

/// TODO document
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

// impl SimulationArgs {
//     /// TODO document
//     fn default_time_end() -> f64 {
//         1000.0
//     }

//     /// TODO document
//     fn default_time_step() -> f64 {
//         0.014
//     }
// }
