//! For writing VTK files.
//!
//! Author: Anatoly Weinstein
//! Created: April 2026

use crate::output_writer::OutputWriter;
use crate::particle::Particle;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use vtkio::model::*;

pub struct VTKWriter;

impl OutputWriter for VTKWriter {
    fn plot_particles(
        &self,
        particles: &mut Vec<Particle>,
        filename: &str,
        iteration: usize,
    ) -> Result<()> {
        let output_filename = format!("{filename}_{iteration:0>4}.xyz");

        let particle_count = particles.len();
        let mut points = Vec::new();
        let mut masses = Vec::new();
        let mut velocities = Vec::new();
        let mut forces = Vec::new();
        let mut type_data = Vec::new();

        let f = File::create(&output_filename)?;
        let mut writer = BufWriter::new(f);

        for p in particles {
            points.push(p.get_x()[0]);
            points.push(p.get_x()[1]);
            points.push(p.get_x()[2]);

            masses.push(p.get_m() as f32);

            velocities.push(p.get_v()[0] as f32);
            velocities.push(p.get_v()[1] as f32);
            velocities.push(p.get_v()[2] as f32);

            forces.push(p.get_f()[0] as f32);
            forces.push(p.get_f()[1] as f32);
            forces.push(p.get_f()[2] as f32);

            type_data.push(p.get_type() as i32);
        }

        let attributes = Attributes {
            point: vec![
                Attribute::DataArray(DataArray {
                    name: "mass".to_string(),
                    elem: ElementType::Scalars {
                        num_comp: 1,
                        lookup_table: None,
                    },
                    data: IOBuffer::F32(masses),
                }),
                Attribute::DataArray(DataArray {
                    name: "velocity".to_string(),
                    elem: ElementType::Vectors,
                    data: IOBuffer::F32(velocities),
                }),
                Attribute::DataArray(DataArray {
                    name: "force".to_string(),
                    elem: ElementType::Vectors,
                    data: IOBuffer::F32(forces),
                }),
                Attribute::DataArray(DataArray {
                    name: "type".to_string(),
                    elem: ElementType::Scalars {
                        num_comp: 1,
                        lookup_table: None,
                    },
                    data: IOBuffer::I32(type_data),
                }),
            ],
            cell: vec![],
        };

        let grid = UnstructuredGridPiece {
            points: IOBuffer::F64(points),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    connectivity: (0u64..particle_count as u64).collect(),
                    offsets: (1u64..=particle_count as u64).collect(),
                },
                types: vec![CellType::Vertex; particle_count],
            },
            data: attributes,
        };

        let vtk = Vtk {
            version: Version::new((1, 0)),
            title: String::default(),
            byte_order: ByteOrder::BigEndian,
            data: DataSet::inline(grid),
            file_path: None,
        };

        vtk.write_xml(writer)?;

        Ok(())
    }
}
