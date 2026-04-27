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
    std::unique_ptr<std::ifstream> input_file;

    Args()
        .required_details(&input_file, "The input file for the simulation. The parser will be selected from the file extension.")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .version()
        .parse(argc, argv);

    std::cout << "Hello, C++!\n";

    return 0;
}
