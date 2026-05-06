/**
 * @file ParticleContainer.h
 */

#include "Particle.h"
#include <iterator>

struct ParticleContainer
{
    /**
     * @brief Iterator over Particles.
     * 
     * https://en.cppreference.com/cpp/iterator/iterator
     */
    using iterator = std::iterator<std::input_iterator_tag, Particle>;

    /**
     * @brief Iterator over Particles.
     */
    virtual iterator particles() = 0;
};
