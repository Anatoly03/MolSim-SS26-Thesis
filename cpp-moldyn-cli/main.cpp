/**
 * @file main.cpp
 * @brief Main entry point for the cpp-moldyn-cli application.
 */

#include <iostream>
#include <memory>

#include "Args.h"
#include "container/DirectSum.h"
#include "YAMLReader.h"

/**
 * @brief Entry point for the application.
 */
int main(int argc, char *argv[])
{
    std::string input_file_path;

    // Parses the command line arguments and sets the input variables.
    Args()
        .required_details(&input_file_path, "The input file for the simulation. The parser will be selected from the file extension.")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .version()
        .parse(argc, argv);

    YAMLReader reader(input_file_path);
    auto simulation = reader.consume();

    simulation.for_each_particles([](const Particle &particle) {
        std::cout << "Particle position: " << particle.get_position() << "\n";
    });

    return 0;
}
