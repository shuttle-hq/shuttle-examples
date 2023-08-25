# Standalone binary - run an app with Shuttle or standalone

This example shows how to separate a project's Shuttle logic from its core functionality so that two binaries can be made: one running with Shuttle using `cargo shuttle run` that can be deployed to Shuttle, and one that can be run with `cargo run --bin ...`.

The main idea is to have a main binary that Shuttle runs, and another binary that runs standalone.
All startup logic is placed in the binary source files, while the implementation (endpoints etc) is moved to the library of the crate.

- `src/main.rs` is the main binary with Shuttle, run with `cargo shuttle run`
- `src/standalone.rs` is without Shuttle, run with `cargo run --bin standalone` (you can change the name)

This example shows how to use separate logic for getting secrets, but the same approach can be applied to other resources that are initiated by Shuttle's main function.
