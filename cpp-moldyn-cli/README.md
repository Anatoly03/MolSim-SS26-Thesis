# Molsim Thesis

This codebase consists of the molecular dynamics C++ project. It's divided into the following modules.

- `cpp-moldyn-cli`: C++ Executable Workspace
- `cpp-moldyn-core`: C++ Library Workspace
- `cpp-moldyn-test`: C++ Tests

Below is an example how to build the project and print the help message.

```sh
cmake . -B target/cpp
make -C target/cpp -j4 --no-print-directory
./target/cpp/MolSim --help
```
