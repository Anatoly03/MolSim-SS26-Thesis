/**
 * @file Reader.h
 * @author Anatoly Weinstein
 *
 * @brief
 */

#pragma once

#include "simulation/Simulation.h"

#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <string>

class Reader
{
private:
    /**
     * @brief The input file stream for reading particle data. The parser
     * will be selected from the file extension.
     */
    std::unique_ptr<std::ifstream> input_file;

    // TODO create simulation struct
    // when reading file, write here

public:
    /**
     * @brief Initializes a file stream for a given file path.
     * 
     * @param file_path The path to the input file for the simulation. The
     * parser will be selected from the file extension.
     * 
     * @note This constructor does not consume the file stream. If the file
     * stream could not be opened, the program will be terminated with an
     * error message.
     * 
     * @example
     * 
     * Here is an example of how to construct the Reader struct.
     * 
     * ```cpp
     * Reader reader("input.yaml");
     * ```
     */
    Reader(std::filesystem::path file_path) {
        std::unique_ptr<std::ifstream> pointer = std::make_unique<std::ifstream>(file_path);

        if (!pointer.get()->is_open())
        {
            std::cerr << "Error: Could not open file: " << file_path << "\n";
            exit(1);
        }

        input_file = std::move(pointer);
    }

    /**
     * @brief Closes the file stream if it is open upon Reader destructor
     * invocation.
     */
    ~Reader() {
        if (input_file.get()->is_open())
        {
            input_file.get()->close();
        }
    }

    /**
     * @brief Consumes the file stream and produces a Simulation struct from
     * the input data.
     */
    virtual Simulation consume() = 0;

    /**
     * @brief Static factory method to create a Reader instance based on the
     * file etxension.
     */
    static std::unique_ptr<Reader> create(const std::filesystem::path &file_path);
};
