/**
 * @file YAMLReader.h
 * @author Anatoly Weinstein
 *
 * @brief
 */

#pragma once

#include <fstream>
#include <iostream>
#include <memory>
#include <string>

#include "Reader.h"

struct YAMLReader : public Reader
{
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
    YAMLReader(std::string file_path) : Reader(file_path) {}

    // TODO Reader.consume() -> Simulation
};
