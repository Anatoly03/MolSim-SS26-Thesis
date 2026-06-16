# Adds commands to compile C++ for `perf` and analytics tools compatibility.
# https://gitlab.kitware.com/cmake/community/-/wikis/FAQ#how-can-i-extend-the-build-modes-with-a-custom-made-one-

# If CMAKE_BUILD_TYPE is `Performance`, then add the following flags to the compiler and linker.
if (CMAKE_BUILD_TYPE STREQUAL "Performance")
    message(STATUS "Configuring for Performance build type")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O2 -g -march=native -flto")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -flto")
    target_compile_definitions(MolSim PRIVATE PERF)
    target_compile_definitions(MolSimCore PRIVATE PERF)
    target_compile_definitions(MolSimIO PRIVATE PERF)
endif()
