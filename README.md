# MolSim Thesis

[![C++ Build & Test](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cpp.yml/badge.svg)](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cpp.yml) [![Cargo Build & Test](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cargo.yml/badge.svg)](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cargo.yml) [![Deploy Rustdoc](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/rustdoc.yml/badge.svg)](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/rustdoc.yml)

[Rust Documentation](https://anatoly03.github.io/MolSim-SS26-Thesis/moldyn_core/index.html)

This project implements molecular dynamics simulations in C++ and Rust. The goal of this workspace is to look at the programming languages in terms of (developer experience of) program architecture design and comparative benchmarking.

- [`cpp-moldyn-cli`](./cppmoldyn-cli/src/): C++ Executable Workspace
- [`cpp-moldyn-core`](./cppmoldyn-core/src/): C++ Library Workspace
- [`cpp-moldyn-test`](./cppmoldyn-test/src/): C++ Tests
- [`moldyn-cli`](./moldyn-cli/src/): Rust Executable Workspace
- [`moldyn-core`](./moldyn-core/src/): Rust Library Workspace
- [`moldyn-io`](./moldyn-io/src/): Rust Library File System Bindings
- [`moldyn-wasm`](./moldyn-wasm/src/): Rust Molecular Dynamics WebAssembly Bindings
- [`template-cpp`](./template-cpp/): C++ Molecular Dynamics template (Copy, Reformatted)
- [`template-rust`](./template-rust/): Rust Molecular Dynamics template (Rewrite)

# Building & Running Rust

```
cargo build --release
./target/release/moldyn-cli --help
```

# Building & Running C++

```sh
cmake . -B target/cpp
make -C target/cpp  -j4 --no-print-directory
./target/cpp/MolSim --help
```
