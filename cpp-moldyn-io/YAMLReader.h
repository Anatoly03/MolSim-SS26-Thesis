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
#include <sstream>
#include <string>

#include "yaml-cpp/yaml.h"

#include "YAMLImplementation.h"
#include "container/DirectSum.h"
#include "container/ParticleContainer.h"
#include "force/Newton.h"
#include "Reader.h"
#include "Simulation.h"
#include "Vec3.h"

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

    /**
     * @brief Consume the YAML file and return a Simulation struct.
     */
    Simulation<DirectSum> consume() override
    {
        std::stringstream buffer;
        buffer << input_file->rdbuf();
        YAML::Node config = YAML::Load(buffer.str());

        Simulation<DirectSum> sim;

        // TODO parse different algorithms
        // config["algorithm"]
        
        // Simulation force
        sim.force = config["force"] ? config["force"].as<std::unique_ptr<Force>>() : std::make_unique<Newton>();

        // Parse simulations args
        // TODO
        // YAML::Node args = config["args"];
        // if (args) {
        //     if (args["delta_time"])
        //     {
        //         sim.delta_time = args["delta_time"].as<double>();
        //     }

        //     if (args["total_time"])
        //     {
        //         sim.total_time = args["total_time"].as<double>();
        //     }

        //     if (args["frame_period"])
        //     {
        //         sim.frame_period = args["frame_period"].as<int>();
        //     }
        // }

        // Simulation particles
        YAML::Node particles = config["particles"];
        if (particles && particles.IsSequence())
        {
            for (const auto &particle_node : particles)
            {
                Particle particle = particle_node.as<Particle>();
                sim.add_particle(particle);
            }
        }

        return sim;
    }
};
