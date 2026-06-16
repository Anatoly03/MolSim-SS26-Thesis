/**
 * @file Particle.h
 * @brief Definition of the Vec3 particle class for three-dimensional vector operations.
 */

#pragma once

#include <memory>
#include <type_traits>

#include "force/Force.h"
#include "force/LennardJones.h"
#include "container/ParticleContainer.h"
#include "container/DirectSum.h"

/**
 * @brief A struct representing a simulation, which contains the particles and
 * the logic for updating their states over time.
 */
struct Simulation
{
public:
    /**
     * @brief The container which holds the particles in the simulation.
     */
    std::unique_ptr<ParticleContainer> particle_container;

    /**
     * @brief The force method which calculates the forces between particles in the simulation.
     */
    std::unique_ptr<Force> force;

    /**
     * @brief Default constructor.
     */
    Simulation() : particle_container(std::make_unique<DirectSum>()), force(std::make_unique<LennardJones>()) {};

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
    void for_each_particles(std::function<void(const Particle &)> callback) const
    {
        particle_container->for_each_particles(callback);
    }

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
    void for_each_particles_mut(std::function<void(Particle &)> callback)
    {
        particle_container->for_each_particles_mut(callback);
    }

    /**
     * @brief Iterates over each pair of particles.
     */
    void for_each_particle_pairs(std::function<void(const Particle &, const Particle &)> callback) const
    {
        particle_container->for_each_particle_pairs(callback);
    }

    /**
     * @brief Iterates over each pair of particles.
     */
    void for_each_particle_pairs_mut(std::function<void(Particle &, Particle &)> callback)
    {
        particle_container->for_each_particle_pairs_mut(callback);
    }

    /**
     * @brief Adds a new particle to the container by cloning.
     */
    void add_particle(const Particle &particle)
    {
        particle_container->add_particle(particle);
    }

    /**
     * Amount of particles in the container.
     */
    size_t size() const
    {
        return particle_container->size();
    }

    /**
     * Updates the position of every particle.
     */
    void update_position(const double delta_time)
    {
        for_each_particles_mut(
            [delta_time](Particle &particle)
            { particle.update_position(delta_time); });
    }

    /**
     * Delays the force.
     */
    void delay_force()
    {
        for_each_particles_mut(
            [](Particle &particle)
            { particle.delay_force(); });
    }

    /**
     * Applies the force to every particle.
     */
    void apply_force()
    {
        for_each_particle_pairs_mut(
            [this](Particle &p1, Particle &p2)
            {
                this->force->apply(p1, p2);
            });
    }

    /**
     * Updates the velocity of every particle.
     */
    void update_velocity(const double delta_time)
    {
        for_each_particles_mut(
            [delta_time](Particle &particle)
            {
                particle.update_velocity(delta_time);
            });
    }

    /**
     * TODO document (see rust)
     */
    void step(const double delta_t)
    {
        update_position(delta_t);
        // self.container.on_after_position_update();
        delay_force();
        apply_force();
        // APPLY GRAVITY HERE
        // self.container.on_after_force_update();
        // TODO CALCULATE BORDER BEHAVIOUR in `on_after_force_update`
        update_velocity(delta_t);
        // self.container.on_after_velocity_update();
        // TODO UPDATE CURRENT TIME += DELTA TIME
    }

    // TODO PLOT PARTICLES

    std::string algorithm_name() const {
        return particle_container->algorithm_name();
    }
};
