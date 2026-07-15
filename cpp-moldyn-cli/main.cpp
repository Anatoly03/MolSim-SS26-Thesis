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
#include "YAMLReader.h"
#include "YAMLWriter.h"

/**
 * @brief Entry point for the application.
 */
int main(int argc, char *argv[])
{
    std::filesystem::path input_file_path;
    std::filesystem::path output_file_path = "./output/out.xyz";
    double start_time = 0.0;
    std::optional<double> delta_time;
    std::optional<double> total_time;
    std::optional<int> frame_period;

    // Parses the command line arguments and sets the input variables.
    Args()
        .required_details(&input_file_path, "The input file for the simulation. The parser will be selected from the file extension.")
        .required_details('o', "output", &output_file_path, "The output directory for the simulation results.")
        .optional_details('d', "delta-time", &delta_time, "The time step for the simulation.")
        .optional_details('t', "total-time", &total_time, "The total time for the simulation to run.")
        .optional_details('s', "frame-period", &frame_period, "The period (in frames) for writing the simulation output. This defines the frequency of output writes. If set to zero, disables output writing (used for benchmarking).")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .version()
        .parse(argc, argv);

    double total_time_v = total_time.value_or(1000.0);
    double delta_time_v = delta_time.value_or(0.0014);
    int frame_period_v = frame_period.value_or(250);

    if (frame_period_v > 0) {
        std::filesystem::create_directories(output_file_path.parent_path());
    }

    YAMLReader reader(input_file_path);
    auto simulation = reader.consume();
    auto writer = Writer::create(output_file_path, simulation);

    // std::cout << "Simulation Algorithm: " << simulation.particle_container->algorithm_name() << std::endl;
    // std::cout << "Force System: " << simulation.force->system_name() << std::endl;

    // current clock monotonic time
    // https://en.cppreference.com/cpp/chrono/steady_clock
    auto current_monotonic_time = std::chrono::steady_clock::now();

    double current_time = start_time;
    int frame = 0;

    while (current_time < total_time_v)
    {
        bool print_frame = frame_period_v > 0 && frame % frame_period_v == 0;

        simulation.step(delta_time_v);

        if (print_frame) {
            writer->write();
        }

        current_time += delta_time_v;
        frame++;
    }

    auto end_time = std::chrono::steady_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - current_monotonic_time);
    std::cout << "Simulation completed in " << duration.count() << " ms" << std::endl;

    return 0;
}
