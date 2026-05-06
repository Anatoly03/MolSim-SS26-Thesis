cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(MolDyn VERSION 0.0.1 LANGUAGES CXX)

file(GLOB_RECURSE CLI_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-cli/*.cpp")
file(GLOB_RECURSE TEST_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-test/*.cc")
file(GLOB_RECURSE CORE_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-core/*.cpp")
file(GLOB_RECURSE IO_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-io/*.cpp")

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/.cmake)

# Library for the core molecular dynamics simulation logic
add_library(MolSimCore ${CORE_SRC})
target_include_directories(MolSimCore PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-core)

# Library for the file system bindings and input/output operations
add_library(MolSimIO ${IO_SRC})
target_include_directories(MolSimCore PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-io)

# Executable for the command-line interface application
add_executable(MolSim ${CLI_SRC})
target_link_libraries(MolSim PRIVATE MolSimCore)
target_link_libraries(MolSim PRIVATE MolSimIO)
target_include_directories(MolSim PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-cli)
target_compile_definitions(MolSim PRIVATE PROGRAM_VERSION="${CMAKE_PROJECT_VERSION}")

# Executable for the test runner
add_executable(MolSimTest ${TEST_SRC})
target_link_libraries(MolSimTest gtest gtest_main MolSimCore)
target_include_directories(MolSimTest PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-test)

# CMake Dependency Configurations
include(doxygen)
include(test-google)

# https://cmake.org/cmake/help/latest/module/GoogleTest.html
# do not remove. the issue is that gitub ci uses command `ctest --test-dir
# 'target/cpp'` and could not discover tests without this addition. it also
# breaks when added in test-google for some reason.
include(GoogleTest)
gtest_discover_tests(MolSimTest)
