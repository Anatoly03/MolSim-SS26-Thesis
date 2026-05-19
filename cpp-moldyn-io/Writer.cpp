/**
 * @file Writer.cpp
 */

#include "Writer.h"
#include "YAMLWriter.h"

#include <filesystem>

std::unique_ptr<Writer> Writer::create(const std::filesystem::path &file_path, const Simulation<DirectSum> &simulation)
{
    std::string extension = file_path.extension().string();

    if (extension == ".yml" || extension == ".yaml")
    {
        return std::make_unique<YAMLWriter>(file_path, simulation);
    }

    std::cerr << "Error: Unsupported file extension: " << extension << "\n";
    exit(1);
}
