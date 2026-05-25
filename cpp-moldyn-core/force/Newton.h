/**
 * @file Newton.h
 */

#pragma once

#include <string>

#include "Force.h"
#include "Newton.h"
#include "Particle.h"
#include "TracyHelper.h"

/**
 * @brief The Force representing a Newton (or Coloumb-like) force, which implements
/// the [Force] trait.
 */
class Newton : public Force
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
        PROFILE_ZONE_NAMED("newton potential");

        if (auto distance = particle.distance(other))
        {
            return -factor * particle.mass_product(other) / distance;
        }

        return 0.0;
    }
};
