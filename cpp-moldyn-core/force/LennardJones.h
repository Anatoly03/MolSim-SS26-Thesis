/**
 * @file Newton.h
 */

#pragma once

#include <string>

#include "Force.h"
#include "Newton.h"
#include "Particle.h"

/**
 * @brief The Force representing a Lennard-Jones force, which implements
 * the [Force] trait.
 */
class LennardJones : public Force
{
public:
    // @brief the
    double factor = 1.0;

    /**
     * @brief Returns the name of the force system.
     */
    std::string system_name() const override
    {
        return "newton";
    }

    // @brief Calculates the Newtons' potential between two particles. The formula
    // is given by `U = -G * M / r` where `G = -1` is the gravitational constant and
    // `M` the product of particle masses.
    double potential(const Particle &particle, const Particle &other) const override
    {
        double epsilon = 5.0;
        double sigma = 1.0;
        double cutoff_radius = 3.0;
        double distance;

        if (distance = particle.distance(other) && distance < cutoff_radius)
        {
            auto frac = sigma / distance;
            auto frac6 = pow(frac, 6);
            auto frac12 = pow(frac6, 2);

            return 4.0 * epsilon * (frac12 - frac6);
        }

        return 0.0;
    }
};
