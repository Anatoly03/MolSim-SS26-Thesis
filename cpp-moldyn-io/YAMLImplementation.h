/**
 * @file YAMLImplementation.h
 * @author Anatoly Weinstein
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

namespace YAML
{
    template <typename VecType>
    struct convert<Vec3<VecType>>
    {
        static Node encode(const Vec3<VecType> &rhs)
        {
            Node node;
            node.push_back(rhs.x);
            node.push_back(rhs.y);
            node.push_back(rhs.z);
            return node;
        }

        static bool decode(const Node &node, Vec3<VecType> &rhs)
        {
            if (node.IsMap())
            {
                rhs.x = node["x"] ? node["x"].as<VecType>() : VecType(0);
                rhs.y = node["y"] ? node["y"].as<VecType>() : VecType(0);
                rhs.z = node["z"] ? node["z"].as<VecType>() : VecType(0);
                return true;
            }

            if (node.IsSequence() || node.size() != 3)
            {
                rhs.x = node.size() > 0 ? node[0].as<VecType>() : VecType(0);
                rhs.y = node.size() > 1 ? node[1].as<VecType>() : VecType(0);
                rhs.z = node.size() > 2 ? node[2].as<VecType>() : VecType(0);
                return true;
            }

            return false;
        }
    };

    template <>
    struct convert<Particle>
    {
        static Node encode(const Particle &rhs)
        {
            Node node;
            node["position"] = rhs.get_position();
            node["velocity"] = rhs.get_velocity();
            node["mass"] = rhs.get_mass();
            node["force"] = rhs.get_force();
            return node;
        }

        static bool decode(const Node &node, Particle &rhs)
        {
            if (!node.IsMap())
            {
                return false;
            }

            Vec3 position = node["position"] ? node["position"].as<Vec3<double>>() : Vec3<double>();
            Vec3 velocity = node["velocity"] ? node["velocity"].as<Vec3<double>>() : Vec3<double>();
            double mass = node["mass"] ? node["mass"].as<double>() : 1.0;

            rhs = Particle(position, velocity, mass);

            return true;
        }
    };

    template <>
    struct convert<std::unique_ptr<Force>>
    {
        /**
         * @brief Returns the name of the force system.
         */
        static Node encode(const std::unique_ptr<Force> &rhs)
        {
            return Node(rhs->system_name());
        }

        static bool decode(const Node &node, std::unique_ptr<Force> &rhs)
        {
            if (node.IsScalar())
            {
                std::string system_name = node.as<std::string>();

                if (system_name == "newton" || system_name == "gravitational")
                {
                    rhs = std::make_unique<Newton>();
                    return true;
                }
            }

            return false;
        }
    };

    template <typename ParticleContainerT>
    struct convert<Simulation<ParticleContainerT>>
    {
        // assert that particle container implements `convert<ParticleContainerT>`
        static_assert(std::is_base_of<convert<ParticleContainerT>, convert<Simulation<ParticleContainerT>>>::value, "Simulation requires a ParticleContainer that implements YAML::convert");

        static Node encode(const Simulation<ParticleContainerT> &rhs)
        {
            Node node;

            node["force"] = rhs.force.get();

            return node;
        }

        static bool decode(const Node &node, Simulation<ParticleContainerT> &rhs)
        {
            rhs.force = node["force"] ? node["force"].as<std::unique_ptr<Force>>() : std::make_unique<Newton>();

            return false;
        }
    };
}
