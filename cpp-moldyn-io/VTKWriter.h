/**
 * @file VTKWriter.h
 * @author Anatoly Weinstein
 *
 * @brief
 */

#ifdef ENABLE_VTK_SUPPORT

#include <fstream>
#include <iostream>
#include <memory>
#include <sstream>
#include <string>

#include "yaml-cpp/yaml.h"

#include "YAMLImplementation.h"
#include "Writer.h"
#include "container/DirectSum.h"
#include "container/ParticleContainer.h"
#include "force/Newton.h"
#include "Reader.h"
#include "Simulation.h"
#include "Vec3.h"

struct VTKWriter : public Writer
{
public:
    /**
     * @brief Creates a new VTK file writer.
     */
    VTKWriter(const std::filesystem::path file_path, const Simulation<DirectSum> &simulation) : Writer(file_path, simulation)
    {
        // override extension for VTK files
        output_extension = ".vtu";
    }

    /**
     * @brief Consume the VTK file and return a Simulation struct.
     */
    void write() override
    {
        // Initialize points
        auto points = vtkSmartPointer<vtkPoints>::New();

        // Create and configure data arrays
        vtkNew<vtkFloatArray> massArray;
        massArray->SetName("mass");
        massArray->SetNumberOfComponents(1);

        vtkNew<vtkFloatArray> velocityArray;
        velocityArray->SetName("velocity");
        velocityArray->SetNumberOfComponents(3);

        vtkNew<vtkFloatArray> forceArray;
        forceArray->SetName("force");
        forceArray->SetNumberOfComponents(3);

        vtkNew<vtkIntArray> typeArray;
        typeArray->SetName("type");
        typeArray->SetNumberOfComponents(1);

        for (auto &p : particles)
        {
            points->InsertNextPoint(p.getX().data());
            massArray->InsertNextValue(static_cast<float>(p.getM()));
            velocityArray->InsertNextTuple(p.getV().data());
            forceArray->InsertNextTuple(p.getF().data());
            typeArray->InsertNextValue(p.getType());
        }

        // Set up the grid
        auto grid = vtkSmartPointer<vtkUnstructuredGrid>::New();
        grid->SetPoints(points);

        // Add arrays to the grid
        grid->GetPointData()->AddArray(massArray);
        grid->GetPointData()->AddArray(velocityArray);
        grid->GetPointData()->AddArray(forceArray);
        grid->GetPointData()->AddArray(typeArray);

        // Create writer and set data
        vtkNew<vtkXMLUnstructuredGridWriter> writer;
        writer->SetFileName(frame_file_path().c_str());
        writer->SetInputData(grid);
        writer->SetDataModeToAscii();

        // Write the file
        writer->Write();
    }
};
#endif
