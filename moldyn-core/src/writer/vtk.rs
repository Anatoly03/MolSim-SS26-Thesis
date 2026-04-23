//! Defines a simple text writer for the simulation data. This is a basic
//! implementation used only to verify the capacity of the program to write
//! output files and is deprecated for future use.

use crate::{OutputWriter, Simulation};
use std::io::BufWriter;
use vtkio::model::*;

/// A simple text writer for the simulation data.
///
/// # C++ Source Code
///
/// The [VtkWriter] attempts to rewrite the functionality of the C++ code from
/// the [original project](https://github.com/TUM-I5/MolSim/blob/master/src/outputWriter/VTKWriter.cpp)
/// in Rust utilizing the [vtkio] crate.
///
/// ```cpp
/// void VTKWriter::plotParticles(std::list<Particle> particles, const std::string &filename, int iteration)
/// {
///     // Initialize points
///     auto points = vtkSmartPointer<vtkPoints>::New();
///
///     // Create and configure data arrays
///     vtkNew<vtkFloatArray> massArray;
///     massArray->SetName("mass");
///     massArray->SetNumberOfComponents(1);
///
///     vtkNew<vtkFloatArray> velocityArray;
///     velocityArray->SetName("velocity");
///     velocityArray->SetNumberOfComponents(3);
///
///     vtkNew<vtkFloatArray> forceArray;
///     forceArray->SetName("force");
///     forceArray->SetNumberOfComponents(3);
///
///     vtkNew<vtkIntArray> typeArray;
///     typeArray->SetName("type");
///     typeArray->SetNumberOfComponents(1);
///
///     for (auto &p : particles)
///     {
///         points->InsertNextPoint(p.getX().data());
///         massArray->InsertNextValue(static_cast<float>(p.getM()));
///         velocityArray->InsertNextTuple(p.getV().data());
///         forceArray->InsertNextTuple(p.getF().data());
///         typeArray->InsertNextValue(p.getType());
///     }
///
///     // Set up the grid
///     auto grid = vtkSmartPointer<vtkUnstructuredGrid>::New();
///     grid->SetPoints(points);
///
///     // Add arrays to the grid
///     grid->GetPointData()->AddArray(massArray);
///     grid->GetPointData()->AddArray(velocityArray);
///     grid->GetPointData()->AddArray(forceArray);
///     grid->GetPointData()->AddArray(typeArray);
///
///     // Create filename with iteration number
///     std::stringstream strstr;
///     strstr << filename << "_" << std::setfill('0') << std::setw(4) << iteration << ".vtu";
///
///     // Create writer and set data
///     vtkNew<vtkXMLUnstructuredGridWriter> writer;
///     writer->SetFileName(strstr.str().c_str());
///     writer->SetInputData(grid);
///     writer->SetDataModeToAscii();
///
///     // Write the file
///     writer->Write();
/// }
/// ```
#[derive(Default)]
pub struct VtkWriter {
    pub frame_number: usize,
}

impl OutputWriter for VtkWriter {
    fn next_frame_number(&mut self) -> usize {
        self.frame_number += 1;
        self.frame_number
    }

    fn write_frame(
        &mut self,
        writer: &mut BufWriter<std::fs::File>,
        state: &Box<dyn Simulation>,
    ) -> std::io::Result<()> {
        // equivalent cpp: auto points = vtkSmartPointer<vtkPoints>::New();
        let particle_count = state.particle_count();
        let mut points = Vec::new();
        let mut masses = Vec::new();
        let mut velocities = Vec::new();
        let mut forces = Vec::new();

        state.for_each_particles(&mut |p| {
            // equivalent cpp: points->InsertNextPoint(p.getX().data());
            points.push(p.get_position().x);
            points.push(p.get_position().y);
            points.push(p.get_position().z);

            // equivalent cpp: massArray->InsertNextValue(static_cast<float>(p.getM()));
            masses.push(p.get_mass() as f32);

            // equivalent cpp: velocityArray->InsertNextTuple(p.getV().data());
            velocities.push(p.get_velocity().x as f32);
            velocities.push(p.get_velocity().y as f32);
            velocities.push(p.get_velocity().z as f32);

            // equivalent cpp: forceArray->InsertNextTuple(p.getF().data());
            forces.push(p.get_force().x as f32);
            forces.push(p.get_force().y as f32);
            forces.push(p.get_force().z as f32);
        });

        let attributes = Attributes {
            point: vec![
                Attribute::DataArray(DataArray {
                    // equivalent cpp:
                    // vtkNew<vtkFloatArray> massArray;
                    // massArray->SetName("mass");
                    // massArray->SetNumberOfComponents(1);
                    name: "mass".to_string(),
                    elem: ElementType::Scalars {
                        num_comp: 1,
                        lookup_table: None,
                    },
                    data: IOBuffer::F32(masses),
                }),
                Attribute::DataArray(DataArray {
                    // equivalent cpp:
                    // vtkNew<vtkFloatArray> velocityArray;
                    // velocityArray->SetName("velocity");
                    // velocityArray->SetNumberOfComponents(3);
                    name: "velocity".to_string(),
                    elem: ElementType::Vectors,
                    data: IOBuffer::F32(velocities),
                }),
                Attribute::DataArray(DataArray {
                    // equivalent cpp:
                    // vtkNew<vtkFloatArray> forceArray;
                    // forceArray->SetName("force");
                    // forceArray->SetNumberOfComponents(3);
                    name: "force".to_string(),
                    elem: ElementType::Vectors,
                    data: IOBuffer::F32(forces),
                }),
            ],
            cell: vec![],
        };

        // equivalent cpp: vtkNew<vtkXMLUnstructuredGridWriter> writer;
        let grid = UnstructuredGridPiece {
            points: IOBuffer::F64(points),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    // ERROR: In vtkXMLUnstructuredDataReader.cxx, line 752
                    // vtkXMLUnstructuredGridReader (0x7fcddf9d8e60): Cannot read cell offsets from Cells in piece 0 because the "offsets" array is not long enough.
                    connectivity: (0u64..particle_count as u64).collect(),
                    offsets: (1u64..=particle_count as u64).collect(),
                },
                // ERROR: In vtkXMLUnstructuredGridReader.cxx, line 142
                // vtkXMLUnstructuredGridReader (0x7fcdda790fc0): Piece 0 is missing its NumberOfCells attribute.
                types: vec![CellType::Vertex; particle_count],
            },
            data: attributes,
        };

        let vtk = Vtk {
            version: Version::new((1, 0)),
            // equivalent cpp: writer->SetFileName(strstr.str().c_str());
            // TODO set title, set file path
            title: String::default(),
            // equivalent cpp: writer->SetDataModeToAscii();
            // equivalent cpp: writer->SetInputData(grid);
            byte_order: ByteOrder::BigEndian,
            data: DataSet::inline(grid),
            // this is for vtk parallel files, we do not need it
            file_path: None,
        };

        vtk.write_xml(writer)?;

        Ok(())
    }

    fn extension(&self) -> &str {
        "vtu"
    }
}
