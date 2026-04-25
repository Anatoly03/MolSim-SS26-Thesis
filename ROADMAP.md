# Chapter 1

- [x] Rust Template
- [x] CLI Argument Parser
- [x] Input File Parser
  - [x] YAMl Support
  - [ ] JSON Support
- [ ] Output File Writer

***

> The following bullet points have not been inserted into a proper timeline.

- [ ] As we discussed in the meeting, the basic algorithm of the molecular dynamics
simulation consists of the following steps:
  - [ ] Force calculation
  - [ ] Calculation of the new position according to these forces.
  - [ ] Calculation of the new velocities according to these forces
- [ ] Complete the steps of the simulation in the program frame.
- [ ] Create VTK output for visualization with the VTKWriter class.
- [ ] Pass the parameters `t_end` and `delta_t` via the command line

- [ ] Run the simulation from the input file eingabe-sonne.txt. As simulation parameters use at least the following:
  - `delta_t = 0.014`, `t_end = 1000`
- [ ] Visualize the particles in Paraview (e.g. with a glyph filter ).
- [ ] Which particle represents which celestial body?

- [ ] Documentation
- [ ] Clippy

- [ ] Write at least one simple unit test for the particle container you created in sheet 1.
- [ ] Always briefly state the idea of each test in a comment above it! E.g. : “Check correctness of force calculation against hand-calculated values”

- [ ] GitHub CI
- [ ] Further expand your CI pipeline to check your unit tests pass for every pull request to master.

- [ ] Logging
  - Standard Library best for logging https://github.com/rust-lang/log
- [ ] The user should be able to select the log level either via any form of input or through CMake. Having to edit any source file to change the log level is not acceptable.

- [ ] TODO: include worksheet 2 task 4

- [ ] TODO: include worksheet 3

- [ ] SIMD
  - https://doc.rust-lang.org/std/simd/struct.Simd.html

- [ ] Profilers
  - https://nnethercote.github.io/perf-book/profiling.html
  - https://valgrind.org/docs/manual/cg-manual.html

- [ ] Benchmarking
  - https://nnethercote.github.io/perf-book/benchmarking.html