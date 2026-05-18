/**
 * @file DirectSum.h
 */

#pragma once

#include <vector>

#include "Particle.h"
#include "ParticleContainer.h"

struct DirectSumPairIterator
{
private:
    const std::vector<Particle> &particles;
    size_t i, j;

public:
    DirectSumPairIterator(const std::vector<Particle> &particles) : particles(particles), i(0), j(1) {}

    bool operator!=(const DirectSumPairIterator &other) const
    {
        return i != other.i || j != other.j;
    }

    std::pair<const Particle, const Particle> operator*() const
    {
        return {particles[i], particles[j]};
    }
}

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
class DirectSum : public ParticleContainer
{
private:
    std::vector<Particle> particles_vec;

public:
    /**
     * @brief Iterates over each particle.
     *
     * # Example
     *
     * ```cpp
     * DirectSum sum;
     *
     * for (Particle &p : sum.particles())
     * {
     *     // do something with p
     * }
     * ```
     */
    virtual particle_iterator &particles()
    {
        return particles_vec;
    }

    /**
     * @brief Iterates over each particle.
     *
     * # Example
     *
     * ```cpp
     * DirectSum sum;
     *
     * for (Particle &p : sum.particles_mut())
     * {
     *     p.apply_force(Vec3<double>(1.0, 0.0, 0.0));
     * }
     * ```
     */
    virtual particle_mut_iterator &particles_mut()
    {
        return particles_vec;
    }

    /**
     * @brief Iterates over each pair of particles.
     *
     * # Example
     *
     * ```cpp
     * DirectSum sum;
     *
     * for (std::pair<Particle, Particle> &pp : sum.particle_pairs())
     * {
     *     Particle &p1 = pp.first;
     *     Particle &p2 = pp.second;
     *
     *     // do something with p1 and p2
     * }
     * ```
     */
    virtual particle_pair_iterator &particle_pairs() {
        return DirectSumPairIterator(particles_vec);
    }

    /**
     * @brief Iterates over each pair of particles.
     *
     * # Example
     *
     * ```cpp
     * DirectSum sum;
     * Force force;
     *
     * for (std::pair<Particle, Particle> &pp : sum.particle_pairs())
     * {
     *     Particle &p1 = pp.first;
     *     Particle &p2 = pp.second;
     *
     *     force.apply(p1, p2);
     * }
     * ```
     */
    virtual particle_pair_mut_iterator &particle_pairs_mut() {
        return DirectSumPairIterator(particles_vec);
    }

    /**
     * Amount of particles in the container.
     */
    virtual size_t size() const = 0;
};
