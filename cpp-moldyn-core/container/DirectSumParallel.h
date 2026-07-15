/**
 * @file DirectSumParallel.h
 */

#pragma once

#include <vector>

#include "Particle.h"
#include "ParticleContainer.h"

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
class DirectSumParallel : public ParticleContainer
{
private:
    std::vector<Particle> particles_vec;

public:
    /**
     * @brief Default constructor.
     */
    DirectSumParallel() : particles_vec() {};

    /**
     * @brief Returns the identifier `direct-sum-parallel` for this algorithm.
     */
    virtual std::string algorithm_name() const override
    {
        return "direct-sum-parallel";
    }

    // /**
    //  * @brief Constructor from a vector of particles.
    //  */
    // DirectSumParallel(const std::vector<Particle> particles) : particles_vec(particles) {}

    void for_each_particles(std::function<void(const Particle &)> callback) const override
    {
        for (const auto &particle : particles_vec)
        {
            callback(particle);
        }
    }

    void for_each_particles_mut(std::function<void(Particle &)> callback) override
    {
        for (auto &particle : particles_vec)
        {
            callback(particle);
        }
    }

    void for_each_particle_pairs(std::function<void(const Particle &, const Particle &)> callback) const override
    {
#pragma omp parallel for
        for (size_t i = 0; i < particles_vec.size(); ++i)
        {
            for (size_t j = 0; j < particles_vec.size(); ++j)
            {
                if (i == j) continue; // skip self-interaction
                const auto& p1 = particles_vec[i];
                const auto& p2 = particles_vec[j];
                callback(p1, p2);
            }
        }
    }

    void for_each_particle_pairs_mut(std::function<void(Particle &, const Particle &)> callback) override
    {
#pragma omp parallel for
        for (size_t i = 0; i < particles_vec.size(); ++i)
        {
            for (size_t j = 0; j < particles_vec.size(); ++j)
            {
                if (i == j) continue; // skip self-interaction
                auto& p1 = particles_vec[i];
                const auto& p2 = particles_vec[j];
                callback(p1, p2);
            }
        }
    }

    void add_particle(const Particle &particle) override
    {
        particles_vec.push_back(particle.clone());
    }

    size_t size() const override
    {
        return particles_vec.size();
    }
};
