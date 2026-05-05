/**
 * @file main.cpp
 * @brief Main entry point for the cpp-moldyn-cli application.
 */

#include <iostream>
#include <memory>

#include "Args.h"
#include <filesystem>

/**
 * @brief Entry point for the application.
 */
int main(int argc, char *argv[])
{
    std::string input_file_path;
    std::filesystem::path output_pattern = "./output/out.vtk";

    Args()
        .required_details(&input_file_path, "The input file for the simulation. The parser will be selected from the file extension.")
        .required_details('o', "output", &output_pattern, "The output directory for the simulation results.")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .version()
        .parse(argc, argv);

    printf("Input file: %s\n", input_file_path.c_str());
    printf("Output directory: %s\n", output_pattern.parent_path().string().c_str());
    printf("Output prefix: %s\n", output_pattern.stem().string().c_str());
    printf("Output extension: %s\n", output_pattern.extension().string().c_str());

    return 0;
}
