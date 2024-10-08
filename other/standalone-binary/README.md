# Standalone binary - run an app with Shuttle or standalone

This example shows how to separate a project's Shuttle logic from its core functionality so that two binaries can be made: one for running with `shuttle run` and deploying to Shuttle, and one that can be run with `cargo run --bin ...`.

All startup logic is placed in the binary source files, while the implementation (endpoints etc) is moved to the library of the crate.

- `src/bin/shuttle.rs` is the main binary with Shuttle, run with `shuttle run`. Note that the `[[bin]]` entry in `Cargo.toml` needs to have the same name as the crate. The file can have any name you want.
- `src/bin/standalone.rs` is without Shuttle, run with `cargo run --bin standalone` (you can change the name)

This example shows how to use separate logic for getting secrets (Shuttle secrets vs homemade solution), but the same approach can be applied to other resources that are initiated by Shuttle's main function.
