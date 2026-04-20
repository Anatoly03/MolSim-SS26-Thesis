use crate::output_writer::OutputWriter;
use crate::particle::Particle;
use std::fs::File;
use std::io::{BufRead, BufWriter, Result, Write};

pub struct VTKWriter;

impl OutputWriter for VTKWriter {
    fn plot_particles(
        &self,
        particles: &mut Vec<Particle>,
        filename: &str,
        iteration: usize,
    ) -> Result<()> {
        todo!()
    }
}
