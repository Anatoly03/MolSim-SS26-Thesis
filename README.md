# MolSim Thesis

[![Cargo Build & Test](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cargo.yml/badge.svg)](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cargo.yml)
[![C++ Build & Test](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cpp.yml/badge.svg)](https://github.com/Anatoly03/MolSim-SS26-Thesis/actions/workflows/cpp.yml)
[![Rust Documentation](https://badges.ws/badge?icon=rust&value=Rustdoc)](https://anatoly03.github.io/MolSim-SS26-Thesis/moldyn_core/index.html)
[![C++ Doxygen](https://badges.ws/badge?icon=c%2b%2b&value=Doxygen)](https://anatoly03.github.io/MolSim-SS26-Thesis/cpp/index.html)

This project implements molecular dynamics simulations in C++ and Rust. The goal of this workspace is to look at the programming languages in terms of (developer experience of) program architecture design and comparative benchmarking.

- [`cpp-moldyn-cli`](./cppmoldyn-cli/src/): C++ Executable Workspace
- [`cpp-moldyn-core`](./cppmoldyn-core/src/): C++ Library Workspace
- [`cpp-moldyn-io`](./cppmoldyn-io/src/): C++ File System Bindings
- [`cpp-moldyn-test`](./cppmoldyn-test/src/): C++ Tests
- [`moldyn-cli`](./moldyn-cli/src/): Rust Executable Workspace
- [`moldyn-core`](./moldyn-core/src/): Rust Library Workspace
- [`moldyn-io`](./moldyn-io/src/): Rust File System Bindings
- [`moldyn-wasm`](./moldyn-wasm/src/): Rust Molecular Dynamics WebAssembly Bindings
- [`template-cpp`](./template-cpp/): C++ Molecular Dynamics template (Copy, Reformatted)
- [`template-rust`](./template-rust/): Rust Molecular Dynamics template (Rewrite)

## Building & Running Rust

```
cargo build --release
./target/release/moldyn-cli --help
```

## Building & Running C++

```sh
cmake . -B target/cpp
make -C target/cpp -j4 --no-print-directory
./target/cpp/MolSim --help
```

## Benchmarking

```sh
cargo +nightly bench
```

Rust benchmarking is currently done on the nightly channel using the macro [`#[bench]`](https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html).

## Documentation [![Rust Documentation](https://badges.ws/badge?icon=rust&value=Rustdoc)](https://anatoly03.github.io/MolSim-SS26-Thesis/moldyn_core/index.html) [![C++ Documentation](https://badges.ws/badge?icon=c%2b%2b&value=Doxygen)](https://anatoly03.github.io/MolSim-SS26-Thesis/cpp/index.html)

You can find a very detailed code documentation generated with [Rustdoc](https://doc.rust-lang.org/rustdoc/index.html) for the Rust codebase and [Doxygen](https://www.doxygen.nl/index.html) for the C++ codebase. You can build the documentation locally by running the following shell commands.

```sh
# rust documentation only
cargo doc --no-deps --workspace

# c++ documentation only
mkdir target/doc
doxygen Doxyfile

# rust + c++ documentation together
cargo doc --no-deps --workspace
doxygen Doxyfile
```
