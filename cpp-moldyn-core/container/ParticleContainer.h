/**
 * @file ParticleContainer.h
 */

#pragma once

#include "Particle.h"

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
class ParticleContainer
{
    /**
     * Abstraction of a generic input iterator.
     */
    template <typename T>
    using input_iterator = std::iterator<
        std::input_iterator_tag,
        T,
        std::ptrdiff_t,
        T *,
        T &>;

    /**
     * @brief An iterator type for iterating over particles in the container.
     */
    using particle_iterator = input_iterator<const Particle>;

    /**
     * @brief An iterator type for iterating over particles in the container.
     */
    using particle_mut_iterator = input_iterator<Particle>;

    /**
     * @brief An iterator type for iterating over pairs of particles in
     * the container.
     */
    using particle_pair_iterator = input_iterator<std::pair<const Particle, const Particle>>;

    /**
     * @brief An iterator type for iterating over pairs of particles in
     * the container.
     */
    using particle_pair_mut_iterator = input_iterator<std::pair<Particle, Particle>>;

public:
    /**
     * @brief Iterates over each particle.
     *
     * # Example
     *
     * ```cpp
     * ParticleContainer container;
     *
     * for (Particle &p : container.particles())
     * {
     *     // do something with p
     * }
     * ```
     */
    virtual particle_iterator &particles() = 0;

    /**
     * @brief Iterates over each particle.
     *
     * # Example
     *
     * ```cpp
     * ParticleContainer container;
     *
     * for (Particle &p : container.particles_mut())
     * {
     *     p.apply_force(Vec3<double>(1.0, 0.0, 0.0));
     * }
     * ```
     */
    virtual particle_mut_iterator &particles_mut() = 0;

    /**
     * @brief Iterates over each pair of particles.
     *
     * # Example
     *
     * ```cpp
     * ParticleContainer container;
     *
     * for (std::pair<Particle, Particle> &pp : container.particle_pairs())
     * {
     *     Particle &p1 = pp.first;
     *     Particle &p2 = pp.second;
     *
     *     // do something with p1 and p2
     * }
     * ```
     */
    virtual particle_pair_iterator &particle_pairs() = 0;

    /**
     * @brief Iterates over each pair of particles.
     *
     * # Example
     *
     * ```cpp
     * ParticleContainer container;
     * Force force;
     *
     * for (std::pair<Particle, Particle> &pp : container.particle_pairs())
     * {
     *     Particle &p1 = pp.first;
     *     Particle &p2 = pp.second;
     *
     *     force.apply(p1, p2);
     * }
     * ```
     */
    virtual particle_pair_mut_iterator &particle_pairs_mut() = 0;

    /**
     * Amount of particles in the container.
     */
    virtual size_t size() const = 0;
};
