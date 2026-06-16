cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(MolDyn VERSION 0.0.1 LANGUAGES CXX)

file(GLOB_RECURSE CLI_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-cli/*.cpp")
file(GLOB_RECURSE TEST_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-test/*.cc")
file(GLOB_RECURSE CORE_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-core/*.cpp")
file(GLOB_RECURSE IO_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-io/*.cpp")

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/.cmake)

# Compiler Flags
# option(ENABLE_YAML_SUPPORT "Enables YAML support" ON)
option(ENABLE_VTK_SUPPORT "Enables VTK support" OFF)
# option(ENABLE_JSON_SUPPORT "Enables JSON support" OFF)
option(PERF "Performance Flag. Set to true if the project is being benchmarked." OFF)

# let ccmake and cmake-gui offer the default build type options
set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug;Release;RelWithDebInfo;MinSizeRel")

# set Release as the default build type if it is not yet set.
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release" CACHE STRING
            "Choose the type of build, options are: Debug Release RelWithDebInfo MinSizeRel." FORCE)
endif(NOT CMAKE_BUILD_TYPE)

# Library for the core molecular dynamics simulation logic
add_library(MolSimCore ${CORE_SRC})
target_include_directories(MolSimCore PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-core)

# Library for the file system bindings and input/output operations
add_library(MolSimIO ${IO_SRC})
target_include_directories(MolSimIO PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-io)
target_link_libraries(MolSimIO PUBLIC MolSimCore yaml-cpp::yaml-cpp)

# Executable for the command-line interface application
add_executable(MolSim ${CLI_SRC})
target_link_libraries(MolSim PRIVATE MolSimCore)
target_link_libraries(MolSim PRIVATE MolSimIO)
target_include_directories(MolSim PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-cli)
target_compile_definitions(MolSim PRIVATE PROGRAM_VERSION="${CMAKE_PROJECT_VERSION}")

# Executable for the test runner
add_executable(MolSimTest ${TEST_SRC})
target_link_libraries(MolSimTest gtest gtest_main MolSimCore MolSimIO)
target_include_directories(MolSimTest PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-test)

# # activate all compiler warnings. Clean up your code :P
# # depending on the compiler different flags are used
# target_compile_options(MolSim
#         PRIVATE
#             $<$<CXX_COMPILER_ID:GNU>:-Wall -Wextra -Wpedantic -Werror>
#             # disable some spam warnings for icpc...
#             $<$<CXX_COMPILER_ID:Intel>:-w3 -wd383,981,1418,1572,2259>
# )

# CMake Dependency Configurations
include(doxygen)
include(perf-config)
include(spdlog)
include(test-google)
include(yaml)
include(vtk-config)

# https://cmake.org/cmake/help/latest/module/GoogleTest.html
# do not remove. the issue is that gitub ci uses command `ctest --test-dir
# 'target/cpp'` and could not discover tests without this addition. it also
# breaks when added in test-google for some reason.
include(GoogleTest)
gtest_discover_tests(MolSimTest)
