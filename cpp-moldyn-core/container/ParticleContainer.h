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
    virtual void for_each_particles(std::function<void(const Particle&)> callback) const = 0;

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
    virtual void for_each_particles_mut(std::function<void(Particle&)> callback) = 0;

    /**
     * @brief Iterates over each pair of particles.
     */
    virtual void for_each_particle_pairs(std::function<void(const std::pair<Particle&, Particle&>)> callback) const = 0;

    /**
     * @brief Iterates over each pair of particles.
     */
    virtual void for_each_particle_pairs_mut(std::function<void(std::pair<Particle&, Particle&>)> callback) = 0;

    /**
     * Amount of particles in the container.
     */
    virtual size_t size() const = 0;
};
