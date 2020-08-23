# Conductor: A cross-platform FRC Driver Station

Conductor is a Driver Station for FRC robots for Linux and macOS which supports nearly<sup>*</sup> every feature of the NI Driver Station.

<sup>*</sup>Conductor cannot yet auto-launch dashboards, and is not and will never be compatible with the Field Management System. 

# Building

Before building, ensure that you have a Rust toolchain (e.g. installed via [Rustup](https://rustup.rs), and [NodeJS](https://nodejs.org) with NPM installed.

Building a release build of Conductor is simple, after cloning the repository run `make setup && make release` to install all the dependencies of the react applications, and then compile both the React apps and the Rust backend into a single executable. When this process is completed, you can find the compiled driver station at `target/release/conductor`. 

# Contributing

Building debug builds is almost as simple, after initially cloning run `make setup && make all` to compile both the react apps and the Rust app once. Afterwards, assuming you're only changing the Rust app you can use `cargo run` to skip the compile time of the react apps. If you're changing the main window as well you can use `make` to only recompile the main window as well as the Rust app. 
