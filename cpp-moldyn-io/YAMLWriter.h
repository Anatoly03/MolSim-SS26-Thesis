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

#include "container/DirectSum.h"
#include "container/ParticleContainer.h"
#include "force/Newton.h"
#include "Reader.h"
#include "Simulation.h"
#include "Vec3.h"

struct YAMLWriter
{
private:
    /**
     * @brief The prefix for the output file path.
     */
    std::string prefix;

    /**
     * @brief The simulation to write.
     */
    Simulation<DirectSum> &simulation;

public:
    /**
     * @brief Creates a new YAML file writer.
     */
    YAMLWriter(std::string prefix, Simulation<DirectSum> &simulation) : prefix(prefix), simulation(simulation) {}

    std::string frame_file_path(const int frame) const
    {
        return prefix + "-" + std::to_string(frame) + ".yaml";
    }

    /**
     * @brief Consume the YAML file and return a Simulation struct.
     */
    void write(const int frame) const
    {
        YAML::Node node;
        YAML::Node particles = YAML::Node(YAML::NodeType::Sequence);

        simulation.for_each_particles(
            [&particles](const Particle &particle)
            {
                particles.push_back(particle);
            });

        node["force"] = simulation.force.get()->system_name();
        node["particles"] = particles;

        std::ofstream output_file(frame_file_path(frame));
        output_file << node;
    }
};
