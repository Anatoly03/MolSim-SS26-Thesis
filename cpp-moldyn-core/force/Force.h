/**
 * @file Force.h
 */

#pragma once

#include <string>

#include "Particle.h"
#include "Vec3.h"

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
class Force
{
public:
    /**
     * Name of the force system.
     */
    virtual std::string system_name() const = 0;

    /**
     * @brief Calculates the potential energy between two particles.
     * 
     * ```text
     * potential = -G * M / r
     * potential = -M / r          (assuming G = 1)
     * ```
     */
    virtual double potential(const Particle &particle, const Particle &other) const;

    // @brief Calculates the force between two particles. Specifically it
    // computes the term `-U / r`.
    Vec3<double> force(const Particle &particle, const Particle &other) const;

    // @brief Applies the calculated force to a particle pair.
    void apply(Particle &particle, Particle other) const;
};
