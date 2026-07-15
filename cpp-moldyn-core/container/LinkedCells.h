/**
 * @file LinkedCells.h
 */

#pragma once

#include <iterator>
#include <map>
#include <vector>
#include <type_traits>
#include <cmath>

#include "Particle.h"
#include "ParticleContainer.h"
#include "Vec3.h"

/**
 * @brief [Interface](https://www.tutorialspoint.com/cplusplus/cpp_interfaces.htm)
 * representing a force system.
 */
template <typename ParticleContainerT>
class LinkedCells : public ParticleContainer
{
    // assert that ParticleContainerT is a ParticleContainer
    static_assert(std::is_base_of<ParticleContainer, ParticleContainerT>::value, "ParticleContainerT must extend ParticleContainer");
    static_assert(std::is_default_constructible<ParticleContainerT>::value, "ParticleContainerT must be default-constructible");

public:
    /**
     * @brief The size of the cells in the linked cells algorithm.
     */
    Vec3<double> cell_size;

private:
    /**
     * @brief Hashmap of cell chunks
     */
    std::map<Vec3<int>, ParticleContainerT> particle_containers_chunk;

public:
    /**
     * @brief Default constructor.
     */
    LinkedCells() : cell_size(5.0, 5.0, 5.0) {};

    /**
     * @brief Returns the identifier `linked-cells` for this algorithm.
     */
    virtual std::string algorithm_name() const override
    {
        return "linked-cells";
    }

    // /**
    //  * @brief Constructor from a vector of particles.
    //  */
    // LinkedCells(const std::vector<Particle> particles) : particles_vec(particles) {}

    void for_each_particles(std::function<void(const Particle &)> callback) const override
    {
        for (const auto &chunk : particle_containers_chunk)
        {
            chunk.second.for_each_particles(callback);
        }
    }

    void for_each_particles_mut(std::function<void(Particle &)> callback) override
    {
        for (auto &chunk : particle_containers_chunk)
        {
            chunk.second.for_each_particles_mut(callback);
        }
    }

    void for_each_particle_pairs(std::function<void(const Particle &, const Particle &)> callback) const override
    {
        std::vector<Vec3<int>> coords;
        coords.reserve(particle_containers_chunk.size());

        for (const auto &chunk : particle_containers_chunk)
        {
            coords.push_back(chunk.first);
        }

        // Visit each neighboring cell pair only once using positive half-space offsets.
        for (const auto &cell_coords : coords)
        {
            auto cell_it = particle_containers_chunk.find(cell_coords);
            if (cell_it == particle_containers_chunk.end())
            {
                continue;
            }

            for (int dx = -1; dx <= 1; ++dx)
            {
                for (int dy = -1; dy <= 1; ++dy)
                {
                    for (int dz = -1; dz <= 1; ++dz)
                    {
                        // Skip same-cell interactions here and avoid mirrored duplicates.
                        if (dx < 0 || (dx == 0 && dy < 0) || (dx == 0 && dy == 0 && dz <= 0))
                        {
                            continue;
                        }

                        const Vec3<int> neighbour_coords(
                            cell_coords.x + dx,
                            cell_coords.y + dy,
                            cell_coords.z + dz);

                        const auto neighbour_it = particle_containers_chunk.find(neighbour_coords);
                        if (neighbour_it == particle_containers_chunk.end())
                        {
                            continue;
                        }

                        std::vector<const Particle *> lhs;
                        std::vector<const Particle *> rhs;

                        cell_it->second.for_each_particles([&](const Particle &p) { lhs.push_back(&p); });
                        neighbour_it->second.for_each_particles([&](const Particle &p) { rhs.push_back(&p); });

                        for (const Particle *p1p : lhs)
                        {
                            for (const Particle *p2p : rhs)
                            {
                                callback(*p1p, *p2p);
                            }
                        }
                    }
                }
            }
        }

        // For each cell, invoke the local in-cell particle pairs.
        for (const auto &chunk : particle_containers_chunk)
        {
            const auto &cell = chunk.second;
            std::vector<const Particle *> cell_particles;
            cell.for_each_particles([&](const Particle &p) { cell_particles.push_back(&p); });

            for (size_t i = 0; i < cell_particles.size(); ++i)
            {
                for (size_t j = i + 1; j < cell_particles.size(); ++j)
                {
                    callback(*cell_particles[i], *cell_particles[j]);
                }
            }
        }
    }

