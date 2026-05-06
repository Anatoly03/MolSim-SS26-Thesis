/**
 * @file YamlReader.h
 */

#include "Reader.h"

struct YamlReader : public Reader
{
public:
    YamlReader(const std::string &filename) : Reader(filename) {}

    Simulation consume() override;
};
