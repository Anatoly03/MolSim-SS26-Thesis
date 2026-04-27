/**
 * @file Particle.h
 * @brief Definition of the Vec3 particle class for three-dimensional vector operations.
 */

#pragma once

#include <optional>

#include "Vec3.h"

struct Particle
{
private:
    /**
     * @brief The position of the particle in three-dimensional space.
     */
    Vec3<double> position;

    /**
     * @brief The velocity of the particle in three-dimensional space.
     */
    Vec3<double> velocity;

    /**
     * @brief The force of the particle in three-dimensional space.
     */
    Vec3<double> force;

    /**
     * @brief Force which was effective on the particle in the previous
     * time step.
     */
    Vec3<double> old_force;

    /**
     * @brief The mass of the particle.
     */
    double mass;

public:
    /**
     * @brief Initializes a particle with default values.
     */
    Particle() : position(), velocity(), force(), old_force(), mass(1.0) {}

    /**
     * @brief Particle constructor from position, initial velocity and mass.
     */
    Particle(const Vec3<double> position, const Vec3<double> velocity, double mass)
        : position(position.clone()), velocity(velocity.clone()), force(), old_force(), mass(mass) {}

    // remove implicit copy constructors
    // https://stackoverflow.com/questions/33776697/deleting-copy-constructors-and-copy-assignment-operators-which-of-them-are-esse
    Particle(const Particle &) = delete;
    Particle &operator=(const Particle &) = delete;
    inline Particle clone() const { return Particle(position.clone(), velocity.clone(), mass); }

    /**
     * @brief Returns the current position of the particle.
     */
    inline const Vec3<double> &get_position() const { return position; }

    /**
     * @brief Returns the current velocity of the particle.
     */
    inline const Vec3<double> &velocity() const { return velocity; }

    /**
     * @brief Returns the current force of the particle.
     */
    inline const Vec3<double> &get_force() const { return force; }

    /**
     * @brief Returns the constant mass of the particle.
     */
    inline const double &get_mass() const { return mass; }

    /**
     * @brief Propagates the current force to the old force. This has to be
     * called every time step before invoking [Particle::apply_force] to apply
     * new forces.
     */
    inline void delay_force()
    {
        old_force = force.clone();
        force = Vec3<double>();
    }

    /**
     * @brief Applies the given force to the particle (addition). It assumes that
     * the force was reset with [Particle::delay_force] in a timestep.
     */
    inline void apply_force(const Vec3<double> &new_force) {
        force += new_force;
    }

    /**
     * @brief Calculate the updated position of the particle given a delta time
     * step. This functionality is constant across different simulation algorithms,
     * so it is implemented here.
     */
    inline void update_position(const double delta_time) {
        position += velocity * delta_time + force * (delta_time * delta_time / (2.0 * mass));
    }

    /**
     * @brief Calculate the updated velocity of the particle given a delta time
     * step. This functionality is constant across different simulation algorithms,
     * so it is implemented here.
     */
    inline void update_velocity(const double delta_time) {
        velocity += (force + old_force) * (delta_time / (2.0 * mass));
    }

    /**
     * @brief Calculate the vector difference between two particles' positions. Note
     * that the order of the particles affects the sign.
     * 
     * - `direction(a, b) == -direction(b, a)`.
     */
    inline Vec3<double> position_difference(const Particle &other) const {
        return position - other.position;
    }

    /**
     * @brief Calculate the normalized vector difference between two particles'
     * positions. Note that the order of the particles affects the sign.
     * 
     * - If result is `Some`: `direction(a, b) == -direction(b, a)`.
     * - If result is `None`: `direction(a, b) == direction(b, a) == None`.
     */
    inline std::optional<Vec3<double>> direction(const Particle &other) const {
        return position_difference(other).normal();
    }

    /**
     * @brief Calculate the distance between two particles' positions. This
     * function is symmetric:
     * 
     * - `distance(a, b) == distance(b, a)`.
     */
    inline double distance(const Particle &other) const {
        return position_difference(other).length();
    }

    /**
     * @brief Calculate the product of the masses of two particles.
     */
    inline double mass_product(const Particle &other) const {
        return mass * other.mass;
    }
};
