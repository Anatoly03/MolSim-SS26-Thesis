/**
 * @file main.cpp
 * @brief Main entry point for the cpp-moldyn-cli application.
 */

#include <iostream>
#include <memory>
#include <string>
#include <optional>

#include "Args.h"
#include <filesystem>
#include "container/DirectSum.h"
#include "YAMLReader.h"
#include "YAMLWriter.h"

/**
 * @brief Entry point for the application.
 */
int main(int argc, char *argv[])
{
    std::filesystem::path input_file_path;
    std::filesystem::path output_pattern = "./output/out.vtk";
    double start_time = 0.0;
    std::optional<double> delta_time = 0.0014;
    std::optional<double> total_time = 1000.0;
    std::optional<int> frame_period = 250;

    // Parses the command line arguments and sets the input variables.
    Args()
        .required_details(&input_file_path, "The input file for the simulation. The parser will be selected from the file extension.")
        .required_details('o', "output", &output_pattern, "The output directory for the simulation results.")
        .optional_details('d', "delta_time", &delta_time, "The time step for the simulation.")
        .optional_details('t', "total_time", &total_time, "The total time for the simulation to run.")
        .optional_details('f', "frame_period", &frame_period, "The period (in frames) for writing the simulation output. This defines the frequency of output writes.")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .version()
        .parse(argc, argv);

    printf("Input file: %s\n", input_file_path.c_str());
    printf("Output directory: %s\n", output_pattern.parent_path().string().c_str());
    printf("Output prefix: %s\n", output_pattern.stem().string().c_str());
    printf("Output extension: %s\n", output_pattern.extension().string().c_str());
    std::filesystem::create_directories(output_pattern.parent_path());

    DirectSum container;
    container.add_particle(Particle());
    std::string output_file_path = "output/out";

    YAMLReader reader(input_file_path);
    auto simulation = reader.consume();
    YAMLWriter writer(output_file_path, simulation);

    double total_time_v = total_time.value_or(1000.0);
    double delta_time_v = delta_time.value_or(0.0014);
    int frame_period_v = frame_period.value_or(250);
    double total_frames = total_time_v / delta_time_v;

    simulation.for_each_particles([](const Particle &particle)
                                  { std::cout << "Particle position: " << particle.get_position() << "\n"; });

    for (int frame = 0; frame < total_frames; frame++)
    {
        simulation.step(delta_time_v);

        if (frame % frame_period_v == 0)
            writer.write(frame);
    }

    simulation.for_each_particles([](const Particle &particle)
                                  { std::cout << "Resulting particle position: " << particle.get_position() << "\n"; });

    return 0;
}
