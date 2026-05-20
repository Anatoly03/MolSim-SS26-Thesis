/**
 * @file Writer.cpp
 */

#include "Writer.h"
#include "XYZWriter.h"
#include "YAMLWriter.h"

#ifdef ENABLE_VTK_SUPPORT
#include "VTKWriter.h"

#include <vtkCellArray.h>
#include <vtkDoubleArray.h>
#include <vtkFloatArray.h>
#include <vtkIntArray.h>
#include <vtkPointData.h>
#include <vtkXMLUnstructuredGridWriter.h>

#include <iomanip>
#include <sstream>
#endif

#include <filesystem>

std::unique_ptr<Writer> Writer::create(const std::filesystem::path &file_path, const Simulation<DirectSum> &simulation)
{
    std::string extension = file_path.extension().string();

    if (extension == ".yml" || extension == ".yaml")
    {
        return std::make_unique<YAMLWriter>(file_path, simulation);
    }

    if (extension == ".xyz")
    {
        return std::make_unique<XYZWriter>(file_path, simulation);
    }

    #ifdef ENABLE_VTK_SUPPORT
    if (extension == ".vtk" || extension == ".vtu")
    {
        return std::make_unique<VTKWriter>(file_path, simulation);
    }
    #endif

    std::cerr << "Error: Unsupported file extension: " << extension << "\n";
    exit(1);
}
