# Integrates VTK support into the project if enabled

if(ENABLE_VTK_SUPPORT)
    find_package(VTK REQUIRED COMPONENTS CommonCore CommonDataModel IOXML)

    if(VTK_FOUND)
        message (STATUS "VTK: ${VTK_VERSION}")

        if(VTK_VERSION VERSION_GREATER_EQUAL 8.9)
            include_directories(${VTK_INCLUDE_DIRS})
        else()
            include(${VTK_USE_FILE})
        endif ()

        target_compile_definitions(MolSimIO PRIVATE ENABLE_VTK_SUPPORT)
        target_link_libraries(MolSimIO PRIVATE ${VTK_LIBRARIES})
    else ()
        message(FATAL_ERROR "VTK not found. Please install VTK and ensure it is discoverable by CMake, or disable the `ENABLE_VTK_SUPPORT` option.")
    endif ()
endif()
