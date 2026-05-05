/**
 * @file main.cpp
 * @brief Main entry point for the cpp-moldyn-cli application.
 */

#include <iostream>
#include <memory>

#include "Args.h"

/**
 * @brief Entry point for the application.
 */
int main(int argc, char *argv[])
{
    std::string input_file_path;
    std::string output_directory = "./output";

    Args()
        .required_details(&input_file_path, "The input file for the simulation. The parser will be selected from the file extension.")
        .required_details('o', "output", &output_directory, "The output directory for the simulation results.")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .version()
        .parse(argc, argv);

    std::cout << "Input file: " << input_file_path << std::endl;
    std::cout << "Output directory: " << output_directory << std::endl;

    return 0;
}
