/**
 * @file Force.h
 */

#include "Force.h"
#include "Particle.h"
#include "Vec3.h"

/**
 * @brief Calculates the force between two particles equivalent to the
 * negative fraction of potential and distance.
 */
Vec3<double> Force::force(const Particle &particle, const Particle &other) const
{
    auto potenergy = potential(particle, other);
    auto diff = other.position_difference(particle);
    auto dist2 = diff.length2();

    if (dist2 == 0.0)
    {
        return Vec3<double>();
    }
    else
    {
        return -diff * (potenergy / dist2);
    }
}

/**
 * @brief Applies the calculated force to a particle pair, invoking
 * the third law of motion. For a computed force `F` on a particle,
 * the force `-F` is applied to the other particle.
 */
void Force::apply(Particle &particle, Particle &other) const
{
    auto f = force(particle, other);
    particle.apply_force(f);
    other.apply_force(-f);
}

double Force::potential(const Particle &particle, const Particle &other) const
{
    // Default potential: no interaction. Concrete force implementations
    // (e.g., Newton) should override this.
    return 0.0;
}
