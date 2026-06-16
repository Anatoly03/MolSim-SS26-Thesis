/**
 * @file Macro.h
 * @brief Contains Macro definitions for the project.
 */

#pragma once

#ifdef PERF
    /**
     * Forces method to not be inlined in benchmarking builds.
     * 
     * - **Outlining is enabled.**
     */
    #define outline __declspec(noinline)
#else
    /**
     * Forces method to not be inlined in benchmarking builds.
     * 
     * - **Outlining is disabled.**
     */
    #define outline
#endif
