/**
 * @file Writer.h
 * @author Anatoly Weinstein
 *
 * @brief
 */

#pragma once

#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <string>
#include "Simulation.h"
#include "container/DirectSum.h"

class Writer
{
private:
    std::filesystem::path output_prefix;

protected:
    const Simulation<DirectSum> &simulation;

public:
    /**
     * @brief The output file extension, including the dot (e.g., `.yaml`).
     */
    std::string output_extension;

    /**
     * TODO document
     *
     * @param output_file_path Path to the output file, including file name prefix and
     * extension.
     */
    Writer(const std::filesystem::path output_file_path, const Simulation<DirectSum> &simulation) : simulation(simulation)
    {
        // save extension
        output_extension = output_file_path.extension().string();

        // trim extension
        output_prefix = output_file_path;
        output_prefix.replace_extension("");
    }

    /**
     * @brief Consumes the file stream and returns a Simulation struct.
     */
    virtual void write(const int frame) const = 0;

    /**
     * @brief Generates the file path for a given frame number based on the output
     * prefix and extension.
     */
    std::string frame_file_path(const int frame) const
    {
        return output_prefix.string() + "-" + std::to_string(frame) + output_extension;
    }

    /**
     * @brief Static factory method to create a Reader instance based on the
     * file etxension.
     */
    static std::unique_ptr<Writer> create(const std::filesystem::path &file_path, const Simulation<DirectSum> &simulation);
};
