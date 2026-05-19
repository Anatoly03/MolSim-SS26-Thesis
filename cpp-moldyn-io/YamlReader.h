/**
 * @file YamlReader.h
 */

#include "Reader.h"
#include "Simulation.h"
#include "container/DirectSum.h"

struct YamlReader : public Reader
{
public:
    YamlReader(const std::filesystem::path &filename) : Reader(filename) {}

    Simulation<DirectSum> consume() override;
};
