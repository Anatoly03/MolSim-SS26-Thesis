#!/bin/bash

# This script sets up the prerequisites for the project, including
# the required toolchains and dependencies.

# 1. Installs Rust toolchain if not already installed
if ! command -v rustc &> /dev/null
then
    # https://rust-lang.org/tools/install/
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    rustc --version
    cargo --version
fi

# 2. Installs CMake if not already installed
if ! command -v cmake &> /dev/null
then
    # https://cmake.org/download/
    echo "CMake is not installed. Installing CMake..."

    # Linux GNU / Ubuntu
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update
        sudo apt-get install -y cmake
    # Mac OS (assume Homebrew is installed)
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install cmake
    else
        echo "Error! Install CMake manually: https://cmake.org/download/"
        exit 1
    fi
else
    cmake --version | head -n 1
fi

# 3. Installs Tracy



# TODO: install gcc, clang, etc.
# TODO: remember macbook uses g++ and `xcode-select``

# TODO: doxygen

# TODO: vtk

# TODO: clang-format

# TODO: clang-tidy

# TODO: google test
# https://github.com/Anatoly03/MolSim-WS25-GroupA/blob/assignment5/cmake/modules/test-google.cmake

# TODO: google benchmark or some other C++ benchmarking library
# TODO: research if some Rust/C++ benchmark library exists that is trusted

# TODO: note: remember that yaml and some other lirbaries were added to cmake and they need custom installation too.
# https://github.com/fmtlib/fmt.git ||| https://github.com/Anatoly03/MolSim-WS25-GroupA/blob/assignment5/cmake/modules/add-fmt.cmake
# https://github.com/gabime/spdlog.git ||| https://github.com/Anatoly03/MolSim-WS25-GroupA/blob/assignment5/cmake/modules/add-spdlog.cmake
# https://github.com/jbeder/yaml-cpp ||| https://github.com/Anatoly03/MolSim-WS25-GroupA/blob/assignment5/cmake/modules/add-yaml.cmake

