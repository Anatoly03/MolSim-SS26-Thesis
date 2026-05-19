/**
 * @file Particle.h
 * @brief Definition of the Vec3 particle class for three-dimensional vector operations.
 */

#pragma once

#include "Force.h"
#include "ParticleContainer.h"

/**
 * @brief A struct representing a simulation, which contains the particles and
 * the logic for updating their states over time.
 */
template <typename ParticleContainerT>
struct Simulation
{
    // assert that ParticleContainerT is a ParticleContainer
    static_assert(std::is_base_of<ParticleContainer, ParticleContainerT>::value, "ParticleContainerT must extend ParticleContainer");

private:
    /**
     * @brief The container which holds the particles in the simulation.
     */
    ParticleContainerT particle_container;

public:
    /**
     * @brief The force method which calculates the forces between particles in the simulation.
     */
    std::unique_ptr<Force> force;

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
        particle_container.for_each_particles(callback);
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
        particle_container.for_each_particles_mut(callback);
    }

    /**
     * @brief Iterates over each pair of particles.
     */
    void for_each_particle_pairs(std::function<void(const Particle &, const Particle &)> callback) const
    {
        particle_container.for_each_particle_pairs(callback);
    }

    /**
     * @brief Iterates over each pair of particles.
     */
    void for_each_particle_pairs_mut(std::function<void(Particle &, Particle &)> callback)
    {
        particle_container.for_each_particle_pairs_mut(callback);
    }

    /**
     * @brief Adds a new particle to the container by cloning.
     */
    void add_particle(const Particle &particle)
    {
        particle_container.add_particle(particle);
    }

    /**
     * Amount of particles in the container.
     */
    size_t size() const
    {
        return particle_container.size();
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
        for_each_particle_pairs(
            [this](const Particle &p1, const Particle &p2)
            {
            Vec3<double> force = this->force->calculate(p1, p2);
            p1.apply_force(force);
            p2.apply_force(-force); });
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

    void step(const double delta_t) {
        update_position(delta_t);
        // self.container.on_after_position_update();
        delay_force();
        update_force();
        // APPLY GRAVITY HERE
        // self.container.on_after_force_update();
        // TODO CALCULATE BORDER BEHAVIOUR in `on_after_force_update`
        update_velocity(delta_t);
        // self.container.on_after_velocity_update();
        // TODO UPDATE CURRENT TIME += DELTA TIME
    }

    // TODO PLOT PARTICLES
};
