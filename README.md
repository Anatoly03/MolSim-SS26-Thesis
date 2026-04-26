# MolSim Thesis

[Rust Documentation](https://anatoly03.github.io/MolSim-SS26-Thesis/moldyn_core/index.html)

This project implements molecular dynamics simulations in C++ and Rust. The goal of this workspace is to compare C++ and Rust.

- [`cpp-moldyn-cli`](./cppmoldyn-cli/src/): C++ Executable Workspace
- [`cpp-moldyn-core`](./cppmoldyn-core/src/): C++ Library Workspace
- [`cpp-moldyn-test`](./cppmoldyn-test/src/): C++ Tests
- [`moldyn-cli`](./moldyn-cli/src/): Executable Workspace
- [`moldyn-core`](./moldyn-core/src/): Library Workspace
- [`moldyn-io`](./moldyn-io/src/): Library File System Bindings
- [`moldyn-wasm`](./moldyn-wasm/src/): Molecular Dynamics WebAssembly Bindings
- [`template-cpp`](./template-cpp/): Copy of original Molecular Dynamics template
- [`template-rust`](./template-rust/): Rewrite of template codebase in Rust

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
