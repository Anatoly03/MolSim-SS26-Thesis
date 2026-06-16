#include <gtest/gtest.h>
#include "Vec3.h"
#include "Simulation.h"
#include "container/DirectSum.h"
#include "container/LinkedCells.h"
#include "Particle.h"

/**
 * @brief Tests equivalence between `direct-sum` and `linked-cells` algorithms.
 */
TEST(DirectCellsTest, EquivalenceAssertions) {
    Simulation simsum;
    Simulation simcells;

    simsum.particle_container = std::make_unique<DirectSum>();
    simcells.particle_container = std::make_unique<LinkedCells<DirectSum>>();

    Particle p1(Vec3(0.0), Vec3(0.01), 1.0);
    Particle p2(Vec3(6.0), Vec3(-0.01), 1.0);

    simsum.add_particle(p1);
    simsum.add_particle(p2);
    simcells.add_particle(p1);
    simcells.add_particle(p2);

    EXPECT_EQ(simsum.particle_container->size(), 2);
    EXPECT_EQ(simcells.particle_container->size(), 2);

    // TODO more tests
}
