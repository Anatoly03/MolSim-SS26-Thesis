/**
 * @file Reader.cpp
 */

#include "Reader.h"
#include "YamlReader.h"

#include <filesystem>

std::unique_ptr<Reader> Reader::create(const std::filesystem::path &file_path)
{
    std::string extension = file_path.extension().string();

    if (extension == ".yml" || extension == ".yaml")
    {
        return std::make_unique<YamlReader>(file_path);
    }

    std::cerr << "Error: Unsupported file extension: " << extension << "\n";
    exit(1);
}
