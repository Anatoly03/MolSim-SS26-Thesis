/**
 * @file ParticleContainer.h
 */

#pragma once

#include <functional>
#include "Particle.h"

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
class ParticleContainer
{
public:
    /**
     * @brief Default constructor.
     */
    ParticleContainer() = default;

    /**
     * Destructor is virtual to allow for proper cleanup of derived classes.
     */
    virtual ~ParticleContainer() = default;

    /**
     * @brief Returns the name (or identifier) of the algorithm used for serialization.
     */
    virtual std::string algorithm_name() const
    {
        return "unknown";
    }

    /**
     * @brief Iterates over each particle.
     *
     * # Example
     *
     * ```cpp
     * ParticleContainer container;
     *
     * container.for_each_particles([](const Particle &particle) {
     *     // do something with particle
     * });
     * ```
     */
    virtual void for_each_particles(std::function<void(const Particle &)> callback) const = 0;

    /**
     * @brief Iterates over each particle mutably.
     *
     * # Example
     *
     * ```cpp
     * ParticleContainer container;
     *
     * container.for_each_particles([](const Particle &particle) {
     *     // do something with particle mutably
     * });
     * ```
     */
    virtual void for_each_particles_mut(std::function<void(Particle &)> callback) = 0;

    /**
     * @brief Iterates over each pair of particles.
     */
    virtual void for_each_particle_pairs(std::function<void(const Particle &, const Particle &)> callback) const = 0;

    /**
     * @brief Iterates over each pair of particles.
     */
    virtual void for_each_particle_pairs_mut(std::function<void(Particle &, const Particle &)> callback) = 0;

    /**
     * @brief Adds a new particle to the container by cloning.
     */
    virtual void add_particle(const Particle &particle) = 0;

    /**
     * Amount of particles in the container.
     */
    virtual size_t size() const = 0;
};
