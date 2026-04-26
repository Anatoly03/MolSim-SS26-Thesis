//! Defines a simple text writer for the simulation data. This is a basic
//! implementation used only to verify the capacity of the program to write
//! output files and is deprecated for future use.

use super::OutputWriter;
use moldyn_core::Simulation;
use std::io::{BufWriter, Write};

/// A simple text writer for the simulation data. This implementation logs
/// the particles using Rusts' built-in [std::fmt::Debug] trait. It is not
/// a seriously usable output format.
/// 
/// # Output Example
/// 
/// ```rust,no_run
/// Particle { position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, velocity: Vec3 { x: -8.563553373154629e-10, y: 9.058561490167886e-7, z: 0.0 }, force: Vec3 { x: -1.7127106746309258e-7, y: 0.0001811712298033577, z: 0.0 }, old_force: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, mass: 1.0 }
/// Particle { position: Vec3 { x: -0.01, y: 1.0, z: 0.0 }, velocity: Vec3 { x: -0.9999500035551665, y: -0.004998404868413561, z: 0.0 }, force: Vec3 { x: 2.999786690007546e-8, y: -2.9990429210481366e-6, z: 0.0 }, old_force: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, mass: 3e-6 }
/// Particle { position: Vec3 { x: -0.00425, y: 5.36, z: 0.0 }, velocity: Vec3 { x: -0.42499926034973373, y: -0.0009328386747764901, z: 0.0 }, force: Vec3 { x: 1.4127320085105618e-7, y: -0.0001781721868823096, z: 0.0 }, old_force: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, mass: 0.000955 }
/// Particle { position: Vec3 { x: 34.75, y: 0.00029600000000000004, z: 0.0 }, velocity: Vec3 { x: -0.0001440195248088764, y: 0.029600019482983273, z: 0.0 }, force: Vec3 { x: -2.8803904961775283e-16, y: 3.896596654088676e-20, z: 0.0 }, old_force: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, mass: 1e-14 }
/// ```
#[derive(Default)]
pub struct TxtWriter {
    pub frame_number: usize,
}

impl OutputWriter for TxtWriter {
    fn next_frame_number(&mut self) -> usize {
        self.frame_number += 1;
        self.frame_number
    }

    fn write_frame(
        &mut self,
        writer: &mut BufWriter<std::fs::File>,
        state: &dyn Simulation,
    ) -> std::io::Result<()> {
        for p in state.particles() {
            writeln!(writer, "{p:?}").expect("Error writing to output file");
        }
        Ok(())
    }

    fn extension(&self) -> &str {
        "txt"
    }
}