    void for_each_particle_pairs_mut(std::function<void(Particle &, const Particle &)> callback) override
    {
        std::vector<Vec3<int>> coords;
        coords.reserve(particle_containers_chunk.size());

        for (const auto &chunk : particle_containers_chunk)
        {
            coords.push_back(chunk.first);
        }

        // code from old molsim repository, Vec3 iteration has been unpacked to loops dx, dy, dz
        // Visit each neighboring cell pair only once using positive half-space offsets.
        for (const auto &cell_coords : coords)
        {
            auto cell_it = particle_containers_chunk.find(cell_coords);
            if (cell_it == particle_containers_chunk.end())
            {
                continue;
            }

            for (int dx = -1; dx <= 1; ++dx)
            {
                for (int dy = -1; dy <= 1; ++dy)
                {
                    for (int dz = -1; dz <= 1; ++dz)
                    {
                        // Skip same-cell interactions here and avoid mirrored duplicates.
                        if (dx < 0 || (dx == 0 && dy < 0) || (dx == 0 && dy == 0 && dz <= 0))
                        {
                            continue;
                        }

                        const Vec3<int> neighbour_coords(
                            cell_coords.x + dx,
                            cell_coords.y + dy,
                            cell_coords.z + dz);

                        const auto neighbour_it = particle_containers_chunk.find(neighbour_coords);
                        if (neighbour_it == particle_containers_chunk.end())
                        {
                            continue;
                        }

                        std::vector<Particle *> lhs_mut;
                        std::vector<Particle *> rhs_mut;

                        cell_it->second.for_each_particles_mut([&](Particle &p) { lhs_mut.push_back(&p); });
                        neighbour_it->second.for_each_particles_mut([&](Particle &p) { rhs_mut.push_back(&p); });

                        for (Particle *p1p : lhs_mut)
                        {
                            for (Particle *p2p : rhs_mut)
                            {
                                callback(*p1p, *p2p);
                                callback(*p2p, *p1p);
                            }
                        }
                    }
                }
            }
        }

        // For each cell, invoke the local in-cell particle pairs.
        for (const auto &chunk : particle_containers_chunk)
        {
            const auto &cell = chunk.second;
            std::vector<Particle *> cell_particles_mut;
            const_cast<ParticleContainerT&>(cell).for_each_particles_mut([&](Particle &p) { cell_particles_mut.push_back(&p); });

            for (size_t i = 0; i < cell_particles_mut.size(); ++i)
            {
                for (size_t j = i + 1; j < cell_particles_mut.size(); ++j)
                {
                    callback(*cell_particles_mut[i], *cell_particles_mut[j]);
                    callback(*cell_particles_mut[j], *cell_particles_mut[i]);
                }
            }
        }
    }

    void add_particle(const Particle &particle) override
    {
        Vec3<double> pos = particle.get_position();
        Vec3<int> chunk_pos(
            static_cast<int>(std::floor(pos.x / cell_size.x)),
            static_cast<int>(std::floor(pos.y / cell_size.y)),
            static_cast<int>(std::floor(pos.z / cell_size.z)));
        particle_containers_chunk[chunk_pos].add_particle(particle);
    }

    size_t size() const override
    {
        size_t total_size = 0;

        for (const auto &chunk : particle_containers_chunk)
        {
            total_size += chunk.second.size();
        }

        return total_size;
    }
};
