/**
 * @file DirectSum.h
 */

#pragma once

#include <vector>

#include "Particle.h"
#include "ParticleContainer.h"

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
class DirectSum : public ParticleContainer
{
private:
    std::vector<Particle> particles_vec;

public:
    DirectSum(const std::vector<Particle> &particles) : particles_vec(particles) {}

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

    void for_each_particle_pairs(std::function<void(const std::pair<Particle&, Particle&>)> callback) const override
    {
        for (size_t i = 0; i < particles_vec.size(); ++i)
        {
            for (size_t j = i + 1; j < particles_vec.size(); ++j)
            {
                auto p1 = particles_vec[i];
                auto p2 = particles_vec[j];
                callback(std::make_pair(p1, p2));
            }
        }
    }

    void for_each_particle_pairs_mut(std::function<void(std::pair<Particle&, Particle&>)> callback) override
    {
        for (size_t i = 0; i < particles_vec.size(); ++i)
        {
            for (size_t j = i + 1; j < particles_vec.size(); ++j)
            {
                auto p1 = particles_vec[i];
                auto p2 = particles_vec[j];
                callback(std::make_pair(p1, p2));
            }
        }
    }

    size_t size() const override
    {
        return particles_vec.size();
    }
};
